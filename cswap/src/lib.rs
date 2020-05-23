#[allow(unused)]
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

pub mod parser;
pub mod solver;
pub mod transforms;

use crate::solver::RSolve;
use crate::transforms::rv;
use ast::Script;
use bit_vec::BitVec;
use crossbeam::queue::ArrayQueue;
use crossbeam::queue::PushError;
use crossbeam::queue::SegQueue;
use crossbeam::utils::Backoff;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::multi::many1;

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

type InputPPQ = Arc<SegQueue<Result<PathBuf, PoisonPill>>>;
type SkeletonQueue = Arc<SegQueue<(PathBuf, PathBuf)>>;
type BavAssingedQ = Arc<ArrayQueue<(PathBuf, PathBuf)>>;
type StageCompleteA = Arc<StageComplete>;

struct MyBackoff {
    current_wait: Duration,
    max: Duration,
    min: Duration,
}

impl MyBackoff {
    fn new() -> Self {
        let min = Duration::from_millis(5);
        MyBackoff {
            min: min,
            max: Duration::from_secs(5),
            current_wait: min,
        }
    }

    fn snooze(&mut self) {
        if self.max > self.current_wait {
            thread::sleep(self.current_wait);
            self.current_wait = 2 * self.current_wait;
        } else {
            thread::sleep(self.max);
        }
    }

    fn reset(&mut self) {
        self.current_wait = self.min;
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Metadata {
    bavns: Vec<String>,
    constvns: Vec<String>,
    seed_file: PathBuf,
    skeleton_file: PathBuf,
    metadata_file: PathBuf,
}

impl Metadata {
    pub fn new_empty() -> Self {
        Metadata {
            bavns: vec![],
            constvns: vec![],
            seed_file: Path::new("").to_path_buf(),
            skeleton_file: Path::new("").to_path_buf(),
            metadata_file: Path::new("").to_path_buf(),
        }
    }
}

pub struct Timer {
    done: Arc<AtomicBool>,
}

impl Timer {
    fn new_started(time: Duration) -> Self {
        let t = Timer {
            done: Arc::new(AtomicBool::new(false)),
        };
        t.start(time);
        t
    }

    fn new() -> Self {
        Timer {
            done: Arc::new(AtomicBool::new(false)),
        }
    }

    fn clone(&self) -> Self {
        Timer {
            done: Arc::clone(&self.done),
        }
    }

    fn start(&self, time: Duration) {
        let t_b = Arc::clone(&self.done);
        thread::spawn(move || {
            thread::sleep(time);
            t_b.store(true, Ordering::Relaxed)
        });
    }

    fn is_done(&self) -> bool {
        self.done.load(Ordering::Relaxed)
    }

    fn reset(&self) {
        self.done.store(false, Ordering::Relaxed);
    }
}

struct StageComplete {
    number_workers_finished: Mutex<u8>,
    total_workers: u8,
}

impl StageComplete {
    fn new(num_workers: u8) -> Self {
        StageComplete {
            number_workers_finished: Mutex::new(0),
            total_workers: num_workers,
        }
    }

    fn finished() -> Self {
        StageComplete::new(0)
    }

    fn finish(&self) {
        let mut guard = self.number_workers_finished.lock().unwrap();
        *guard = *guard + 1;
    }

    fn is_complete(&self) -> bool {
        let guard = self.number_workers_finished.lock().unwrap();
        *guard == self.total_workers
    }
}

fn launch(qs: (InputPPQ, SkeletonQueue), worker_counts: (u8, u8, u8)) {
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

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(|| mutator_worker(t_q, t_q2, t_s1))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    let bav_handles = (0..worker_counts.1)
        .map(|_| {
            let t_q2 = Arc::clone(&qs.1);
            let t_baq = Arc::clone(&a_baq);
            let t_s1 = Arc::clone(&stage0);
            let t_s2 = Arc::clone(&stage1);

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(|| bav_assign_worker(t_q2, t_s1, t_baq, t_s2))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    let solver_handles = (0..worker_counts.2)
        .map(|_| {
            let t_baq = Arc::clone(&a_baq);
            let t_s2 = Arc::clone(&stage1);

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(|| solver_worker(t_baq, t_s2))
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

pub fn from_skels(dirname: &str, worker_counts: (u8, u8)) {
    let stage1 = Arc::new(StageComplete::finished());

    let q2 = SegQueue::new();
    for entry in WalkDir::new(dirname)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| s.starts_with("md_skel"))
                .unwrap_or(false)
        })
    {
        let metad_path = entry.into_path();
        let script_path = match script_f_from_metadata_f(&metad_path) {
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
    launch((aq, aq2), (0, worker_counts.0, worker_counts.1));
}

pub fn exec(dirname: &str, worker_counts: (u8, u8, u8)) {
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

    launch((aq, aq2), (2, 2, 2));
}

fn mutator_worker(qin: InputPPQ, qout: SkeletonQueue, stage: StageCompleteA) {
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

        println!("Begin file {:?}", filepath);
        backoff.reset();
        match strip_and_transform(&filepath) {
            Some((script, md)) => {
                let skel_name = get_new_name(&filepath, "skel");

                let skel_file = Path::new(&skel_name).to_path_buf();
                match serialize_to_f(&skel_file.as_path(), &script, &md) {
                    Ok(to_push) => qout.push(to_push),
                    Err(e) => {
                        println!("Error serializing skel {}", e);
                        continue;
                    }
                }
            }
            None => continue,
        }
        println!("End file {:?}", filepath);
    }

    stage.finish();
}

fn bav_assign_worker(
    qin: SkeletonQueue,
    prev_stage: StageCompleteA,
    qout: BavAssingedQ,
    stage: StageCompleteA,
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

        report_any_bugs(&filepaths.0, &results);

        if results.iter().all(|r| r.has_unrecoverable_error()) {
            println!("All err on file {}", empty_skel_fstr);
            fs::remove_file(filepaths.0).unwrap_or(());
            fs::remove_file(filepaths.1).unwrap_or(());
        } else {
            add_iterations_to_q(script, md, &filepaths.0, &filepaths.1, Arc::clone(&qout));
        }
    }

    stage.finish();
}

fn add_iterations_to_q(
    mut script: Script,
    md: Metadata,
    filepath: &Path,
    md_fp: &Path,
    qout: BavAssingedQ,
) -> Option<()> {
    let mut backoff = MyBackoff::new();

    let eip = end_insert_pt(&script);
    script.init(eip);
    script.replace(eip, get_bav_assign_fmt_str(&md.bavns));
    let script_str = script.to_string_dfltto()?;

    let num_bavs = md.bavns.len();
    const MAX_ITER: u32 = 2000;
    println!("starting max(2^{}, {}) iterations", num_bavs, MAX_ITER);
    let mut urng = RandUniqPermGen::new_definite(num_bavs, MAX_ITER);
    while let Some(truth_values) = urng.sample() {
        let new_filename = get_iter_fileout_name(filepath, urng.get_count());
        let new_file = Path::new(&new_filename[..]).to_path_buf();

        let str_with_model = dyn_fmt(&script_str, to_strs(&truth_values)).ok()?;
        fs::write(&new_file, str_with_model).ok()?;
        let mut to_push = (new_file.to_path_buf(), md_fp.to_path_buf());
        while let Err(PushError(reject)) = qout.push(to_push) {
            to_push = reject;
            backoff.snooze();
        }
    }
    Some(())
}

fn report_any_bugs(file: &Path, results: &Vec<RSolve>) -> bool {
    results
        .iter()
        .find(|r| r.has_bug_error())
        .map(|r| {
            println!("Error bug in {:?} !!! {:?}", file, r);
            println!("{}", r);
        })
        .is_some()
        || if !RSolve::differential_test(&results).is_ok() {
            println!("Soundness bug in {:?} !!!", file);
            true
        } else {
            false
        }
}

fn get_new_name(source_file: &Path, prefix: &str) -> String {
    let source_filename = match source_file.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => "unknown",
    };
    prefix.to_owned() + "_" + source_filename
}

fn serialize_to_f(
    based_off_of: &Path,
    script: &Script,
    md: &Metadata,
) -> Result<(PathBuf, PathBuf), String> {
    let md_name = get_new_name(based_off_of, "md");
    let script_name = get_new_name(based_off_of, "script");
    let md_file = Path::new(&md_name);
    let script_file = Path::new(&script_name);

    let md_serial = serde_lexpr::to_string(&(&md, &script_name)).map_err(|e| format!("{:?}", e))?;
    let script_serial = script.to_string_dfltto().ok_or("Timeout on to_string()")?;

    fs::write(md_file, md_serial).map_err(|e| format!("{:?}", e))?;
    fs::write(script_file, script_serial).map_err(|e| format!("{:?}", e))?;
    Ok((script_file.to_path_buf(), md_file.to_path_buf()))
}

fn deserialize_from_f(
    (script_file, md_file): &(PathBuf, PathBuf),
) -> Result<(Script, Metadata), String> {
    let script_contents =
        fs::read_to_string(&script_file).map_err(|e| e.to_string() + " from IO")?;
    let presult = script(&script_contents).map_err(|e| e.to_string() + " from parsing")?;

    if presult.0 != "" {
        Err("Incomplete Parse!".to_owned())
    } else {
        let script = presult.1;

        let md_contents =
            fs::read_to_string(&md_file).map_err(|e| e.to_string() + " from metadata IO")?;
        let (md, _): (Metadata, PathBuf) =
            serde_lexpr::from_str(&md_contents).map_err(|e| e.to_string() + " from serde")?;

        Ok((script, md))
    }
}

fn script_f_from_metadata_f(md_file: &PathBuf) -> Result<PathBuf, String> {
    let md_contents =
        fs::read_to_string(&md_file).map_err(|e| e.to_string() + " from metadata IO")?;
    let (_, script_file): (Metadata, PathBuf) =
        serde_lexpr::from_str(&md_contents).map_err(|e| e.to_string() + " from serde")?;

    match md_file.parent() {
        Some(dir) => {
            let mut f = dir.clone().to_path_buf();
            f.push(script_file);
            Ok(f)
        }
        None => Ok(script_file),
    }
}

pub struct PoisonPill {}

fn solver_worker(qin: BavAssingedQ, prev_stage: StageCompleteA) {
    let mut backoff = MyBackoff::new();

    while !prev_stage.is_complete() || qin.len() > 0 {
        let filepaths = match qin.pop() {
            Ok(item) => item,
            Err(_) => {
                backoff.snooze();
                continue;
            }
        };

        println!("Checking file {:?}", filepaths.0);
        let results = solve(filepaths.0.to_str().unwrap_or("defaultname"));

        results
            .iter()
            .filter(|r| r.has_sat())
            .for_each(|r| resub_model(r, &filepaths, &qin));

        if !report_any_bugs(filepaths.0.as_path(), &results) {
            fs::remove_file(&filepaths.0).unwrap_or(());
        }

        println!("Done hecking file {:?}", &filepaths.0);
    }
}

pub fn resub_model(result: &RSolve, filepaths: &(PathBuf, PathBuf), q: &BavAssingedQ) {
    let mut backoff = MyBackoff::new();
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

        let new_name = get_new_name(&filepaths.0, &format!("resub_{}", result.solver.name()));
        let resubbed_fs = serialize_to_f(Path::new(&new_name), &script, &md).unwrap();
        println!("RESUB~~~ {:?} ", resubbed_fs);

        let results = solve(resubbed_fs.0.to_str().unwrap_or("defaultname"));

        if !report_any_bugs(resubbed_fs.0.as_path(), &results) {
            fs::remove_file(&resubbed_fs.0).unwrap_or(());
            fs::remove_file(&resubbed_fs.1).unwrap_or(());
        }

        println!("Done hecking file {:?}", &resubbed_fs.0);
    }
}

#[allow(unused)]
struct RandUniqPermGen {
    rng: Xoshiro256Plus,
    numbits: usize,
    buf: Vec<u8>,
    seen: BTreeSet<BitVec>,
    retries: u16,
    max: u32,
    use_max: bool,
    use_retries: bool,
}

use rand_xoshiro::Xoshiro256Plus;
impl RandUniqPermGen {
    fn new_definite(numbits: usize, maxiter: u32) -> Self {
        let buf = BitVec::from_elem(numbits, false).to_bytes();
        let seen = BTreeSet::new();
        let rng = Xoshiro256Plus::seed_from_u64(8085);

        let true_max = if (maxiter as f64).log2() < (numbits as f64) {
            maxiter
        } else {
            (numbits as f64).exp2() as u32
        };

        RandUniqPermGen {
            rng: rng,
            numbits: numbits,
            buf: buf,
            seen: seen,
            retries: 0,
            max: true_max,
            use_max: true,
            use_retries: false,
        }
    }

    fn get_count(&self) -> u32 {
        self.seen.len() as u32
    }

    #[allow(unused)]
    fn sample(&mut self) -> Option<BitVec> {
        if self.max <= self.seen.len() as u32 {
            return None;
        }

        let mut is_new = false;
        let mut attempt = 0;
        while true || (self.use_retries && attempt < self.retries) {
            self.rng.fill(&mut self.buf[..]);
            let mut bv = BitVec::from_bytes(&self.buf[..]);
            bv.truncate(self.numbits);
            is_new = self.seen.insert(bv.clone());
            if is_new {
                return Some(bv);
            }
            attempt = attempt + 1;
        }

        None
    }
}

pub fn strip_and_transform(source_file: &Path) -> Option<(Script, Metadata)> {
    let contents: String = fs::read_to_string(source_file).ok()?;
    let stripped_contents = &rmv_comments(&contents[..]).ok()?.1.join(" ")[..];
    let mut script = script(&stripped_contents[..]).ok()?.1;
    // TODO error handling here on prev 3 lines
    println!("Done parsing");

    if script.is_unsupported_logic() {
        return None;
    }

    let mut md = Metadata::new_empty();
    to_skel(&mut script, &mut md).ok()?;
    println!("Done skelling");
    return Some((script, md));
}

fn get_iter_fileout_name(source_file: &Path, iter: u32) -> String {
    let source_filename = match source_file.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => "unknown",
    };
    (iter).to_string() + "_" + source_filename
}
use nom::error::ErrorKind;
use nom::error_position;
fn dynamic_format_parser<'a, 'b>(
    (s, vs): (&'a str, Vec<&'a str>),
) -> IResult<(&'a str, Vec<&'a str>), Vec<&'a str>> {
    let replace = |(s, mut vs): (&'a str, Vec<&'a str>)| {
        tag("{}")(s)
            .map_err(|e| match e {
                nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
                nom::Err::Error((i, ek)) => nom::Err::Error(((i, vs.clone()), ek)),
                nom::Err::Failure((i, ek)) => nom::Err::Failure(((i, vs.clone()), ek)),
            })
            .map(|(i, _o)| {
                let v = vs.pop().unwrap_or("");
                ((i, vs), v)
            })
    };

    let keep = |(s, vs): (&'a str, Vec<&'a str>)| {
        take_until("{}")(s)
            .map_err(|e| match e {
                nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
                nom::Err::Error((i, ek)) => nom::Err::Error(((i, vs.clone()), ek)),
                nom::Err::Failure((i, ek)) => nom::Err::Failure(((i, vs.clone()), ek)),
            })
            .map(|(i, o)| ((i, vs), o))
    };

    many1(alt((replace, keep)))((s, vs)).map(|(i, mut o)| {
        o.push(i.0); // if there are no more {}, push the rest of the str
        (("", i.1), o)
    })
}

fn eof_str<'a>(s: &'a str) -> IResult<&'a str, &'a str> {
    if s.len() == 0 {
        Ok(("", ""))
    } else {
        Err(nom::Err::Error(error_position!(s, ErrorKind::Eof)))
    }
}
type DFormatParseError<'a> = nom::Err<((&'a str, std::vec::Vec<&'a str>), nom::error::ErrorKind)>;
fn dyn_fmt<'a>(s: &'a str, mut vs: Vec<&'a str>) -> Result<String, DFormatParseError<'a>> {
    vs.reverse();
    let (_rem, ss) = dynamic_format_parser((s, vs))?;
    let cap = ss.iter().map(|s| s.len()).sum();
    let mut v = Vec::with_capacity(cap);
    for s in ss {
        v.extend_from_slice(s.as_bytes());
    }
    unsafe { Ok(String::from_utf8_unchecked(v)) }
}

fn to_strs(bv: &BitVec) -> Vec<&'static str> {
    bv.iter()
        .map(|b| if b { "true" } else { "false" })
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use std::thread;

    const STACK_SIZE: usize = 20 * 1024 * 1024;

    #[test]
    fn bv_replace() {
        let fmt_str = get_bav_assign_fmt_str(&vec!["BAV1".to_owned()])
            .to_string(Timer::new())
            .unwrap();
        println!("{}", fmt_str);
        let bv = BitVec::from_elem(1, true);
        assert_eq!(
            dyn_fmt(&fmt_str, to_strs(&bv)).unwrap(),
            "(assert (= BAV1 true))"
        );
    }

    #[test]
    fn dfmt() {
        assert_eq!(
            dyn_fmt("{} asdf {}!", vec!["sub", "stitute"]).unwrap(),
            "sub asdf stitute!"
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

    fn parse_file(f: &str) -> Script {
        let contents = &fs::read_to_string(f).expect("error reading file")[..];
        let contents_sans_comments = &rmv_comments(contents)
            .expect("failed to rmv comments")
            .1
            .join(" ")[..];

        script(contents_sans_comments).expect("parser error").1
    }

    fn parse_unparse() {
        let files = fs::read_dir("samples").expect("error with sample dir");

        for file_res in files.into_iter().take(10) {
            let file = file_res.expect("problem with file");
            println!("Starting {:?}", file);
            let filepath = file.path();
            let contents = &fs::read_to_string(filepath).expect("error reading file")[..];
            let contents_sans_comments = &rmv_comments(contents)
                .expect("failed to rmv comments")
                .1
                .join(" ")[..];

            let p = script(contents_sans_comments).expect("parser error").1;

            let up = match p.to_string_dfltto() {
                Some(contents) => contents,
                None => continue,
            };

            let pup = script(&up[..]).expect("reparse error").1;
            assert_eq!(p, pup);
        }
    }

    #[test]
    fn timer_test() {
        let timer = Timer::new();
        timer.start(Duration::from_millis(200));
        thread::sleep(Duration::from_millis(100));
        assert!(!timer.is_done());
        thread::sleep(Duration::from_millis(200));
        assert!(timer.is_done());
    }

    #[test]
    fn ru_definite_reaches_maxiter() {
        let mut rng = RandUniqPermGen::new_definite(10, 1);
        assert!(rng.sample().is_some());
        assert!(rng.sample().is_none());
    }

    #[test]
    fn ru_definite_reaches_maxpossible() {
        let mut rng = RandUniqPermGen::new_definite(1, 100);
        assert!(rng.sample().is_some());
        assert!(rng.sample().is_some());
        assert!(rng.sample().is_none());
    }

    #[test]
    fn ru_definite_correct_size() {
        let mut rng = RandUniqPermGen::new_definite(9, 1);
        assert_eq!(rng.sample().expect("should hold value").len(), 9);
    }

    #[test]
    fn ru_definite_distinct() {
        let mut set = BTreeSet::new();
        let mut rng = RandUniqPermGen::new_definite(2, 4);
        set.insert(rng.sample().expect("Should hold value"));
        set.insert(rng.sample().expect("Should hold value"));
        set.insert(rng.sample().expect("Should hold value"));
        set.insert(rng.sample().expect("Should hold value"));
        assert_eq!(set.len(), 4);
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

            let contents: String = fs::read_to_string(&filepath).unwrap();
            let stripped_contents = &rmv_comments(&contents[..]).unwrap().1.join(" ")[..];
            let script = script(&stripped_contents[..]).unwrap().1;

            let new_file =
                "temp_".to_owned() + filepath.as_path().file_name().unwrap().to_str().unwrap();

            fs::write(&new_file, script.to_string_dfltto().unwrap()).unwrap();
            let new_results = check_valid_solve(filepath.as_path().to_str().unwrap());

            if new_results.iter().all(|r| r.has_unrecoverable_error()) {
                println!("Bad parse {:?}", filepath);
            } else {
                fs::remove_file(&new_file);
            }
        }
    }
}
