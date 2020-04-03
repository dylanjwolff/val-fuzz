#[macro_use]
extern crate nom;
extern crate itertools;

pub mod parser;

use parser::{rmv_comments, script, Command, Sort, Constant, SExp, Script, BoolOp};

use std::path::PathBuf;
use std::fs;
use std::process;
use std::str::from_utf8;
use itertools::Itertools;

struct VarNameGenerator {
    basename: String,
    counter: u32,
    vars_generated: Vec<(String, Sort)>,
}

impl VarNameGenerator {
    fn get_name(&mut self, sort : Sort) -> String {
        self.counter = self.counter + 1;
        let name = format!("{}{}", self.basename, self.counter);
        self.vars_generated.push((name.clone(), sort));
        name
    }

    fn new(base : &str) -> VarNameGenerator {
        VarNameGenerator {
            basename : base.to_owned(),
            counter : 0,
            vars_generated : vec![],
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
        SExp::Compound(sexps) |
        SExp::BExp(_, sexps) => {
            for sexp in sexps {
                rc_se(sexp, vng)
            }
        }
        _ => (),
    }
}

fn bav(script: &mut Script, vng : &mut VarNameGenerator, bava : &mut Vec<(String, SExp)>){
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds {
                bav_c(cmd, vng, bava);
            }
        }
    }
}

fn bav_c(cmd: &mut Command, vng : &mut  VarNameGenerator, bava : &mut Vec<(String, SExp)>){
    match cmd {
        Command::Assert(sexp) | Command::CheckSatAssuming(sexp) => bav_se(sexp, vng, bava),
        _ => (),
    }
}

fn bav_se(sexp: &mut SExp, vng : &mut VarNameGenerator, bavs : &mut Vec<(String, SExp)>) {
    match sexp {
        SExp::BExp(bop, sexps) => {
            let name = vng.get_name(Sort::Bool());
            let sec = SExp::BExp(bop.clone(), sexps.clone());
            bavs.push((name, sec));
            for sexp in sexps {
                bav_se(sexp, vng, bavs);
            }
        },
        SExp::Compound(sexps) => {
            for sexp in sexps {
                bav_se(sexp, vng, bavs);
            }
        },
        _ => (),
    }
}


fn init_vars(script : &mut Script, vars : Vec<(String, Sort)>) {
    let Script::Commands(cmds) = script;

    let maybe_log_pos = cmds.iter().position(|cmd| cmd.is_logic());

    let log_pos = match maybe_log_pos {
        Some(pos) => pos,
        None => 0,
    };

    let mut end = cmds.split_off(log_pos + 1);

    let mut decls = vars.into_iter()
        .map(|(vname, sort)| Command::DeclConst(vname, sort))
        .collect::<Vec<Command>>();

    cmds.append(&mut decls);
    cmds.append(&mut end);
}

fn add_ba(script : &mut Script, bavs : Vec<(String, SExp)>) {
    let Script::Commands(cmds) = script;

    let maybe_cs_pos = cmds.iter().position(|cmd| cmd.is_checksat());

    let cs_pos = match maybe_cs_pos {
        Some(pos) => pos,
        None => cmds.len(),
    };

    let mut baveq_iter = bavs.into_iter()
        .map(|(vname, sexp)| {
            SExp::BExp(BoolOp::Equals(), vec![SExp::Symbol(vname), sexp])
        });

    cmds.insert(cs_pos, assert_many(&mut baveq_iter));
}

fn assert_many(iter: &mut dyn Iterator<Item = SExp>) -> Command {

    let init = match iter.next() {
        Some(bav_eq) => bav_eq,
        _ => SExp::Symbol("true".to_owned()), // shouldn't ever actually be added
    };

    let intersection = iter
        .fold(init, |acc, curr| SExp::BExp(BoolOp::And(), vec![acc, curr]));

    return Command::Assert(intersection)
}

fn end_insert_pt(script : &Script) -> usize {
    let Script::Commands(cmds) = script;

    let maybe_cs_pos = cmds.iter().position(|cmd| cmd.is_checksat());

    match maybe_cs_pos {
        Some(pos) => pos,
        None => cmds.len(),
    }
}

fn get_bav_assign(bavns : &Vec<String>, ta : Vec<bool>) -> Command {
    let bavs = bavns.into_iter().zip(ta.into_iter());
    let mut baveq = bavs.into_iter()
        .map(|(vname, bval)| {
            let val = if bval { SExp::true_sexp() } else { SExp::false_sexp() };
            SExp::BExp(BoolOp::Equals(), vec![SExp::Symbol(vname.clone()), val])
        });

    assert_many(&mut baveq)
}

/// returns the Boolean Abstract Variables added as vector of their names 
fn to_skel(script : &mut Script) -> Vec<String> {
    let mut vng = VarNameGenerator::new("GEN");
    rc(script, &mut vng);

    vng.basename = "BAV".to_owned();
    let mut bavs = vec![];
    bav(script, &mut vng, &mut bavs);

    init_vars(script, vng.vars_generated);
    let bavns = bavs.iter()
        .map(|(name, _)| name.clone()).collect::<Vec<String>>();
    add_ba(script, bavs);
    bavns
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


pub fn strip_and_test_file(source_file: &PathBuf) {
    let contents: String =
        fs::read_to_string(source_file).expect("Something went wrong reading the file");
    let stripped_contents = &rmv_comments(&contents[..]).expect("Error stripping comments").1.join(" ")[..];
    let mut script = script(&stripped_contents[..]).expect("Parsing error").1;
    // TODO error handling here on prev 3 lines

    if script.is_unsupported_logic() { return }

    let bavns = to_skel(&mut script);
    let eip = end_insert_pt(&script);
    script.init(eip);
    let num_bavs = bavns.len();
    let mut iterations = 0;

    println!("starting {} iterations", 2_u64.pow(num_bavs as u32));
    for truths in 0..num_bavs + 1 {
        let mut unordered_tvs = vec![true; truths];
        unordered_tvs.extend(vec![false; num_bavs - truths]);
        let truth_value_assigments = unordered_tvs.into_iter().permutations(num_bavs).unique();
        let mut inner_iterations = 0;
        for truth_values in truth_value_assigments {
            let source_filename = match source_file.file_name().and_then(|n| n.to_str()) {
                Some(name) => name,
                None => "unknown",
            };
            let filename = (iterations).to_string() + "_" + source_filename;



            script.replace(eip, get_bav_assign(&bavns, truth_values));
            fs::write(&filename, script.to_string());
            solve(&filename);



            inner_iterations = inner_iterations + 1;
            iterations = iterations + 1;
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    #[ignore]
    fn parse_unparse() {
        let files = fs::read_dir("samples").expect("error with sample dir");

        for file_res in files {
            let file = file_res.expect("problem with file");
            println!("Starting {:?}", file);
            let filepath = file.path();
            let contents = &fs::read_to_string(filepath).expect("error reading file")[..];
            let contents_sans_comments = &rmv_comments(contents)
                .expect("failed to rmv comments").1.join(" ")[..];

            let p = script(contents_sans_comments).expect("parser error").1;

            let pup = script(&p.to_string()[..]).expect("reparse error").1;

            assert_eq!(p, pup);
        }
    }

    #[test]
    fn smoke_test() {
        let mut pb = PathBuf::new();
        pb.push("samples/bug272.smtv1.smt2");
        strip_and_test_file(&pb);
    }
}
