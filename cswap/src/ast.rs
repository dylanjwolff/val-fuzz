use std::cell::RefCell;
use std::fmt;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

pub type ScriptRc = Rc<RefCell<Script>>;
pub type CommandRc = Rc<RefCell<Command>>;
pub type LogicRc = Rc<RefCell<Logic>>;
pub type SExpRc = Rc<RefCell<SExp>>;
pub type SExpBoxRc = Rc<RefCell<Box<SExp>>>;
pub type SortRc = Rc<RefCell<Sort>>;
pub type SymbolRc = Rc<RefCell<Symbol>>;
pub type ConstantRc = Rc<RefCell<Constant>>;
pub type BoolOpRc = Rc<RefCell<BoolOp>>;

macro_rules! rccell {
    ($x:expr) => {{
        std::rc::Rc::new(std::cell::RefCell::new($x))
    }};
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Script {
    Commands(Vec<CommandRc>),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Command {
    Logic(LogicRc),
    CheckSat(),
    CheckSatAssuming(SExpRc),
    Assert(SExpRc),
    GetModel(),
    GenericDecl(String, Vec<Vec<String>>),
    DeclConst(String, SortRc),
    DeclFn(Symbol, Vec<Sort>, Sort),
    Generic(String),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Sort {
    UInt(),
    Dec(),
    Str(),
    Bool(),
    BitVec(u32),
    Fp(String, String),
    Array(),
    UserDef(String),
    Compound(Vec<SortRc>),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum SExp {
    Compound(Vec<SExpRc>),
    Let(Vec<(SymbolRc, SExpRc)>, SExpBoxRc),
    QForAll(Vec<(SymbolRc, SortRc)>, SExpBoxRc),
    QExists(Vec<(SymbolRc, SortRc)>, SExpBoxRc),
    BExp(BoolOpRc, Vec<SExpRc>),
    NExp(NumOp, Vec<SExpRc>),
    FPExp(FpOp, Option<(String, String)>, Vec<SExpRc>),
    StrExp(StrOp, Vec<SExpRc>),
    Constant(ConstantRc),
    Symbol(SymbolRc),
}

#[derive(Serialize, PartialEq, PartialOrd, Ord, Deserialize, Debug, Clone, Eq)]
pub enum Symbol {
    Token(String),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum FpOp {
    Abs(),
    Neg(),
    Add(),
    Sub(),
    Mul(),
    Div(),
    Fma(),
    Sqrt(),
    Rem(),
    RTI(),
    Min(),
    Max(),
    ToFp(String, String),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum BoolOp {
    Equals(),
    And(),
    Or(),
    Xor(),
    Implies(),
    Distinct(),
    Gt(),
    Lt(),
    Gte(),
    Lte(),
    Bvgt(),
    Bvge(),
    Bvslt(),
    Bvsle(),
    Bvsgt(),
    Bvsge(),
    Fpleq(),
    Fplt(),
    Fpgeq(),
    Fpgt(),
    Fpeq(),
    Fpnorm(),
    Fpsubn(),
    Fpzero(),
    Fpinf(),
    Fpnana(),
    Fpneg(),
    Fppos(),
    Strlt(),
    Strlte(),
    Strre(),
    Strpref(),
    Strsuff(),
    Strcont(),
    Strisdig(),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum StrOp {
    Strcat(),
    StrAt(),
    StrFromCode(),
    Substr(),
    StrReplace(),
    StrReplaceRe(),
    StrReplaceReAll(),
    StrReplaceAll(),
}
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum NumOp {
    Sub(),
    Add(),
    Mul(),
    Div(),
    FpToReal(),
    FpToInt(),
    IntDiv(),
    Mod(),
    StrLen(),
    StrToCode(),
    StrToInt(),
    StrIndexOf(),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Constant {
    UInt(String),
    Dec(String),
    Hex(String),
    Bin(Vec<char>),
    Str(String),
    Bool(bool),
    Fp(FPConst),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum FPConst {
    Num(BitVecConst, BitVecConst, BitVecConst),
    PZero(String, String),
    NZero(String, String),
    PInf(String, String),
    NInf(String, String),
    Nan(String, String),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum BitVecConst {
    Hex(String),
    Bin(Vec<char>),
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Logic {
    QF_SLIA(),
    Other(String),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum AstNode {
    Script(ScriptRc),
    Command(CommandRc),
    Constant(ConstantRc),
    Symbol(SymbolRc),
    SExp(SExpRc),
    Logic(LogicRc),
    BoolOp(BoolOpRc),
    Sort(SortRc),
    Open(),
    Close(),
}

impl Script {
    #[allow(unused)]
    pub fn to_f(&mut self, filepath: &Path) {
        fs::write(filepath, self.to_string());
    }

    pub fn insert(&mut self, i: usize, cmd: Command) {
        let Script::Commands(cmds) = self;
        if i > cmds.len() {
            cmds.push(rccell!(cmd));
        } else {
            cmds.insert(i, rccell!(cmd));
        }
    }

    pub fn insert_all(&mut self, i: usize, new_cmds: &Vec<CommandRc>) {
        let Script::Commands(cmds) = self;
        if i > cmds.len() {
            for cmd in new_cmds {
                cmds.push(Rc::clone(cmd));
            }
        } else {
            let mut after = cmds.split_off(i);
            for cmd in new_cmds {
                cmds.push(Rc::clone(cmd));
            }
            cmds.append(&mut after);
        }
    }

    pub fn split(mut me: Self, i: usize) -> (Script, Script) {
        let Script::Commands(cmds) = &mut me;
        if i > cmds.len() {
            (me, Script::Commands(vec![]))
        } else {
            let after = cmds.split_off(i);
            (me, Script::Commands(after))
        }
    }

    pub fn replace(&mut self, i: usize, cmd: Command) {
        let Script::Commands(cmds) = self;
        cmds[i] = rccell!(cmd);
    }

    pub fn init(&mut self, i: usize) {
        let Script::Commands(cmds) = self;
        cmds.insert(i, rccell!(Command::Assert(rccell!(SExp::true_sexp()))));
    }

    pub fn index_is_gm(&self, i: usize) -> bool {
        let Script::Commands(cmds) = self;
        if i >= cmds.len() {
            return false;
        }
        match &*cmds[i].borrow() {
            Command::GetModel() => true,
            _ => false,
        }
    }

    pub fn is_unsupported_logic(&self) -> bool {
        let Script::Commands(cmds) = self;
        let maybe_logic = cmds.iter().find(|cmd| cmd.borrow().is_logic());
        match maybe_logic {
            Some(cmd) => match &*cmd.borrow() {
                Command::Logic(l) => match *l.borrow() {
                    Logic::QF_SLIA() => true,
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Script::Commands(cmds) => cmds
                .iter()
                .map(|cmd| write!(f, "{}\n", cmd.borrow()))
                .fold(Ok(()), acc_result),
        }
    }
}

impl Command {
    pub fn is_logic(&self) -> bool {
        match self {
            Command::Logic(_) => true,
            _ => false,
        }
    }

    pub fn is_checksat(&self) -> bool {
        match self {
            Command::CheckSat() | Command::CheckSatAssuming(_) => true,
            _ => false,
        }
    }

    pub fn no_op() -> Self {
        Command::Generic("".to_string())
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Logic(l) => write!(f, "(set-logic {})", l.borrow()),
            Command::CheckSat() => write!(f, "(check-sat)"),
            Command::CheckSatAssuming(sexp) => write!(f, "(check-sat-assuming {})", sexp.borrow()),
            Command::GetModel() => write!(f, "(get-model)"),
            Command::DeclConst(v, s) => write!(f, "(declare-const {} {})", v, s.borrow()),
            Command::GenericDecl(name, rest) => {
                write!(f, "({} ", name)?;
                rest.iter()
                    .enumerate()
                    .map(|(i, chunks)| {
                        if i != 0 {
                            write!(f, " ")?;
                        }
                        chunks
                            .iter()
                            .map(|s| write!(f, "{}", s))
                            .fold(Ok(()), acc_result)
                    })
                    .fold(Ok(()), acc_result)?;
                write!(f, ")")
            }
            Command::DeclFn(name, args, rtype) => {
                write!(f, "(declare-fun {} (", name)?;
                args.iter()
                    .enumerate()
                    .map(|(i, arg)| match i == 0 {
                        true => write!(f, "{}", arg),
                        false => write!(f, " {}", arg),
                    })
                    .fold(Ok(()), acc_result)?;
                write!(f, ") {})", rtype)
            }
            Command::Generic(s) => write!(f, "{}", s),
            Command::Assert(s) => write!(f, "(assert {})", s.borrow()),
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::UInt(s) => write!(f, "{}", s),
            Constant::Dec(d) => write!(f, "{}", d),
            Constant::Hex(s) => write!(f, "#x{}", s),
            Constant::Str(s) => write!(f, "\"{}\"", s),
            Constant::Bool(b) => write!(f, "{}", b),
            Constant::Bin(bv) => write!(f, "#b{}", bv.into_iter().collect::<String>()),
            Constant::Fp(fp) => write!(f, "{}", fp),
        }
    }
}

impl Constant {
    pub fn sort(&self) -> Sort {
        match self {
            Constant::UInt(_) => Sort::UInt(),
            Constant::Dec(_) => Sort::Dec(),
            Constant::Str(_) => Sort::Str(),
            Constant::Bool(_) => Sort::Bool(),
            Constant::Bin(bit_s) => Sort::BitVec(bit_s.len() as u32),
            Constant::Hex(hit_s) => Sort::BitVec((hit_s.len() as u32) * 4),
            Constant::Fp(fp) => match fp {
                FPConst::PInf(m, n)
                | FPConst::NInf(m, n)
                | FPConst::PZero(m, n)
                | FPConst::NZero(m, n)
                | FPConst::Nan(m, n) => Sort::Fp(m.to_owned(), n.to_owned()),
                FPConst::Num(_, e, s) => Sort::Fp(e.len().to_string(), (s.len() + 1).to_string()),
            },
        }
    }
}

impl BitVecConst {
    fn len(&self) -> u32 {
        match self {
            BitVecConst::Bin(bits) => bits.len() as u32,
            BitVecConst::Hex(hits) => (hits.len() as u32) * 4,
        }
    }
}

impl FPConst {
    pub fn get_eb_sb(&self) -> (String, String) {
        match self {
            FPConst::Num(_, ebv, sbv) => (ebv.len().to_string(), (sbv.len() + 1).to_string()),
            FPConst::PZero(eb, sb)
            | FPConst::NZero(eb, sb)
            | FPConst::PInf(eb, sb)
            | FPConst::NInf(eb, sb)
            | FPConst::Nan(eb, sb) => (eb.to_owned(), sb.to_owned()),
        }
    }
}

impl fmt::Display for FPConst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FPConst::Num(i, e, s) => write!(f, "(fp {} {} {})", i, e, s),
            FPConst::PInf(m, n) => write!(f, "(_ +oo {} {})", m, n),
            FPConst::NInf(m, n) => write!(f, "(_ -oo {} {})", m, n),
            FPConst::PZero(m, n) => write!(f, "(_ +zero {} {})", m, n),
            FPConst::NZero(m, n) => write!(f, "(_ -zero {} {})", m, n),
            FPConst::Nan(m, n) => write!(f, "(_ NaN {} {})", m, n),
        }
    }
}

impl fmt::Display for BitVecConst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BitVecConst::Hex(s) => write!(f, "#x{}", s),
            BitVecConst::Bin(bv) => write!(f, "#b{}", bv.into_iter().collect::<String>()),
        }
    }
}

fn acc_result(acc: fmt::Result, curr: fmt::Result) -> fmt::Result {
    match curr {
        Ok(_) => acc,
        Err(e) => Err(e),
    }
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sort::UInt() => write!(f, "Int"),
            Sort::Dec() => write!(f, "Real"),
            Sort::Bool() => write!(f, "Bool"),
            Sort::Str() => write!(f, "String"),
            Sort::Fp(m, n) => write!(f, "(_ FloatingPoint {} {})", m, n),
            Sort::BitVec(len) => write!(f, "(_ BitVec {})", len),
            Sort::Array() => write!(f, "Array"),
            Sort::UserDef(s) => write!(f, "{}", s),
            Sort::Compound(v) => {
                write!(f, "(")?;
                v.iter()
                    .enumerate()
                    .map(|(i, sort)| match i == 0 {
                        true => write!(f, "{}", sort.borrow()),
                        false => write!(f, " {}", sort.borrow()),
                    })
                    .fold(Ok(()), acc_result)?;
                write!(f, ")")
            }
        }
    }
}

impl SExp {
    pub fn true_sexp() -> SExp {
        SExp::Symbol(rccell!(Symbol::Token("true".to_owned())))
    }

    pub fn false_sexp() -> SExp {
        SExp::Symbol(rccell!(Symbol::Token("false".to_owned())))
    }

    pub fn fp_sort(&self) -> Option<(String, String)> {
        match self {
            SExp::Constant(c) => match &*c.borrow() {
                Constant::Fp(fpc) => Some(fpc.get_eb_sb()),
                _ => None,
            },
            SExp::FPExp(_, sort, _) => sort.clone(),
            SExp::Compound(v) => v
                .iter()
                .map(|sexp| sexp.borrow().fp_sort())
                .find(|r| r.is_some())
                .flatten(),
            // Could do variable lookups here?
            _ => None,
        }
    }

    pub fn sort(&self) -> Option<Sort> {
        match self {
            SExp::Constant(c) => Some(c.borrow().sort()),
            SExp::BExp(_, _) => Some(Sort::Bool()),
            SExp::FPExp(_, s, _) => match s {
                Some((eb, sb)) => Some(Sort::Fp(eb.clone(), sb.clone())),
                None => None,
            }
            SExp::NExp(_, _) => Some(Sort::Dec()),
            SExp::StrExp(_, _) => Some(Sort::Str()),
            SExp::Let(_, s) | SExp::QForAll(_, s) | SExp::QExists(_, s) => s.borrow().sort(),
            SExp::Symbol(_) | SExp::Compound(_) => None, 
        }
    }
}

impl fmt::Display for SExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExp::Constant(c) => write!(f, "{}", c.borrow()),
            SExp::Symbol(s) => write!(f, "{}", s.borrow()),
            SExp::Let(vbs, s) => {
                write!(f, "(let (")?;
                vbs.iter()
                    .map(|(v, s)| write!(f, "({} {})", v.borrow(), s.borrow()))
                    .fold(Ok(()), acc_result)?;
                write!(f, ") {})", s.borrow())
            }
            SExp::Compound(v) => {
                write!(f, "(")?;
                v.iter()
                    .enumerate()
                    .map(|(i, sexp)| match i == 0 {
                        true => write!(f, "{}", sexp.borrow()),
                        false => write!(f, " {}", sexp.borrow()),
                    })
                    .fold(Ok(()), acc_result)?;
                write!(f, ")")
            }
            SExp::BExp(o, v) => {
                write!(f, "({} ", o.borrow())?;
                v.iter()
                    .enumerate()
                    .map(|(i, sexp)| match i == 0 {
                        true => write!(f, "{}", sexp.borrow()),
                        false => write!(f, " {}", sexp.borrow()),
                    })
                    .fold(Ok(()), acc_result)?;
                write!(f, ")")
            }
            SExp::FPExp(o, _s, v) => {
                write!(f, "({} ", o)?;
                v.iter()
                    .enumerate()
                    .map(|(i, sexp)| match i == 0 {
                        true => write!(f, "{}", sexp.borrow()),
                        false => write!(f, " {}", sexp.borrow()),
                    })
                    .fold(Ok(()), acc_result)?;
                write!(f, ")")
            }
            SExp::NExp(o, v) => {
                write!(f, "({} ", o)?;
                v.iter()
                    .enumerate()
                    .map(|(i, sexp)| match i == 0 {
                        true => write!(f, "{}", sexp.borrow()),
                        false => write!(f, " {}", sexp.borrow()),
                    })
                    .fold(Ok(()), acc_result)?;
                write!(f, ")")
            }
            SExp::StrExp(o, v) => {
                write!(f, "({} ", o)?;
                v.iter()
                    .enumerate()
                    .map(|(i, sexp)| match i == 0 {
                        true => write!(f, "{}", sexp.borrow()),
                        false => write!(f, " {}", sexp.borrow()),
                    })
                    .fold(Ok(()), acc_result)?;
                write!(f, ")")
            }
            SExp::QForAll(v, s) => {
                write!(f, "(forall (")?;
                v.iter()
                    .map(|(va, vl)| write!(f, "({} {})", va.borrow(), vl.borrow()))
                    .fold(Ok(()), acc_result)?;
                write!(f, ") {})", s.borrow())
            }
            SExp::QExists(v, s) => {
                write!(f, "(exists (")?;
                v.iter()
                    .map(|(va, vl)| write!(f, "({} {})", va.borrow(), vl.borrow()))
                    .fold(Ok(()), acc_result)?;
                write!(f, ") {})", s.borrow())
            }
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Token(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for StrOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StrOp::Strcat() => write!(f, "str.++"),
            StrOp::StrAt() => write!(f, "str.at"),
            StrOp::StrFromCode() => write!(f, "str.from_code"),
            StrOp::Substr() => write!(f, "str.substr"),
            StrOp::StrReplace() => write!(f, "str.replace"),
            StrOp::StrReplaceAll() => write!(f, "str.replace_all"),
            StrOp::StrReplaceRe() => write!(f, "str.replace_re"),
            StrOp::StrReplaceReAll() => write!(f, "str.replace_re_all"),
        }
    }
}

impl fmt::Display for NumOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumOp::Sub() => write!(f, "-"),
            NumOp::Add() => write!(f, "+"),
            NumOp::Mul() => write!(f, "*"),
            NumOp::Div() => write!(f, "/"),
            NumOp::FpToReal() => write!(f, "fp.to_real"),
            NumOp::FpToInt() => write!(f, "fp.to_int"),
            NumOp::IntDiv() => write!(f, "div"),
            NumOp::Mod() => write!(f, "mod"),
            NumOp::StrLen() => write!(f, "str.len"),
            NumOp::StrToCode() => write!(f, "str.to_code"),
            NumOp::StrToInt() => write!(f, "str.to_int"),
            NumOp::StrIndexOf() => write!(f, "str.indexof"),
        }
    }
}

impl fmt::Display for FpOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FpOp::Abs() => write!(f, "fp.abs"),
            FpOp::Neg() => write!(f, "fp.neg"),
            FpOp::Add() => write!(f, "fp.add"),
            FpOp::Sub() => write!(f, "fp.sub"),
            FpOp::Mul() => write!(f, "fp.mul"),
            FpOp::Div() => write!(f, "fp.div"),
            FpOp::Fma() => write!(f, "fp.fma"),
            FpOp::Sqrt() => write!(f, "fp.sqrt"),
            FpOp::Rem() => write!(f, "fp.rem"),
            FpOp::RTI() => write!(f, "fp.roundToIntegral"),
            FpOp::Min() => write!(f, "fp.min"),
            FpOp::Max() => write!(f, "fp.max"),
            FpOp::ToFp(eb, sb) => write!(f, "(_ to_fp {} {})", eb, sb),
        }
    }
}

impl fmt::Display for BoolOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoolOp::And() => write!(f, "and"),
            BoolOp::Or() => write!(f, "or"),
            BoolOp::Xor() => write!(f, "xor"),
            BoolOp::Implies() => write!(f, "=>"),
            BoolOp::Distinct() => write!(f, "distinct"),
            BoolOp::Equals() => write!(f, "="),
            BoolOp::Gt() => write!(f, ">"),
            BoolOp::Lt() => write!(f, "<"),
            BoolOp::Gte() => write!(f, ">="),
            BoolOp::Lte() => write!(f, "<="),
            BoolOp::Bvgt() => write!(f, "bvugt"),
            BoolOp::Bvge() => write!(f, "bvuge"),
            BoolOp::Bvslt() => write!(f, "bvslt"),
            BoolOp::Bvsle() => write!(f, "bvsle"),
            BoolOp::Bvsgt() => write!(f, "bvsgt"),
            BoolOp::Bvsge() => write!(f, "bvsge"),
            BoolOp::Fplt() => write!(f, "fp.lt"),
            BoolOp::Fpgeq() => write!(f, "fp.geq"),
            BoolOp::Fpgt() => write!(f, "fp.gt"),
            BoolOp::Fpleq() => write!(f, "fp.leq"),
            BoolOp::Fpeq() => write!(f, "fp.eq"),
            BoolOp::Fpnorm() => write!(f, "fp.isNormal"),
            BoolOp::Fpsubn() => write!(f, "fp.isSubnormal"),
            BoolOp::Fpzero() => write!(f, "fp.isZero"),
            BoolOp::Fpinf() => write!(f, "fp.isInfinite"),
            BoolOp::Fpnana() => write!(f, "fp.isNaN"),
            BoolOp::Fpneg() => write!(f, "fp.isNegative"),
            BoolOp::Fppos() => write!(f, "fp.isPositive"),
            BoolOp::Strlt() => write!(f, "str.<"),
            BoolOp::Strlte() => write!(f, "str.<="),
            BoolOp::Strre() => write!(f, "str.in_re"),
            BoolOp::Strpref() => write!(f, "str.prefixof"),
            BoolOp::Strsuff() => write!(f, "str.suffixof"),
            BoolOp::Strcont() => write!(f, "str.contains"),
            BoolOp::Strisdig() => write!(f, "str.is_digit"),
        }
    }
}

impl fmt::Display for Logic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Logic::QF_SLIA() => write!(f, "QF_SLIA"),
            Logic::Other(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    use insta::assert_debug_snapshot;
    use insta::assert_display_snapshot;

    #[test]
    fn decl_generic_display_snap() {
        let str_script = "(define-fun smt_set_cup ((s1 mySet) (s2 mySet)) mySet (union s1 s2))";
        let p = script(str_script).unwrap().1;

        assert_display_snapshot!(p.to_string().trim());
    }

    #[test]
    fn split_snap() {
        let script = Script::Commands(vec![
            rccell!(Command::Assert(rccell!(SExp::Constant(rccell!(
                Constant::Bool(true)
            ))))),
            rccell!(Command::CheckSat()),
        ]);
        let (s1, s2) = Script::split(script, 1);
        assert_debug_snapshot!((s1.to_string(), s2.to_string()));
    }

    #[test]
    fn decl_fn_display_snap() {
        let my_fn = Command::DeclFn(Symbol::Token("f".to_owned()), vec![], Sort::Bool());
        assert_debug_snapshot!(format!("{}", my_fn));
    }

    #[test]
    fn decl_fn_args_display_snap() {
        let args = vec![Sort::Dec(), Sort::Bool()];
        let my_fn = Command::DeclFn(Symbol::Token("f".to_owned()), args, Sort::Bool());
        assert_debug_snapshot!(format!("{}", my_fn));
    }

    #[test]
    fn sort_display_snap() {
        let sort = Sort::Compound(vec![rccell!(Sort::UInt()), rccell!(Sort::Bool())]);
        assert_debug_snapshot!(format!("{}", sort));
    }

    #[test]
    fn let_to_string_snap() {
        assert_debug_snapshot!(script(
            "(assert (let ((max (seq.nth A m))(n   (seq.len A))) (ite (or true false))))"
        )
        .unwrap()
        .1
        .to_string());
    }
}
