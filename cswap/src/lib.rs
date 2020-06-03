extern crate nom;
extern crate rand;
extern crate rand_core;
extern crate rand_xoshiro;
extern crate serde;
extern crate serde_lexpr;
extern crate tempfile;
extern crate walkdir;
#[macro_use]
extern crate lazy_static;
#[macro_use]
pub mod ast;

pub mod config;
pub mod parser;
pub mod solver;
pub mod transforms;
pub mod utils;

use crate::solver::RSolve;
use crate::transforms::rv;
use crate::utils::StageComplete;
use ast::Script;
use bit_vec::BitVec;
use config::Config;
use config::FileProvider;
use config::Metadata;
use crossbeam::queue::ArrayQueue;
use crossbeam::queue::PushError;
use crossbeam::queue::SegQueue;
use crossbeam::utils::Backoff;
use std::io;
use utils::dyn_fmt;
use utils::to_strs;
use utils::MyBackoff;
use utils::RandUniqPermGen;
use utils::Timer;

use crate::ast::SExp;
use crate::ast::Symbol;
use nom::{bytes::complete::tag, IResult};
use parser::{rmv_comments, script};
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use solver::check_valid_solve;
use solver::solve;
use solver::SolveResult;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use transforms::{end_insert_pt, get_bav_assign_fmt_str, to_skel};
use walkdir::WalkDir;
#[macro_use]
use serde::{Serialize, Deserialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tempfile::tempfile;
use tempfile::Builder;
use tempfile::NamedTempFile;

type InputPPQ = Arc<SegQueue<Result<PathBuf, PoisonPill>>>;
type SkeletonQueue = Arc<SegQueue<(PathBuf, PathBuf)>>;
type BavAssingedQ = Arc<ArrayQueue<(PathBuf, PathBuf)>>;
type StageCompleteA = Arc<StageComplete>;

fn launch(qs: (InputPPQ, SkeletonQueue), worker_counts: (u8, u8, u8), cfg: Config) {
    const STACK_SIZE: usize = 500 * 1024 * 1024; // 100mb
    let baq = ArrayQueue::new(100);
    let a_baq = Arc::new(baq);

    let stage0 = match worker_counts.0 {
        0 => Arc::new(StageComplete::finished()),
        _ => Arc::new(StageComplete::new(worker_counts.0)),
    };
    let stage1 = Arc::new(StageComplete::new(worker_counts.1));

    let handles = (0..worker_counts.0)
        .map(|_| {
            let t_q = Arc::clone(&qs.0);
            let t_q2 = Arc::clone(&qs.1);
            let t_s1 = Arc::clone(&stage0);
            let t_fp = cfg.file_provider.clone();

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(move || mutator_worker(t_q, t_q2, t_s1, t_fp))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    let bav_handles = (0..worker_counts.1)
        .map(|_| {
            let t_q2 = Arc::clone(&qs.1);
            let t_baq = Arc::clone(&a_baq);
            let t_s1 = Arc::clone(&stage0);
            let t_s2 = Arc::clone(&stage1);
            let t_cfg = cfg.clone();

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(move || bav_assign_worker(t_q2, t_s1, t_baq, t_s2, t_cfg))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    let solver_handles = (0..worker_counts.2)
        .map(|_| {
            let t_baq = Arc::clone(&a_baq);
            let t_s2 = Arc::clone(&stage1);
            let t_fp = cfg.file_provider.clone();

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(move || solver_worker(t_baq, t_s2, t_fp))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    let mut backoff = MyBackoff::new();
    for h in handles {
        h.unwrap().join().unwrap();
        backoff.snooze();
    }

    backoff.reset();
    for h in bav_handles {
        h.unwrap().join().unwrap();
        backoff.snooze();
    }

    backoff.reset();
    for h in solver_handles {
        h.unwrap().join().unwrap();
        backoff.snooze();
    }

    println!(
        "Queue lengths {} {} {}",
        qs.0.len(),
        qs.1.len(),
        a_baq.len()
    );
}

pub fn from_skels(dirname: &str, worker_counts: (u8, u8), mut cfg: Config) {
    cfg.file_provider.skeldir = Path::new(dirname).to_path_buf();
    cfg.file_provider.mddir = Path::new(dirname).to_path_buf();
    let q2 = SegQueue::new();
    for entry in WalkDir::new(dirname)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| s.starts_with("md_"))
                .unwrap_or(false)
        })
    {
        let metad_path = entry.into_path();
        let script_path = match script_f_from_metadata_f(&metad_path, &cfg.file_provider) {
            Ok(r) => r,
            Err(e) => {
                println!("can't find script for {:?} error: {}", metad_path, e);
                continue;
            }
        };

        println!("push {:?}", metad_path);
        q2.push((script_path, metad_path));
    }

    let q = SegQueue::new();
    let aq = Arc::new(q);
    let aq2 = Arc::new(q2);
    launch((aq, aq2), (0, worker_counts.0, worker_counts.1), cfg);
}

pub fn exec(dirname: &str, worker_counts: (u8, u8, u8), cfg: Config) {
    let q = SegQueue::new();
    for entry in WalkDir::new(dirname)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let filepath = entry.into_path();
        println!("push {:?}", filepath);
        q.push(Ok(filepath));
    }
    q.push(Err(PoisonPill {}));

    //stage 0
    let aq = Arc::new(q);
    let q2 = SegQueue::new();
    let aq2 = Arc::new(q2);

    launch((aq, aq2), worker_counts, cfg);
}

fn mutator_worker(
    qin: InputPPQ,
    qout: SkeletonQueue,
    stage: StageCompleteA,
    file_provider: FileProvider,
) {
    let mut backoff = MyBackoff::new();
    let mut stage_finished = false;
    while !stage_finished {
        let filepath = match qin.pop() {
            Ok(Ok(filepath)) => filepath,
            Ok(Err(poison_pill)) => {
                qin.push(Err(poison_pill));
                stage_finished = true;
                continue;
            }
            Err(_) => {
                backoff.snooze();
                continue;
            }
        };

        backoff.reset();
        match strip_and_transform(&filepath) {
            Some((script, mut md)) => {
                match file_provider.serialize_skel_and_md(&script, &mut md) {
                    Ok((skel_file, md_file)) => {
                        qout.push((skel_file.to_path_buf(), md_file.to_path_buf()))
                    }
                    Err(e) => println!("Skel/Md Serial Err: {}", e),
                };
            }
            None => continue,
        }
    }

    stage.finish();
}

fn bav_assign_worker(
    qin: SkeletonQueue,
    prev_stage: StageCompleteA,
    qout: BavAssingedQ,
    stage: StageCompleteA,
    cfg: Config,
) {
    let mut backoff = MyBackoff::new();

    while !prev_stage.is_complete() || qin.len() > 0 {
        let filepaths = match qin.pop() {
            Ok(item) => item,
            Err(_) => {
                backoff.snooze();
                continue;
            }
        };

        let (script, md) = match deserialize_from_f(&filepaths) {
            Ok(deserial) => deserial,
            Err(e) => {
                println!("baw deserial err in {:?}: {}", filepaths, e);
                continue;
            }
        };

        let empty_skel_fstr = &filepaths.0.to_str().unwrap();

        let results = solve(empty_skel_fstr);

        report_any_bugs(&filepaths.0, &results, &cfg.file_provider);

        if results.iter().all(|r| r.has_unrecoverable_error()) {
            println!("All err on file {}", empty_skel_fstr);
            fs::remove_file(filepaths.0).unwrap_or(());
            fs::remove_file(filepaths.1).unwrap_or(());
        } else {
            add_iterations_to_q(script, md, Arc::clone(&qout), &cfg);
        }
    }

    stage.finish();
}

fn add_iterations_to_q(
    mut script: Script,
    mut md: Metadata,
    qout: BavAssingedQ,
    cfg: &Config,
) -> Option<()> {
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

    while let Some(truth_values) = urng.sample() {
        let new_file = cfg.file_provider.iterfile(&mut md).ok()?;
        let str_with_model = dyn_fmt(&script_str, to_strs(&truth_values)).ok()?;
        fs::write(&new_file, str_with_model).ok()?;
        let mut to_push = (new_file.to_path_buf(), md.md_path(&cfg.file_provider));
        while let Err(PushError(reject)) = qout.push(to_push) {
            to_push = reject;
            backoff.snooze();
        }
    }
    Some(())
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
    let script = parser::script_from_f_unsanitized(script_file)?;

    let md_contents =
        fs::read_to_string(&md_file).map_err(|e| e.to_string() + " from metadata IO")?;
    let md: Metadata =
        serde_lexpr::from_str(&md_contents).map_err(|e| e.to_string() + " from serde")?;

    Ok((script, md))
}

fn script_f_from_metadata_f(md_file: &PathBuf, fp: &FileProvider) -> Result<PathBuf, String> {
    let md_contents =
        fs::read_to_string(&md_file).map_err(|e| e.to_string() + " from metadata IO")?;
    let md: Metadata =
        serde_lexpr::from_str(&md_contents).map_err(|e| e.to_string() + " from serde")?;
    Ok(md.skel_path(fp))
}

pub struct PoisonPill {}

fn solver_worker(qin: BavAssingedQ, prev_stage: StageCompleteA, file_provider: FileProvider) {
    let mut backoff = MyBackoff::new();

    while !prev_stage.is_complete() || qin.len() > 0 {
        let filepaths = match qin.pop() {
            Ok(item) => item,
            Err(_) => {
                backoff.snooze();
                continue;
            }
        };

        let results = solve(filepaths.0.to_str().unwrap_or("defaultname"));

        results
            .iter()
            .filter(|r| r.has_sat())
            .for_each(|r| resub_model(r, &filepaths, &qin, &file_provider));

        if !report_any_bugs(filepaths.0.as_path(), &results, &file_provider) {
            fs::remove_file(&filepaths.0).unwrap_or(());
        }
    }
}

fn resub_model(
    result: &RSolve,
    filepaths: &(PathBuf, PathBuf),
    _q: &BavAssingedQ,
    file_provider: &FileProvider,
) {
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

        let resubbed_f = match file_provider.serialize_resub(&script, &md) {
            Ok(f) => f,
            Err(e) => {
                println!("Resub file name err {}", e);
                return;
            }
        };

        let results = solve(resubbed_f.to_str().unwrap_or("defaultname"));

        if !report_any_bugs(&resubbed_f, &results, &file_provider) {
            fs::remove_file(&resubbed_f).unwrap_or(());
        }
    }
}

pub fn strip_and_transform(source_file: &Path) -> Option<(Script, Metadata)> {
    let mut script = parser::script_from_f(source_file).ok()?;
    // TODO error handling here on prev 3 lines

    if script.is_unsupported_logic() {
        return None;
    }

    let mut md = Metadata::new(source_file);
    to_skel(&mut script, &mut md).ok()?;
    return Some((script, md));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use std::thread;

    const STACK_SIZE: usize = 20 * 1024 * 1024;

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

    #[ignore]
    #[test]
    fn parse_unparse_test() {
        // Spawn thread with explicit stack size
        let child = thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn(parse_unparse)
            .unwrap();

        // Wait for thread to join
        child.join().unwrap();
    }

    fn parse_unparse() {
        let files = fs::read_dir("samples").expect("error with sample dir");

        for file_res in files.into_iter().take(10) {
            let file = file_res.expect("problem with file");
            println!("Starting {:?}", file);
            let filepath = file.path();

            let p = parser::script_from_f(&filepath).expect("parser error");

            let up = p.to_string();

            let pup = script(&up[..]).expect("reparse error").1;
            assert_eq!(p, pup);
        }
    }

    #[ignore]
    #[test]
    fn pup_test() {
        // Spawn thread with explicit stack size
        let child = thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn(pup)
            .unwrap();

        // Wait for thread to join
        child.join().unwrap();
    }

    fn pup() {
        for entry in WalkDir::new("test/parse_problems")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            let filepath = entry.into_path();
            let results = check_valid_solve(filepath.as_path().to_str().unwrap());

            println!("Checking {:?}", filepath);
            if results.iter().all(|r| r.has_unrecoverable_error()) {
                println!("Bad seed {:?}", filepath);
                continue;
            }

            let script = parser::script_from_f(&filepath).unwrap();

            let new_file =
                "temp_".to_owned() + filepath.as_path().file_name().unwrap().to_str().unwrap();

            fs::write(&new_file, script.to_string()).unwrap();
            let new_results = check_valid_solve(filepath.as_path().to_str().unwrap());

            if new_results.iter().all(|r| r.has_unrecoverable_error()) {
                println!("Bad parse {:?}", filepath);
            } else {
                fs::remove_file(&new_file);
            }
        }
    }
}
