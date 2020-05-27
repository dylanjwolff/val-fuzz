use crate::Timer;
use std::cell::RefCell;
use std::fmt;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;
#[macro_use]
use serde::{Serialize, Deserialize};

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
    DeclConst(String, SortRc),
    Generic(String),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Sort {
    UInt(),
    Dec(),
    Str(),
    Bool(),
    BitVec(u32),
    Array(),
    UserDef(String),
    Compound(Vec<SortRc>),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum SExp {
    Compound(Vec<SExpRc>),
    Let(Vec<(SymbolRc, SExpRc)>, SExpBoxRc),
    QForAll(Vec<(SymbolRc, SortRc)>, SExpBoxRc),
    BExp(BoolOpRc, Vec<SExpRc>),
    Constant(ConstantRc),
    Symbol(SymbolRc),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Symbol {
    Var(String), // Currently, it is hard to detect all variables, so some Tokens may also be
    // variables too
    Token(String),
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
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Constant {
    UInt(String),
    Dec(String),
    Hex(String),
    Bin(Vec<char>),
    Str(String),
    Bool(bool),
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
                write!(f, " {})", s.borrow())
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
            SExp::QForAll(v, s) => {
                write!(f, "(forall (")?;
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
            Symbol::Var(s) | Symbol::Token(s) => write!(f, "{}", s),
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
    use walkdir::WalkDir;

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
