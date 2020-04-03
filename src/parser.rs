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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Script {
    Commands(Vec<Command>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command {
    Logic(Logic),
    CheckSat(),
    CheckSatAssuming(SExp),
    Assert(SExp),
    GetModel(),
    DeclConst(String, Sort),
    Generic(Vec<String>),
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
    Compound(Vec<Sort>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SExp {
    Compound(Vec<SExp>),
    Let(Vec<(SExp, Sort)>, Box<SExp>),
    BExp(BoolOp, Vec<SExp>),
    Constant(Constant),
    Symbol(String),
    Var(String), // Not used for parsing, only manipulation of the ast so we don't need to do
                 // lifetime gymnastics... vars are always parsed as Symbols
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

impl Script {
    pub fn to_string(&self) -> String {
        match self {
            Script::Commands(cmds) => cmds
                .iter()
                .map(|cmd| cmd.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }

    pub fn insert(&mut self, i : usize, cmd : Command) {
        let Script::Commands(cmds) = self;
        cmds.insert(i, cmd);
    }

    pub fn replace(&mut self, i : usize, cmd : Command) {
        let Script::Commands(cmds) = self;
        cmds[i] = cmd;
    }

    pub fn init(&mut self, i : usize) {
        let Script::Commands(cmds) = self;
        cmds[i] = Command::Assert(SExp::true_sexp());
    }

    pub fn is_unsupported_logic(&self) -> bool {
        let Script::Commands(cmds) = self;
        let maybe_logic = cmds.iter().find(|cmd| cmd.is_logic());
        match maybe_logic {
            Some(Command::Logic(Logic::QF_SLIA())) => true,
            _ => false,
        }
    }
}

impl Command {
    pub fn to_string(&self) -> String {
        match self {
            Command::Logic(l) => "(set-logic ".to_owned() + &l.to_string()[..] + ")",
            Command::CheckSat() => "(check-sat)".to_string(),
            Command::CheckSatAssuming(sexp) => {
                ("(check-sat-assuming ".to_owned() + &sexp.to_string()[..] + ")").to_string()
            } // TODO
            Command::GetModel() => "(get-model)".to_string(),
            Command::DeclConst(v, s) => {
                ("(declare-const ".to_string() + v + " " + &s.to_string()[..] + ")").to_string()
            }
            Command::Generic(v) => v.join(""),
            Command::Assert(s) => ("(assert ".to_string() + &s.to_string()[..] + ")").to_string(),
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
            Constant::Hex(s) => "#x".to_string() + &s.to_string()[..],
            Constant::Str(s) => "\"".to_string() + &s.to_string()[..] + "\"",
            Constant::Bool(b) => b.to_string(),
            Constant::Bin(bv) => "#b".to_string() + &bv.into_iter().collect::<String>()[..],
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
                let mut rec_s = v
                    .iter()
                    .map(|sort| sort.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                rec_s.insert(0, '('); // TODO
                rec_s.push(')');
                rec_s
            }
        }
    }
}

impl SExp {
    pub fn true_sexp() -> SExp {
        SExp::Symbol("true".to_owned())
    }

    pub fn false_sexp() -> SExp {
        SExp::Symbol("false".to_owned())
    }

    pub fn to_string(&self) -> String {
        match self {
            SExp::Constant(c) => c.to_string(),
            SExp::Symbol(s) => s.to_string(),
            SExp::Let(vbs, s) => {
                let vbss = vbs.iter()
                    .map(|(v, s)| {
                       format!("({} {})", v.to_string(), s.to_string())
                    }).collect::<String>();
                format!("(let {} {})", vbss, s.to_string())
            },
            SExp::Compound(v) => {
                let mut rec_s = v
                    .iter()
                    .map(|sexp| sexp.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                rec_s.insert(0, '('); // TODO
                rec_s.push(')');
                rec_s
            }
            SExp::BExp(o, v) => {
                let rec_s = v
                    .iter()
                    .map(|sexp| sexp.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                let mut s = o.to_string() + " " + &rec_s[..];
                s.insert(0, '('); // TODO
                s.push(')');
                s
            },
            SExp::Var(s) => s.clone(),
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

    delimited(multispace0, alt((naked_bop_tags, naked_bop_chars)), multispace0)(s)
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
    let inner_int = map(tuple((bool_int_ops, many1(sexp))), 
                        |(o, v)| SExp::BExp(o, v));
    let inner_core = map(tuple((bool_core_ops, many1(sexp))), 
                        |(o, v)| SExp::BExp(o, v));

    let naked_b = alt((inner_int, inner_core));
    delimited(char('('), naked_b, char(')'))(s)
}


fn let_sexp(s : &str) -> IResult<&str, (Vec<(SExp, Sort)>, SExp)> {
    let ws_symbol = delimited(multispace0, symbol, multispace0);
    let mapped_ws_symbol = map(ws_symbol, |s| SExp::Symbol(s.to_owned()));
    let ws_sort = delimited(multispace0, sort, multispace0);
    let var_binding = delimited(char('('),
                        tuple((mapped_ws_symbol, ws_sort)),
                    char(')'));
    let ws_var_b = delimited(multispace0, var_binding, multispace0);
    let var_bs = delimited(char('('), many1(ws_var_b), char(')'));
    let ws_var_bs = delimited(multispace0, var_bs, multispace0);
    let inner = preceded(tag("let"), tuple((ws_var_bs, sexp)));
    let ws_inner = delimited(multispace0, inner, multispace0);
    let wrapped = delimited(char('('), ws_inner, char(')'));

    wrapped(s)
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
        map(ws_let_sexp, |(tbs, sexp)| SExp::Let(tbs, Box::new(sexp))),
        map(ws_rec_sexp, |e| SExp::Compound(e)),
        map(ws_constant, |c| SExp::Constant(c)),
        map(ws_symbol, |s| SExp::Symbol(s.to_owned())),
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
        map(ws_rec_sort, |s| Sort::Compound(s)),
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

fn naked_command(s: &str) -> IResult<&str, Command> {
    alt((
        map(naked_assert, |a| Command::Assert(a)),
        map(naked_csa, |a| Command::CheckSatAssuming(a)),
        map(tag("check-sat"), |_| Command::CheckSat()),
        map(tag("get-model"), |_| Command::GetModel()),
        map(naked_logic, |l| Command::Logic(l)),
        map(naked_decl_const, |(v, s)| Command::DeclConst(v.to_owned(), s)),
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
    let command = delimited(char('('), ws_ncommand, char(')'));
    alt((
        delimited(multispace0, command, multispace0),
        map(unknown_balanced, |v| Command::Generic(
                v.into_iter()
                    .map(|g| g.to_owned())
                    .collect::<Vec<String>>())),
    ))(s)
}

pub fn script(s: &str) -> IResult<&str, Script> {
    map(
        many0(delimited(multispace0, command, multispace0)),
        |cmds| Script::Commands(cmds),
    )(s)
}

pub fn rmv_comments(s: &str) -> IResult<&str, Vec<&str>> {
    let not_comment = take_while1(|c| !(c == ';'));
    let comment = delimited(char(';'), not_line_ending, line_ending);
    many1(alt((not_comment, map(comment, |_| ""))))(s)
}
