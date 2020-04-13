use crate::ast::{
    AstNode, BoolOp, Command, Constant, SExp, Script, Sort, Symbol,
};
use crate::ast::{CommandRc, SExpRc, SymbolRc, SortRc};
use crate::parser::{script};
use bit_vec::BitVec;
use std::iter::once;
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct VarNameGenerator {
    basename: String,
    counter: u32,
    vars_generated: Vec<(String, Sort)>,
}

impl VarNameGenerator {
    pub fn get_name(&mut self, sort: Sort) -> String {
        self.counter = self.counter + 1;
        let name = format!("{}{}", self.basename, self.counter);
        self.vars_generated.push((name.clone(), sort));
        name
    }

    pub fn new(base: &str) -> VarNameGenerator {
        VarNameGenerator {
            basename: base.to_owned(),
            counter: 0,
            vars_generated: vec![],
        }
    }
}

fn init_vars(script: &mut Script, vars: Vec<(String, Sort)>) {
    let Script::Commands(cmds) = script;
    if cmds.len() == 0 {
        return;
    }

    let maybe_log_pos = cmds.iter().position(|cmd| cmd.borrow().is_logic());

    let log_pos = match maybe_log_pos {
        Some(pos) => pos,
        None => 0,
    };

    let mut end = cmds.split_off(log_pos + 1);

    let mut decls = vars
        .into_iter()
        .map(|(vname, sort)| Command::DeclConst(vname, rccell!(sort)))
        .map(|cmd| rccell!(cmd))
        .collect::<Vec<CommandRc>>();

    cmds.append(&mut decls);
    cmds.append(&mut end);
}

fn add_ba(script: &mut Script, bavs: Vec<(String, SExp, VarBindings)>) {
    let Script::Commands(cmds) = script;

    let maybe_cs_pos = cmds.iter().position(|cmd| cmd.borrow().is_checksat());

    let cs_pos = match maybe_cs_pos {
        Some(pos) => pos,
        None => cmds.len(),
    };

    let mut baveq_iter = bavs.into_iter().map(|(vname, sexp, vbs)| {
        let rhs = if vbs.len() > 0 {
            SExp::QForAll(vbs, rccell!(Box::new(sexp)))
        } else {
            sexp
        };

        SExp::BExp(
            rccell!(BoolOp::Equals()),
            vec![
                rccell!(SExp::Symbol(rccell!(Symbol::Var(vname)))),
                rccell!(rhs),
            ],
        )
    });

    cmds.insert(cs_pos, rccell!(assert_many(&mut baveq_iter)));
}

fn assert_many(iter: &mut dyn Iterator<Item = SExp>) -> Command {
    let init = match iter.next() {
        Some(bav_eq) => bav_eq,
        _ => SExp::true_sexp(), // shouldn't ever actually be added
    };

    let intersection = iter.fold(init, |acc, curr| {
        SExp::BExp(rccell!(BoolOp::And()), vec![rccell!(acc), rccell!(curr)])
    });

    return Command::Assert(rccell!(intersection));
}

pub fn end_insert_pt(script: &Script) -> usize {
    let Script::Commands(cmds) = script;

    let maybe_cs_pos = cmds.iter().position(|cmd| cmd.borrow().is_checksat());

    match maybe_cs_pos {
        Some(pos) => pos,
        None => cmds.len(),
    }
}

pub fn get_bav_assign(bavns: &Vec<String>, ta: BitVec) -> Command {
    let bavs = bavns.into_iter().zip(ta.into_iter());
    let mut baveq = bavs.into_iter().map(|(vname, bval)| {
        let val = if bval {
            SExp::true_sexp()
        } else {
            SExp::false_sexp()
        };
        SExp::BExp(
            rccell!(BoolOp::Equals()),
            vec![
                rccell!(SExp::Symbol(rccell!(Symbol::Var(vname.clone())))),
                rccell!(val),
            ],
        )
    });

    assert_many(&mut baveq)
}

/// returns the Boolean Abstract Variables added as vector of their names
pub fn to_skel(script: &mut Script) -> Vec<String> {
    let mut vng = VarNameGenerator::new("GEN");
    rc(script, &mut vng);

    let mut scopes = BTreeMap::new();
    rl(script, &mut scopes);

    vng.basename = "BAV".to_owned();
    let mut bavs = vec![];
    bav(script, &mut vng, &mut bavs);

    init_vars(script, vng.vars_generated);
    let bavns = bavs
        .iter()
        .map(|(name, _, _)| name.clone())
        .collect::<Vec<String>>();
    add_ba(script, bavs);
    bavns
}

pub fn rl(script: &mut Script, scoped_vars: &mut BTreeMap<String, Vec<SExp>>) {
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                rl_c(&mut *cmd.borrow_mut(), scoped_vars);
            }
        }
    }
}

fn rl_c(cmd: &mut Command, scoped_vars: &mut BTreeMap<String, Vec<SExp>>) {
    match cmd {
        Command::Assert(s) | Command::CheckSatAssuming(s) => {
            rl_s(&mut *s.borrow_mut(), scoped_vars)
        }
        _ => (),
    }
}

fn rl_s(sexp: &mut SExp, scoped_vars: &mut BTreeMap<String, Vec<SExp>>) {
    match sexp {
        SExp::Let(v, rest) => {
            // This looks a bit strange, but if we don't explore these first, those expressions are
            // each copied multiple times. By doing the exploration on these originals first, we
            // don't need to later on the copies. We can't add the variable values to the tree yet,
            // because they may overwrite variables in the original expressions. All solutions are
            // at least O(2n) before the recursive call on rest, and this one at least is clear,
            // and n = number of variables in a let expression = typically a very small number anyways.

            let mut new_vars: Vec<(&SymbolRc, &SExpRc)> = vec![];
            for (var, val) in v {
                rl_s(&mut *val.borrow_mut(), scoped_vars); // first make sure the val is "let-free"
                new_vars.push((var, val)); // make note of the mapping to add to the rest
            }

            // Add all of the allocated variabled to the scope
            for (var, val) in new_vars.iter() {
                let maybe_vals = scoped_vars.get_mut(&var.borrow().to_string()[..]);
                match maybe_vals {
                    Some(vals) => vals.push((*val).borrow().clone()),
                    None => {
                        scoped_vars.insert(var.borrow().to_string(), vec![val.borrow().clone()]);
                    }
                };
            }

            // Recurse on the rest of the SExp
            rl_s(&mut *rest.borrow_mut(), scoped_vars);

            // Pop our variables off of the stack
            for (var, _) in new_vars {
                scoped_vars
                    .get_mut(&var.borrow().to_string()[..])
                    .map(|v| v.pop());
            }

            let b = (**rest.borrow()).clone();
            *sexp = b; // the let expression isn't doing anything anymore, replace it with rest
        }
        SExp::Symbol(s) => {
            let new_s = scoped_vars
                .get(&s.borrow().to_string()[..])
                .and_then(|v| v.last());

            match new_s {
                Some(e) => {
                    let b = e.clone();
                    *sexp = b;
                }
                None => (),
            }
        }

        SExp::Compound(v) | SExp::BExp(_, v) => {
            for e in v {
                rl_s(&mut *e.borrow_mut(), scoped_vars)
            }
        }
        SExp::QForAll(_, s) => rl_s(&mut s.borrow_mut(), scoped_vars),
        SExp::Constant(_) => (),
    }
}

pub fn rc(script: &mut Script, vng: &mut VarNameGenerator) {
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                rc_c(&mut *cmd.borrow_mut(), vng);
            }
        }
    }
}

fn rc_c(cmd: &mut Command, vng: &mut VarNameGenerator) {
    match cmd {
        Command::Assert(sexp) | Command::CheckSatAssuming(sexp) => {
            rc_se(&mut *sexp.borrow_mut(), vng)
        }
        _ => (),
    }
}

fn rc_se(sexp: &mut SExp, vng: &mut VarNameGenerator) {
    match sexp {
        SExp::Constant(c) => {
            let sort = match *c.borrow_mut() {
                Constant::UInt(_) => Sort::UInt(),
                Constant::Dec(_) => Sort::Dec(),
                Constant::Str(_) => Sort::Str(),
                Constant::Bool(_) => Sort::Bool(),
                Constant::Bin(_) | Constant::Hex(_) => Sort::BitVec(),
            };
            let name = vng.get_name(sort);
            *sexp = SExp::Symbol(rccell!(Symbol::Var(name)));
        }
        SExp::Compound(sexps) | SExp::BExp(_, sexps) => {
            for sexp in sexps {
                rc_se(&mut *sexp.borrow_mut(), vng)
            }
        }
        _ => (),
    }
}

pub fn bav(script: &mut Script, vng: &mut VarNameGenerator,
           bava: &mut Vec<(String, SExp, VarBindings)>) {
    let mut qvars = vec![];
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                bav_c(&mut *cmd.borrow_mut(), vng, bava, &mut qvars);
            }
        }
    }
}

type VarBindings = Vec<(SymbolRc, SortRc)>;

fn bav_c(cmd: &mut Command, vng: &mut VarNameGenerator,
         bava: &mut Vec<(String, SExp, VarBindings)>, qvars : &mut VarBindings) {
    match cmd {
        Command::Assert(sexp) | Command::CheckSatAssuming(sexp) => {
            bav_se(&mut *sexp.borrow_mut(), vng, bava, qvars)
        }
        _ => (),
    }
}

fn bav_se(sexp: &mut SExp, vng: &mut VarNameGenerator,
          bavs: &mut Vec<(String, SExp, VarBindings)>, qvars : &mut VarBindings) {
    match sexp {
        SExp::BExp(bop, sexps) => {
            let name = vng.get_name(Sort::Bool());
            let sec = SExp::BExp(bop.clone(), sexps.clone());
            bavs.push((name, sec, qvars.clone()));
            for sexp in sexps {
                bav_se(&mut *sexp.borrow_mut(), vng, bavs, qvars);
            }
        }
        SExp::Compound(sexps) => {
            for sexp in sexps {
                bav_se(&mut *sexp.borrow_mut(), vng, bavs, qvars);
            }
        }
        SExp::Let(_, _) => panic!("Let statments should be filtered out!"),
        SExp::QForAll(vbs, rest) => {
            qvars.extend(vbs.clone());
            bav_se(&mut *rest.borrow_mut(), vng, bavs, qvars);
            qvars.truncate(qvars.len() - vbs.len());
        },
        _ => (),
    }
}

fn get_children(node: &AstNode) -> Vec<AstNode> {
    match node {
        AstNode::Script(s_rc) => match &*s_rc.borrow() {
            Script::Commands(cmds) => cmds
                .iter()
                .map(|cmd| AstNode::Command(Rc::clone(cmd)))
                .rev()
                .collect(),
        },
        AstNode::Command(c_rc) => match &*c_rc.borrow() {
            Command::Logic(l_rc) => vec![AstNode::Logic(Rc::clone(l_rc))],
            Command::Assert(a) | Command::CheckSatAssuming(a) => vec![AstNode::SExp(Rc::clone(a))],
            Command::DeclConst(_, s) => vec![AstNode::Sort(Rc::clone(s))],
            _ => vec![],
        },
        AstNode::Sort(s_rc) => match &*s_rc.borrow() {
            Sort::Compound(ss) => ss
                .iter()
                .map(|s| AstNode::Sort(Rc::clone(s)))
                .rev()
                .collect(),
            _ => vec![],
        },
        AstNode::SExp(s_rc) => match &*s_rc.borrow() {
            SExp::Compound(ss) => ss
                .iter()
                .map(|s| AstNode::SExp(Rc::clone(s)))
                .rev()
                .collect(),
            SExp::BExp(bop, ss) => ss
                .iter()
                .map(|s| AstNode::SExp(Rc::clone(s)))
                .rev()
                .chain(once(AstNode::BoolOp(Rc::clone(bop))))
                .collect(),
            SExp::Let(vs, s) => {
                let mut astns = vs.iter().fold(vec![AstNode::Open()], |mut asts, (vr, vl)| {
                    asts.push(AstNode::Open());
                    asts.push(AstNode::Symbol(Rc::clone(vr)));
                    asts.push(AstNode::SExp(Rc::clone(vl)));
                    asts.push(AstNode::Close());
                    asts
                });
                astns.push(AstNode::Close());
                astns.push(AstNode::SExp(rccell!(*(s.borrow()).clone())));
                astns.into_iter().rev().collect()
            },
            SExp::QForAll(vs, s) => {
                let mut astns = vs.iter().fold(vec![AstNode::Open()], |mut asts, (vr, vl)| {
                    asts.push(AstNode::Open());
                    asts.push(AstNode::Symbol(Rc::clone(vr)));
                    asts.push(AstNode::Sort(Rc::clone(vl)));
                    asts.push(AstNode::Close());
                    asts
                });
                astns.push(AstNode::Close());
                astns.push(AstNode::SExp(rccell!(*(s.borrow()).clone())));
                astns.into_iter().rev().collect()
            }
            SExp::Constant(c) => vec![AstNode::Constant(Rc::clone(c))],
            SExp::Symbol(s) => vec![AstNode::Symbol(Rc::clone(s))],
        },
        _ => vec![],
    }
}

pub fn traverse(node: AstNode, mut visitors: Vec<&mut dyn Visitor>) {
    let mut to_visit = vec![vec![node]];
    let mut visiting = vec![];

    while let Some(child_group) = to_visit.last_mut() {
        match child_group.pop() {
            Some(node) => {
                visitors.iter_mut().for_each(|v| v.entry(&node));
                to_visit.push(get_children(&node));
                visiting.push(node);
            }
            None => {
                to_visit.pop();
                visiting
                    .pop()
                    .map(|node| visitors.iter_mut().for_each(|v| v.exit(&node)));
            }
        }
    }
}

pub trait Visitor {
    fn entry(&mut self, node: &AstNode);
    fn exit(&mut self, node: &AstNode);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bav_qual() {
        let str_script = "(assert (forall ((x Real)) (= x 4)))";
        let mut p = script(str_script).unwrap().1;

        let mut vng = VarNameGenerator::new("BAV");
        let mut bavs = vec![];
        bav(&mut p, &mut vng, &mut bavs);
        add_ba(&mut p, bavs);

        let Script::Commands(mut cmds) = p;

        let assertion_cmd = cmds.last().unwrap();
        match & *assertion_cmd.borrow() {
            Command::Assert(s) => match &*s.borrow() {
                SExp::BExp(v, rest) => match &*rest[1].borrow() {
                    SExp::QForAll(v, rest) => assert!(v.len() > 0),
                    _ => panic!("inner should be qual"),
                }
                _ => panic!("Assert should be BExp"),
            }
            _ => panic!("No assert!"),
        };
    }

    #[test]
    fn qc_rls() {
        let v = Symbol::Var("x".to_owned());
        let e = SExp::Symbol(rccell!(Symbol::Token("changed".to_owned())));
        let expected = e.clone();
        let mut sexp = SExp::Let(
            vec![(rccell!(v.clone()), rccell!(e))],
            rccell!(Box::new(SExp::Symbol(rccell!(v)))),
        );
        rl_s(&mut sexp, &mut BTreeMap::new());
        assert_eq!(sexp, expected);
    }
}
