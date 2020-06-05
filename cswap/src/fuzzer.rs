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
use crate::utils::MyBackoff;
use crate::utils::RandUniqPermGen;
use crate::utils::StageComplete;
use crate::utils::Timer;
use walkdir::WalkDir;

use crate::ast::SExp;
use crate::transforms::{end_insert_pt, get_bav_assign_fmt_str, to_skel};

use rand::Rng;

use crate::solver::solve;

use std::fs;
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
        println!("push {:?}", filepath);
        q.push(filepath);
    }

    for f in q.into_iter() {
        mutator(f, &cfg.file_provider)
            .and_then(|skel| bav_assign(skel, &cfg))
            .map(|iters| {
                for iter in iters.into_iter() {
                    solver_fn(iter, &cfg);
                }
            });
    }
}

pub fn mutator(filepath: PathBuf, file_provider: &FileProvider) -> Option<(PathBuf, PathBuf)> {
    let (script, mut md) = strip_and_transform(&filepath)?;
    let (mdf, skelf) = file_provider.serialize_skel_and_md(&script, &mut md).ok()?;
    Some((skelf.to_path_buf(), mdf.to_path_buf()))
}

pub fn bav_assign(filepaths: (PathBuf, PathBuf), cfg: &Config) -> Option<Vec<(PathBuf, PathBuf)>> {
    let (script, md) = deserialize_from_f(&filepaths).ok()?;
    let empty_skel_fstr = &filepaths.0.to_str()?;

    let results = profiles_solve(empty_skel_fstr, &cfg.profiles);

    report_any_bugs(&filepaths.0, &results, &cfg.file_provider);

    if results.iter().all(|r| r.has_unrecoverable_error()) {
        println!("All err on file {}", empty_skel_fstr);
        fs::remove_file(filepaths.0).unwrap_or(());
        fs::remove_file(filepaths.1).unwrap_or(());
        return Some(vec![]);
    } else {
        return do_iterations(script, md, &cfg);
    }
}

fn do_iterations(
    mut script: Script,
    mut md: Metadata,
    cfg: &Config,
) -> Option<Vec<(PathBuf, PathBuf)>> {
    let mut backoff = MyBackoff::new();

    let eip = end_insert_pt(&script);
    script.init(eip);
    script.replace(eip, get_bav_assign_fmt_str(&md.bavns));
    let script_str = script.to_string();

    let num_bavs = md.bavns.len();

    let mut urng = RandUniqPermGen::new_definite_seeded(
        cfg.get_specific_seed(&md.seed_file),
        num_bavs,
        cfg.max_iter,
    );

    let mut fs = vec![];
    while let Some(truth_values) = urng.sample() {
        let new_file = cfg.file_provider.iterfile(&mut md).ok()?;
        let str_with_model = dyn_fmt(&script_str, to_strs(&truth_values)).ok()?;
        fs::write(&new_file, str_with_model).ok()?;
        fs.push((new_file.to_path_buf(), md.md_path(&cfg.file_provider)));
    }
    Some(fs)
}

fn report_any_bugs(file: &Path, results: &Vec<RSolve>, fp: &FileProvider) -> bool {
    results
        .iter()
        .find(|r| r.has_bug_error())
        .map(|r| {
            fp.bug_report(file, &format!("{}", r));
        })
        .is_some()
        || if !RSolve::differential_test(&results).is_ok() {
            fp.bug_report(file, &format!("{:?}", results));
            true
        } else {
            false
        }
}

fn deserialize_from_f(
    (script_file, md_file): &(PathBuf, PathBuf),
) -> Result<(Script, Metadata), String> {
    let script = script_from_f_unsanitized(script_file)?;

    let md_contents =
        fs::read_to_string(&md_file).map_err(|e| e.to_string() + " from metadata IO")?;
    let md: Metadata =
        serde_lexpr::from_str(&md_contents).map_err(|e| e.to_string() + " from serde")?;

    Ok((script, md))
}

pub fn solver_fn(filepaths: (PathBuf, PathBuf), cfg: &Config) {
    let results = profiles_solve(filepaths.0.to_str().unwrap_or("defaultname"), &cfg.profiles);

    results
        .iter()
        .filter(|r| r.has_sat())
        .for_each(|r| resub_model(r, &filepaths, &cfg));

    if !report_any_bugs(filepaths.0.as_path(), &results, &cfg.file_provider) {
        fs::remove_file(&filepaths.0).unwrap_or(());
    }
}

fn resub_model(result: &RSolve, filepaths: &(PathBuf, PathBuf), cfg: &Config) {
    let (mut script, md) = match deserialize_from_f(&filepaths) {
        Ok(deserial) => deserial,
        Err(e) => {
            println!("solver deserial err from {:?}: {}", filepaths, e);
            return;
        }
    };

    let mut to_replace: Vec<(String, SExp)> = result
        .extract_const_var_vals(&md.constvns)
        .into_iter()
        .map(|(sym, val)| (sym.to_string(), val.clone()))
        .collect();

    if to_replace.len() > 0 {
        rv(&mut script, &mut to_replace);

        let resubbed_f = match cfg.file_provider.serialize_resub(&script, &md) {
            Ok(f) => f,
            Err(e) => {
                println!("Resub file name err {}", e);
                return;
            }
        };

        let results = profiles_solve(resubbed_f.to_str().unwrap_or("defaultname"), &cfg.profiles);

        if !report_any_bugs(&resubbed_f, &results, &cfg.file_provider) {
            fs::remove_file(&resubbed_f).unwrap_or(());
        }
    }
}

pub fn strip_and_transform(source_file: &Path) -> Option<(Script, Metadata)> {
    let mut script = script_from_f(source_file).ok()?;
    // TODO error handling here on prev 3 lines

    if script.is_unsupported_logic() {
        return None;
    }

    let mut md = Metadata::new(source_file);
    to_skel(&mut script, &mut md).ok()?;
    return Some((script, md));
}

#[cfg(test)]
mod test {
    use super::*;
    use bit_vec::BitVec;

    #[test]
    fn bv_replace() {
        let fmt_str = get_bav_assign_fmt_str(&vec!["BAV1".to_owned()]).to_string();
        println!("{}", fmt_str);
        let bv = BitVec::from_elem(1, true);
        assert_eq!(
            dyn_fmt(&fmt_str, to_strs(&bv)).unwrap(),
            "(assert (= BAV1 true))"
        );
    }
}
