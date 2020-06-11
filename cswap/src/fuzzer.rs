use crate::ast::Script;
use crate::config::Config;
use crate::config::FileProvider;
use crate::config::Metadata;
use crate::parser::script_from_f;
use crate::parser::script_from_f_unsanitized;
use crate::solver::profiles_solve;
use crate::solver::RSolve;
use crate::transforms::rv;
use crate::utils::dyn_fmt;
use crate::utils::to_strs;
use crate::utils::DFormatParseError;
use crate::utils::MyBackoff;
use crate::utils::RandUniqPermGen;

use std::fs::OpenOptions;
use std::io;
use walkdir::WalkDir;

use crate::ast::SExp;
use crate::transforms::{end_insert_pt, get_bav_assign_fmt_str, ba_script, replace_constants_with_fresh_vars};

use crate::liftio;
use crate::liftio_e;
use bit_vec::BitVec;
use log::debug;
use log::trace;
use log::warn;
use rand::Rng;
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

    for f in q.into_iter() {
        match mutator(f.clone(), &cfg.file_provider)
            .and_then(|skel| group_bav_assign(skel, &cfg))
            .map(|iters| {
                for iter in iters.into_iter() {
                    solver_fn(iter, &cfg);
                }
            }) {
            Err(e) => warn!("{} in file {:?}", e, f),
            _ => (),
        };
    }
}

pub fn mutator(filepath: PathBuf, file_provider: &FileProvider) -> io::Result<(PathBuf, PathBuf)> {
    let (script, mut md) = strip_and_transform(&filepath, &file_provider)?;
    let (skelf, mdf) = file_provider.serialize_skel_and_md(&script, &mut md)?;
    Ok((skelf.to_path_buf(), mdf.to_path_buf()))
}

pub struct StatefulBavAssign<'a> {
    top_of_script: Box<String>,
    bav_fmt_string: Box<String>,
    bottom_of_script: Box<String>,
    md: Metadata,
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

        let results = profiles_solve(empty_skel_fstr, &cfg.profiles);

        report_any_bugs(&filepaths.0, &results, &cfg.file_provider);

        if results.iter().all(|r| r.has_unrecoverable_error()) {
            fs::remove_file(&filepaths.0).unwrap_or(());
            fs::remove_file(&filepaths.1).unwrap_or(());
            return liftio!(Err(format!("All error on file {:?}", filepaths.0)));
        }

        let (top, fmt_str, bottom) = Self::split(script, &md.bavns);
        let num_bavs = md.bavns.len();

        let urng = RandUniqPermGen::new_definite_seeded(
            cfg.get_specific_seed(&md.seed_file),
            num_bavs,
            cfg.max_iter,
        );

        Ok(StatefulBavAssign {
            top_of_script: top,
            bav_fmt_string: fmt_str,
            bottom_of_script: bottom,
            md: md,
            urng: urng,
            cfg: cfg,
        })
    }

    fn split(script: Script, bavns: &Vec<String>) -> (Box<String>, Box<String>, Box<String>) {
        let eip = end_insert_pt(&script);
        let (top, bottom) = Script::split(script, eip);
        let fmt_str = Box::new(format!("{}\n", get_bav_assign_fmt_str(bavns)));
        (
            Box::new(top.to_string()),
            fmt_str,
            Box::new(bottom.to_string()),
        )
    }

    pub fn do_iteration_tv(&mut self, truth_values: BitVec) -> io::Result<(PathBuf, PathBuf)> {
        let new_file: PathBuf = self.cfg.file_provider.iterfile(&self.md)?;
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&new_file)?;

        let str_with_model = liftio!(dyn_fmt(&self.bav_fmt_string, to_strs(&truth_values))
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

fn group_bav_assign(
    filepaths: (PathBuf, PathBuf),
    cfg: &Config,
) -> io::Result<Vec<(PathBuf, PathBuf)>> {
    let mut sba = StatefulBavAssign::new(filepaths, cfg)?;
    let mut fs = vec![];
    while let Some(tv) = sba.urng.sample() {
        fs.push(sba.do_iteration_tv(tv)?);
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

fn report_any_bugs(file: &Path, results: &Vec<RSolve>, fp: &FileProvider) -> bool {
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
            fp.bug_report(file, &format!("{:?}", results));
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

pub fn solver_fn(filepaths: (PathBuf, PathBuf), cfg: &Config) {
    let results = profiles_solve(filepaths.0.to_str().unwrap_or("defaultname"), &cfg.profiles);

    results
        .iter()
        .filter(|r| r.has_sat())
        .for_each(|r| match resub_model(r, &filepaths, &cfg) {
            Ok(()) => (),
            Err(e) => warn!("Resub error {} for file {:?}", e, filepaths.0),
        } ); 

    if !report_any_bugs(filepaths.0.as_path(), &results, &cfg.file_provider) {
        fs::remove_file(&filepaths.0).unwrap_or(());
    }
}

fn resub_model(result: &RSolve, filepaths: &(PathBuf, PathBuf), cfg: &Config) -> io::Result<()> {
    trace!("RESUB! for file {:?}", filepaths.0);
    let md: Metadata = liftio!(serde_lexpr::from_str(&fs::read_to_string(&filepaths.1)?))?;
    let mut choles_script = script_from_f_unsanitized(&md.choles_path(&cfg.file_provider))?;

    let mut to_replace: Vec<(String, SExp)> = result
        .extract_const_var_vals(&md.constvns)
        .into_iter()
        .map(|(sym, val)| (sym.to_string(), val.clone()))
        .collect();

    if to_replace.len() > 0 {
        rv(&mut choles_script, &mut to_replace);

        let resubbed_f = cfg.file_provider.serialize_resub(&choles_script, &md)?;

        let results = profiles_solve(resubbed_f.to_str().unwrap_or("defaultname"), &cfg.profiles);

        if !report_any_bugs(&resubbed_f, &results, &cfg.file_provider) {
            fs::remove_file(&resubbed_f).unwrap_or(());
        }
    }
    Ok(())
}

pub fn strip_and_transform(source_file: &Path, fp: &FileProvider) -> io::Result<(Script, Metadata)> {
    let mut script = script_from_f(source_file)?;

    if script.is_unsupported_logic() {
        return liftio!(Err("Unsupported Logic"));
    }

    let mut md = Metadata::new(source_file);

    replace_constants_with_fresh_vars(&mut script, &mut md);
    let chf = fp.cholesfile(&mut md)?;
    fs::write(chf, script.to_string())?;
    
    let ba_script = ba_script(&mut script, &mut md)?;

    return Ok((ba_script, md));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::script;
    use crate::parser::script_from_f_unsanitized;
    use bit_vec::BitVec;
    use insta::assert_debug_snapshot;
    use std::collections::HashSet;
    use tempfile::TempDir;

    #[test]
    fn bav_stateful() {
        let script =
            script("(decl-const BAV1 bool)(assert (< x 5))(assert (= BAV1 (< x 5)))(check-sat)")
                .unwrap()
                .1;
        let (top, mid, bottom) =
            StatefulBavAssign::split(script, &vec!["BAV1".to_string(), "BAV2".to_string()]);
        let num_bavs = 2;
        let urng = RandUniqPermGen::new_definite_seeded(1, num_bavs, 1);
        let md = Metadata::new_empty();
        let base = TempDir::new().unwrap();
        let mut fpdir = base.path().to_path_buf();
        fpdir.push("test");

        let cfg = Config {
            file_provider: FileProvider::new(fpdir.to_str().unwrap()),
            max_iter: 1,
            rng_seed: 1,
            stack_size: 1,
            profiles: HashSet::new(),
        };

        let mut sba = StatefulBavAssign {
            top_of_script: top,
            bav_fmt_string: mid,
            bottom_of_script: bottom,
            md: md,
            urng: urng,
            cfg: &cfg,
        };

        let tv = sba.urng.sample().unwrap();
        let (f, _) = sba.do_iteration_tv(tv).unwrap();
        let result = fs::read_to_string(f).unwrap();
        assert_debug_snapshot!(result);
    }

    #[test]
    fn bv_replace() {
        let fmt_str = get_bav_assign_fmt_str(&vec!["BAV1".to_owned()]).to_string();

        let bv = BitVec::from_elem(1, true);
        assert_eq!(
            dyn_fmt(&fmt_str, to_strs(&bv)).unwrap(),
            "(assert (= BAV1 true))"
        );
    }
}
