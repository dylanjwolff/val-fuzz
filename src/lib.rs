#[macro_use]
extern crate nom;

pub mod parser;

use parser::{rmv_comments, script, Command, Sort, Constant, SExp, Script};

use std::fs;
use std::process;
use std::str::from_utf8;

struct VarNameGenerator<'a> {
    basename: String,
    counter: u32,
    vars_generated: Vec<(String, Sort<'a>)>,
}

impl<'a> VarNameGenerator<'a> {
    fn get_name(&mut self, sort : Sort<'a>) -> String {
        self.counter = self.counter + 1;
        let name = format!("{}{}", self.basename, self.counter);
        self.vars_generated.push((name.clone(), sort));
        name
    }
}

pub fn exec() {
    let files = fs::read_dir("samples").expect("error with sample dir");

    for file_res in files {
        let file = file_res.expect("problem with file");
        println!("Starting {:?}", file);
        let filepath = file.path();
        let contents = &fs::read_to_string(filepath).expect("error reading file")[..];
        let contents_ = &rmv_comments(contents)
            .expect("failed to rmv comments").1.join(" ")[..];
        match (script(contents_), script(contents_)) {
            (Ok((_, a)), Ok((_, b))) => assert_eq!(
                a,
                script(&b.to_string()[..])
                    .expect("Failed on second parse")
                    .1
            ),
            (Err(e), _) | (_, Err(e)) => panic!("SMT Parse Error {}", e),
        }
    }
}

fn rc(script: &mut Script, vng : &mut VarNameGenerator){
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds {
                rc_c(cmd, vng);
            }
        }
    }
}

fn rc_c(cmd: &mut Command, vng : &mut  VarNameGenerator) {
    match cmd {
        Command::Assert(sexp) | Command::CheckSatAssuming(sexp) => rc_se(sexp, vng),
        _ => (),
    }
}

fn rc_se(sexp: &mut SExp, vng : &mut VarNameGenerator) {
    match sexp {
        SExp::Constant(c) => {
            let sort = match c {
                Constant::UInt(_) => Sort::UInt(),
                Constant::Dec(_) => Sort::Dec(),
                Constant::Str(_) => Sort::Str(),
                Constant::Bool(_) => Sort::Bool(),
                Constant::Bin(_) |
                Constant::Hex(_) => Sort::BitVec(),
            };
            let name = vng.get_name(sort);
            *sexp = SExp::Var(name);
        },
        SExp::Compound(sexps) => {
            for sexp in sexps {
                rc_se(sexp, vng)
            }
        }
        _ => (),
    }
}

fn bav_se<'a>(sexp: &'a mut SExp<'a>, vng : &mut VarNameGenerator, bavs : &'a mut Vec<(String, SExp<'a>)>) {
    match sexp {
        SExp::Compound(sexps) => {
            let name = vng.get_name(Sort::Bool());
            let sec = sexp.clone();
            bavs.push((name, sec));
            for sexp in sexps {
                bav_se(sexp, vng, bavs);
            }
        },
        _ => (),
    }
}

fn solve(filename: &str) {
    let cvc4_res = process::Command::new("cvc4")
        .args(&[filename, "--produce-model", "--tlimit", "5000"])
        .output();

    let z3_res = process::Command::new("z3")
        .args(&[filename, "-T:5"])
        .output();

    let cvc4_stdout_res = cvc4_res
        .and_then(|out| {
            if !out.status.success() && out.stderr.len() > 0 {
                println!(
                    "cvc4 error on file {} : {}",
                    filename,
                    from_utf8(&out.stderr[..]).unwrap()
                );
                Err(std::io::Error::last_os_error()) // really sloppy hack for now, needs to be fixed
            } else {
                Ok(out)
            }
        })
        .map(|out| from_utf8(&out.stdout.clone()[..]).map(|s| s.to_string()));

    let z3_stdout_res = z3_res
        .and_then(|out| {
            if !out.status.success() && out.stderr.len() > 0 {
                println!("z3 error on file {}", filename);
                Err(std::io::Error::last_os_error()) // really sloppy hack for now, needs to be fixed
            } else {
                Ok(out)
            }
        })
        .map(|out| from_utf8(&out.stdout.clone()[..]).map(|s| s.to_string()));

    match (cvc4_stdout_res, z3_stdout_res) {
        (Ok(Ok(cvc4_stdout)), Ok(Ok(z3_stdout))) => {
            // also sloppy hack above
            if cvc4_stdout.contains("unsat") && !z3_stdout.contains("unsat") {
                println!("file {} has soundness problem!!!", filename);
            } else if cvc4_stdout.contains("sat") && !z3_stdout.contains("sat") {
                println!("file {} has soundness problem!!!", filename);
            } else {
                fs::remove_file(filename);
            }
            ()
        }
        _ => println!("Error with file {}", filename),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn t() {
    }

    #[test]
    fn smoke_test() {
        // exec();
    }

    #[test]
    fn visual_test() {
        let contents = &fs::read_to_string("ex.smt2").expect("error reading file")[..];
        let mut vng = VarNameGenerator {
            basename : "GEN".to_string(),
            counter : 0,
            vars_generated : vec![],
        };
        let mut s = script(contents).expect("parse problem").1;
        rc(&mut s, &mut vng);

        println!("Script: {:?}", s);
        println!("restrung: {}", s.to_string());
    }
}
