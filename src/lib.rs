extern crate itertools;
#[allow(unused)]
extern crate nom;
extern crate rand;
extern crate rand_core;
extern crate rand_xoshiro;
extern crate serde;
extern crate serde_lexpr;
extern crate walkdir;

#[macro_use]
pub mod ast;

pub mod parser;
pub mod solver;
pub mod transforms;

use ast::Script;
use bit_vec::BitVec;
use crossbeam::queue::ArrayQueue;
use crossbeam::queue::PushError;
use crossbeam::queue::SegQueue;
use crossbeam::utils::Backoff;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::hex_digit1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::not;
use nom::combinator::peek;
use nom::eof;
use nom::multi::many0;
use nom::multi::many1;
use nom::named;
use nom::number::complete::recognize_float;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use parser::{rmv_comments, script};
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
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
use transforms::{end_insert_pt, get_bav_assign, get_bav_assign_fmt_str, to_skel};
use walkdir::WalkDir;

type InputPPQ = Arc<SegQueue<Result<PathBuf, PoisonPill>>>;
type SkeletonQueue = Arc<SegQueue<(PathBuf, PathBuf)>>;
type BavAssingedQ = Arc<ArrayQueue<(PathBuf, PathBuf)>>;
type StageCompleteA = Arc<StageComplete>;

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

pub fn from_skels() {
    let stage1 = Arc::new(StageComplete::finished());

    let q2 = SegQueue::new();
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| s.starts_with("bavns_skel"))
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

    let aq2 = Arc::new(q2);
    let num_stage2_workers = 2;
    let stage2 = Arc::new(StageComplete::new(num_stage2_workers));

    let baq = ArrayQueue::new(100);
    let a_baq = Arc::new(baq);

    const STACK_SIZE: usize = 500 * 1024 * 1024; // 100mb

    let bav_handles = (0..2)
        .map(|_| {
            let t_q2 = Arc::clone(&aq2);
            let t_baq = Arc::clone(&a_baq);
            let t_s1 = Arc::clone(&stage1);
            let t_s2 = Arc::clone(&stage2);

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(|| bav_assign_worker(t_q2, t_s1, t_baq, t_s2))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    let solver_handles = (0..2)
        .map(|_| {
            let t_baq = Arc::clone(&a_baq);
            let t_s2 = Arc::clone(&stage2);

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(|| solver_worker(t_baq, t_s2))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    for h in bav_handles {
        h.unwrap().join().unwrap();
    }

    for h in solver_handles {
        h.unwrap().join().unwrap();
    }

    println!("Queue lengths {} {}", aq2.len(), a_baq.len());
}

pub fn exec() {
    let q = SegQueue::new();
    for entry in WalkDir::new("samples")
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
    let num_stage1_workers = 2;
    let stage1 = Arc::new(StageComplete::new(num_stage1_workers));

    let q2 = SegQueue::new();
    let aq2 = Arc::new(q2);
    let num_stage2_workers = 2;
    let stage2 = Arc::new(StageComplete::new(num_stage2_workers));

    let baq = ArrayQueue::new(100);
    let a_baq = Arc::new(baq);

    const STACK_SIZE: usize = 500 * 1024 * 1024; // 100mb
    let handles = (0..2)
        .map(|_| {
            let t_q = Arc::clone(&aq);
            let t_q2 = Arc::clone(&aq2);
            let t_s1 = Arc::clone(&stage1);

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(|| mutator_worker(t_q, t_q2, t_s1))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    let bav_handles = (0..2)
        .map(|_| {
            let t_q2 = Arc::clone(&aq2);
            let t_baq = Arc::clone(&a_baq);
            let t_s1 = Arc::clone(&stage1);
            let t_s2 = Arc::clone(&stage2);

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(|| bav_assign_worker(t_q2, t_s1, t_baq, t_s2))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    let solver_handles = (0..8)
        .map(|_| {
            let t_baq = Arc::clone(&a_baq);
            let t_s2 = Arc::clone(&stage2);

            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(|| solver_worker(t_baq, t_s2))
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    for h in handles {
        h.unwrap().join().unwrap();
    }

    for h in bav_handles {
        h.unwrap().join().unwrap();
    }

    for h in solver_handles {
        h.unwrap().join().unwrap();
    }

    println!("Queue lengths {} {} {}", aq.len(), aq2.len(), a_baq.len());
}

fn mutator_worker(qin: InputPPQ, qout: SkeletonQueue, stage: StageCompleteA) {
    let backoff = Backoff::new();
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
            Some((script, bavns)) => {
                let skel_name = get_new_name(&filepath, "skel");

                let skel_file = Path::new(&skel_name).to_path_buf();
                match serialize_to_f(&skel_file.as_path(), &script, &bavns) {
                    Ok(to_push) => qout.push(to_push),
                    Err(_) => continue,
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
    let backoff = Backoff::new();

    while !prev_stage.is_complete() || qin.len() > 0 {
        let filepaths = match qin.pop() {
            Ok(item) => item,
            Err(_) => {
                backoff.snooze();
                continue;
            }
        };

        let (script, bavns) = match deserialize_from_f(&filepaths) {
            Ok(deserial) => deserial,
            Err(e) => {
                println!("baw deserial err: {}", e);
                continue;
            }
        };

        // TODO below
        let empty_skel_name = &filepaths.0.file_name().and_then(|s| s.to_str()).unwrap();

        match solve(empty_skel_name) {
            SolveResult::Timeout => {
                println!("Timeout on file {}", empty_skel_name);
                fs::remove_file(filepaths.0).unwrap_or(());
                fs::remove_file(filepaths.1).unwrap_or(());
            }
            SolveResult::Error | SolveResult::ProcessError => {
                fs::remove_file(filepaths.0).unwrap_or(());
                fs::remove_file(filepaths.1).unwrap_or(());
            }
            SolveResult::ErrorBug => report_bug(filepaths.0.as_path(), SolveResult::ErrorBug),
            SolveResult::SoundnessBug => {
                report_bug(filepaths.0.as_path(), SolveResult::SoundnessBug)
            }
            SolveResult::Sat | SolveResult::Unsat | SolveResult::Unknown => {
                add_iterations_to_q(script, bavns, &filepaths.0, Arc::clone(&qout));
                ()
            }
        }
    }

    stage.finish();
}

fn add_iterations_to_q(
    mut script: Script,
    bavns: Vec<String>,
    filepath: &Path,
    qout: BavAssingedQ,
) -> Option<()> {
    let backoff = Backoff::new();

    let eip = end_insert_pt(&script);
    script.init(eip);
    script.replace(eip, get_bav_assign_fmt_str(&bavns));

    let num_bavs = bavns.len();
    const MAX_ITER: u32 = 1;
    println!("starting max(2^{}, {}) iterations", num_bavs, MAX_ITER);
    let mut urng = RandUniqPermGen::new_definite(num_bavs, MAX_ITER);
    while let Some(truth_values) = urng.sample() {
        let new_filename = get_iter_fileout_name(filepath, urng.get_count());
        script.replace(eip, get_bav_assign(&bavns, truth_values));

        let mut to_push = Path::new(&new_filename[..]).to_path_buf();

        let mut to_push = match serialize_to_f(&to_push, &script, &bavns) {
            Ok(tp) => tp,
            Err(_) => continue,
        };

        while let Err(PushError(reject)) = qout.push(to_push) {
            to_push = reject;
            backoff.snooze();
        }
    }
    Some(())
}

fn report_bug(file: &Path, kind: SolveResult) {
    println!(
        "{:?} bug in {} !!!",
        kind,
        file.to_str().unwrap_or("defaultname")
    );
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
    bavns: &Vec<String>,
) -> Result<(PathBuf, PathBuf), ()> {
    let bavns_name = get_new_name(based_off_of, "bavns");
    let script_name = get_new_name(based_off_of, "script");
    let bavns_file = Path::new(&bavns_name);
    let script_file = Path::new(&script_name);

    match {
        let bavns_serial = serde_lexpr::to_string(&(&bavns, &script_name)).map_err(|_| ())?;
        let script_serial = script.to_string_dfltto().ok_or(())?;

        fs::write(bavns_file, bavns_serial).map_err(|_| ())?;
        fs::write(script_file, script_serial).map_err(|_| ())?;
        Ok((script_file.to_path_buf(), bavns_file.to_path_buf()))
    } {
        Ok(r) => Ok(r),
        Err(()) => Err(println!("serial error file {:?}", based_off_of)),
    }
}

fn deserialize_from_f(
    (script_file, bavns_file): &(PathBuf, PathBuf),
) -> Result<(Script, Vec<String>), String> {
    let script_contents =
        fs::read_to_string(&script_file).map_err(|e| e.to_string() + " from IO")?;
    let presult = script(&script_contents).map_err(|e| e.to_string() + " from parsing")?;

    if presult.0 != "" {
        Err("Incomplete Parse!".to_owned())
    } else {
        let script = presult.1;

        let bavns_contents =
            fs::read_to_string(&bavns_file).map_err(|e| e.to_string() + " from bavn IO")?;
        let (bavns, _): (Vec<String>, PathBuf) =
            serde_lexpr::from_str(&bavns_contents).map_err(|e| e.to_string() + " from serde")?;

        Ok((script, bavns))
    }
}

fn deserialize_from_metadata_f(bavns_file: &PathBuf) -> Result<(Script, Vec<String>), String> {
    let bavns_contents =
        fs::read_to_string(&bavns_file).map_err(|e| e.to_string() + " from bavn IO")?;
    let (bavns, script_file): (Vec<String>, PathBuf) =
        serde_lexpr::from_str(&bavns_contents).map_err(|e| e.to_string() + " from serde")?;

    let script_contents =
        fs::read_to_string(&script_file).map_err(|e| e.to_string() + " from IO")?;
    let presult = script(&script_contents).map_err(|e| e.to_string() + " from parsing")?;

    if presult.0 != "" {
        Err("Incomplete Parse!".to_owned())
    } else {
        let script = presult.1;
        Ok((script, bavns))
    }
}

fn script_f_from_metadata_f(bavns_file: &PathBuf) -> Result<PathBuf, String> {
    let bavns_contents =
        fs::read_to_string(&bavns_file).map_err(|e| e.to_string() + " from bavn IO")?;
    let (_, script_file): (Vec<String>, PathBuf) =
        serde_lexpr::from_str(&bavns_contents).map_err(|e| e.to_string() + " from serde")?;

    Ok(script_file)
}

pub struct PoisonPill {}

fn solver_worker(qin: BavAssingedQ, prev_stage: StageCompleteA) {
    let backoff = Backoff::new();

    while !prev_stage.is_complete() || qin.len() > 0 {
        let filepaths = match qin.pop() {
            Ok(item) => item,
            Err(_) => {
                backoff.snooze();
                continue;
            }
        };

        let (script, _) = match deserialize_from_f(&filepaths) {
            Ok(deserial) => deserial,
            Err(e) => {
                println!("solver deserial err: {}", e);
                continue;
            }
        };

        println!("Checking file {:?}", filepaths.0);
        let outcome = solve(filepaths.0.to_str().unwrap_or("defaultname"));
        match outcome {
            SolveResult::ErrorBug | SolveResult::SoundnessBug => {
                report_bug(filepaths.0.as_path(), outcome)
            }
            SolveResult::Timeout => {
                println!("Timeout on file {:?}", filepaths.0);
                fs::remove_file(&filepaths.0).unwrap_or(());
                fs::remove_file(&filepaths.1).unwrap_or(());
            }
            _ => {
                fs::remove_file(&filepaths.0).unwrap_or(());
                fs::remove_file(&filepaths.1).unwrap_or(());
            }
        }
        println!("Done hecking file {:?}", &filepaths.0);
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
        let rng = Xoshiro256Plus::seed_from_u64(99990);

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

pub fn strip_and_transform(source_file: &Path) -> Option<(Script, Vec<String>)> {
    let contents: String = fs::read_to_string(source_file).ok()?;
    let stripped_contents = &rmv_comments(&contents[..]).ok()?.1.join(" ")[..];
    let mut script = script(&stripped_contents[..]).ok()?.1;
    // TODO error handling here on prev 3 lines
    println!("Done parsing");

    if script.is_unsupported_logic() {
        return None;
    }

    let bavns = to_skel(&mut script).ok()?;
    println!("Done skelling");
    return Some((script, bavns));
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
    (s, mut vs): (&'a str, Vec<&'a str>),
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
    let (rem, ss) = dynamic_format_parser((s, vs))?;
    println!("rem {:?}", rem);
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
    use std::path::PathBuf;
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
}
