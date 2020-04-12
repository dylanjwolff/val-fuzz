use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Script {
    Commands(Vec<CommandRc>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command {
    Logic(LogicRc),
    CheckSat(),
    CheckSatAssuming(SExpRc),
    Assert(SExpRc),
    GetModel(),
    DeclConst(String, SortRc),
    Generic(String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Sort {
    UInt(),
    Dec(),
    Str(),
    Bool(),
    BitVec(),
    Array(),
    UserDef(String),
    Compound(Vec<SortRc>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SExp {
    Compound(Vec<SExpRc>),
    Let(Vec<(SymbolRc, SExpRc)>, SExpBoxRc),
    BExp(BoolOpRc, Vec<SExpRc>),
    Constant(ConstantRc),
    Symbol(SymbolRc),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Symbol {
    Var(String), // Currently, it is hard to detect all variables, so some Tokens may also be
    // variables too
    Token(String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Constant {
    UInt(String),
    Dec(String),
    Hex(String),
    Bin(Vec<char>),
    Str(String),
    Bool(bool),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Logic {
    QF_SLIA(),
    Other(String),
}

#[derive(Debug, Eq, PartialEq)]
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
    pub fn to_string(&self) -> String {
        match self {
            Script::Commands(cmds) => cmds
                .iter()
                .map(|cmd| cmd.borrow().to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }

    pub fn insert(&mut self, i: usize, cmd: Command) {
        let Script::Commands(cmds) = self;
        cmds.insert(i, rccell!(cmd));
    }

    pub fn replace(&mut self, i: usize, cmd: Command) {
        let Script::Commands(cmds) = self;
        cmds[i] = rccell!(cmd);
    }

    pub fn init(&mut self, i: usize) {
        let Script::Commands(cmds) = self;
        cmds.insert(i, rccell!(Command::Assert(rccell!(SExp::true_sexp()))));
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

impl Command {
    pub fn to_string(&self) -> String {
        match self {
            Command::Logic(l) => format!("(set-logic {})", l.borrow().to_string()),
            Command::CheckSat() => "(check-sat)".to_string(),
            Command::CheckSatAssuming(sexp) =>
                format!("(check-sat-assuming {})", sexp.borrow().to_string()),
            Command::GetModel() => "(get-model)".to_string(),
            Command::DeclConst(v, s) => 
                format!("(declare-const {} {})", v, s.borrow().to_string()),
            Command::Generic(s) => s.clone(),
            Command::Assert(s) => format!("(assert {})", s.borrow().to_string()),
        }
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
}

impl Constant {
    pub fn to_string(&self) -> String {
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

impl Sort {
    pub fn to_string(&self) -> String {
        match self {
            Sort::UInt() => "Int".to_string(),
            Sort::Dec() => "Real".to_string(),
            Sort::Bool() => "Bool".to_string(),
            Sort::Str() => "String".to_string(),
            Sort::BitVec() => "BitVec".to_string(),
            Sort::Array() => "Array".to_string(),
            Sort::UserDef(s) => s.to_string(),
            Sort::Compound(v) => {
                let rec_s = v
                    .iter()
                    .map(|sort| Box::new(sort.borrow().to_string()))
                    .map(|bs| *bs)
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("({})", rec_s)
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

    pub fn to_string(&self) -> String {
        match self {
            SExp::Constant(c) => c.borrow().to_string(),
            SExp::Symbol(s) => s.borrow().to_string(),
            SExp::Let(vbs, s) => {
                let vbss = Box::new(
                    vbs.iter()
                        .map(|(v, s)| {
                            Box::new(format!(
                                "({} {})",
                                v.borrow().to_string(),
                                s.borrow().to_string()
                            ))
                        })
                        .map(|bs| *bs)
                        .collect::<String>(),
                );
                format!("(let ({}) {})", vbss, s.borrow().to_string())
            }
            SExp::Compound(v) => {
                let rec_s = v
                    .iter()
                    .map(|sexp| Box::new(sexp.borrow().to_string()))
                    .map(|bs| *bs)
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("({})", rec_s)
            }
            SExp::BExp(o, v) => {
                let rec_s = v
                    .iter()
                    .map(|sexp| Box::new(sexp.borrow().to_string()))
                    .map(|bs| *bs)
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("({} {})", o.borrow().to_string(), rec_s)
            }
        }
    }
}

impl Symbol {
    pub fn to_string(&self) -> String {
        match self {
            Symbol::Var(s) | Symbol::Token(s) => s.clone(),
        }
    }
}

impl BoolOp {
    pub fn to_string(&self) -> String {
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

impl Logic {
    pub fn to_string(&self) -> String {
        match self {
            Logic::QF_SLIA() => "QF_SLIA".to_owned(),
            Logic::Other(s) => s.clone(),
        }
    }
}


