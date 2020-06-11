use crate::ast::{AstNode, BoolOp, Command, Logic, SExp, Script, Sort, Symbol};
use crate::ast::{CommandRc, SExpRc, SortRc, SymbolRc};

use crate::liftio;
use crate::solver::check_valid_solve_as_temp;
use crate::Metadata;
use crate::Timer;
use bit_vec::BitVec;
use log::warn;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io;
use std::iter::once;
use std::rc::Rc;
use std::time::Duration;

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

fn set_logic_all(script: &mut Script) {
    let Script::Commands(cmds) = script;
    let _maybe_log_pos = cmds
        .iter()
        .find(|cmd| cmd.borrow().is_logic())
        .map(|cmd| cmd.replace(Command::Logic(rccell!(Logic::Other("ALL".to_owned())))));
}

fn init_vars(script: &mut Script, vars: Vec<(String, Sort)>) -> Vec<CommandRc> {
    let Script::Commands(cmds) = script;
    if cmds.len() == 0 {
        return vec![];
    }

    let maybe_log_pos = cmds.iter().position(|cmd| cmd.borrow().is_logic());

    let after_log_pos = match maybe_log_pos {
        Some(pos) => pos + 1,
        None => 0,
    };

    let mut end = cmds.split_off(after_log_pos);

    let decls = vars
        .into_iter()
        .map(|(vname, sort)| Command::DeclConst(vname, rccell!(sort)))
        .map(|cmd| rccell!(cmd))
        .collect::<Vec<CommandRc>>();

    cmds.append(&mut decls.clone());
    cmds.append(&mut end);
    decls
}

fn add_ba(script: &mut Script, bavs: Vec<(String, SExp, VarBindings)>) {
    let Script::Commands(cmds) = script;

    let maybe_cs_pos = cmds.iter().position(|cmd| cmd.borrow().is_checksat());

    let cs_pos = match maybe_cs_pos {
        Some(pos) => pos,
        None => cmds.len(),
    };

    let mut end = cmds.split_off(cs_pos);
    cmds.append(&mut get_boolean_abstraction(bavs));
    cmds.append(&mut end);
}

pub fn get_boolean_abstraction(bavs: Vec<(String, SExp, VarBindings)>) -> Vec<CommandRc> {
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

    many_assert(&mut baveq_iter)
}

fn many_assert(iter: &mut dyn Iterator<Item = SExp>) -> Vec<CommandRc> {
    iter.map(|bexp| rccell!(Command::Assert(rccell!(bexp))))
        .collect()
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

pub fn checksat_positions(script: &Script) -> Vec<usize> {
    let Script::Commands(cmds) = script;

    let mut cmds_iter = cmds.iter();

    let mut checksats = vec![];
    while let Some(pos) = cmds_iter.position(|cmd| cmd.borrow().is_checksat()) {
        checksats.push(pos);
    }

    checksats
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

pub fn get_bav_assign_fmt_str(bavns: &Vec<String>) -> Command {
    let mut baveq = bavns.into_iter().map(|vname| {
        SExp::BExp(
            rccell!(BoolOp::Equals()),
            vec![
                rccell!(SExp::Symbol(rccell!(Symbol::Var(vname.clone())))),
                rccell!(SExp::Symbol(rccell!(Symbol::Token("{}".to_owned())))),
            ],
        )
    });

    assert_many(&mut baveq)
}

pub fn replace_constants_with_fresh_vars(script : &mut Script, md : &mut Metadata) {
    let choles = choles(script);
    if !try_all_rcholes(script, &choles, md, is_valid) {
        rcholes(script, &choles, md, is_valid);
    }
}

pub fn grab_all_decls(script : &Script) -> Vec<CommandRc> {
    let Script::Commands(cmds) = script;
    let mut decl_cmds = vec![];
    for cmd in cmds {
        
        decl_cmds.push(Rc::clone(cmd));
    }
    decl_cmds
}

pub fn to_skel(script: &mut Script, md: &mut Metadata) -> io::Result<()> {
    set_logic_all(script);

    replace_constants_with_fresh_vars(script, md);
    let mut scopes = BTreeMap::new();
    rl(script, &mut scopes)?;

    let mut vng = VarNameGenerator::new("BAV");
    let mut bavs = vec![];
    bav(script, &mut vng, &mut bavs)?;

    init_vars(script, vng.vars_generated);
    let mut bavns = bavs
        .iter()
        .map(|(name, _, _)| name.clone())
        .collect::<Vec<String>>();
    md.bavns.append(&mut bavns);
    add_ba(script, bavs);
    add_get_model(script);
    Ok(())
}

pub fn add_get_model(script: &mut Script) {
    let csps = checksat_positions(script);
    csps.iter().for_each(|p| {
        if !script.index_is_gm(p + 1) {
            script.insert(p + 1, Command::GetModel());
        }
    });
}

pub fn rl(script: &mut Script, scoped_vars: &mut BTreeMap<String, Vec<SExp>>) -> io::Result<()> {
    let timer = Timer::new();
    timer.start(Duration::from_secs(5));
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                rl_c(&mut *cmd.borrow_mut(), scoped_vars, &timer)?;
            }
            Ok(())
        }
    }
}

fn rl_c(
    cmd: &mut Command,
    scoped_vars: &mut BTreeMap<String, Vec<SExp>>,
    timer: &Timer,
) -> io::Result<()> {
    if timer.is_done() {
        return liftio!(Err("Timeout Replacing 'Let' statements"));
    }

    match cmd {
        Command::Assert(s) | Command::CheckSatAssuming(s) => {
            rl_s(&mut *s.borrow_mut(), scoped_vars, timer, 0)
        }
        _ => Ok(()),
    }
}

static RECUR_LIMIT: u8 = 10;
fn rl_s(
    sexp: &mut SExp,
    scoped_vars: &mut BTreeMap<String, Vec<SExp>>,
    timer: &Timer,
    mut recur_count: u8,
) -> io::Result<()> {
    if timer.is_done() {
        return liftio!(Err("Timeout Replacing 'Let' statements"));
    }

    recur_count = recur_count + 1;
    if recur_count > RECUR_LIMIT {
        return liftio!(Err("Reached Recursion Limit Replacing 'Let' statements"));
    }

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
                rl_s(&mut *val.borrow_mut(), scoped_vars, timer, recur_count)?; // first make sure the val is "let-free"
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
            rl_s(&mut *rest.borrow_mut(), scoped_vars, timer, recur_count)?;

            // Pop our variables off of the stack
            for (var, _) in new_vars {
                scoped_vars
                    .get_mut(&var.borrow().to_string()[..])
                    .map(|v| v.pop());
            }

            let b = (**rest.borrow()).clone();
            *sexp = b; // the let expression isn't doing anything anymore, replace it with rest
            Ok(())
        }
        SExp::Symbol(s) => {
            let new_s = scoped_vars
                .get(&s.borrow().to_string()[..])
                .and_then(|v| v.last());

            match new_s {
                Some(e) => {
                    let b = e.clone();
                    *sexp = b;
                    Ok(())
                }
                None => Ok(()),
            }
        }

        SExp::Compound(v) | SExp::BExp(_, v) => {
            for e in v {
                rl_s(&mut *e.borrow_mut(), scoped_vars, timer, recur_count)?
            }
            Ok(())
        }
        SExp::QForAll(_, s) => rl_s(&mut s.borrow_mut(), scoped_vars, timer, recur_count),
        SExp::Constant(_) => Ok(()),
    }
}

pub fn rv(script: &mut Script, to_replace: &Vec<(String, SExp)>) {
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                rv_c(&mut *cmd.borrow_mut(), to_replace);
            }
        }
    }
}

fn rv_c(cmd: &mut Command, to_replace: &Vec<(String, SExp)>) {
    match cmd {
        Command::Assert(sexp) | Command::CheckSatAssuming(sexp) => {
            rv_se(&mut *sexp.borrow_mut(), to_replace)
        }
        Command::DeclConst(vname, _) => {
            let vname = vname.clone();
            for (to_be_rep, _) in to_replace {
                if vname == *to_be_rep {
                    *cmd = Command::no_op();
                }
            }
        }
        _ => (),
    }
}

fn rv_se(sexp: &mut SExp, to_replace: &Vec<(String, SExp)>) {
    match sexp {
        SExp::Compound(sexps) | SExp::BExp(_, sexps) => {
            for sexp in sexps {
                rv_se(&mut *sexp.borrow_mut(), to_replace)
            }
        }
        SExp::Let(vbs, rest) => {
            for (_, sexp) in vbs {
                rv_se(&mut *sexp.borrow_mut(), to_replace)
            }
            rv_se(&mut *rest.borrow_mut(), to_replace);
        }
        SExp::QForAll(_, rest) => rv_se(&mut *rest.borrow_mut(), to_replace),
        SExp::Symbol(sym) => {
            let name = match &*sym.borrow() {
                Symbol::Token(n) | Symbol::Var(n) => n,
            }
            .clone();
            for (to_be_rep, val) in to_replace {
                if name == *to_be_rep {
                    *sexp = val.clone();
                    break;
                }
            }
        }
        _ => (),
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Rcse {
    nb(Rc<RefCell<SExp>>),
    b(Rc<RefCell<Box<SExp>>>),
}

impl Rcse {
    fn swap(&self, new_sexp: SExp) {
        match self {
            Rcse::nb(sexp) => *sexp.borrow_mut() = new_sexp,
            Rcse::b(bsexp) => **bsexp.borrow_mut() = new_sexp,
        }
    }

    fn clone_v(&self) -> SExp {
        match self {
            Rcse::nb(sexp) => sexp.borrow_mut().clone(),
            Rcse::b(bsexp) => (**bsexp.borrow_mut()).clone(),
        }
    }
}

fn rmv_cmds(cmds: Vec<CommandRc>) {
    for cmd in cmds {
        let mut inner = cmd.borrow_mut();
        *inner = Command::no_op();
    }
}

pub fn try_all_rcholes(
    script: &mut Script,
    choles: &Vec<(Rcse, Sort)>,
    md: &mut Metadata,
    validator: fn(&Script) -> bool,
) -> bool {
    let mut vng = VarNameGenerator::new("GEN");
    let mut names = vec![];
    let mut ogvs = vec![];
    for (chole, sort) in choles.iter() {
        ogvs.push(chole.clone_v());
        let name = vng.get_name(sort.clone());
        chole.swap(SExp::Symbol(rccell!(Symbol::Var(name.clone()))));
        names.push(name);
    }

    let inits = init_vars(script, vng.vars_generated);

    if validator(script) {
        md.constvns.extend(names);
        true
    } else {
        for ((chole, _), ogv) in choles.iter().zip(ogvs.iter()) {
            chole.swap(ogv.clone());
        }
        rmv_cmds(inits);
        false
    }
}

pub fn rcholes(
    script: &mut Script,
    choles: &Vec<(Rcse, Sort)>,
    md: &mut Metadata,
    validator: fn(&Script) -> bool,
) {
    let timer = Timer::new();
    timer.start(Duration::from_secs(60));
    let mut vng = VarNameGenerator::new("GEN");
    for (chole, sort) in choles {
        if timer.is_done() {
            warn!("Timeout filling Constant Holes");
            break;
        }

        let o = chole.clone_v();
        let name = vng.get_name(sort.clone());
        chole.swap(SExp::Symbol(rccell!(Symbol::Var(name.clone()))));
        let inits = init_vars(script, vec![vng.vars_generated.pop().unwrap()]);

        if validator(script) {
            md.constvns.push(name.clone())
        } else {
            rmv_cmds(inits);
            chole.swap(o);
        }
    }
}

pub fn choles(script: &mut Script) -> Vec<(Rcse, Sort)> {
    match script {
        Script::Commands(cmds) => {
            let mut v = vec![];
            for cmd in cmds.iter_mut() {
                v.extend(choles_c(&mut *cmd.borrow_mut()));
            }
            v
        }
    }
}

fn choles_c(cmd: &mut Command) -> Vec<(Rcse, Sort)> {
    match cmd {
        Command::Assert(sexp) | Command::CheckSatAssuming(sexp) => {
            choles_rcse(&Rcse::nb(Rc::clone(&sexp)))
        }
        _ => vec![],
    }
}

fn choles_rcse(rcse: &Rcse) -> Vec<(Rcse, Sort)> {
    let inner = |sexp: &SExp| match sexp {
        SExp::Constant(c) => {
            let sort = c.borrow().sort();
            vec![(rcse.clone(), sort)]
        }
        SExp::Compound(sexps) | SExp::BExp(_, sexps) => {
            let mut v = vec![];
            for sexp in sexps {
                v.extend(choles_rcse(&Rcse::nb(Rc::clone(sexp))));
            }
            v
        }
        SExp::Let(vbs, rest) => {
            let mut v = vec![];
            for (_, sexp) in vbs {
                v.extend(choles_rcse(&Rcse::nb(Rc::clone(sexp))));
            }
            v.extend(choles_rcse(&Rcse::b(Rc::clone(rest))));
            v
        }
        SExp::QForAll(_, rest) => choles_rcse(&Rcse::b(Rc::clone(rest))),
        _ => vec![],
    };

    match rcse {
        Rcse::nb(s) => inner(&*s.borrow()),
        Rcse::b(bs) => inner(&**bs.borrow()),
    }
}

pub fn bav(
    script: &mut Script,
    vng: &mut VarNameGenerator,
    bava: &mut Vec<(String, SExp, VarBindings)>,
) -> io::Result<()> {
    let timer = Timer::new_started(Duration::from_secs(30));
    let mut qvars = vec![];
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                bav_c(&mut *cmd.borrow_mut(), vng, bava, &mut qvars, timer.clone())?;
            }
        }
    };
    Ok(())
}

type VarBindings = Vec<(SymbolRc, SortRc)>;

fn bav_c(
    cmd: &mut Command,
    vng: &mut VarNameGenerator,
    bava: &mut Vec<(String, SExp, VarBindings)>,
    qvars: &mut VarBindings,
    timer: Timer,
) -> io::Result<()> {
    if timer.is_done() {
        return liftio!(Err("Timeout creating Boolean Abstraction"));
    }
    match cmd {
        Command::Assert(sexp) | Command::CheckSatAssuming(sexp) => {
            bav_se(&mut *sexp.borrow_mut(), vng, bava, qvars, timer.clone())
        }
        _ => Ok(()),
    }
}

fn bav_se(
    sexp: &mut SExp,
    vng: &mut VarNameGenerator,
    bavs: &mut Vec<(String, SExp, VarBindings)>,
    qvars: &mut VarBindings,
    timer: Timer,
) -> io::Result<()> {
    if timer.is_done() {
        return liftio!(Err("Timeout creating Boolean Abstraction"));
    }

    match sexp {
        SExp::BExp(bop, sexps) => {
            let name = vng.get_name(Sort::Bool());
            let sec = SExp::BExp(bop.clone(), sexps.clone());
            bavs.push((name, sec, qvars.clone()));
            for sexp in sexps {
                bav_se(&mut *sexp.borrow_mut(), vng, bavs, qvars, timer.clone())?;
            }
            Ok(())
        }
        SExp::Compound(sexps) => {
            for sexp in sexps {
                bav_se(&mut *sexp.borrow_mut(), vng, bavs, qvars, timer.clone())?;
            }
            Ok(())
        }
        SExp::Let(_, _) => panic!("Let statments should be filtered out!"),
        SExp::QForAll(vbs, rest) => {
            qvars.extend(vbs.clone());
            bav_se(&mut *rest.borrow_mut(), vng, bavs, qvars, timer.clone())?;
            qvars.truncate(qvars.len() - vbs.len());
            Ok(())
        }
        _ => Ok(()),
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
            }
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

fn is_valid(s: &Script) -> bool {
    match check_valid_solve_as_temp(s) {
        Ok(responses) => responses.iter().any(|r| !r.has_unrecoverable_error()),
        Err(e) => {
            warn!("validator error!: {}", e);
            false
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
    use crate::parser::script;
    use crate::parser::sexp;
    use insta::assert_debug_snapshot;

    #[test]
    fn all_rcholes_undo_then_inc_snap() {
        let str_script = "(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        try_all_rcholes(&mut p, &choles, &mut md, |_s| false);
        rcholes(&mut p, &choles, &mut md, is_valid);

        assert_debug_snapshot!(p.to_string());
    }

    #[test]
    fn inc_rcholes_undo_snap() {
        let str_script = "(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        rcholes(&mut p, &choles, &mut md, |_s| false);

        assert_debug_snapshot!(p.to_string());
    }

    #[test]
    fn all_rcholes_undo_snap() {
        let str_script = "(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        assert!(!try_all_rcholes(&mut p, &choles, &mut md, |_s| false));

        assert_debug_snapshot!(p.to_string() + "\n" + &md.constvns.join("\n"));
    }

    #[test]
    fn all_rcholes_snap() {
        let str_script = "(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        assert!(try_all_rcholes(&mut p, &choles, &mut md, is_valid));

        assert_debug_snapshot!(p.to_string() + "\n" + &md.constvns.join("\n"));
    }

    #[test]
    fn choles_snap() {
        let str_script = "(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        rcholes(&mut p, &choles, &mut md, is_valid);

        assert_debug_snapshot!(p.to_string());
    }

    #[test]
    fn add_get_model_already_snap() {
        let str_script = "(declare-const x Int)(assert (= x 4))(check-sat)(get-model)";
        let mut p = script(str_script).unwrap().1;
        add_get_model(&mut p);
        assert_debug_snapshot!(p.to_string());
    }

    #[test]
    fn add_get_model_snap() {
        let str_script = "(declare-const x Int)(assert (= x 4))(check-sat)";
        let mut p = script(str_script).unwrap().1;
        add_get_model(&mut p);
        assert_debug_snapshot!(p.to_string());
    }

    #[test]
    fn checksat_pts_none_snap() {
        let str_script = "(declare-const x Int)(assert (= x 4))";
        let p = script(str_script).unwrap().1;
        let pts = checksat_positions(&p);
        assert_debug_snapshot!(pts);
    }

    #[test]
    fn checksat_pts_many_snap() {
        let str_script =
            "(declare-const x Int)(assert (= x 4))(check-sat)(assert (= x 5))(check-sat)";
        let p = script(str_script).unwrap().1;
        let pts = checksat_positions(&p);
        assert_debug_snapshot!(pts);
    }

    #[test]
    fn checksat_pts_single_snap() {
        let str_script = "(declare-const x Int)(assert (= x 4))(check-sat)";
        let p = script(str_script).unwrap().1;
        let pts = checksat_positions(&p);
        assert_debug_snapshot!(pts);
    }
    #[test]
    fn rv_with_decl_snap() {
        let str_script = "(declare-const x Int)(assert (forall ((x Real)) (= x 4)))";
        let mut p = script(str_script).unwrap().1;
        rv(&mut p, &vec![("x".to_owned(), sexp("7").unwrap().1)]);
        assert_debug_snapshot!(p.to_string());
    }

    #[test]
    fn rv_snap() {
        let str_script = "(assert (forall ((x Real)) (= x 4)))";
        let mut p = script(str_script).unwrap().1;
        rv(&mut p, &vec![("x".to_owned(), sexp("7").unwrap().1)]);
        assert_debug_snapshot!(p);
    }

    #[test]
    fn bav_fmt_str() {
        assert_debug_snapshot!(
            "{:?}",
            get_bav_assign_fmt_str(&vec!["BAV1".to_owned(), "BAV2".to_owned()]).to_string()
        );
    }

    #[test]
    fn bav_qual() {
        let str_script = "(assert (forall ((x Real)) (= x 4)))";
        let mut p = script(str_script).unwrap().1;

        let mut vng = VarNameGenerator::new("BAV");
        let mut bavs = vec![];
        bav(&mut p, &mut vng, &mut bavs);
        add_ba(&mut p, bavs);

        let Script::Commands(cmds) = p;

        let assertion_cmd = cmds.last().unwrap();
        match &*assertion_cmd.borrow() {
            Command::Assert(s) => match &*s.borrow() {
                SExp::BExp(_v, rest) => match &*rest[1].borrow() {
                    SExp::QForAll(v, _rest) => assert!(v.len() > 0),
                    _ => panic!("inner should be qual"),
                },
                _ => panic!("Assert should be BExp"),
            },
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
        let timer = Timer::new();
        rl_s(&mut sexp, &mut BTreeMap::new(), &timer, 0);
        assert_eq!(sexp, expected);
    }
}
