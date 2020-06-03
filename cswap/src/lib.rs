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
use nom::error::ErrorKind;
use nom::error_position;
use nom::multi::many1;
use std::io;

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

macro_rules! liftio {
    ($x:expr) => {
        $x.map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Lifted: {:?}", e)))
    };
}

#[derive(Debug, Clone)]
pub struct Config {
    pub file_provider: FileProvider,
    pub max_iter: u32,
    pub rng_seed: u64,
}

impl Config {
    fn get_specific_seed<T: Hash>(&self, t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        self.rng_seed.hash(&mut s);
        s.finish()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FileProvider {
    basedir: PathBuf,
    skeldir: PathBuf,
    mddir: PathBuf,
    scratchdir: PathBuf,
    bugdir: PathBuf,
}

impl FileProvider {
    pub fn new(dirname: &str) -> FileProvider {
        let dirpath = Path::new(dirname).to_path_buf();
        let skeldir = Self::get_subdir(&dirpath, "skel");
        let mddir = Self::get_subdir(&dirpath, "md");
        let scratchdir = Self::get_subdir(&dirpath, "scratch");
        let bugdir = Self::get_subdir(&dirpath, "bugs");
        fs::create_dir(&dirpath).unwrap();
        fs::create_dir(&skeldir).unwrap();
        fs::create_dir(&mddir).unwrap();
        fs::create_dir(&scratchdir).unwrap();
        fs::create_dir(&bugdir).unwrap();
        FileProvider {
            basedir: dirpath,
            skeldir: skeldir,
            mddir: mddir,
            scratchdir: scratchdir,
            bugdir: bugdir,
        }
    }

    fn skelfile<'a>(&self, md: &'a mut Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("skel_")
            .rand_bytes(0)
            .suffix(&md.seed_file)
            .tempfile_in(&self.skeldir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        md.skeleton_file = name(&pb);
        Ok(pb)
    }

    fn mdfile<'a>(&self, md: &'a mut Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("md_")
            .rand_bytes(0)
            .suffix(&md.seed_file)
            .tempfile_in(&self.mddir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        md.metadata_file = name(&pb);
        Ok(pb)
    }

    fn iterfile(&self, md: &Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("iter_")
            .rand_bytes(10)
            .suffix(&md.seed_file)
            .tempfile_in(&self.scratchdir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        Ok(pb)
    }

    fn resubfile(&self, md: &Metadata) -> io::Result<PathBuf> {
        let tfile = Builder::new()
            .prefix("resub_")
            .rand_bytes(10)
            .suffix(&md.seed_file)
            .tempfile_in(&self.scratchdir)?;
        let (_, pb) = liftio!(tfile.keep())?;
        Ok(pb)
    }

    fn bug_report(&self, buggy_file: &Path, report: &str) {
        let r = Builder::new()
            .prefix("bug_report_")
            .rand_bytes(0)
            .suffix(
                buggy_file
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("default_name"),
            )
            .tempfile_in(&self.bugdir)
            .and_then(|tf| {
                fs::write(tf.path(), report)?;
                liftio!(tf.keep())
            });
        match r {
            Err(e) => println!("Error writing bug report for {:?}: {}", buggy_file, e),
            _ => (),
        }
    }

    fn cleanup_all(&self) {
        match fs::remove_dir_all(&self.basedir) {
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => panic!(e),
            },
            _ => (),
        }
    }

    fn serialize_skel<'a, 'b>(
        &self,
        script: &'a Script,
        md: &'b mut Metadata,
    ) -> Result<PathBuf, io::Error> {
        let f = self.skelfile(md)?;
        fs::write(&f, script.to_string())?;
        Ok(f)
    }

    fn serialize_md<'a>(&self, md: &'a mut Metadata) -> Result<PathBuf, io::Error> {
        let f = self.mdfile(md)?;
        let mdstr = serde_lexpr::to_string(&md)?;
        fs::write(&f, mdstr)?;
        Ok(f)
    }

    fn serialize_skel_and_md<'a, 'b>(
        &self,
        script: &'a Script,
        md: &'b mut Metadata,
    ) -> Result<(PathBuf, PathBuf), io::Error> {
        let fs = self.serialize_skel(script, md)?;
        let fm = self.serialize_md(md)?;
        Ok((fs, fm))
    }

    fn serialize_resub(&self, script: &Script, md: &Metadata) -> Result<PathBuf, io::Error> {
        let f = self.resubfile(md)?;
        fs::write(&f, script.to_string())?;
        Ok(f)
    }

    fn get_subdir(base: &Path, subname: &str) -> PathBuf {
        let mut skeldir = base.to_path_buf();
        skeldir.push(subname);
        skeldir
    }
}

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
    seed_file: String,
    skeleton_file: String,
    metadata_file: String,
}
pub fn name(pb: &Path) -> String {
    pb.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("default_filename")
        .to_owned()
}

impl Metadata {
    pub fn new_empty() -> Self {
        Metadata {
            bavns: vec![],
            constvns: vec![],
            seed_file: "".to_owned(),
            skeleton_file: "".to_owned(),
            metadata_file: "".to_owned(),
        }
    }
    pub fn new(seed: &Path) -> Self {
        Metadata {
            bavns: vec![],
            constvns: vec![],
            seed_file: name(seed),
            skeleton_file: "".to_owned(),
            metadata_file: "".to_owned(),
        }
    }

    pub fn seed_path(&self, fp: &FileProvider) -> PathBuf {
        panic!("Unimplemented!");
    }

    pub fn skel_path(&self, fp: &FileProvider) -> PathBuf {
        let mut p = fp.skeldir.clone();
        p.push(&self.skeleton_file);
        p
    }

    pub fn md_path(&self, fp: &FileProvider) -> PathBuf {
        let mut p = fp.mddir.clone();
        p.push(&self.metadata_file);
        p
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
    fn new_definite_seeded(seed: u64, numbits: usize, maxiter: u32) -> Self {
        let buf = BitVec::from_elem(numbits, false).to_bytes();
        let seen = BTreeSet::new();
        let rng = Xoshiro256Plus::seed_from_u64(seed);

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

    fn new_definite(numbits: usize, maxiter: u32) -> Self {
        Self::new_definite_seeded(0, numbits, maxiter)
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
    let mut script = parser::script_from_f(source_file).ok()?;
    // TODO error handling here on prev 3 lines

    if script.is_unsupported_logic() {
        return None;
    }

    let mut md = Metadata::new(source_file);
    to_skel(&mut script, &mut md).ok()?;
    return Some((script, md));
}

fn get_iter_fileout_name(source_file: &Path, iter: u32) -> String {
    let source_filename = match source_file.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => "unknown",
    };
    (iter).to_string() + "_" + source_filename
}

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
        let fmt_str = get_bav_assign_fmt_str(&vec!["BAV1".to_owned()]).to_string();
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
