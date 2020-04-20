extern crate nom;
extern crate itertools;
extern crate rand;
extern crate rand_core;
extern crate rand_xoshiro;
extern crate walkdir;

#[macro_use]
pub mod ast;

pub mod solver;
pub mod parser;
pub mod transforms;

use solver::solve;
use bit_vec::BitVec;
use parser::{
    rmv_comments, script,
};
#[allow(unused)]
use ast::Script;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use transforms::{end_insert_pt, get_bav_assign, to_skel};
use crossbeam::queue::SegQueue;
use std::path::PathBuf;
use std::thread;
use std::sync::Arc;
use crossbeam::utils::Backoff;

pub fn exec() {
    let q = SegQueue::new();
    for entry in WalkDir::new("known/8")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
                let filepath = entry.into_path();
                q.push(Ok(filepath));
    }
    q.push(Err(PoisonPill{}));

    let aq = Arc::new(q);

    const STACK_SIZE: usize = 20 * 1024 * 1024; // 20mb
    let handles = (0..2).map(|_| {
             let t_q = Arc::clone(&aq);

             thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(|| mutator_worker(t_q))
    });

    for h in handles {
        h.unwrap().join().unwrap();
    }
}

fn mutator_worker(q : Arc<SegQueue<Result<PathBuf, PoisonPill>>>) {
    let backoff = Backoff::new();

    let mut stage_finished = false; 
    while !stage_finished {
        let filepath = match q.pop() {
            Ok(Ok(filepath)) => filepath,
            Ok(Err(poison_pill)) => {
                q.push(Err(poison_pill));
                stage_finished = true;
                continue;
            },
            Err(_) => { backoff.snooze(); continue; },
        };

        println!("fp {:?}", filepath);
        backoff.reset();
        match strip_and_transform(&filepath) {
            Some((script, bavns)) => {
                let skel_name = get_new_name(&filepath, "skel");
                fs::write(skel_name, script.to_string())
                    .unwrap_or(());
            },
            None => continue,
        }
    }
}

fn get_new_name(source_file : &PathBuf, prefix : &str) -> String {
    let source_filename = match source_file.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => "unknown",
    };
    prefix.to_owned() + "_" + source_filename
}

pub struct PoisonPill {}

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

pub fn strip_and_transform(source_file: &Path) ->
        Option<(Script, Vec<String>)> {
    let contents: String =
        fs::read_to_string(source_file).ok()?;
    let stripped_contents = &rmv_comments(&contents[..]).ok()?
        .1
        .join(" ")[..];
    let mut script = script(&stripped_contents[..]).ok()?.1;
    // TODO error handling here on prev 3 lines

    if script.is_unsupported_logic() {
        return None;
    }

    let bavns = to_skel(&mut script);
    return Some((script, bavns));
}

fn iter(mut script : Script, bavns : Vec<String>, source_file : &Path) {
    let eip = end_insert_pt(&script);
    script.init(eip);

    let num_bavs = bavns.len();
    const MAX_ITER : u32 = 1;
    println!("starting max(2^{}, {}) iterations", num_bavs, MAX_ITER);
    let mut urng = RandUniqPermGen::new_definite(num_bavs, MAX_ITER);
    while let Some(truth_values) = urng.sample() {
        let filename = get_iter_fileout_name(source_file, urng.get_count());
        script.replace(eip, get_bav_assign(&bavns, truth_values));
        fs::write(&filename, script.to_string()).unwrap_or(());
        if solve(&filename) {
            fs::remove_file(filename).unwrap_or(());
        }
    }
    println!("Done with seed file");
}

fn get_iter_fileout_name(source_file: &Path, iter: u32) -> String {
    let source_filename = match source_file.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => "unknown",
    };
    (iter).to_string() + "_" + source_filename
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::path::PathBuf;

    const STACK_SIZE: usize = 20 * 1024 * 1024;

    #[test]
    fn debug() {
        exec();
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

            let up = p.to_string();
            let pup = script(&up[..]).expect("reparse error").1;
            assert_eq!(p, pup);
        }
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
        assert_eq!(rng.sample().expect("Should hold value").len(), 9);
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

    #[test]
    fn quick_visual() {
        let mut s = parse_file("samples/bug272.minimized.smtv1.smt2");
        println!("Before \n {} \n\n", s.to_string());
        to_skel(&mut s);
        println!("Skeleton \n {}", s.to_string());
    }
}
