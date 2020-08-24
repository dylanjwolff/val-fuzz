use crate::ast::BitVecConst;
use crate::ast::Command;
use crate::ast::Constant;
use crate::ast::FPConst;
use crate::ast::SExp;
use crate::ast::Script;
use crate::ast::Sort;
use crate::config::Config;
use crate::config::Metadata;
use crate::parser::script_from_f;
use crate::solver::profiles_solve;
use crate::utils::RunStats;

use crate::fuzzer::report_any_bugs;
use arbitrary::Arbitrary;
use arbitrary::Unstructured;
use log::warn;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

pub fn rand_fuzz_iter_and_solve(
    seed_f: &Path,
    cfg: &Config,
    stats: &mut RunStats,
) -> io::Result<()> {
    let mut md = Metadata::new(&seed_f);
    let mut script = script_from_f(&seed_f)?;
    let mut rng = Xoshiro256Plus::seed_from_u64(1);
    for i in 0..cfg.max_iter as u64 {
        fuzz(&mut script, &mut rng);
        let script_str = script.to_string();
        if !stats.record_sub(&script_str) {
            continue; // Reporting this as an error is probably a bit too noisy
        }
        let iter_f = match cfg.file_provider.serialize_iterfile(&script, i, &mut md) {
            Err(e) => {
                warn!("Serialization error {} for {}", e, md.seed_file);
                continue;
            }
            Ok(iter_f) => iter_f,
        };

        let results = profiles_solve(iter_f.to_str().unwrap_or("defaultname"), &cfg.profiles);
        stats.record_stats_for_sub_results(&results);

        if !report_any_bugs(&iter_f, &results, &cfg.file_provider) {
            if cfg.remove_files {
                fs::remove_file(&iter_f).unwrap_or(());
            }
        }
    }

    Ok(())
}

pub fn fuzz(script: &mut Script, rng: &mut Xoshiro256Plus) {
    let Script::Commands(cmds) = script;
    for cmd in cmds {
        fuzz_cmd(&mut cmd.borrow_mut(), rng);
    }
}

fn fuzz_cmd(cmd: &mut Command, rng: &mut Xoshiro256Plus) {
    match cmd {
        Command::Assert(sexp) => fuzz_sexp(&mut sexp.borrow_mut(), rng),
        _ => (),
    }
}

fn fuzz_sexp(sexp: &mut SExp, rng: &mut Xoshiro256Plus) {
    match sexp {
        SExp::Constant(c) => fuzz_const(&mut c.borrow_mut(), rng),
        SExp::Compound(v)
        | SExp::BExp(_, v)
        | SExp::NExp(_, v)
        | SExp::FPExp(_, _, v)
        | SExp::StrExp(_, v) => {
            for rec_sexp in v {
                fuzz_sexp(&mut rec_sexp.borrow_mut(), rng)
            }
        }
        SExp::QExists(_, rec_sexp) | SExp::QForAll(_, rec_sexp) => {
            fuzz_sexp(&mut *rec_sexp.borrow_mut(), rng)
        }
        SExp::Let(bindings, rec_sexp) => {
            for (_, binding_sexp) in bindings {
                fuzz_sexp(&mut binding_sexp.borrow_mut(), rng)
            }
            fuzz_sexp(&mut rec_sexp.borrow_mut(), rng)
        }
        SExp::Symbol(_) => (),
    }
}

struct BinStr {
    buf: Vec<u8>,
    chars: usize,
}

impl BinStr {
    fn new_random(num_chars: usize, rng: &mut Xoshiro256Plus) -> Self {
        let buf_len = num_chars / 8 + 1;
        let mut v = vec![0; buf_len];
        rng.fill(&mut v[..]);
        BinStr {
            buf: v,
            chars: num_chars,
        }
    }
}

impl fmt::Display for BinStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, z) in self.buf.iter().enumerate() {
            if 8 * (i + 1) > self.chars {
                for j in 8 * i..self.chars {
                    write!(
                        f,
                        "{}",
                        format!("{:b}", z)
                            .chars()
                            .nth(j - 8 * i)
                            .unwrap()
                            .to_string()
                    )?;
                }
            } else if 8 * i < self.chars {
                write!(f, "{:08b}", z)?;
            } else {
                break;
            }
        }
        Ok(())
    }
}

fn fuzz_const(constant: &mut Constant, rng: &mut Xoshiro256Plus) {
    let b: &mut [u8] = &mut [0; 4];
    rng.fill(b);
    let mut u = Unstructured::new(b);

    match constant {
        Constant::UInt(_) => {
            *constant = Constant::UInt(u64::arbitrary(&mut u).unwrap().to_string())
        }
        Constant::Dec(_) => *constant = Constant::Dec(f64::arbitrary(&mut u).unwrap().to_string()),
        Constant::Str(_) => *constant = Constant::Str(String::arbitrary(&mut u).unwrap()),
        Constant::Bool(_) => *constant = Constant::Bool(rng.gen_bool(0.5)),
        Constant::Bin(_) | Constant::Hex(_) | Constant::Fp(_) => match constant.sort() {
            Sort::BitVec(length) => {
                *constant = Constant::Bin(
                    format!("{}", BinStr::new_random(length as usize, rng))
                        .chars()
                        .collect(),
                )
            }
            Sort::Fp(e, s) => {
                let e = e.parse::<usize>().unwrap();
                let s = s.parse::<usize>().unwrap();
                let signb = if rng.gen_bool(0.5) {
                    BitVecConst::Bin(vec!['1'])
                } else {
                    BitVecConst::Bin(vec!['0'])
                };
                let eb =
                    BitVecConst::Bin(format!("{}", BinStr::new_random(e, rng)).chars().collect());
                let sb = BitVecConst::Bin(
                    format!("{}", BinStr::new_random(s - 1, rng))
                        .chars()
                        .collect(),
                );

                *constant = Constant::Fp(FPConst::Num(signb, eb, sb))
            }
            _ => panic!("Unreachable Code!"),
        },
    }
}

mod test {
    use super::*;

    use crate::parser::script;
    use insta::assert_display_snapshot;

    #[test]
    fn randomized_baseline() {
        let mut script =
            script("(declare-fun z () Real)(assert (= \" \" #xfff (+ #b101 4 (_ +zero 2 4))))")
                .unwrap()
                .1;
        let mut md = Metadata::new_empty();
        let mut rng = Xoshiro256Plus::seed_from_u64(1);
        fuzz(&mut script, &mut rng);
        assert_display_snapshot!(script);
    }
}
