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
use std::collections::HashMap;
use std::io;
use std::iter::once;
use std::rc::Rc;
use std::time::Duration;

#[derive(Debug, Clone)]
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

    pub fn store_name(&mut self, base: &Symbol, sort: &Sort) {
        self.vars_generated.push((base.to_string(), sort.clone()));
    }

    pub fn new(base: &str) -> VarNameGenerator {
        VarNameGenerator {
            basename: base.to_owned(),
            counter: 0,
            vars_generated: vec![],
        }
    }

    pub fn merge_generated(&mut self, other: Self) {
        self.vars_generated.extend(other.vars_generated);
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

fn get_boolean_abstraction(bavs: Vec<(String, SExp, VarBindings)>) -> Vec<CommandRc> {
    let mut baveq_iter = bavs.into_iter().map(|(vname, sexp, vbs)| {
        let rhs = if vbs.len() > 0 {
            SExp::QForAll(vbs, rccell!(Box::new(sexp)))
        } else {
            sexp
        };

        SExp::BExp(
            rccell!(BoolOp::Equals()),
            vec![
                rccell!(SExp::Symbol(rccell!(Symbol::Token(vname)))),
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

fn get_bav_assign(bavns: &Vec<String>, ta: BitVec) -> Command {
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
                rccell!(SExp::Symbol(rccell!(Symbol::Token(vname.clone())))),
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
                rccell!(SExp::Symbol(rccell!(Symbol::Token(vname.clone())))),
                rccell!(SExp::Symbol(rccell!(Symbol::Token("{}".to_owned())))),
            ],
        )
    });

    assert_many(&mut baveq)
}

pub fn replace_constants_with_fresh_vars(script: &mut Script, md: &mut Metadata) {
    let choles = choles(script);
    if !try_all_rcholes(script, &choles, md, is_valid) {
        rcholes(script, &choles, md, is_valid);
    }
}

pub fn grab_all_decls(script: &Script) -> Vec<CommandRc> {
    let Script::Commands(cmds) = script;
    let mut decl_cmds = vec![];
    for cmd in cmds {
        match *cmd.borrow() {
            Command::DeclFn(_, _, _) | Command::DeclConst(_, _) | Command::GenericDecl(_, _) => {
                decl_cmds.push(Rc::clone(cmd))
            }
            _ => (),
        }
    }
    decl_cmds
}

pub fn ba_script(script: &mut Script, md: &mut Metadata) -> io::Result<Script> {
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

    let mut decls = grab_all_decls(script);
    let mut ba = get_boolean_abstraction(bavs);
    decls.append(&mut ba);
    decls.push(rccell!(Command::CheckSat()));
    let mut ba_script = Script::Commands(decls);
    add_get_model(&mut ba_script);
    Ok(ba_script)
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
    let timer = Timer::new_started(Duration::from_secs(5));
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

    match sexp {
        SExp::Let(v, rest) => {
            recur_count = recur_count + 1;
            if recur_count > RECUR_LIMIT {
                return liftio!(Err("Reached Recursion Limit Replacing 'Let' statements"));
            }

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
        SExp::QExists(_, s) => rl_s(&mut s.borrow_mut(), scoped_vars, timer, recur_count),
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
        SExp::QExists(_, rest) => rv_se(&mut *rest.borrow_mut(), to_replace),
        SExp::Symbol(sym) => {
            let name = match &*sym.borrow() {
                Symbol::Token(n) | Symbol::Token(n) => n,
            }
            .clone();
            for (to_be_rep, val) in to_replace {
                if name == *to_be_rep {
                    *sexp = val.clone();
                    break;
                }
            }
        }
        SExp::Constant(_) => (),
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
        chole.swap(SExp::Symbol(rccell!(Symbol::Token(name.clone()))));
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
    let timer = Timer::new_started(Duration::from_secs(60));
    let mut vng = VarNameGenerator::new("GEN");
    for (chole, sort) in choles {
        if timer.is_done() {
            warn!("Timeout filling Constant Holes");
            break;
        }

        let o = chole.clone_v();
        let name = vng.get_name(sort.clone());
        chole.swap(SExp::Symbol(rccell!(Symbol::Token(name.clone()))));
        let inits = init_vars(script, vec![vng.vars_generated.pop().unwrap()]);

        if validator(script) {
            md.constvns.push(name.clone())
        } else {
            rmv_cmds(inits);
            if retry_coerce_hole(script, name.clone(), sort, validator) {
                md.constvns.push(name.clone())
            } else {
                chole.swap(o);
            }
        }
    }
}

fn retry_coerce_hole(
    script: &mut Script,
    name: String,
    sort: &Sort,
    validator: fn(&Script) -> bool,
) -> bool {
    match sort {
        Sort::UInt() => {
            let inits = init_vars(script, vec![(name, Sort::Dec())]);
            validator(script) || {
                rmv_cmds(inits);
                false
            }
        }
        _ => false,
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
        SExp::QExists(_, rest) => choles_rcse(&Rcse::b(Rc::clone(rest))),
        SExp::Symbol(_) => vec![],
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
    let mut qvars = QualedVars::new();
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                bav_c(&mut *cmd.borrow_mut(), vng, bava, &mut qvars, timer.clone())?;
            }
        }
    };
    vng.merge_generated(qvars.vng);
    Ok(())
}

type VarBindings = Vec<(SymbolRc, SortRc)>;

fn bav_c(
    cmd: &mut Command,
    vng: &mut VarNameGenerator,
    bava: &mut Vec<(String, SExp, VarBindings)>,
    qvars: &mut QualedVars,
    timer: Timer,
) -> io::Result<()> {
    if timer.is_done() {
        return liftio!(Err("Timeout creating Boolean Abstraction"));
    }
    match cmd {
        Command::Assert(sexp) | Command::CheckSatAssuming(sexp) => bav_se(
            true,
            &mut *sexp.borrow_mut(),
            vng,
            bava,
            qvars,
            timer.clone(),
        ),
        _ => Ok(()),
    }
}

#[derive(Debug, Clone)]
struct QualedVars {
    uqvars: VarBindings,
    replacer: BTreeMap<SymbolRc, Vec<SymbolRc>>,
    vng: VarNameGenerator,
}

impl QualedVars {
    pub fn new() -> Self {
        QualedVars {
            uqvars: vec![],
            replacer: BTreeMap::new(),
            vng: VarNameGenerator::new("QUAL_REPLACE"),
        }
    }

    pub fn add_existential(&mut self, name: &SymbolRc, sort: &SortRc) {
        if !self.replacer.contains_key(name) {
            self.replacer.insert(Rc::clone(name), vec![]);
        }

        self.replacer
            .get_mut(name)
            .unwrap()
            .push(rccell!(Symbol::Token(
                self.vng.get_name(sort.borrow().clone())
            )));
    }

    pub fn add_existentials(&mut self, vbs: &VarBindings) {
        vbs.iter().for_each(|(n, s)| self.add_existential(n, s));
    }

    pub fn add_universal(&mut self, name: &SymbolRc, sort: &SortRc) {
        self.uqvars.push((Rc::clone(name), Rc::clone(sort)));
    }

    pub fn add_universals(&mut self, vbs: &VarBindings) {
        vbs.iter().for_each(|(n, s)| self.add_universal(n, s));
    }

    pub fn replace_if_necessary(&self, name: &mut SymbolRc) {
        self.replacer
            .get(name)
            .and_then(|v| v[..].last())
            .map(|rpmt| *name = Rc::clone(rpmt));
    }

    pub fn pop_n_universal(&mut self, n_to_pop: usize) {
        self.uqvars.truncate(self.uqvars.len() - n_to_pop);
    }

    pub fn pop_e(&mut self, to_pop: &SymbolRc) {
        self.replacer.get_mut(to_pop).and_then(|v| v.pop());
    }

    pub fn pop_all_e(&mut self, to_pop: &VarBindings) {
        to_pop.iter().for_each(|(ntp, _)| self.pop_e(ntp));
    }
}

fn bav_se(
    _is_root: bool,
    sexp: &mut SExp,
    vng: &mut VarNameGenerator,
    bavs: &mut Vec<(String, SExp, VarBindings)>,
    qvars: &mut QualedVars,
    timer: Timer,
) -> io::Result<()> {
    if timer.is_done() {
        return liftio!(Err("Timeout creating Boolean Abstraction"));
    }

    match sexp {
        SExp::BExp(bop, sexps) => {
            let sec = SExp::BExp(bop.clone(), sexps.clone());
            let pre_uqvars = qvars.uqvars.clone();
            let before_exploration_num_bavs = bavs.len();
            for sexp in sexps {
                bav_se(
                    false,
                    &mut *sexp.borrow_mut(),
                    vng,
                    bavs,
                    qvars,
                    timer.clone(),
                )?;
            }
            if bavs.len() <= before_exploration_num_bavs {
                let name = vng.get_name(Sort::Bool());
                bavs.push((name, sec, pre_uqvars));
            }
            Ok(())
        }
        SExp::Compound(sexps) => {
            for sexp in sexps {
                bav_se(
                    false,
                    &mut *sexp.borrow_mut(),
                    vng,
                    bavs,
                    qvars,
                    timer.clone(),
                )?;
            }
            Ok(())
        }
        SExp::Let(_, _) => panic!("Let statments should be filtered out!"),
        SExp::QForAll(vbs, rest) => {
            let num_vbs = vbs.len();
            qvars.add_universals(vbs);
            bav_se(
                false,
                &mut *rest.borrow_mut(),
                vng,
                bavs,
                qvars,
                timer.clone(),
            )?;
            qvars.pop_n_universal(num_vbs);
            Ok(())
        }
        SExp::QExists(vbs, rest) => {
            qvars.add_existentials(vbs);
            bav_se(
                false,
                &mut *rest.borrow_mut(),
                vng,
                bavs,
                qvars,
                timer.clone(),
            )?;
            qvars.pop_all_e(vbs);
            Ok(())
        }
        SExp::Constant(_) => Ok(()),
        SExp::Symbol(s) => {
            qvars.replace_if_necessary(s);
            Ok(())
        }
    }
}

fn is_valid(s: &Script) -> bool {
    match check_valid_solve_as_temp(s) {
        Ok(responses) => responses
            .iter()
            .any(|r| !r.has_unrecoverable_error() || r.has_bug_error()),
        Err(e) => {
            warn!("validator error!: {}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::script;
    use crate::parser::sexp;
    use insta::assert_debug_snapshot;
    use insta::assert_display_snapshot;

    #[test]
    fn double_eq_snap() {
        let str_script =
            "(assert (exists ((a Int)) (< a 4)))(assert (exists ((a String)) (= a \"\")))";
        let mut p = script(str_script).unwrap().1;
        let ba_str = ba_script(&mut p, &mut Metadata::new_empty())
            .unwrap()
            .to_string();

        assert_display_snapshot!(ba_str);
    }

    #[test]
    fn ba_script_eqv() {
        let str_script = "(assert (exists ((a Int)) (< a 4)))";
        let mut p = script(str_script).unwrap().1;
        let ba_str = ba_script(&mut p, &mut Metadata::new_empty())
            .unwrap()
            .to_string();

        assert!(ba_str.contains("declare-const QUAL") || ba_str.contains("declare-fun QUAL"));
    }

    #[test]
    fn ba_script_snap() {
        let str_script =
            "(declare-const x Int)(declare-const y Int)(assert (or (and (> x 3) (< y 7)) (= y x)))(assert (distinct y x))";
        let mut p = script(str_script).unwrap().1;
        assert_display_snapshot!(ba_script(&mut p, &mut Metadata::new_empty()).unwrap());
    }

    #[test]
    fn grab_all_decls_snap() {
        let str_script = "(define-sort myset () (Set (Set (_ BitVec 1))))(declare-const x Int)(assert (= 3 4))(check-sat)(declare-fun z () Bool)(declare-const y Real)";
        let mut p = script(str_script).unwrap().1;
        assert_display_snapshot!(Script::Commands(grab_all_decls(&p)));
    }

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
    fn inc_rcholes_coerce_snap() {
        let str_script = "(set-logic NRA)(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

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
        let v = Symbol::Token("x".to_owned());
        let e = SExp::Symbol(rccell!(Symbol::Token("changed".to_owned())));
        let expected = e.clone();
        let mut sexp = SExp::Let(
            vec![(rccell!(v.clone()), rccell!(e))],
            rccell!(Box::new(SExp::Symbol(rccell!(v)))),
        );
        let timer = Timer::new_started(Duration::from_secs(100));
        rl_s(&mut sexp, &mut BTreeMap::new(), &timer, 0);
        assert_eq!(sexp, expected);
    }
}
