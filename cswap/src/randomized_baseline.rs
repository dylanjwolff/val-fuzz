use crate::ast::BitVecConst;
use crate::ast::Command;
use crate::ast::Constant;
use crate::ast::FPConst;
use crate::ast::SExp;
use crate::ast::Script;
use crate::ast::Sort;
use crate::config::Config;
use crate::config::Metadata;
use crate::liftio;
use crate::parser::script_from_f;
use crate::solver::all_non_err_timed_out;
use crate::solver::check_valid_solve;
use crate::solver::profiles_solve;
use crate::utils::RunStats;

use crate::fuzzer::report_any_bugs;
use arbitrary::Arbitrary;
use arbitrary::Unstructured;
use log::warn;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub struct StatefulRandBaseFuzzer<'a> {
    script: Script,
    md_file: PathBuf,
    md: Metadata,
    cfg: &'a Config,
    rng: Xoshiro256Plus,
    stats: &'a mut RunStats,
    current_iter: u64,
}

impl<'a> StatefulRandBaseFuzzer<'a> {
    pub fn new(seed_f: &'a Path, cfg: &'a Config, stats: &'a mut RunStats) -> io::Result<Self> {
        let mut md = Metadata::new(&seed_f);
        if check_valid_solve(seed_f.to_str().unwrap())
            .iter()
            .all(|r| r.has_unrecoverable_error() && !r.has_bug_error())
        {
            return liftio!(Err(format!(
                "Unmodified file {} failed to pass initial validation check",
                md.seed_file
            )));
        }

        let mut script = script_from_f(&seed_f)?;
        let mut rng = Xoshiro256Plus::seed_from_u64(cfg.get_specific_seed(&seed_f));
        let md_file = cfg.file_provider.serialize_md(&mut md)?;
        Ok(StatefulRandBaseFuzzer {
            script: script,
            md_file: md_file,
            md: md,
            cfg: cfg,
            rng: rng,
            stats: stats,
            current_iter: 0,
        })
    }

    pub fn next(&mut self) -> io::Result<Option<(PathBuf, PathBuf)>> {
        if self.current_iter >= self.cfg.max_iter as u64 {
            return liftio!(Err("done")); // TODO hack
        }
        self.current_iter = self.current_iter + 1;

        let consts = fuzz(&mut self.script, &mut self.rng);
        if consts == 0 {
            return liftio!(Err(format!(
                "{} has no constants to replace",
                self.md.seed_file
            )));
        }

        let script_str = self.script.to_string();
        if !self.stats.record_sub(&script_str) {
            return Ok(None);
        }

        self.cfg
            .file_provider
            .serialize_iterfile_str(&script_str, self.current_iter, &mut self.md)
            .map(|r| Some((r, self.md_file.clone())))
    }
}

pub fn rand_fuzz_solve(
    iter_f: &Path,
    md_f: &Path,
    cfg: &Config,
    stats: &mut RunStats,
    to_ctr: &mut BTreeMap<String, u64>,
) -> io::Result<()> {
    let md_contents = fs::read_to_string(md_f)?;
    let md: Metadata = liftio!(serde_lexpr::from_str(&md_contents))?;

    if to_ctr.get(&md.seed_file).map(|sf| *sf > 3).unwrap_or(false) {
        return liftio!(Err(format!(
            "{} exceeded max consec timeouts",
            md.seed_file
        )));
    }

    let results = profiles_solve(iter_f.to_str().unwrap_or("defaultname"), &cfg.profiles);
    stats.record_stats_for_sub_results(&results);

    let ct = to_ctr.entry(md.seed_file.clone()).or_insert(0);
    if all_non_err_timed_out(&results) {
        *ct = *ct + 1;
    } else if *ct > 0 {
        *ct = *ct - 1;
    }

    if !report_any_bugs(&iter_f, &results, &cfg.file_provider) {
        if cfg.remove_files {
            fs::remove_file(&iter_f).unwrap_or(());
        }
    }
    Ok(())
}

fn fuzz(script: &mut Script, rng: &mut Xoshiro256Plus) -> u64 {
    let Script::Commands(cmds) = script;
    let mut count = 0;
    for cmd in cmds {
        count = count + fuzz_cmd(&mut cmd.borrow_mut(), rng);
    }
    count
}

fn fuzz_cmd(cmd: &mut Command, rng: &mut Xoshiro256Plus) -> u64 {
    match cmd {
        Command::Assert(sexp) => fuzz_sexp(&mut sexp.borrow_mut(), rng),
        _ => 0,
    }
}

fn fuzz_sexp(sexp: &mut SExp, rng: &mut Xoshiro256Plus) -> u64 {
    match sexp {
        SExp::Constant(c) => {
            fuzz_const(&mut c.borrow_mut(), rng);
            1
        }
        SExp::Compound(v)
        | SExp::BExp(_, v)
        | SExp::NExp(_, v)
        | SExp::FPExp(_, _, v)
        | SExp::StrExp(_, v) => {
            let mut count = 0;
            for rec_sexp in v {
                count = count + fuzz_sexp(&mut rec_sexp.borrow_mut(), rng)
            }
            count
        }
        SExp::QExists(_, rec_sexp) | SExp::QForAll(_, rec_sexp) => {
            fuzz_sexp(&mut *rec_sexp.borrow_mut(), rng)
        }
        SExp::Let(bindings, rec_sexp) => {
            let mut count = 0;
            for (_, binding_sexp) in bindings {
                count = count + fuzz_sexp(&mut binding_sexp.borrow_mut(), rng);
            }
            count = count + fuzz_sexp(&mut rec_sexp.borrow_mut(), rng);
            count
        }
        SExp::Symbol(_) => 0,
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
                        format!("{:08b}", z)
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
    let b: &mut [u8] = &mut [0; 16];
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
            script("(assert (= 0.0 \" \"))(assert (= #xfff (+ #b101 4 (_ +zero 2 4))))")
                .unwrap()
                .1;
        let mut md = Metadata::new_empty();
        let mut rng = Xoshiro256Plus::seed_from_u64(18);
        fuzz(&mut script, &mut rng);
        assert_display_snapshot!(script);
    }
}
