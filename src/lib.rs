#[macro_use]
extern crate nom;
extern crate itertools;
extern crate rand_xoshiro;
extern crate rand_core;
extern crate rand;
#[macro_use]
pub mod parser;

use std::cell::RefMut;
use std::cell::RefCell;
use rand_core::RngCore;
use bit_vec::BitVec;
use parser::{ToStringVisitor, traverse, AstNode, rmv_comments, script, Symbol, Command, Sort, Constant, SExp, Script, BoolOp};
use parser::{SymbolRc, CommandRc, SortRc, ConstantRc, SExpRc, ScriptRc, BoolOpRc};
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use std::path::PathBuf;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::process;
use std::str::from_utf8;
use std::rc::Rc;

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

struct RandUniqPermGen {
    rng : Xoshiro256Plus,
    numbits : usize,
    buf : Vec<u8>,
    seen : BTreeSet<BitVec>,
    retries : u16,
    max : u32,
    use_max : bool,
    use_retries : bool,
}

use rand_xoshiro::Xoshiro256Plus;
impl RandUniqPermGen {

    fn new_definite(numbits : usize, maxiter: u32) -> Self {
        let buf = BitVec::from_elem(numbits, false).to_bytes();
        let seen = BTreeSet::new();
        let rng = Xoshiro256Plus::seed_from_u64(9999);

        let true_max = if (maxiter as f64).log2() < (numbits as f64)  {
            maxiter 
        } else { (numbits as f64).exp2() as u32 };

        RandUniqPermGen {
            rng : rng,
            numbits : numbits,
            buf : buf,
            seen : seen,
            retries : 0,
            max : true_max,
            use_max : true,
            use_retries : false,
        }
    }

    fn get_count(&self) -> u32 {
        self.seen.len() as u32
    }


    fn sample(&mut self) -> Option<BitVec> {
          if self.max <= self.seen.len() as u32 { return None }

          let mut is_new = false;
          let mut attempt = 0;
          while true || (self.use_retries && attempt < self.retries) {
              self.rng.fill(&mut self.buf[..]);
              let mut bv = BitVec::from_bytes(&self.buf[..]);
              bv.truncate(self.numbits);
              is_new = self.seen.insert(bv.clone());
              if is_new {
                  return Some(bv)
              }
              attempt = attempt + 1;
          }

          None
    }
}

fn rl(script: &mut Script, scoped_vars: &mut BTreeMap<String, Vec<SExp>>){
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                rl_c(&mut *cmd.borrow_mut(), scoped_vars);
            }
        }
    }
}

fn rl_c(cmd: &mut Command, scoped_vars: &mut BTreeMap<String, Vec<SExp>>){
    match cmd {
        Command::Assert(s) | Command::CheckSatAssuming(s) => rl_s(&mut *s.borrow_mut(), scoped_vars),
        _ => (),
    }
}

fn rl_s(sexp: &mut SExp, scoped_vars: &mut BTreeMap<String, Vec<SExp>>){
    match sexp {
        SExp::Let(v, rest) => {
            // This looks a bit strange, but if we don't explore these first, those expressions are
            // each copied multiple times. By doing the exploration on these originals first, we
            // don't need to later on the copies. We can't add the variable values to the tree yet,
            // because they may overwrite variables in the original expressions. All solutions are
            // at least O(2n) before the recursive call on rest, and this one at least is clear,
            // and n = number of variables in a let expression = typically a very small number anyways.

            let mut new_vars : Vec<(&SymbolRc, &SExpRc)> = vec![];
            for (var, val) in v {
                rl_s(&mut *val.borrow_mut(), scoped_vars); // first make sure the val is "let-free"
                new_vars.push((var, val)); // make note of the mapping to add to the rest
            }

            // Add all of the allocated variabled to the scope
            for (var, val) in new_vars.iter() {
                let maybe_vals = scoped_vars.get_mut(&var.borrow().to_string()[..]);
                match maybe_vals {
                    Some(vals) => vals.push((*val).borrow().clone()),
                    None => {scoped_vars.insert(var.borrow().to_string(), vec![val.borrow().clone()]);},
                };
            }

            // Recurse on the rest of the SExp
            rl_s(&mut *rest.borrow_mut(), scoped_vars);

            // Pop our variables off of the stack
            for (var, _) in new_vars {
                scoped_vars.get_mut(&var.borrow().to_string()[..]).map(|v| v.pop());
            }

            let b =(**rest.borrow()).clone();
            *sexp = b; // the let expression isn't doing anything anymore, replace it with rest
        },
        SExp::Symbol(s) => {
            let new_s = scoped_vars
                .get(&s.borrow().to_string()[..])
                .and_then(|v| v.last());

            match new_s {
                Some(e) => {
                    let b = e.clone();
                    *sexp = b;
                },
                None => (),
            }
        },

        SExp::Compound(v) |
        SExp::BExp(_, v) => for e in v { rl_s(&mut *e.borrow_mut(), scoped_vars) },
        SExp::Constant(_) => (),
    }
}

fn rc(script: &mut Script, vng : &mut VarNameGenerator){
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                rc_c(&mut *cmd.borrow_mut(), vng);
            }
        }
    }
}

fn rc_c(cmd: &mut Command, vng : &mut  VarNameGenerator) {
    match cmd {
        Command::Assert(sexp) | Command::CheckSatAssuming(sexp) => rc_se(&mut *sexp.borrow_mut(), vng),
        _ => (),
    }
}

fn rc_se(sexp: &mut SExp, vng : &mut VarNameGenerator) {
    match sexp {
        SExp::Constant(c) => {
            let sort = match *c.borrow_mut() {
                Constant::UInt(_) => Sort::UInt(),
                Constant::Dec(_) => Sort::Dec(),
                Constant::Str(_) => Sort::Str(),
                Constant::Bool(_) => Sort::Bool(),
                Constant::Bin(_) |
                Constant::Hex(_) => Sort::BitVec(),
            };
            let name = vng.get_name(sort);
            *sexp = SExp::Symbol(rccell!(Symbol::Var(name)));
        },
        SExp::Compound(sexps) |
        SExp::BExp(_, sexps) => {
            for sexp in sexps {
                rc_se(&mut *sexp.borrow_mut(), vng)
            }
        }
        _ => (),
    }
}

fn bav(script: &mut Script, vng : &mut VarNameGenerator, bava : &mut Vec<(String, SExp)>){
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                bav_c(&mut *cmd.borrow_mut(), vng, bava);
            }
        }
    }
}

fn bav_c(cmd: &mut Command, vng : &mut  VarNameGenerator, bava : &mut Vec<(String, SExp)>){
    match cmd {
        Command::Assert(sexp) | Command::CheckSatAssuming(sexp) => bav_se(&mut *sexp.borrow_mut(), vng, bava),
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
                bav_se(&mut *sexp.borrow_mut(), vng, bavs);
            }
        },
        SExp::Compound(sexps) => {
            for sexp in sexps {
                bav_se(&mut *sexp.borrow_mut(), vng, bavs);
            }
        },
        SExp::Let(_, _) => panic!("Let statments should be filtered out!"),
        _ => (),
    }
}


fn init_vars(script : &mut Script, vars : Vec<(String, Sort)>) {
    let Script::Commands(cmds) = script;
    if cmds.len() == 0 { return }

    let maybe_log_pos = cmds.iter().position(|cmd| cmd.borrow().is_logic());

    let log_pos = match maybe_log_pos {
        Some(pos) => pos,
        None => 0,
    };

    let mut end = cmds.split_off(log_pos + 1);

    let mut decls = vars.into_iter()
        .map(|(vname, sort)| Command::DeclConst(vname, rccell!(sort)))
        .map(|cmd| rccell!(cmd))
        .collect::<Vec<CommandRc>>();

    cmds.append(&mut decls);
    cmds.append(&mut end);
}

fn add_ba(script : &mut Script, bavs : Vec<(String, SExp)>) {
    let Script::Commands(cmds) = script;

    let maybe_cs_pos = cmds.iter().position(|cmd| cmd.borrow().is_checksat());

    let cs_pos = match maybe_cs_pos {
        Some(pos) => pos,
        None => cmds.len(),
    };

    let mut baveq_iter = bavs.into_iter()
        .map(|(vname, sexp)| {
            SExp::BExp(rccell!(BoolOp::Equals()),
                vec![rccell!(SExp::Symbol(rccell!(Symbol::Var(vname)))), rccell!(sexp)]
            )
        });

    cmds.insert(cs_pos, rccell!(assert_many(&mut baveq_iter)));
}

fn assert_many(iter: &mut dyn Iterator<Item = SExp>) -> Command {
    let init = match iter.next() {
        Some(bav_eq) => bav_eq,
        _ => SExp::true_sexp(), // shouldn't ever actually be added
    };

    let intersection = iter
        .fold(init, |acc, curr| SExp::BExp(rccell!(BoolOp::And()), vec![rccell!(acc), rccell!(curr)]));

    return Command::Assert(rccell!(intersection))
}

fn end_insert_pt(script : &Script) -> usize {
    let Script::Commands(cmds) = script;

    let maybe_cs_pos = cmds.iter().position(|cmd| cmd.borrow().is_checksat());

    match maybe_cs_pos {
        Some(pos) => pos,
        None => cmds.len(),
    }
}

fn get_bav_assign(bavns : &Vec<String>, ta : BitVec) -> Command {
    let bavs = bavns.into_iter().zip(ta.into_iter());
    let mut baveq = bavs.into_iter()
        .map(|(vname, bval)| {
            let val = if bval { SExp::true_sexp() } else { SExp::false_sexp() };
            SExp::BExp(rccell!(BoolOp::Equals()), vec![rccell!(SExp::Symbol(rccell!(Symbol::Var(vname.clone())))), rccell!(val)])
        });

    assert_many(&mut baveq)
}

/// returns the Boolean Abstract Variables added as vector of their names 
fn to_skel(script : &mut Script) -> Vec<String> {
    let mut vng = VarNameGenerator::new("GEN");
    rc(script, &mut vng);

    let mut scopes = BTreeMap::new();
    rl(script, &mut scopes);

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
    let cvc4_res = process::Command::new("timeout")
        .args(&["5s", "cvc4", filename, "--produce-model", "--tlimit", "5000"])
        .output();

    let z3_res = process::Command::new("z3")
        .args(&["timeout", "5s", filename, "-T:5"])
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
                println!("remd");
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
    println!("starting 2^{} iterations", num_bavs);
    let mut urng = RandUniqPermGen::new_definite(num_bavs, 10);
    while let Some(truth_values) = urng.sample() {
        let filename = get_iter_fileout_name(source_file, urng.get_count());
        script.replace(eip, get_bav_assign(&bavns, truth_values));
        fs::write(&filename, script.to_string());
        solve(&filename);
    }
}

fn get_iter_fileout_name(source_file : &PathBuf, iter : u32) -> String {
    let source_filename = match source_file.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => "unknown",
    };
    (iter).to_string() + "_" + source_filename
}



pub fn exec() {
    let files = fs::read_dir("test").expect("error with sample dir");

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
                let stripped_contents = &rmv_comments(&contents[..]).expect("Error stripping comments").1.join(" ")[..];
                let mut script = script(&stripped_contents[..]).expect("Parsing error").1;
               
                rl(&mut script, &mut BTreeMap::new());
            }
            Err(_) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn parse_file(f : &str) -> Script {
            let contents = &fs::read_to_string(f).expect("error reading file")[..];
            let contents_sans_comments = &rmv_comments(contents)
                .expect("failed to rmv comments").1.join(" ")[..];

            script(contents_sans_comments).expect("parser error").1
    }

    #[test]
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

            let mut tsv = ToStringVisitor::new();

            let p_rc = rccell!(p); 
    //        traverse(AstNode::Script(Rc::clone(&p_rc)), vec![&mut tsv]);
    //        let up =  tsv.to_string();

     //       let pup = script(&up[..]).expect("reparse error").1;

      //      assert_eq!(*p_rc.borrow_mut(), pup);
        }
    }

    #[test]
    fn qc_rls() {
        let v = Symbol::Var("x".to_owned());
        let e = SExp::Symbol(rccell!(Symbol::Token("changed".to_owned())));
        let expected = e.clone();
        let mut sexp = SExp::Let(vec![(rccell!(v.clone()), rccell!(e))], rccell!(Box::new(SExp::Symbol(rccell!(v)))));
        rl_s(&mut sexp, &mut BTreeMap::new());
        assert_eq!(sexp, expected);
    }

    #[test]
    fn smoke_test() {
        let mut pb = PathBuf::new();
        pb.push("samples/ex.smt2");
        strip_and_test_file(&pb);
    }

    #[test]
    fn rando_calrissian() {
        let mut rng = RandUniqPermGen::new_definite(10, 1);
        println!("samples {:?} {:?}", rng.sample(), rng.sample());
        let mut rng = RandUniqPermGen::new_definite(1, 100);
        println!("samples {:?} {:?} {:?}", rng.sample(), rng.sample(), rng.sample());
    }

    #[test]
    fn quick_visual() {
        let mut s = parse_file("samples/bug272.minimized.smtv1.smt2");
        println!("Before \n {} \n\n", s.to_string());
        to_skel(&mut s);
        println!("Skeleton \n {}", s.to_string());
    }
}
