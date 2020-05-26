use crate::Timer;
use std::cell::RefCell;
use std::fmt;
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
    pub fn to_string_dfltto(&self) -> Option<String> {
        Some(self.to_string())
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
    pub fn to_string_t(&self, timer: Timer) -> Option<String> {
        if timer.is_done() {
            return None;
        }

        Some(match self {
            Command::Logic(l) => format!("(set-logic {})", l.borrow().to_string_t()),
            Command::CheckSat() => "(check-sat)".to_string(),
            Command::CheckSatAssuming(sexp) => format!(
                "(check-sat-assuming {})",
                sexp.borrow().to_string_t(timer.clone())?
            ),
            Command::GetModel() => "(get-model)".to_string(),
            Command::DeclConst(v, s) => format!(
                "(declare-const {} {})",
                v,
                s.borrow().to_string_t(timer.clone())?
            ),
            Command::Generic(s) => s.clone(),
            Command::Assert(s) => format!("(assert {})", s.borrow().to_string_t(timer.clone())?),
        })
    }

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

impl Constant {
    pub fn to_string_t(&self) -> String {
        match self {
            Constant::UInt(s) => s.to_string(),
            Constant::Dec(d) => d.to_string(),
            Constant::Hex(s) => format!("#x{}", s.to_string()),
            Constant::Str(s) => format!("\"{}\"", s.to_string()),
            Constant::Bool(b) => b.to_string(),
            Constant::Bin(bv) => format!("#b{}", bv.into_iter().collect::<String>()),
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

impl Sort {
    pub fn to_string_t(&self, timer: Timer) -> Option<String> {
        if timer.is_done() {
            return None;
        }

        Some(match self {
            Sort::UInt() => "Int".to_string(),
            Sort::Dec() => "Real".to_string(),
            Sort::Bool() => "Bool".to_string(),
            Sort::Str() => "String".to_string(),
            Sort::BitVec(len) => format!("(_ BitVec {})", len),
            Sort::Array() => "Array".to_string(),
            Sort::UserDef(s) => s.to_string(),
            Sort::Compound(v) => {
                let rec_s = v
                    .iter()
                    .map(|sort| Box::new(sort.borrow().to_string_t(timer.clone())))
                    .map(|bs| *bs)
                    .collect::<Option<Vec<String>>>()?
                    .join(" ");
                format!("({})", rec_s)
            }
        })
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
                    .map(|sort| write!(f, " {} ", sort.borrow()))
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

    pub fn to_string_t(&self, timer: Timer) -> Option<String> {
        if timer.is_done() {
            return None;
        }

        Some(match self {
            SExp::Constant(c) => c.borrow().to_string_t(),
            SExp::Symbol(s) => s.borrow().to_string_t(),
            SExp::Let(vbs, s) => {
                let vbss = Box::new(
                    vbs.iter()
                        .map(|(v, s)| {
                            Some((
                                v.borrow().to_string_t(),
                                s.borrow().to_string_t(timer.clone())?,
                            ))
                        })
                        .collect::<Option<Vec<(String, String)>>>()?
                        .iter()
                        .map(|(vs, ss)| Box::new(format!("({} {})", vs, ss)))
                        .map(|bs| *bs)
                        .collect::<String>(),
                );
                format!(
                    "(let ({}) {})",
                    vbss,
                    s.borrow().to_string_t(timer.clone())?
                )
            }
            SExp::Compound(v) => {
                let rec_s = v
                    .iter()
                    .map(|sexp| Box::new(sexp.borrow().to_string_t(timer.clone())))
                    .map(|bs| *bs)
                    .collect::<Option<Vec<String>>>()?
                    .join(" ");
                format!("({})", rec_s)
            }
            SExp::BExp(o, v) => {
                let rec_s = v
                    .iter()
                    .map(|sexp| Box::new(sexp.borrow().to_string_t(timer.clone())))
                    .map(|bs| *bs)
                    .collect::<Option<Vec<String>>>()?
                    .join(" ");
                format!("({} {})", o.borrow().to_string_t(), rec_s)
            }
            SExp::QForAll(v, s) => {
                let vbss = v
                    .iter()
                    .map(|(va, vl)| {
                        Some(format!(
                            "({} {})",
                            va.borrow().to_string_t(),
                            vl.borrow().to_string_t(timer.clone())?
                        ))
                    })
                    .collect::<Option<Vec<String>>>()?
                    .join("");
                format!(
                    "(forall ({}) {})",
                    vbss,
                    s.borrow().to_string_t(timer.clone())?
                )
            }
        })
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
                    .map(|sexp| write!(f, " {} ", sexp.borrow()))
                    .fold(Ok(()), acc_result)?;
                write!(f, ")")
            }
            SExp::BExp(o, v) => {
                write!(f, "({} ", o.borrow())?;
                v.iter()
                    .map(|sexp| write!(f, " {} ", sexp.borrow()))
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

impl Symbol {
    pub fn to_string_t(&self) -> String {
        match self {
            Symbol::Var(s) | Symbol::Token(s) => s.clone(),
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

impl BoolOp {
    pub fn to_string_t(&self) -> String {
        match self {
            BoolOp::And() => "and".to_owned(),
            BoolOp::Or() => "or".to_owned(),
            BoolOp::Xor() => "xor".to_owned(),
            BoolOp::Implies() => "=>".to_owned(),
            BoolOp::Distinct() => "distinct".to_owned(),
            BoolOp::Equals() => "=".to_owned(),
            BoolOp::Gt() => ">".to_owned(),
            BoolOp::Lt() => "<".to_owned(),
            BoolOp::Gte() => ">=".to_owned(),
            BoolOp::Lte() => "<=".to_owned(),
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

impl Logic {
    pub fn to_string_t(&self) -> String {
        match self {
            Logic::QF_SLIA() => "QF_SLIA".to_owned(),
            Logic::Other(s) => s.clone(),
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
        .to_string_dfltto());
    }
}
