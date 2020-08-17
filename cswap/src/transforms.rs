use crate::ast::{BoolOp, Command, Constant, FPConst, SExp, Script, Sort, Symbol};
use crate::ast::{CommandRc, SExpRc, SortRc, SymbolRc};

use crate::config::Config;
use crate::liftio;
use crate::solver::check_valid_solve_as_temp;
use crate::utils::RandUniqPermGen;
use crate::Metadata;
use crate::Timer;
use log::warn;
use std::cell::RefCell;
use std::collections::BTreeMap;

use std::io;

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

fn get_subset_consts(
    consts: Vec<(String, Sort)>,
    num: usize,
    cfg: &Config,
) -> Option<Vec<(String, Sort)>> {
    let mut permgen = RandUniqPermGen::new_masked_with_retries(0, consts.len(), 1000, num);
    // seed, numbits, iter, mask size
    permgen.mask().map(|mask| {
        mask.iter()
            .zip(consts.into_iter())
            .filter_map(|(tv, c)| if tv { Some(c) } else { None })
            .collect()
    })
}

fn get_inter_relation_constant_monitors(
    constants: Vec<(String, Sort)>,
) -> (Vec<(String, Sort)>, Vec<CommandRc>) {
    let mut vng = VarNameGenerator::new("INTER_CMON");
    let mut eq_sexps = vec![];
    for (i, (cname, csort)) in constants.iter().enumerate() {
        for j in (i + 1..constants.len()) {
            let (ocname, ocsort) = &constants[j];
            match (csort, ocsort) {
                (Sort::UInt(), Sort::Dec())
                | (Sort::Dec(), Sort::UInt())
                | (Sort::Dec(), Sort::Dec())
                | (Sort::UInt(), Sort::UInt()) => {
                    let eq_monitor = vng.get_name(Sort::Bool());
                    let eq_sexp = eq_se(
                        SExp::var(&eq_monitor),
                        eq_se(SExp::var(cname), SExp::var(ocname)),
                    );
                    let lt_monitor = vng.get_name(Sort::Bool());
                    let lt_sexp = eq_se(
                        SExp::var(&lt_monitor),
                        SExp::BExp(
                            rccell!(BoolOp::Lt()),
                            vec![rccell!(SExp::var(cname)), rccell!(SExp::var(ocname))],
                        ),
                    );
                    let gt_monitor = vng.get_name(Sort::Bool());
                    let gt_sexp = eq_se(
                        SExp::var(&gt_monitor),
                        SExp::BExp(
                            rccell!(BoolOp::Gt()),
                            vec![rccell!(SExp::var(cname)), rccell!(SExp::var(ocname))],
                        ),
                    );

                    eq_sexps.push(eq_sexp);
                    eq_sexps.push(lt_sexp);
                    eq_sexps.push(gt_sexp);
                }

                (Sort::Str(), Sort::Str()) | (Sort::Bool(), Sort::Bool()) => {
                    let monitor = vng.get_name(Sort::Bool());
                    let eq_sexp = eq_se(
                        SExp::var(&monitor),
                        eq_se(SExp::var(cname), SExp::var(ocname)),
                    );
                    eq_sexps.push(eq_sexp);
                }
                (Sort::Fp(a, b), Sort::Fp(aa, bb)) => {
                    if a == aa && b == bb {
                        let eq_monitor = vng.get_name(Sort::Bool());
                        let eq_sexp = eq_se(
                            SExp::var(&eq_monitor),
                            SExp::BExp(
                                rccell!(BoolOp::Fpeq()),
                                vec![rccell!(SExp::var(cname)), rccell!(SExp::var(ocname))],
                            ),
                        );
                        let lt_monitor = vng.get_name(Sort::Bool());
                        let lt_sexp = eq_se(
                            SExp::var(&lt_monitor),
                            SExp::BExp(
                                rccell!(BoolOp::Fplt()),
                                vec![rccell!(SExp::var(cname)), rccell!(SExp::var(ocname))],
                            ),
                        );
                        let gt_monitor = vng.get_name(Sort::Bool());
                        let gt_sexp = eq_se(
                            SExp::var(&gt_monitor),
                            SExp::BExp(
                                rccell!(BoolOp::Fpgt()),
                                vec![rccell!(SExp::var(cname)), rccell!(SExp::var(ocname))],
                            ),
                        );

                        eq_sexps.push(eq_sexp);
                        eq_sexps.push(lt_sexp);
                        eq_sexps.push(gt_sexp);
                    }
                }
                (Sort::BitVec(a), Sort::BitVec(aa)) => {
                    if a == aa {
                        let monitor = vng.get_name(Sort::Bool());
                        let eq_sexp = eq_se(
                            SExp::var(&monitor),
                            eq_se(SExp::var(cname), SExp::var(ocname)),
                        );
                        eq_sexps.push(eq_sexp);
                    }
                }
                _ => (),
            }
        }
    }

    let cmds = many_assert(&mut eq_sexps.into_iter());
    (vng.vars_generated, cmds)
}

fn eq_se(a: SExp, b: SExp) -> SExp {
    SExp::BExp(rccell!(BoolOp::Equals()), vec![rccell!(a), rccell!(b)])
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

    let decls = get_var_inits(vars);
    cmds.append(&mut decls.clone());
    cmds.append(&mut end);
    decls
}

fn get_var_inits(vars: Vec<(String, Sort)>) -> Vec<CommandRc> {
    vars.into_iter()
        .map(|(vname, sort)| Command::DeclConst(vname, rccell!(sort)))
        .map(|cmd| rccell!(cmd))
        .collect::<Vec<CommandRc>>()
}

// Get sub-exp monitors
fn get_boolean_abstraction(bavs: Vec<(String, SExp, VarBindings)>) -> Vec<CommandRc> {
    let mut baveq_iter = bavs.into_iter().map(|(vname, sexp, vbs)| {
        let rhs = SExp::BExp(
            rccell!(BoolOp::Equals()),
            vec![
                rccell!(SExp::Symbol(rccell!(Symbol::Token(vname)))),
                rccell!(sexp),
            ],
        );
        if vbs.len() > 0 {
            SExp::QForAll(vbs, rccell!(Box::new(rhs)))
        } else {
            rhs
        }
    });

    many_assert(&mut baveq_iter)
}

// Get BOOLEAN domain monitors ... THESE are added to the metadata      Vec<String, Sort>, vng -> Vec<Variables>, Vec<Command>
fn get_boolean_domain_monitors(
    subexp_monitors: Vec<(String, Sort)>,
) -> (Vec<(String, Sort)>, Vec<CommandRc>) {
    let mut vng = VarNameGenerator::new("BDOM");
    let mut baveq = subexp_monitors
        .into_iter()
        .flat_map(|(vname, vsort)| match vsort {
            Sort::Bool() => vec![SExp::BExp(
                rccell!(BoolOp::Equals()),
                vec![
                    rccell!(SExp::Symbol(rccell!(Symbol::Token(
                        vng.get_name(Sort::Bool())
                    )))),
                    rccell!(SExp::Symbol(rccell!(Symbol::Token(vname)))),
                ],
            )],
            Sort::Dec() | Sort::UInt() => vec![
                bav_rel_0(vng.get_name(Sort::Bool()), vname.clone(), BoolOp::Equals()),
                bav_rel_0(vng.get_name(Sort::Bool()), vname.clone(), BoolOp::Lt()),
                bav_rel_0(vng.get_name(Sort::Bool()), vname.clone(), BoolOp::Gt()),
            ],
            Sort::Str() => vec![bav_eq_emptystr(vng.get_name(Sort::Bool()), vname.clone())],
            Sort::Fp(eb, sb) => vec![
                bav_fp_eq(
                    vng.get_name(Sort::Bool()),
                    vname.clone(),
                    FPConst::NInf(eb.clone(), sb.clone()),
                ),
                bav_fp_between(
                    vng.get_name(Sort::Bool()),
                    vname.clone(),
                    FPConst::NInf(eb.clone(), sb.clone()),
                    FPConst::NZero(eb.clone(), sb.clone()),
                ),
                bav_fp_eq(
                    vng.get_name(Sort::Bool()),
                    vname.clone(),
                    FPConst::NZero(eb.clone(), sb.clone()),
                ),
                bav_fp_eq(
                    vng.get_name(Sort::Bool()),
                    vname.clone(),
                    FPConst::PZero(eb.clone(), sb.clone()),
                ),
                bav_fp_between(
                    vng.get_name(Sort::Bool()),
                    vname.clone(),
                    FPConst::PZero(eb.clone(), sb.clone()),
                    FPConst::PInf(eb.clone(), sb.clone()),
                ),
                bav_fp_eq(
                    vng.get_name(Sort::Bool()),
                    vname.clone(),
                    FPConst::PInf(eb.clone(), sb.clone()),
                ),
                bav_fp_eq(
                    vng.get_name(Sort::Bool()),
                    vname.clone(),
                    FPConst::Nan(eb.clone(), sb.clone()),
                ),
            ],
            _ => vec![],
        });

    let cmds = many_assert(&mut baveq);
    (vng.vars_generated, cmds)
}

pub fn many_assert(iter: &mut dyn Iterator<Item = SExp>) -> Vec<CommandRc> {
    iter.map(|bexp| rccell!(Command::Assert(rccell!(bexp))))
        .collect()
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

fn bav_eq_emptystr(bavname: String, strmonitorname: String) -> SExp {
    SExp::BExp(
        rccell!(BoolOp::Equals()),
        vec![
            rccell!(SExp::BExp(
                rccell!(BoolOp::Equals()),
                vec![
                    rccell!(SExp::Symbol(rccell!(Symbol::Token(strmonitorname)))),
                    rccell!(SExp::Constant(rccell!(Constant::Dec("\"\"".to_owned()))))
                ]
            )),
            rccell!(SExp::Symbol(rccell!(Symbol::Token(bavname)))),
        ],
    )
}

fn bav_rel_0(bav_name: String, num_monitor_name: String, relative: BoolOp) -> SExp {
    SExp::BExp(
        rccell!(BoolOp::Equals()),
        vec![
            rccell!(SExp::BExp(
                rccell!(relative),
                vec![
                    rccell!(SExp::Symbol(rccell!(Symbol::Token(num_monitor_name)))),
                    rccell!(SExp::Constant(rccell!(Constant::Dec("0".to_owned()))))
                ]
            )),
            rccell!(SExp::Symbol(rccell!(Symbol::Token(bav_name)))),
        ],
    )
}

fn bav_fp_eq(bav_name: String, fp_monitor_name: String, other: FPConst) -> SExp {
    let var = rccell!(Symbol::Token(fp_monitor_name));
    SExp::BExp(
        rccell!(BoolOp::Equals()),
        vec![
            rccell!(SExp::BExp(
                rccell!(BoolOp::Equals()),
                vec![
                    rccell!(SExp::Symbol(Rc::clone(&var))),
                    rccell!(SExp::Constant(rccell!(Constant::Fp(other))))
                ]
            )),
            rccell!(SExp::Symbol(rccell!(Symbol::Token(bav_name)))),
        ],
    )
}

fn bav_fp_between(bav_name: String, fp_monitor_name: String, low: FPConst, high: FPConst) -> SExp {
    let var = rccell!(Symbol::Token(fp_monitor_name));
    SExp::BExp(
        rccell!(BoolOp::Equals()),
        vec![
            rccell!(SExp::BExp(
                rccell!(BoolOp::And()),
                vec![
                    rccell!(SExp::BExp(
                        rccell!(BoolOp::Fpgt()),
                        vec![
                            rccell!(SExp::Symbol(Rc::clone(&var))),
                            rccell!(SExp::Constant(rccell!(Constant::Fp(low))))
                        ]
                    )),
                    rccell!(SExp::BExp(
                        rccell!(BoolOp::Fplt()),
                        vec![
                            rccell!(SExp::Symbol(Rc::clone(&var))),
                            rccell!(SExp::Constant(rccell!(Constant::Fp(high))))
                        ]
                    ))
                ]
            )),
            rccell!(SExp::Symbol(rccell!(Symbol::Token(bav_name)))),
        ],
    )
}

// Change this to return a vec of SExp that can then be trimmed down / filtered according to how many switches we flip at a time
pub fn get_bav_assign_fmt_str(bavns: &Vec<(String, Sort)>) -> Vec<CommandRc> {
    let mut baveq = bavns.into_iter().flat_map(|(vname, vsort)| match vsort {
        Sort::Bool() => vec![SExp::BExp(
            rccell!(BoolOp::Equals()),
            vec![
                rccell!(SExp::Symbol(rccell!(Symbol::Token(vname.clone())))),
                rccell!(SExp::Symbol(rccell!(Symbol::Token("{}".to_owned())))),
            ],
        )],
        _ => panic!("Unreachable brangch"),
    });

    many_assert(&mut baveq)
}

pub fn replace_constants_with_fresh_vars(script: &mut Script, md: &mut Metadata) -> io::Result<()> {
    let choles = choles(script);

    if choles.len() == 0 {
        return liftio!(Err("No Constants to Replace!"));
    }

    if try_all_rcholes(script, &choles, md, is_valid).is_err() {
        rcholes(script, &choles, md, is_valid)
    } else {
        Ok(())
    }
}

pub fn grab_all_decls(script: &Script) -> Vec<CommandRc> {
    let Script::Commands(cmds) = script;
    let mut decl_cmds = vec![];
    for cmd in cmds {
        match *cmd.borrow() {
            Command::DeclFn(_, _, _)
            | Command::DeclConst(_, _)
            | Command::GenericDecl(_, _)
            | Command::DefineFun(_, _, _, _)
            | Command::DefineFunRec(_, _, _, _)
            | Command::DefineFunsRec(_, _) => decl_cmds.push(Rc::clone(cmd)),
            _ => (),
        }
    }
    decl_cmds
}

pub fn ba_script(script: &mut Script, md: &mut Metadata) -> io::Result<Vec<Script>> {
    rl(script)?;

    let mut vng = VarNameGenerator::new("BAV");
    let mut bavs = vec![];
    bav(script, &mut vng, &mut bavs)?;

    let og_vars = script
        .get_all_global_var_bindings()
        .into_iter()
        .map(|(a, b)| (a.to_string(), b))
        .filter(|(name, _sort)| !name.contains("GEN")); // TODO dont do string compares here

    rl(script)?;

    let mut vng = VarNameGenerator::new("BAV");
    let mut bavs = vec![];
    bav(script, &mut vng, &mut bavs)?;

    let bavns = vng
        .vars_generated
        .clone()
        .into_iter()
        .filter(|(name, _sort)| !name.contains("REPL")) // TODO move quantifiers into same step as "let" replacement to avoid this
        .chain(og_vars)
        .collect();

    let (bdomvs, mut bdomcmds) = get_boolean_domain_monitors(bavns);

    let num_const_to_relate = if USE_RELATIONAL_CONST_MONITORS { 15 } else { 0 };
    let (subset_constvs) =
        get_subset_consts(md.constvns.clone(), num_const_to_relate, &Config::default())
            .unwrap_or(vec![]);
    let (intervs, mut intercmds) = get_inter_relation_constant_monitors(subset_constvs);

    md.bavns.append(&mut bdomvs.clone());
    md.bavns.append(&mut intervs.clone());

    let mut decls = grab_all_decls(script);

    let mut bdom_inits = init_vars(script, bdomvs.clone());
    let mut inter_inits = init_vars(script, intervs.clone());

    let mut vs = init_vars(script, vng.vars_generated);

    decls.append(&mut vs);
    decls.append(&mut bdom_inits);
    decls.append(&mut inter_inits);

    let mut ba = get_boolean_abstraction(bavs);
    let mut iscript = script.invert();

    script.insert_all(end_insert_pt(script), &ba);
    script.insert_all(end_insert_pt(script), &bdomcmds);
    script.insert_all(end_insert_pt(script), &intercmds);

    add_get_model(script);

    decls.append(&mut ba.clone());
    decls.append(&mut bdomcmds.clone());
    decls.append(&mut intercmds.clone());

    decls.push(rccell!(Command::CheckSat()));
    let mut ba_script = Script::Commands(decls);
    add_get_model(&mut ba_script);

    let CP_OG = true;
    if !CP_OG {
        Ok(vec![ba_script])
    } else {
        iscript.insert_all(end_insert_pt(&iscript), &ba);
        iscript.insert_all(end_insert_pt(&iscript), &bdomcmds);
        iscript.insert_all(end_insert_pt(&iscript), &intercmds);
        add_get_model(&mut iscript);
        Ok(vec![iscript, script.clone()])
    }
}

pub fn add_get_model(script: &mut Script) {
    let csps = checksat_positions(script);
    csps.iter().for_each(|p| {
        if !script.index_is_gm(p + 1) {
            script.insert(p + 1, Command::GetModel());
        }
    });
}

pub fn rl(script: &mut Script) -> io::Result<()> {
    let mut scoped_vars = BTreeMap::new();
    let timer = Timer::new_started(Duration::from_secs(5));
    let mut vng = VarNameGenerator::new("RL_LET");
    let mut new_vv = vec![];
    let mut qvars = QualedVars::new_named("QUAL_PRE_REPLACE");
    match script {
        Script::Commands(cmds) => {
            for cmd in cmds.iter_mut() {
                rl_c(
                    &mut *cmd.borrow_mut(),
                    &mut scoped_vars,
                    &mut vng,
                    &mut new_vv,
                    &mut qvars,
                    &timer,
                )?;
            }
        }
    };

    vng.merge_generated(qvars.vng);
    init_vars(script, vng.vars_generated);
    let cmds = many_assert(&mut new_vv.into_iter().map(|v| v.borrow().clone()));

    script.insert_all(end_insert_pt(script), &cmds);
    Ok(())
}

fn rl_c(
    cmd: &mut Command,
    scoped_vars: &mut BTreeMap<String, Vec<SExp>>,
    vng: &mut VarNameGenerator,
    new_var_vals: &mut Vec<SExpRc>,
    qvars: &mut QualedVars,
    timer: &Timer,
) -> io::Result<()> {
    if timer.is_done() {
        return liftio!(Err("Timeout Replacing 'Let' statements"));
    }

    match cmd {
        Command::Assert(s) | Command::CheckSatAssuming(s) => rl_s(
            &mut *s.borrow_mut(),
            scoped_vars,
            vng,
            new_var_vals,
            qvars,
            timer,
            0,
        ),
        _ => Ok(()),
    }?;
    Ok(())
}

fn fresh_eq_val_sexp(fresh_name: String, val: &SExpRc, qvars: &QualedVars) -> (SExpRc, SExpRc) {
    let new_name_sexp = rccell!(SExp::Symbol(rccell!(Symbol::Token(fresh_name))));
    let equiv = SExp::BExp(
        rccell!(BoolOp::Equals()),
        vec![Rc::clone(&new_name_sexp), Rc::clone(val)],
    );
    if qvars.uqvars.len() > 0 {
        (
            new_name_sexp,
            rccell!(SExp::QForAll(
                qvars.uqvars.clone(),
                rccell!(Box::new(equiv))
            )),
        )
    } else {
        (new_name_sexp, rccell!(equiv))
    }
}

static RECUR_LIMIT: u8 = 10;
fn rl_s(
    sexp: &mut SExp,
    scoped_vars: &mut BTreeMap<String, Vec<SExp>>,
    vng: &mut VarNameGenerator,
    new_var_vals: &mut Vec<SExpRc>,
    qvars: &mut QualedVars,
    timer: &Timer,
    mut recur_count: u8,
) -> io::Result<()> {
    if timer.is_done() {
        return liftio!(Err("Timeout Replacing 'Let' statements"));
    }

    match sexp {
        SExp::Let(v, rest) => {
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
                rl_s(
                    &mut *val.borrow_mut(),
                    scoped_vars,
                    vng,
                    new_var_vals,
                    qvars,
                    timer,
                    recur_count,
                )?; // first make sure the val is "let-free"
                new_vars.push((var, val)); // make note of the mapping to add to the rest
            }

            // Add all of the allocated variabled to the scope
            for (var, val) in new_vars.iter() {
                let val = match val.borrow().sort() {
                    Some(sort) => {
                        let new_name = vng.get_name(sort.clone());
                        let (new_name_sexp, new_var_val_sexp) =
                            fresh_eq_val_sexp(new_name, val, qvars);
                        new_var_vals.push(new_var_val_sexp);
                        new_name_sexp
                    }
                    None => {
                        recur_count = recur_count + 1;
                        Rc::clone(*val)
                    }
                };
                // insert or create
                let maybe_vals = scoped_vars.get_mut(&var.borrow().to_string()[..]);
                match maybe_vals {
                    Some(vals) => vals.push((*val).borrow().clone()),
                    None => {
                        scoped_vars.insert(var.borrow().to_string(), vec![val.borrow().clone()]);
                    }
                };
            }

            // Recurse on the rest of the SExp
            rl_s(
                &mut *rest.borrow_mut(),
                scoped_vars,
                vng,
                new_var_vals,
                qvars,
                timer,
                recur_count,
            )?;

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
            qvars.replace_if_necessary(s);
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

        SExp::Compound(v)
        | SExp::BExp(_, v)
        | SExp::FPExp(_, _, v)
        | SExp::NExp(_, v)
        | SExp::StrExp(_, v) => {
            for e in v {
                rl_s(
                    &mut *e.borrow_mut(),
                    scoped_vars,
                    vng,
                    new_var_vals,
                    qvars,
                    timer,
                    recur_count,
                )?
            }
            Ok(())
        }
        SExp::QForAll(vbs, s) => {
            let num_vbs = vbs.len();
            if !UNIVERSAL_AS_EXISTENTIAL {
                qvars.add_universals(vbs);
            } else {
                qvars.add_existentials(vbs);
            }
            rl_s(
                &mut s.borrow_mut(),
                scoped_vars,
                vng,
                new_var_vals,
                qvars,
                timer,
                recur_count,
            )?;

            if !UNIVERSAL_AS_EXISTENTIAL {
                qvars.pop_n_universal(num_vbs);
            } else {
                qvars.pop_all_e(vbs);
                let b = (**s.borrow()).clone();
                *sexp = b; // replace existential
            }
            Ok(())
        }
        SExp::QExists(vbs, s) => {
            qvars.add_existentials(vbs);
            rl_s(
                &mut s.borrow_mut(),
                scoped_vars,
                vng,
                new_var_vals,
                qvars,
                timer,
                recur_count,
            )?;
            qvars.pop_all_e(vbs);
            let b = (**s.borrow()).clone();
            *sexp = b; // replace existential
            Ok(())
        }
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
        SExp::Compound(sexps)
        | SExp::BExp(_, sexps)
        | SExp::FPExp(_, _, sexps)
        | SExp::NExp(_, sexps)
        | SExp::StrExp(_, sexps) => {
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
                Symbol::Token(n) => n,
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
    NotBox(Rc<RefCell<SExp>>),
    Box(Rc<RefCell<Box<SExp>>>),
}

impl Rcse {
    fn swap(&self, new_sexp: SExp) {
        match self {
            Rcse::NotBox(sexp) => *sexp.borrow_mut() = new_sexp,
            Rcse::Box(bsexp) => **bsexp.borrow_mut() = new_sexp,
        }
    }

    fn clone_v(&self) -> SExp {
        match self {
            Rcse::NotBox(sexp) => sexp.borrow_mut().clone(),
            Rcse::Box(bsexp) => (**bsexp.borrow_mut()).clone(),
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
) -> io::Result<()> {
    let mut vng = VarNameGenerator::new("GEN");
    let mut names = vec![];
    let mut ogvs = vec![];
    for (chole, sort) in choles.iter() {
        ogvs.push(chole.clone_v());
        let name = vng.get_name(sort.clone());
        chole.swap(SExp::Symbol(rccell!(Symbol::Token(name.clone()))));
        names.push((name, sort.clone()));
    }

    let inits = init_vars(script, vng.vars_generated);

    if validator(script) {
        md.constvns.extend(names);
        Ok(())
    } else {
        for ((chole, _), ogv) in choles.iter().zip(ogvs.iter()) {
            chole.swap(ogv.clone());
        }
        rmv_cmds(inits);
        liftio!(Err("Failed to replace ALL constants"))
    }
}

pub fn rcholes(
    script: &mut Script,
    choles: &Vec<(Rcse, Sort)>,
    md: &mut Metadata,
    validator: fn(&Script) -> bool,
) -> io::Result<()> {
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
            md.constvns.push((name.clone(), sort.clone()))
        } else {
            rmv_cmds(inits);
            if retry_coerce_hole(script, name.clone(), sort, validator) {
                md.constvns.push((name.clone(), sort.clone()));
            } else {
                chole.swap(o);
            }
        }
    }

    if md.constvns.len() > 0 {
        Ok(())
    } else {
        liftio!(Err("Failed to Replace ANY Constants!"))
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
            choles_rcse(&Rcse::NotBox(Rc::clone(&sexp)))
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
        SExp::Compound(sexps)
        | SExp::BExp(_, sexps)
        | SExp::FPExp(_, _, sexps)
        | SExp::NExp(_, sexps)
        | SExp::StrExp(_, sexps) => {
            let mut v = vec![];
            for sexp in sexps {
                v.extend(choles_rcse(&Rcse::NotBox(Rc::clone(sexp))));
            }
            v
        }
        SExp::Let(vbs, rest) => {
            let mut v = vec![];
            for (_, sexp) in vbs {
                v.extend(choles_rcse(&Rcse::NotBox(Rc::clone(sexp))));
            }
            v.extend(choles_rcse(&Rcse::Box(Rc::clone(rest))));
            v
        }
        SExp::QForAll(_, rest) => choles_rcse(&Rcse::Box(Rc::clone(rest))),
        SExp::QExists(_, rest) => choles_rcse(&Rcse::Box(Rc::clone(rest))),
        SExp::Symbol(_) => vec![],
    };

    match rcse {
        Rcse::NotBox(s) => inner(&*s.borrow()),
        Rcse::Box(bs) => inner(&**bs.borrow()),
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
    pub fn new_named(s: &str) -> Self {
        QualedVars {
            uqvars: vec![],
            replacer: BTreeMap::new(),
            vng: VarNameGenerator::new(s),
        }
    }

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

const USE_RELATIONAL_CONST_MONITORS: bool = true;
const UNIVERSAL_AS_EXISTENTIAL: bool = true;
const LEAF_OPT: bool = true;

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
            let sec = SExp::BExp(Rc::clone(bop), sexps.clone());
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
            if bavs.len() <= before_exploration_num_bavs || !LEAF_OPT {
                let name = vng.get_name(Sort::Bool());
                bavs.push((name, sec, pre_uqvars));
            }
            Ok(())
        }
        SExp::StrExp(_, sexps) | SExp::NExp(_, sexps) | SExp::FPExp(_, _, sexps) => {
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
            if UNIVERSAL_AS_EXISTENTIAL {
                panic!("Universal Quantifiers should be filtered out!");
            }
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
        SExp::QExists(_, _) => panic!("Existential Quantifiers should be filtered out!"),
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
    fn get_idm_consts_snap() {
        let consts = vec![
            ("a".to_string(), Sort::Bool()),
            ("b".to_string(), Sort::Bool()),
            ("yy".to_string(), Sort::UInt()),
            ("zz".to_string(), Sort::UInt()),
            ("111".to_string(), Sort::Dec()),
            ("222".to_string(), Sort::Dec()),
            ("mmmm".to_string(), Sort::Str()),
            ("nnnn".to_string(), Sort::Str()),
            (
                "AAAAA".to_string(),
                Sort::Fp("2".to_string(), "2".to_string()),
            ),
            (
                "BBBBB".to_string(),
                Sort::Fp("2".to_string(), "2".to_string()),
            ),
            (
                "CCCCC".to_string(),
                Sort::Fp("2".to_string(), "3".to_string()),
            ),
            (
                "DDDDD".to_string(),
                Sort::Fp("3".to_string(), "2".to_string()),
            ),
            ("XXXXX".to_string(), Sort::BitVec(3)),
            ("YYYYY".to_string(), Sort::BitVec(3)),
            ("ZZZZZ".to_string(), Sort::BitVec(2)),
        ];

        let idm = get_inter_relation_constant_monitors(consts);
        assert_display_snapshot!(Script::Commands(idm.1));
    }

    #[test]
    fn get_subset_consts_snap() {
        let consts = vec![
            ("a".to_string(), Sort::Bool()),
            ("b".to_string(), Sort::Bool()),
            ("c".to_string(), Sort::Bool()),
            ("d".to_string(), Sort::UInt()),
            ("e".to_string(), Sort::UInt()),
            ("f".to_string(), Sort::UInt()),
        ];

        let cfg = Config::default();

        let subset = get_subset_consts(consts, 3, &cfg);
        assert_debug_snapshot!(subset);
    }

    #[test]
    fn fp_eq_snap() {
        let s = bav_fp_eq(
            "BDOM".to_owned(),
            "MVAR".to_owned(),
            FPConst::PZero("11".to_owned(), "53".to_owned()),
        );

        assert_display_snapshot!(s);
    }

    #[test]
    fn fp_between_snap() {
        let s = bav_fp_between(
            "BDOM".to_owned(),
            "MVAR".to_owned(),
            FPConst::PZero("11".to_owned(), "53".to_owned()),
            FPConst::PInf("11".to_owned(), "53".to_owned()),
        );

        assert_display_snapshot!(s);
    }

    #[test]
    fn num_op_ba_script_snap() {
        let str_script = "(declare-fun x () Real)(assert (< (+ 4 3) x))";
        let mut p = script(str_script).unwrap().1;
        let ba_str = ba_script(&mut p, &mut Metadata::new_empty()).unwrap()[0].to_string();

        assert_display_snapshot!(ba_str);
    }

    #[test]
    fn double_eq_snap() {
        let str_script =
            "(assert (exists ((a Int)) (< a 4)))(assert (exists ((a String)) (= a \"\")))";
        let mut p = script(str_script).unwrap().1;
        let ba_str = ba_script(&mut p, &mut Metadata::new_empty()).unwrap()[0].to_string();

        assert_display_snapshot!(ba_str);
    }

    #[test]
    fn forall_ba_snap() {
        let str_script =
            "(assert (forall ((a Int)) (< a 4)))(assert (exists ((a String)) (= a \"\")))";
        let mut p = script(str_script).unwrap().1;
        let bas = ba_script(&mut p, &mut Metadata::new_empty()).unwrap();
        let ba_stra = bas[0].to_string();
        let ba_strb = bas[1].to_string();
        assert_display_snapshot!(ba_stra + "\n\n ~~~~~~~~~~~~~~~~~~~~~~~ \n\n" + &ba_strb);
    }

    #[test]
    fn ba_script_eqv() {
        let str_script = "(assert (exists ((a Int)) (< a 4)))";
        let mut p = script(str_script).unwrap().1;
        let ba_str = ba_script(&mut p, &mut Metadata::new_empty()).unwrap()[0].to_string();

        assert!(ba_str.contains("declare-const QUAL") || ba_str.contains("declare-fun QUAL"));
    }

    #[test]
    fn ba_script_skel_w_og_snap() {
        let str_script =
            "(declare-const x Int)(declare-const y Int)(assert (or (and (> x (+ y 3)) (< y 7)) (= y x)))(assert (distinct y x))";
        let mut p = script(str_script).unwrap().1;
        ba_script(&mut p, &mut Metadata::new_empty()).unwrap();
        assert_display_snapshot!(p);
    }

    #[test]
    fn ba_script_snap() {
        let str_script =
            "(declare-const x Int)(declare-const y Int)(assert (or (and (> x 3) (< y 7)) (= y x)))(assert (distinct y x))";
        let mut p = script(str_script).unwrap().1;
        assert_display_snapshot!(ba_script(&mut p, &mut Metadata::new_empty()).unwrap()[0]);
    }

    #[test]
    fn decl_order_ba_script_snap() {
        let str_script = "(define-sort FP () (_ FloatingPoint 11 53)) (assert  (exists ((x FP)) (fp.isInfinite (fp.sqrt RTN x))))";
        let mut p = script(str_script).unwrap().1;
        assert_display_snapshot!(ba_script(&mut p, &mut Metadata::new_empty()).unwrap()[0]);
    }

    #[test]
    fn grab_all_decls_snap() {
        let str_script = "(define-sort myset () (Set (Set (_ BitVec 1))))(declare-const x Int)(assert (= 3 4))(check-sat)(declare-fun z () Bool)(declare-const y Real)";
        let p = script(str_script).unwrap().1;
        assert_display_snapshot!(Script::Commands(grab_all_decls(&p)));
    }

    #[test]
    fn all_rcholes_undo_then_inc_snap() {
        let str_script = "(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        assert!(try_all_rcholes(&mut p, &choles, &mut md, |_s| false).is_err());
        rcholes(&mut p, &choles, &mut md, is_valid).unwrap();

        assert_debug_snapshot!(p.to_string());
    }

    #[test]
    fn inc_rcholes_coerce_snap() {
        let str_script = "(set-logic NRA)(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        rcholes(&mut p, &choles, &mut md, is_valid).unwrap();
        assert_debug_snapshot!(p.to_string());
    }
    #[test]
    fn inc_rcholes_undo_snap() {
        let str_script = "(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        assert!(rcholes(&mut p, &choles, &mut md, |_s| false).is_err());

        assert_debug_snapshot!(p.to_string());
    }

    #[test]
    fn all_rcholes_undo_snap() {
        let str_script = "(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        assert!(try_all_rcholes(&mut p, &choles, &mut md, |_s| false).is_err());

        assert_debug_snapshot!(
            p.to_string()
                + "\n"
                + &md
                    .constvns
                    .iter()
                    .map(|c| c.0.clone())
                    .collect::<Vec<String>>()
                    .join("\n")
        );
    }

    #[test]
    fn all_rcholes_snap() {
        let str_script = "(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        assert!(try_all_rcholes(&mut p, &choles, &mut md, is_valid).is_ok());

        assert_debug_snapshot!(
            p.to_string()
                + "\n"
                + &md
                    .constvns
                    .iter()
                    .map(|c| c.0.clone())
                    .collect::<Vec<String>>()
                    .join("\n")
        );
    }

    #[test]
    fn choles_snap() {
        let str_script = "(assert (= 3 4))";
        let mut p = script(str_script).unwrap().1;
        let mut md = Metadata::new_empty();
        let choles = choles(&mut p);

        rcholes(&mut p, &choles, &mut md, is_valid).unwrap();

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
    fn domain_monitors_snap() {
        let r = get_boolean_domain_monitors(vec![
            ("BAV1".to_owned(), Sort::Bool()),
            ("BAV3".to_owned(), Sort::Dec()),
            ("BAV4".to_owned(), Sort::Str()),
            (
                "BAV5".to_owned(),
                Sort::Fp("11".to_owned(), "53".to_owned()),
            ),
        ]);

        assert_display_snapshot!(Script::Commands(r.1));
    }

    #[test]
    fn bav_fmt_str() {
        assert_display_snapshot!(Script::Commands(get_bav_assign_fmt_str(&vec![(
            "BDOM1".to_owned(),
            Sort::Bool()
        ),])));
    }

    #[test]
    fn rl_snap() {
        let str_script =
            "(assert (= x (let ((x 4)) (let ((y (+ x 2))(z (unknown_op x))) (= (- x 4) y z)))))";
        let mut p = script(str_script).unwrap().1;
        rl(&mut p).unwrap();
        assert_display_snapshot!(p);
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
        let mut qvars = QualedVars::new();
        rl_s(
            &mut sexp,
            &mut BTreeMap::new(),
            &mut VarNameGenerator::new("rl_let"),
            &mut vec![],
            &mut qvars,
            &timer,
            0,
        )
        .unwrap();
        assert_eq!(sexp, expected);
    }

    #[test]
    fn qual_inside_let_snap() {
        let mut script = script("(assert (forall ((u Int)) (let ((w (+ u c1))) (> u w)))))")
            .unwrap()
            .1;

        rl(&mut script).unwrap();
        assert_display_snapshot!(script);
    }
}
