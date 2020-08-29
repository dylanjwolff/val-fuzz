extern crate arbitrary;
extern crate log;
extern crate nom;
extern crate rand;
extern crate rand_core;
extern crate rand_xoshiro;
extern crate serde;
extern crate serde_lexpr;
extern crate subprocess;
extern crate tempfile;
extern crate walkdir;
#[macro_use]
extern crate lazy_static;
#[macro_use]
pub mod ast;

pub mod config;
pub mod fuzzer;
pub mod parser;
pub mod randomized_baseline;
pub mod solver;
pub mod transforms;
pub mod utils;

use crate::fuzzer::mutator;
use crate::fuzzer::solver_fn;
use crate::fuzzer::StatefulBavAssign;

use crate::randomized_baseline::rand_fuzz_solve;
use crate::randomized_baseline::StatefulRandBaseFuzzer;
use crate::utils::RunStats;
use crate::utils::StageComplete;

use config::Config;
use config::FileProvider;
use config::Metadata;
use crossbeam::queue::ArrayQueue;
use crossbeam::queue::PushError;
use crossbeam::queue::SegQueue;
use log::debug;
use log::info;
use log::trace;
use log::warn;
use std::collections::BTreeMap;

use utils::MyBackoff;

use utils::Timer;

use std::cmp::max;
use std::cmp::min;

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use std::sync::Arc;

use std::thread;
use std::thread::JoinHandle;

use walkdir::WalkDir;

type InputPPQ = Arc<SegQueue<Result<PathBuf, PoisonPill>>>;
type SkeletonQueue = Arc<SegQueue<(PathBuf, PathBuf)>>;
type BavAssingedQ = Arc<ArrayQueue<(Vec<(String, bool)>, (PathBuf, PathBuf))>>;
type StageCompleteA = Arc<StageComplete>;

fn launch(qs: (InputPPQ, SkeletonQueue), worker_counts: (u8, u8, u8), cfg: Config) {
    let criteria_a = max((worker_counts.1 as usize) * cfg.max_iter as usize, 1);
    let criteria_b = max((worker_counts.2 as usize) * 10 as usize, 1);
    let baq_cap = min(criteria_a, criteria_b);
    info!("Iterations Q has capacity {}", baq_cap);

    let baq = ArrayQueue::new(baq_cap);
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
            let t_cfg = cfg.clone();

            thread::Builder::new()
                .stack_size(cfg.stack_size)
                .spawn(move || mutator_worker(t_q, t_q2, t_s1, t_cfg))
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
                .stack_size(cfg.stack_size)
                .spawn(move || bav_assign_worker(t_q2, t_s1, t_baq, t_s2, t_cfg))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    let solver_handles = (0..worker_counts.2)
        .map(|_| {
            let t_baq = Arc::clone(&a_baq);
            let t_s2 = Arc::clone(&stage1);
            let t_cfg = cfg.clone();

            thread::Builder::new()
                .stack_size(cfg.stack_size)
                .spawn(move || solver_worker(t_baq, t_s2, t_cfg))
        })
        .collect::<Vec<std::io::Result<JoinHandle<_>>>>();

    thread::Builder::new()
        .spawn(move || {
            let mut b = MyBackoff::new();
            loop {
                b.snooze();
                info!(
                    "QLENS: {} {} {}",
                    max(1, qs.0.len()) - 1,
                    qs.1.len(),
                    a_baq.len()
                );
            }
        })
        .unwrap();

    let mut backoff = MyBackoff::new();
    for h in handles {
        h.unwrap().join().unwrap();
        backoff.snooze();
        trace!("Thread finished stage 1");
    }
    info!("Stage 1 Complete");

    backoff.reset();
    for h in bav_handles {
        h.unwrap().join().unwrap();
        backoff.snooze();
        trace!("Thread finished stage 2");
    }
    info!("Stage 2 Complete");

    backoff.reset();
    let mut all_stats = RunStats::new();
    for h in solver_handles {
        let stats = h.unwrap().join().unwrap();
        debug!("Saw {:?} on a single thread", stats);
        all_stats.merge_in_place(&stats);
        backoff.snooze();
        trace!("Thread finished stage 3");
    }
    info!("Saw {:?} across ALL threads", all_stats);
    info!("CSVRUNSTATS:{}", all_stats.to_csv_string());
    info!("CSVCONFIG:{}", cfg.to_csv_string());
    info!("Stage 3 Complete");
}

fn to_csv(bs: &[bool]) -> String {
    bs.iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn from_skels(dirname: &str, worker_counts: (u8, u8), mut cfg: Config) {
    cfg.file_provider.skeldir = Path::new(dirname).to_path_buf();
    cfg.file_provider.mddir = Path::new(dirname).to_path_buf();
    cfg.file_provider.cholesdir = Path::new(dirname).to_path_buf();
    debug!("Working from skeleton files");
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
                warn!("can't find script for {:?} error: {}", metad_path, e);
                continue;
            }
        };

        debug!("push {:?}", metad_path);
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
        debug!("push {:?}", filepath);
        q.push(Ok(filepath));
    }
    q.push(Err(PoisonPill {}));

    //stage 0
    let aq = Arc::new(q);
    let q2 = SegQueue::new();
    let aq2 = Arc::new(q2);

    launch((aq, aq2), worker_counts, cfg);
}

pub fn exec_randomized(dirname: &str, worker_count: (u8, u8), cfg: Config) {
    let q = SegQueue::new();
    for entry in WalkDir::new(dirname)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let filepath = entry.into_path();
        debug!("push {:?}", filepath);
        q.push(Ok(filepath));
    }
    q.push(Err(PoisonPill {}));

    let aq = Arc::new(q);
    let stage0 = match worker_count.0 {
        0 => Arc::new(StageComplete::finished()),
        _ => Arc::new(StageComplete::new(worker_count.0)),
    };

    let q2 = SegQueue::new();
    let aq2 = Arc::new(q2);

    let handles = (0..worker_count.0)
        .map(|_| {
            let t_q = Arc::clone(&aq);
            let t_q2 = Arc::clone(&aq2);
            let t_cfg = cfg.clone();
            let t_stage0 = Arc::clone(&stage0);

            thread::Builder::new()
                .stack_size(cfg.stack_size)
                .spawn(move || randomized_worker(t_q, t_stage0, t_q2, &t_cfg))
        })
        .collect::<Vec<std::io::Result<JoinHandle<_>>>>();

    let solver_handles = (0..worker_count.1)
        .map(|_| {
            let t_q2 = Arc::clone(&aq2);
            let t_cfg = cfg.clone();
            let t_stage0 = Arc::clone(&stage0);

            thread::Builder::new()
                .stack_size(cfg.stack_size)
                .spawn(move || randomized_solver_worker(t_stage0, t_q2, &t_cfg))
        })
        .collect::<Vec<std::io::Result<JoinHandle<_>>>>();

    thread::Builder::new()
        .spawn(move || {
            let mut b = MyBackoff::new();
            loop {
                b.snooze();
                info!("QLENS {} {}", max(1, aq.len()) - 1, aq2.len());
            }
        })
        .unwrap();

    let mut backoff = MyBackoff::new();
    backoff.reset();
    let mut all_stats = RunStats::new();
    for h in handles.into_iter().chain(solver_handles.into_iter()) {
        let stats = h.unwrap().join().unwrap();
        debug!("Saw {:?} on a single thread", stats);
        all_stats.merge_in_place(&stats);
        backoff.snooze();
        trace!("Thread finished");
    }
    info!("Saw {:?} across ALL threads", all_stats);
    info!("CSVRUNSTATS:{}", all_stats.to_csv_string());
    info!("CSVCONFIG:{}", cfg.to_csv_string());
    info!("Stage 3 Complete");
}

fn randomized_solver_worker(
    prev_stage_complete: StageCompleteA,
    qin: SkeletonQueue,
    cfg: &Config,
) -> RunStats {
    let mut backoff = MyBackoff::new();
    let mut stats = RunStats::new();
    let mut to_ctr = BTreeMap::new();
    loop {
        let (file, mdfile) = match qin.pop() {
            Ok(r) => r,
            Err(_) => {
                if prev_stage_complete.is_complete() {
                    break;
                } else {
                    backoff.snooze();
                    continue;
                }
            }
        };
        match rand_fuzz_solve(&file, &mdfile, cfg, &mut stats, &mut to_ctr) {
            Err(e) => warn!("Solver Error {} on file {:?}", e, file),
            Ok(()) => (),
        };
    }
    stats
}

fn randomized_worker(
    qin: InputPPQ,
    stage_complete: StageCompleteA,
    qout: SkeletonQueue,
    cfg: &Config,
) -> RunStats {
    let mut stats = RunStats::new();
    let mut backoff = MyBackoff::new();
    loop {
        let filepath = match qin.pop() {
            Ok(Ok(filepath)) => filepath,
            Ok(Err(poison_pill)) => {
                qin.push(Err(poison_pill));
                break;
            }
            Err(_) => {
                backoff.snooze();
                continue;
            }
        };
        backoff.reset();

        let mut fuzzer = match StatefulRandBaseFuzzer::new(&filepath, cfg, &mut stats) {
            Ok(f) => f,
            Err(e) => {
                warn!(
                    "Initial iteration error on seed {}: {}",
                    filepath.to_str().unwrap(),
                    e
                );
                continue;
            }
        };
        loop {
            match fuzzer.next() {
                Ok(Some(iter)) => qout.push(iter),
                Ok(None) => continue,
                Err(e) => {
                    warn!(
                        "Iteration error on seed {}: {}",
                        filepath.to_str().unwrap(),
                        e
                    );
                    break;
                }
            }
        }
    }
    stage_complete.finish();
    stats
}

pub struct PoisonPill {}
fn mutator_worker(qin: InputPPQ, qout: SkeletonQueue, stage: StageCompleteA, cfg: Config) {
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
        match mutator(filepath.clone(), &cfg) {
            Ok(skelfs) => {
                for (skelf, mdf) in skelfs {
                    qout.push((skelf, mdf))
                }
            }
            Err(e) => warn!("Mutator error: {} in files {:?}", e, filepath),
        };
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

        trace!("Starting assignments of {:?} skeletion", filepaths.0);

        match StatefulBavAssign::new(filepaths.clone(), &cfg) {
            Ok(mut sba) => {
                trace!("Adding assignments of {:?} skeletion to the Q", filepaths.0);
                let mut i = 0;
                while let Some((tv, mask)) = sba.urng.sample_and_mask() {
                    let ef = sba
                        .md
                        .bavns
                        .iter()
                        .zip(mask.iter())
                        .filter_map(
                            |((name, _sort), mbit)| if mbit { Some(name.to_owned()) } else { None },
                        )
                        .zip(tv.iter())
                        .collect::<Vec<(String, bool)>>();

                    i = i + 1;
                    let mut to_push = match sba.do_iteration_tv(i, tv, mask) {
                        Ok(v) => (ef, v),
                        Err(e) => {
                            warn!("Iteration error {}", e);
                            continue;
                        }
                    };
                    while let Err(PushError(reject)) = qout.push(to_push) {
                        to_push = reject;
                        backoff.snooze();
                    }
                }
            }
            Err(e) => warn!("BAV Error: {} in files {:?}", e, filepaths),
        }
        trace!("Done with adding assignments to {:?}", filepaths.0);
    }

    stage.finish();
}

fn solver_worker(qin: BavAssingedQ, prev_stage: StageCompleteA, cfg: Config) -> RunStats {
    let mut backoff = MyBackoff::new();

    let mut to_ctr = BTreeMap::new();
    let mut stats = RunStats::new();

    while !prev_stage.is_complete() || qin.len() > 0 {
        let (enforcemt, filepaths) = match qin.pop() {
            Ok(item) => item,
            Err(_) => {
                backoff.snooze();
                continue;
            }
        };
        trace!("Solving {:?}", filepaths.0);
        match solver_fn(filepaths.clone(), &mut to_ctr, &mut stats, enforcemt, &cfg) {
            Err(e) => {
                if cfg.remove_files {
                    fs::remove_file(&filepaths.0).unwrap_or(());
                }
                warn!("Solve Error: {}", e)
            }
            _ => (),
        };
        trace!("Done solving {:?}", filepaths.0);
    }
    stats
}

fn script_f_from_metadata_f(md_file: &PathBuf, fp: &FileProvider) -> Result<PathBuf, String> {
    trace!("Parsing MD file");
    let md_contents =
        Box::new(fs::read_to_string(&md_file).map_err(|e| e.to_string() + " from metadata IO")?);
    trace!("Parsing MD");
    let md: Box<Metadata> =
        Box::new(serde_lexpr::from_str(&*md_contents).map_err(|e| e.to_string() + " from serde")?);
    trace!("Done Parsing MD");
    Ok(md.skel_path(fp))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::script;
    use crate::solver::check_valid_solve;

    use std::fs;

    use std::thread;

    const STACK_SIZE: usize = 20 * 1024 * 1024;

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
                fs::remove_file(&new_file).unwrap();
            }
        }
    }
}
