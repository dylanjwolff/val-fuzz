extern crate nom;
extern crate itertools;
extern crate rand;
extern crate rand_core;
extern crate rand_xoshiro;

#[macro_use]
pub mod ast;

pub mod parser;
pub mod transforms;

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
use std::path::PathBuf;
use std::process;
use std::str::from_utf8;
use std::str::from_utf8_unchecked;
use transforms::{end_insert_pt, get_bav_assign, to_skel};

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
        let rng = Xoshiro256Plus::seed_from_u64(9999);

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

fn solve(filename: &str) {
    let cvc4_res = process::Command::new("timeout")
        .args(&[
            "6s",
            "cvc4",
            filename,
            "--produce-model",
            "--tlimit",
            "5000",
        ])
        .output();

    let z3_res = process::Command::new("timeout")
        .args(&["6s", "z3", filename, "-T:5"])
        .output();


    let cvc4mrs = cvc4_res.map(|out| {
        let stderr = from_utf8(&out.stderr[..]).unwrap();
        let stdout = from_utf8(&out.stdout[..]).unwrap();
        let success = out.status.success();
        (success, stderr.to_owned(), stdout.to_owned())
    });

    let z3mrs = z3_res.map(|out| {
        let stderr = from_utf8(&out.stderr[..]).unwrap();
        let stdout = from_utf8(&out.stdout[..]).unwrap();
        let success = out.status.success();
        (success, stderr.to_owned(), stdout.to_owned())
    });

    match (cvc4mrs, z3mrs) {
        (Ok((cvc4_succ, cvc4_out, cvc4_err)), Ok((z3_succ, z3_out, z3_err))) => {
            let z3_unsat = z3_out.contains("unsat") || z3_err.contains("\nunsat");
            let z3_sat = !z3_unsat && (z3_out.contains("sat") || z3_err.contains("\nsat"));
            let z3_unknown = !z3_unsat && !z3_sat && (z3_out.contains("unknown")
                                                      || z3_err.contains("\nunknown"));
            let cvc4_unsat = cvc4_out.contains("unsat") || cvc4_err.contains("unsat\n");
            let cvc4_sat = !cvc4_unsat && (cvc4_out.contains("sat") || cvc4_err.contains("sat\n"));
            let cvc4_unknown = !cvc4_unsat && !cvc4_sat && (cvc4_out.contains("unknown")
                                                      || cvc4_err.contains("unknown\n"));

            if !z3_succ && z3_err.len() > 0 && !z3_sat && !z3_unsat {
               println!("z3 unsuccessful on file {} : {}", filename, z3_err);
            } else if !cvc4_succ && cvc4_err.len() > 0 && !cvc4_sat && !cvc4_unsat {
               println!("cvc4 unsuccessful on file {} : {}", filename, cvc4_err);
            } else if z3_unknown || cvc4_unknown {
               println!("unknown result for file {}", filename);
               fs::remove_file(filename)
                    .unwrap_or(());
            } else if cvc4_sat && z3_unsat {
               println!("file {} has soundness problem!!!", filename);
            } else if cvc4_unsat && z3_sat && !z3_err.contains("unknown function/constant")
                && !z3_err.contains("unknown constant")
                && !z3_err.contains("unknown parameter") {
               println!("file {} has soundness problem!!!", filename);
            } else if cvc4_out.contains("timeout") || z3_out.contains("timeout") {
               println!("timeout on file {}", filename);
               fs::remove_file(filename)
                    .unwrap_or(());
            } else {
               fs::remove_file(filename)
                    .unwrap_or(());
            }

            if cvc4_succ && z3_succ {
                println!("parse success for file :{}", filename);
            }

        },
        (Err(e), _) => println!("cvc4 process error on file {} : {}", filename, e),
        (_, Err(e)) => println!("z3 process error on file {} : {}", filename, e),
    };
}

pub fn strip_and_test_file(source_file: &PathBuf) {
    let contents: String =
        fs::read_to_string(source_file).expect("Something went wrong reading the file");
    let stripped_contents = &rmv_comments(&contents[..])
        .expect("Error stripping comments")
        .1
        .join(" ")[..];
    let mut script = script(&stripped_contents[..]).expect("Parsing error").1;
    // TODO error handling here on prev 3 lines

    if script.is_unsupported_logic() {
        return;
    }

    let bavns = to_skel(&mut script);
    let eip = end_insert_pt(&script);
    script.init(eip);

    let num_bavs = bavns.len();
    const MAX_ITER : u32 =1;
    println!("starting max(2^{}, {}) iterations", num_bavs, MAX_ITER);
    let mut urng = RandUniqPermGen::new_definite(num_bavs, MAX_ITER);
    while let Some(truth_values) = urng.sample() {
        let filename = get_iter_fileout_name(source_file, urng.get_count());
        script.replace(eip, get_bav_assign(&bavns, truth_values));
        fs::write(&filename, script.to_string()).unwrap_or(());
        solve(&filename);
    }
    println!("Done with seed file");
}

fn get_iter_fileout_name(source_file: &PathBuf, iter: u32) -> String {
    let source_filename = match source_file.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => "unknown",
    };
    (iter).to_string() + "_" + source_filename
}

pub fn exec() {
    let files = fs::read_dir("samples").expect("error with sample dir");

    for file_res in files {
        match file_res {
            Ok(file) => {
                let filepath = file.path();
                println!("starting file {:?}", filepath);
                strip_and_test_file(&filepath);
            }
            Err(_) => (),
        }
    }
}

pub fn perf_exec() {
    let files = fs::read_dir("perf").expect("error with sample dir");

    for file_res in files {
        match file_res {
            Ok(file) => {
                let filepath = file.path();
                println!("starting file {:?}", filepath);
                let contents: String =
                fs::read_to_string(&filepath).expect("Something went wrong reading the file");
                let stripped_contents = &rmv_comments(&contents[..])
                    .expect("Error stripping comments")
                    .1
                    .join(" ")[..];
                let _script = script(&stripped_contents[..]).expect("Parsing error").1;
            }
            Err(_) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;

    const STACK_SIZE: usize = 20 * 1024 * 1024;

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
    fn smoke_test() {
        let mut pb = PathBuf::new();
        pb.push("samples/ex.smt2");
        strip_and_test_file(&pb);
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
