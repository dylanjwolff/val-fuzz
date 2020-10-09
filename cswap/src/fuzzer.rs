use crate::ast::CommandRc;
use crate::ast::Script;
use crate::ast::Sort;
use crate::config::Config;
use crate::config::FileProvider;
use crate::config::Metadata;
use crate::parser::script_from_f;
use crate::parser::script_from_f_unsanitized;
use crate::solver::check_valid_solve;
use crate::solver::profiles_solve;
use crate::solver::randomized_profiles_solve;
use crate::solver::ProfileIndex;
use crate::solver::RSolve;
use crate::transforms::rv;
use crate::utils::dyn_fmt;
use crate::utils::to_strs;
use crate::utils::DFormatParseError;
use crate::utils::HashHashSet;
use crate::utils::RunStats;
use std::collections::HashMap;

use crate::utils::RandUniqPermGen;
use std::hash::Hash;

use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io;
use walkdir::WalkDir;

use crate::ast::BoolOp;
use crate::ast::SExp;
use crate::transforms::{
    ba_script, end_insert_pt, get_bav_assign_fmt_str, many_assert,
    replace_constants_with_fresh_vars,
};
use std::collections::HashSet;

use crate::liftio;
use crate::liftio_e;
use crate::solver::all_non_err_timed_out;
use bit_vec::BitVec;
use log::debug;
use log::trace;
use log::warn;

use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub fn exec_single_thread(dirname: &str, cfg: Config) {
    let mut q = vec![];
    for entry in WalkDir::new(dirname)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let filepath = entry.into_path();
        debug!("push {:?}", filepath);
        q.push(filepath);
    }

    let mut stats = RunStats::new();
    let mut to_ctr = BTreeMap::new();
    for f in q.into_iter() {
        match mutator(f.clone(), &cfg)
            .map(|skels| {
                skels
                    .into_iter()
                    .flat_map(|skel| single_thread_group_bav_assign(skel, &cfg))
                    .flat_map(|skelfs| skelfs)
                    .collect::<Vec<(PathBuf, PathBuf)>>()
            })
            .map(|iters| {
                for iter in iters.into_iter() {
                    match solver_fn(iter, &mut to_ctr, &mut stats, vec![], &cfg) {
                        Err(e) => warn!("Solver Error: {} in file {:?}", e, f),
                        _ => (),
                    };
                }
            }) {
            Err(e) => warn!("{} in file {:?}", e, f),
            _ => (),
        };
    }
}

pub fn mutator(filepath: PathBuf, cfg: &Config) -> io::Result<Vec<(PathBuf, PathBuf)>> {
    let (scripts, mut md) = strip_and_transform(&filepath, cfg)?;

    let mut mutatedfs = vec![];
    for script in scripts {
        let (skelf, mdf) = cfg.file_provider.serialize_skel_and_md(&script, &mut md)?;
        mutatedfs.push((skelf.to_path_buf(), mdf.to_path_buf()));
    }
    Ok(mutatedfs)
}

pub struct StatefulBavAssign<'a> {
    top_of_script: Box<String>,
    bottom_of_script: Box<String>,
    pub md: Metadata,
    pub urng: RandUniqPermGen,
    cfg: &'a Config,
}

impl<'a> StatefulBavAssign<'a> {
    pub fn new(
        filepaths: (PathBuf, PathBuf),
        cfg: &'a Config,
    ) -> io::Result<StatefulBavAssign<'a>> {
        let (script, md) = deserialize_from_f(&filepaths)?;
        trace!("Validating skeleton for {}", md.seed_file);

        let e = liftio_e!(Err::<(), &str>("Filename not a str"));
        let empty_skel_fstr = &filepaths.0.to_str().ok_or(e)?;

        let results = check_valid_solve(empty_skel_fstr);

        report_any_bugs(&filepaths.0, &results, &cfg.file_provider);

        if results
            .iter()
            .all(|r| r.has_unrecoverable_error() && !r.has_bug_error())
        {
            if cfg.remove_files {
                fs::remove_file(&filepaths.0).unwrap_or(());
                fs::remove_file(&filepaths.1).unwrap_or(());
            }

            let mut stresults = results
                .iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
                .join("\n----------\n");

            stresults = format!("============== \n{}\n =============", stresults);

            let errmsg = format!("All error on file {:?}", filepaths.0);
            warn!("{}\n{}", errmsg, stresults);
            return liftio!(Err(errmsg));
        }

        if results
            .iter()
            .filter(|r| !r.has_unrecoverable_error())
            .any(|r| r.has_unsat() && !r.has_sat() && !r.has_unknown())
        {
            return liftio!(Err("File has unsat skeleton!"));
        }

        let (top, bottom) = Self::split(script);
        let num_bavs = md.bavns.len();

        let urng = RandUniqPermGen::new_masked_with_retries(
            cfg.get_specific_seed(&md.seed_file),
            num_bavs,
            cfg.max_iter,
            cfg.mask_size,
        );

        Ok(StatefulBavAssign {
            top_of_script: top,
            bottom_of_script: bottom,
            md: md,
            urng: urng,
            cfg: cfg,
        })
    }

    fn split(script: Script) -> (Box<String>, Box<String>) {
        let eip = end_insert_pt(&script);
        let (top, bottom) = Script::split(script, eip);
        (Box::new(top.to_string()), Box::new(bottom.to_string()))
    }

    pub fn do_iteration_tv(
        &mut self,
        i: u64,
        truth_values: BitVec,
        mask: BitVec,
    ) -> io::Result<(PathBuf, PathBuf)> {
        let new_file: PathBuf = self.cfg.file_provider.iterfile(i, &self.md)?;
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&new_file)?;

        let bav_unenforced = get_bav_assign_fmt_str(&self.md.bavns);
        let bav_uef_filtered = bav_unenforced
            .into_iter()
            .zip(mask.iter())
            .filter_map(|(cmd, mbit)| if mbit { Some(cmd) } else { None })
            .collect::<Vec<CommandRc>>();

        let num_remaining = bav_uef_filtered.len();
        let bav_fmt_string = Script::Commands(bav_uef_filtered).to_string();

        assert!(
            num_remaining == truth_values.len(),
            format!(
                "fmt str size {:?} != {:?} num truth vals from mask {:?}",
                bav_fmt_string,
                truth_values.len(),
                mask,
            )
        );
        assert!(
            mask.iter().fold(0, |a, b| if b { a + 1 } else { a }) == truth_values.len(),
            "Mask size != num truth vals"
        );
        let str_with_model =
            liftio!(dyn_fmt(&bav_fmt_string, to_strs(&truth_values))
                .map_err(|e| slim_dynfmt_err_msg(e)))?;

        f.write_all(self.top_of_script.as_bytes())?;
        f.write_all(str_with_model.as_bytes())?;
        f.write_all(self.bottom_of_script.as_bytes())?;

        Ok((
            new_file.to_path_buf(),
            self.md.md_path(&self.cfg.file_provider),
        ))
    }
}

fn single_thread_group_bav_assign(
    filepaths: (PathBuf, PathBuf),
    cfg: &Config,
) -> io::Result<Vec<(PathBuf, PathBuf)>> {
    let mut sba = StatefulBavAssign::new(filepaths, cfg)?;
    let mut fs = vec![];
    let mut i = 0;

    while let Some((tv, mask)) = sba.urng.sample_and_mask() {
        let _ef = sba
            .md
            .bavns
            .iter()
            .zip(mask.iter())
            .filter_map(|((name, _sort), mbit)| if mbit { Some(name) } else { None })
            .zip(tv.iter())
            .collect::<Vec<(&String, bool)>>();

        fs.push(sba.do_iteration_tv(i, tv, mask)?);
        i = i + 1;
    }
    Ok(fs)
}
fn slim_dynfmt_err_msg(e: DFormatParseError) -> String {
    match e {
        nom::Err::Incomplete(n) => format!("Incomplete parse on Dynamic Fmt; needed {:?}", n),
        nom::Err::Error(ec) => format!("Dynamic format parse error of kind {:?}", ec.1),
        nom::Err::Failure(ec) => format!("Dynamic format parse failure of kind {:?}", ec.1),
    }
}

pub fn report_any_bugs(file: &Path, results: &Vec<RSolve>, fp: &FileProvider) -> bool {
    results
        .iter()
        .find(|r| r.has_bug_error())
        .map(|r| {
            debug!("Reporting bug for file {:?}", file);
            fp.bug_report(file, &format!("{}", r));
        })
        .is_some()
        || if !RSolve::differential_test(&results).is_ok() {
            debug!("Reporting soundness bug for file {:?}", file);
            fp.bug_report(file, &format!("SOUNDNESS BUG: {:?}", results));
            true
        } else {
            false
        }
}

fn deserialize_from_f(
    (script_file, md_file): &(PathBuf, PathBuf),
) -> io::Result<(Script, Metadata)> {
    trace!("Reading script from {:?}", script_file);
    let script = script_from_f_unsanitized(script_file)?;

    trace!("Reading md from {:?}", md_file);
    let md_contents = fs::read_to_string(&md_file)?;
    let md: Metadata = liftio!(serde_lexpr::from_str(&md_contents))?;

    Ok((script, md))
}

lazy_static! {
    static ref SIMPLE_PROFILE: HashSet<ProfileIndex> =
        vec![ProfileIndex::Z3(0), ProfileIndex::CVC4(1)]
            .into_iter()
            .collect();
}
pub fn solver_fn(
    filepaths: (PathBuf, PathBuf),
    to_ctr: &mut BTreeMap<String, u64>,
    stats: &mut RunStats,
    enforcement: Vec<(String, bool)>,
    cfg: &Config,
) -> io::Result<()> {
    let (iter_file, md_file) = &filepaths;
    let mdstr = fs::read_to_string(md_file)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("File {:?} {:?}", md_file, e)))?;
    let md: Metadata = liftio!(serde_lexpr::from_str(&mdstr))?;
    if to_ctr.get(&md.seed_file).map(|sf| *sf > 3).unwrap_or(false) {
        fs::remove_file(md_file).unwrap_or(()); // TODO Hack! Prevents other threads from using the file
        return liftio!(Err(format!("{:?} exceeded max consec timeouts", iter_file)));
    }

    let seed = cfg.get_specific_seed(iter_file);
    let results = randomized_profiles_solve(
        filepaths.0.to_str().unwrap_or("defaultname"),
        &SIMPLE_PROFILE,
        seed,
        cfg.timeout,
    );

    let ct = to_ctr.entry(md.seed_file.clone()).or_insert(0);
    if all_non_err_timed_out(&results) {
        *ct = *ct + 1;
    } else if *ct > 0 {
        *ct = *ct - 1;
    }

    stats.record_iter();
    stats.record_stats_for_iter_results(&results);

    results.iter().filter(|r| r.has_sat()).for_each(|r| {
        match resub_model(r, &filepaths, &md, stats, &enforcement, &cfg) {
            Ok(()) => (),
            Err(e) => warn!("Resub error {} for file {:?}", e, iter_file),
        }
    });

    if !report_any_bugs(filepaths.0.as_path(), &results, &cfg.file_provider) {
        if cfg.remove_files {
            fs::remove_file(iter_file).unwrap_or(());
        }
    }
    Ok(())
}

fn resub_model(
    result: &RSolve,
    filepaths: &(PathBuf, PathBuf),
    md: &Metadata,
    stats: &mut RunStats,
    enforcemt: &Vec<(String, bool)>,
    cfg: &Config,
) -> io::Result<()> {
    debug!("RESUB! for file {:?}", filepaths.0);

    let f_to_replace = match md.ogwms_path(&cfg.file_provider) {
        Some(ogwms_f) => ogwms_f,
        None => md.choles_path(&cfg.file_provider),
    };

    let mut choles_script = script_from_f_unsanitized(&f_to_replace)?;
    // add enforcement here
    if cfg.enforce_on_resub {
        assert!(
            cfg.monitors_in_final,
            "Must have monitors in final to enforce resub!"
        );
        let efcmt_cmds = many_assert(&mut enforcemt.iter().map(|(name, val)| {
            SExp::BExp(
                rccell!(BoolOp::Equals()),
                vec![rccell!(SExp::var(&name)), rccell!(SExp::bool_sexp(*val))],
            )
        }));
        choles_script.insert_all(end_insert_pt(&choles_script), &efcmt_cmds);
    }

    let mut to_replace: Vec<(String, SExp)> = result
        .extract_const_var_vals(&md.constvns.iter().map(|c| c.0.clone()).collect())
        .into_iter()
        .map(|(sym, val)| (sym.to_string(), val.clone()))
        .collect();

    if to_replace.len() > 0 {
        rv(&mut choles_script, &mut to_replace);

        let script_str = choles_script.to_string();
        if !stats.record_sub(&script_str) {
            return Ok(()); // Reporting this as an error is probably a bit too noisy
        }

        let resubbed_f = cfg
            .file_provider
            .serialize_resub_str(script_str, &filepaths.0, &md)?;

        let results = profiles_solve(
            resubbed_f.to_str().unwrap_or("defaultname"),
            &cfg.profiles,
            cfg.timeout,
        );
        stats.record_stats_for_sub_results(&results);

        log_check_enforce(&results, enforcemt);
        stats
            .coverage
            .log_check_coverage(&results, &md.bavns, md.seed_file.clone());

        if !report_any_bugs(&resubbed_f, &results, &cfg.file_provider) {
            if cfg.remove_files {
                fs::remove_file(&resubbed_f).unwrap_or(());
            }
        }
    }
    Ok(())
}

/// Exit on first success, only report if no successes and at least one failure
fn log_check_enforce(results: &Vec<RSolve>, enforcemt: &Vec<(String, bool)>) {
    let (enames, _evals): (Vec<_>, Vec<_>) = enforcemt.iter().cloned().unzip();

    let strress = results
        .iter()
        .flat_map(|result| {
            result
                .extract_const_var_vals(&enames)
                .into_iter()
                .map(|(sym, sexp)| (sym.to_string(), sexp.to_string()))
        })
        .collect::<Vec<(String, String)>>(); // TODO clean up (no string comp)

    enforcemt.iter().for_each(|(ename, eval)| {
        let filtered = strress
            .iter()
            .filter(|(name, _strval)| *name == *ename)
            .collect::<Vec<&(String, String)>>();
        if filtered.len() > 0 {
            match filtered
                .iter()
                .find(|(_, strval)| eval.to_string() == *strval)
            {
                Some(_) => trace!("SUCCESS ENFORCING {} to be {}", ename, eval),
                None => trace!("FAILURE ENFORCING {} to be {}", ename, eval),
            }
        }
    });
}

pub fn strip_and_transform(
    source_file: &Path,
    cfg: &Config,
) -> io::Result<(Vec<Script>, Metadata)> {
    let mut script = script_from_f(source_file)?;

    let og_f_results = check_valid_solve(&liftio!(source_file.to_str().ok_or(
        Err::<&str, String>(format!("Non-str Filename: {:?}!", source_file))
    ))?);

    if og_f_results
        .iter()
        .all(|r| r.has_unrecoverable_error() && !r.has_bug_error())
    {
        return liftio!(Err(format!(
            "Unmodified file {:?} failed to pass initial validation check",
            source_file
        )));
    }

    let mut md = Metadata::new(source_file);

    replace_constants_with_fresh_vars(&mut script, &mut md, cfg)?;
    let chf = cfg.file_provider.cholesfile(&mut md)?;
    fs::write(chf, script.to_string())?;

    let ba_script = ba_script(&mut script, &mut md, cfg)?;
    if cfg.monitors_in_final {
        cfg.file_provider.serialize_og_w_m(&script, &mut md)?;
    }

    return Ok((ba_script, md));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::Sort;
    use crate::parser::script;

    use bit_vec::BitVec;

    use insta::assert_display_snapshot;
    use std::collections::HashSet;
    use tempfile::TempDir;

    #[test]
    fn ba_script_num() {
        let mut script = script("(declare-fun z () Real)(assert (= 3 (+ z 4)))")
            .unwrap()
            .1;
        let mut md = Metadata::new_empty();
        let cfg = Config::default();
        replace_constants_with_fresh_vars(&mut script, &mut md, &cfg).unwrap();
        let new_script = &ba_script(
            &mut script,
            &mut md,
            &Config {
                use_bdom_vs: true,
                max_const_relations_to_monitor: 2,
                ..Config::default()
            },
        )
        .unwrap()[0];
        assert_display_snapshot!(new_script);
    }

    #[test]
    fn ba_script_fpop() {
        let mut script = script("(declare-const mpfx (_ FloatingPoint 11 53))(assert (= mpfx (fp.mul roundTowardPositive
            ((_ to_fp 11 53) roundNearestTiesToEven 0.5792861143265499723753464422770775854587554931640625 (- 1022))
            ((_ to_fp 11 53) roundNearestTiesToEven 1.3902774452208657152141313417814671993255615234375 (- 17)))))").unwrap().1;
        let mut md = Metadata::new_empty();
        let cfg = Config::default();
        replace_constants_with_fresh_vars(&mut script, &mut md, &cfg).unwrap();
        let new_script = &ba_script(
            &mut script,
            &mut md,
            &Config {
                use_bdom_vs: true,
                ..Config::default()
            },
        )
        .unwrap()[0];
        assert_display_snapshot!(new_script);
    }

    #[test]
    fn bav_stateful() {
        let script =
            script("(decl-const BAV1 bool)(assert (< x 5))(assert (= BAV1 (< x 5)))(check-sat)")
                .unwrap()
                .1;
        let bavns = vec![
            ("BAV1".to_string(), Sort::Bool()),
            ("BAV2".to_string(), Sort::Bool()),
        ];
        let (top, bottom) = StatefulBavAssign::split(script);
        let num_bavs = 2;
        let urng = RandUniqPermGen::new_definite_seeded(1, num_bavs, 1);
        let mut md = Metadata::new_empty();
        md.bavns = bavns;
        let base = TempDir::new().unwrap();
        let mut fpdir = base.path().to_path_buf();
        fpdir.push("test");

        let cfg = Config {
            file_provider: FileProvider::new(fpdir.to_str().unwrap()),
            max_iter: 1,
            rng_seed: 1,
            remove_files: true,
            ..Config::default()
        };

        let mut sba = StatefulBavAssign {
            top_of_script: top,
            bottom_of_script: bottom,
            md: md,
            urng: urng,
            cfg: &cfg,
        };

        let (tv, mask) = sba.urng.sample_and_mask().unwrap();
        let (f, _) = sba.do_iteration_tv(0, tv, mask).unwrap();
        let result = fs::read_to_string(f).unwrap();
        assert_display_snapshot!(result);
    }

    #[test]
    fn bv_replace() {
        let fmt_str = Script::Commands(get_bav_assign_fmt_str(&vec![(
            "BAV1".to_owned(),
            Sort::Bool(),
        )]))
        .to_string();

        let bv = BitVec::from_elem(1, true);
        assert_eq!(
            dyn_fmt(&fmt_str, to_strs(&bv)).unwrap().trim(),
            "(assert (= BAV1 true))"
        );
    }
}
