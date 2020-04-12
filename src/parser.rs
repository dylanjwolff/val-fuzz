use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::hex_digit1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::not;
use nom::combinator::peek;
use nom::multi::many0;
use nom::multi::many1;
use nom::number::complete::recognize_float;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Script {
    Commands(Vec<CommandRc>),
}

macro_rules! rccell {
    ($x:expr) => {{
        Rc::new(RefCell::new($x))
    }};
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
            Command::Logic(l) => "(set-logic ".to_owned() + &l.borrow().to_string()[..] + ")",
            Command::CheckSat() => "(check-sat)".to_string(),
            Command::CheckSatAssuming(sexp) => {
                ("(check-sat-assuming ".to_owned() + &sexp.borrow().to_string()[..] + ")")
                    .to_string()
            } // TODO
            Command::GetModel() => "(get-model)".to_string(),
            Command::DeclConst(v, s) => {
                ("(declare-const ".to_string() + v + " " + &s.borrow().to_string()[..] + ")")
                    .to_string()
            }
            Command::Generic(s) => s.clone(),
            Command::Assert(s) => {
                ("(assert ".to_string() + &s.borrow().to_string()[..] + ")").to_string()
            }
        }
    }

    pub fn entry_string(&self) -> String {
        match self {
            Command::Logic(_) => "(set-logic ".to_owned(),
            Command::CheckSat() => "(check-sat)".to_owned(),
            Command::GetModel() => "(get-model)".to_owned(),
            Command::CheckSatAssuming(_) => "(check-sat-assuming ".to_owned(),
            Command::DeclConst(s, _) => format!("(declare-const {} ", s),
            Command::Assert(_) => "(assert ".to_owned(),
            Command::Generic(s) => s.clone(),
        }
    }

    pub fn exit_str(&self) -> &str {
        match self {
            Command::CheckSatAssuming(_)
            | Command::DeclConst(_, _)
            | Command::Assert(_)
            | Command::Logic(_) => ")\n",
            _ => "\n",
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

    pub fn entry_str(&self) -> &str {
        match self {
            Sort::UInt() => "Int",
            Sort::Dec() => "Real",
            Sort::Bool() => "Bool",
            Sort::Str() => "String",
            Sort::BitVec() => "BitVec",
            Sort::Array() => "Array",
            Sort::UserDef(s) => &s,
            Sort::Compound(_) => "(",
        }
    }

    pub fn exit_str(&self) -> &str {
        match self {
            Sort::Compound(_) => ")",
            _ => "",
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
            SExp::Symbol(s) => s.borrow().to_string(),
        }
    }

    pub fn entry_str(&self) -> &str {
        match self {
            SExp::Let(_, _) => "(let ",
            SExp::Compound(_) | SExp::BExp(_, _) => "(",
            _ => "",
        }
    }

    pub fn exit_str(&self) -> &str {
        match self {
            SExp::Let(_, _) | SExp::Compound(_) | SExp::BExp(_, _) => ")",
            _ => "",
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
    fn to_string(&self) -> String {
        match self {
            Logic::QF_SLIA() => "QF_SLIA".to_owned(),
            Logic::Other(s) => s.clone(),
        }
    }
}

fn integer(s: &str) -> IResult<&str, &str> {
    // let inner = |(sn, _peek): (&str, ())| {
    //    sn.parse::<u64>().unwrap()
    // };
    map(tuple((digit1, peek(not(char('.'))))), |(sn, _)| sn)(s)
}

fn decimal(s: &str) -> IResult<&str, &str> {
    recognize_float(s)
}

fn hex(s: &str) -> IResult<&str, &str> {
    map(tuple((tag("#x"), hex_digit1)), |(_, h)| h)(s)
}

fn bin(s: &str) -> IResult<&str, Vec<char>> {
    // Fn doesn't implement clone for some reason
    let bstring_orig = many1(alt((char('0'), char('1'))));
    let bstring_clne = many1(alt((char('0'), char('1'))));

    alt((
        bstring_orig,
        map(tuple((tag("#b"), bstring_clne)), |(_, b)| b),
    ))(s)
}

fn not_quote(s: &str) -> IResult<&str, &str> {
    take_while(|c| c != '"')(s)
}

fn string(s: &str) -> IResult<&str, &str> {
    map(tuple((char('"'), not_quote, char('"'))), |(_, sout, _)| {
        sout
    })(s)
}

fn constant(s: &str) -> IResult<&str, Constant> {
    alt((
        map(integer, |i| Constant::UInt(i.to_owned())),
        map(decimal, |d| Constant::Dec(d.to_owned())),
        map(hex, |h| Constant::Hex(h.to_owned())),
        map(bin, |b| Constant::Bin(b.to_owned())),
        map(string, |s| Constant::Str(s.to_owned())),
    ))(s)
}

fn symbol(s: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && !(c == '(') && !(c == ')'))(s)
}

fn bool_int_ops(s: &str) -> IResult<&str, BoolOp> {
    let naked_bop_tags = alt((
        map(tag(">="), |_| BoolOp::Gte()),
        map(tag("<="), |_| BoolOp::Lte()),
    ));
    let naked_bop_chars = alt((
        map(char('<'), |_| BoolOp::Lt()),
        map(char('>'), |_| BoolOp::Gt()),
    ));

    delimited(
        multispace0,
        alt((naked_bop_tags, naked_bop_chars)),
        multispace0,
    )(s)
}

fn bool_core_ops(s: &str) -> IResult<&str, BoolOp> {
    let naked_bool_tags = alt((
        map(tag("and"), |_| BoolOp::And()),
        map(tag("or"), |_| BoolOp::Or()),
        map(tag("xor"), |_| BoolOp::Xor()),
        map(tag("=>"), |_| BoolOp::Implies()),
        map(tag("distinct"), |_| BoolOp::Distinct()),
    ));
    let naked_eq = map(char('='), |_| BoolOp::Equals());

    delimited(multispace0, alt((naked_bool_tags, naked_eq)), multispace0)(s)
}

fn bool_sexp(s: &str) -> IResult<&str, SExp> {
    let inner_int = map(tuple((bool_int_ops, many1(sexp))), |(o, v)| {
        SExp::BExp(rccell!(o), v.into_iter().map(|s| rccell!(s)).collect())
    });
    let inner_core = map(tuple((bool_core_ops, many1(sexp))), |(o, v)| {
        SExp::BExp(rccell!(o), v.into_iter().map(|s| rccell!(s)).collect())
    });

    let naked_b = alt((inner_int, inner_core));
    delimited(char('('), naked_b, char(')'))(s)
}

fn let_sexp(s: &str) -> IResult<&str, (Vec<(SymbolRc, SExpRc)>, SExpBoxRc)> {
    let ws_symbol = delimited(multispace0, symbol, multispace0);
    let mapped_ws_symbol = map(ws_symbol, |s| Symbol::Var(s.to_owned()));
    let ws_sexp = delimited(multispace0, sexp, multispace0);
    let var_binding = delimited(char('('), tuple((mapped_ws_symbol, ws_sexp)), char(')'));
    let ws_var_b = delimited(multispace0, var_binding, multispace0);
    let var_bs = delimited(char('('), many1(ws_var_b), char(')'));
    let ws_var_bs = delimited(multispace0, var_bs, multispace0);
    let inner = preceded(tag("let"), tuple((ws_var_bs, sexp)));
    let ws_inner = delimited(multispace0, inner, multispace0);
    let wrapped = delimited(char('('), ws_inner, char(')'));
    let mapped = map(wrapped, |(a, b)| {
        (
            a.into_iter()
                .map(|(x, y)| (rccell!(x), rccell!(y)))
                .collect(),
            rccell!(Box::new(b)),
        )
    });
    mapped(s)
}

fn sexp(s: &str) -> IResult<&str, SExp> {
    let rec_sexp = delimited(char('('), many1(sexp), char(')'));
    let ws_rec_sexp = delimited(multispace0, rec_sexp, multispace0);
    let ws_constant = delimited(multispace0, constant, multispace0);
    let ws_symbol = delimited(multispace0, symbol, multispace0);
    let ws_bexp = delimited(multispace0, bool_sexp, multispace0);
    let ws_let_sexp = delimited(multispace0, let_sexp, multispace0);
    alt((
        ws_bexp,
        map(ws_let_sexp, |(tbs, sexp)| SExp::Let(tbs, sexp)),
        map(ws_rec_sexp, |es| {
            SExp::Compound(es.into_iter().map(|e| rccell!(e)).collect())
        }),
        map(ws_constant, |c| SExp::Constant(rccell!(c))),
        map(ws_symbol, |s| {
            SExp::Symbol(rccell!(Symbol::Token(s.to_owned())))
        }),
    ))(s)
}

fn sort(s: &str) -> IResult<&str, Sort> {
    let ws_int = delimited(multispace0, tag("Int"), multispace0);
    let ws_dec = delimited(multispace0, tag("Real"), multispace0);
    let ws_userdef = delimited(multispace0, symbol, multispace0);
    let rec_sort = delimited(char('('), many1(sort), char(')'));
    let ws_rec_sort = delimited(multispace0, rec_sort, multispace0);
    alt((
        map(ws_int, |_| Sort::UInt()),
        map(ws_dec, |_| Sort::Dec()),
        map(ws_userdef, |s| Sort::UserDef(s.to_owned())),
        map(ws_rec_sort, |ss| {
            Sort::Compound(ss.into_iter().map(|s| rccell!(s)).collect())
        }),
    ))(s)
}

fn naked_decl_const(s: &str) -> IResult<&str, (&str, Sort)> {
    let ws_decl = delimited(multispace0, tag("declare-const"), multispace0);
    let ws_symbol = delimited(multispace0, symbol, multispace0);
    let ws_sort = delimited(multispace0, sort, multispace0);
    preceded(ws_decl, tuple((ws_symbol, ws_sort)))(s)
}

fn naked_assert(s: &str) -> IResult<&str, SExp> {
    let ws_atag = delimited(multispace0, tag("assert"), multispace0);
    let ws_sexp = delimited(multispace0, sexp, multispace0);
    preceded(ws_atag, ws_sexp)(s)
}

fn naked_csa(s: &str) -> IResult<&str, SExp> {
    let ws_csatag = delimited(multispace0, tag("check-sat-assuming"), multispace0);
    let ws_sexp = delimited(multispace0, sexp, multispace0);
    preceded(ws_csatag, ws_sexp)(s)
}

fn naked_logic(s: &str) -> IResult<&str, Logic> {
    let ws_ltag = delimited(multispace0, tag("set-logic"), multispace0);
    let qslia = map(tag("QF_SLIA"), |_| Logic::QF_SLIA());
    let other = map(symbol, |s| Logic::Other(s.to_owned()));
    let ws_l = delimited(multispace0, alt((qslia, other)), multispace0);
    preceded(ws_ltag, ws_l)(s)
}

fn set_info_status(s: &str) -> IResult<&str, (&str, &str)> {
    let ws_val = delimited(multispace0, alt((tag("sat"), tag("unsat"))), multispace0);
    let ws_status = delimited(multispace0, tag(":status"), multispace0);
    let naked_si = preceded(tag("set-info"), tuple((ws_status, ws_val)));

    let wrapped = delimited(
        char('('),
        delimited(multispace0, naked_si, multispace0),
        char(')'),
    );
    delimited(multispace0, wrapped, multispace0)(s)
}

fn naked_command(s: &str) -> IResult<&str, Command> {
    alt((
        map(naked_assert, |a| Command::Assert(rccell!(a))),
        map(naked_csa, |a| Command::CheckSatAssuming(rccell!(a))),
        map(tag("check-sat"), |_| Command::CheckSat()),
        map(tag("get-model"), |_| Command::GetModel()),
        map(naked_logic, |l| Command::Logic(rccell!(l))),
        map(naked_decl_const, |(v, s)| {
            Command::DeclConst(v.to_owned(), rccell!(s))
        }),
    ))(s)
}

fn unknown_balanced(s: &str) -> IResult<&str, Vec<&str>> {
    alt((
        map(
            tuple((char('('), many0(unknown_balanced), char(')'))),
            |(_, v, _)| {
                let mut vflat = v.concat();
                vflat.insert(0, "(");
                vflat.push(")");
                vflat
            },
        ),
        // Trim whitespace here
        map(take_while1(|c| !(c == '(') && !(c == ')')), |s| vec![s]),
    ))(s)
}

fn command(s: &str) -> IResult<&str, Command> {
    let ws_ncommand = delimited(multispace0, naked_command, multispace0);
    let delim_command = delimited(char('('), ws_ncommand, char(')'));
    alt((
        preceded(set_info_status, command), // just drop for now
        delimited(multispace0, delim_command, multispace0),
        map(unknown_balanced, |v| {
            Command::Generic(v.into_iter().map(|g| g.to_owned()).collect::<String>())
        }),
    ))(s)
}

pub fn script(s: &str) -> IResult<&str, Script> {
    map(
        many0(delimited(multispace0, command, multispace0)),
        |cmds| Script::Commands(cmds.into_iter().map(|cmd| rccell!(cmd)).collect()),
    )(s)
}

pub fn rmv_comments(s: &str) -> IResult<&str, Vec<&str>> {
    let not_comment = take_while1(|c| !(c == ';'));
    let comment = delimited(char(';'), not_line_ending, line_ending);
    many1(alt((not_comment, map(comment, |_| ""))))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn parse_file(f: &str) -> Script {
        let contents = &fs::read_to_string(f).expect("error reading file")[..];
        let contents_sans_comments = &rmv_comments(contents)
            .expect("failed to rmv comments")
            .1
            .join(" ")[..];

        script(contents_sans_comments).expect("parser error").1
    }
}
