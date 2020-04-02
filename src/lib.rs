#[macro_use]
extern crate nom;

use nom::{
    bytes::complete::{tag},
    combinator::map,
    sequence::tuple,
    IResult,
};

use std::fs;
use nom::character::complete::not_line_ending;
use nom::character::complete::line_ending;
use nom::number::complete::recognize_float;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::character::complete::multispace0;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::combinator::not;
use nom::multi::many0;
use nom::multi::many1;
use nom::character::complete::digit1;
use nom::character::complete::hex_digit1;
use nom::combinator::peek;


#[derive(Debug, Eq, PartialEq)]
enum Script<'a> {
    Commands(Vec<Command<'a>>),
}

#[derive(Debug, Eq, PartialEq)]
enum Command<'a> {
    Logic(),
    CheckSat(),
    CheckSatAssuming(SExp<'a>),
    Assert(SExp<'a>),
    GetModel(),
    DeclConst(&'a str, Sort<'a>),
    Generic(Vec<&'a str>),
}

#[derive(Debug, Eq, PartialEq)]
enum Sort<'a> {
    UInt(),
    Dec(),
    Str(),
    Bool(),
//    Hex(),
//    Bin(),
    BitVec(),
    Array(),
    UserDef(&'a str),
    Compound(Vec<Sort<'a>>),
}

#[derive(Debug, Eq, PartialEq)]
enum SExp<'a> {
    Compound(Vec<SExp<'a>>),
    Constant(Constant<'a>),
    Symbol(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
enum Constant<'a> {
    UInt(&'a str),
    Dec(&'a str),
    Hex(&'a str),
    Bin(Vec<char>),
    Str(&'a str),
    Bool(bool),
}

impl<'a> Script<'a> {
    fn to_string(&'a self) -> String {
        match self {
            Script::Commands(cmds) => 
                cmds.iter()
                .map(|cmd| cmd.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }
}

impl<'a> Command<'a> {
    fn to_string(&'a self) -> String {
      match self {
          Command::Logic() => "(set-logic QLIA)".to_string(), // TODO
          Command::CheckSat() => "(check-sat)".to_string(),
          Command::CheckSatAssuming(sexp) => ("(check-sat-assuming ".to_owned() + &sexp.to_string()[..] + ")").to_string(), // TODO
          Command::GetModel() => "(get-model)".to_string(),
          Command::DeclConst(v, s) => ("(declare-const ".to_string() + v + " " + &s.to_string()[..] + ")").to_string(),
          Command::Generic(v) => v.join(""),
          Command::Assert(s) => ("(assert ".to_string() + &s.to_string()[..] + ")").to_string(),
      }
    }
}

impl<'a> Constant<'a> {
    fn to_string(&'a self) -> String {
        match self {
            Constant::UInt(s) => s.to_string(),
            Constant::Dec(d) => d.to_string(),
            Constant::Hex(s) => {
                "#x".to_string() + &s.to_string()[..]
            },
            Constant::Str(s) => {
                "\"".to_string() + &s.to_string()[..] + "\""
            },
            Constant::Bool(b) => b.to_string(),
            Constant::Bin(bv) => {
                "#b".to_string() + &bv.into_iter().collect::<String>()[..]
            },
        }
    }
}

impl<'a> Sort<'a> {
    fn to_string(&'a self) -> String {
        match self {
            Sort::UInt() => "Int".to_string(),
            Sort::Dec() =>  "Real".to_string(),
            Sort::Bool() =>  "Bool".to_string(),
            Sort::Str() =>  "String".to_string(),
            Sort::BitVec() =>  "BitVec".to_string(),
            Sort::Array() =>  "Array".to_string(),
            Sort::UserDef(s) =>  s.to_string(),
            Sort::Compound(v) =>  {
                let mut rec_s = v.iter().map(|sort| sort.to_string()).collect::<Vec<String>>().join(" ");
                rec_s.insert(0, '(');  // TODO
                rec_s.push(')'); 
                rec_s
            },
        }
    }
}

impl<'a> SExp<'a> {
    fn to_string(&'a self) -> String {
        match self {
            SExp::Constant(c) => c.to_string(),
            SExp::Symbol(s) =>   s.to_string(),
            SExp::Compound(v) => {
                let mut rec_s = v.iter().map(|sexp| sexp.to_string()).collect::<Vec<String>>().join(" ");
                rec_s.insert(0, '(');  // TODO
                rec_s.push(')'); 
                rec_s
            },
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
    map(tuple((tag("#x"), hex_digit1)),|(_, h)| h)(s)
}

fn bin(s: &str) -> IResult<&str, Vec<char>> {
    // Fn doesn't implement clone for some reason
    let bstring_orig = many1(alt((char('0'), char('1'))));
    let bstring_clne = many1(alt((char('0'), char('1'))));

    alt((
        bstring_orig,
        map(tuple((tag("#b"), bstring_clne)), |(_, b)| b)
    ))(s)
}

fn not_quote(s: &str) -> IResult<&str, &str> {
    take_while(|c| c != '"')(s)
}

fn string(s: &str) -> IResult<&str, &str> {
    map(tuple((char('"'), not_quote, char('"'))), |(_, sout, _)| sout)(s)
}

fn constant(s: &str) -> IResult<&str, Constant> {
    alt((
        map(integer, |i| Constant::UInt(i)),
        map(decimal, |d| Constant::Dec(d)),
        map(hex,     |h| Constant::Hex(h)),
        map(bin,     |b| Constant::Bin(b)),
        map(string,  |s| Constant::Str(s)),
    ))(s)
}

fn symbol(s: &str) -> IResult<&str, &str> {
    take_while1(|c : char| !c.is_whitespace() && !(c=='(') && !(c ==')'))(s)
}

fn sexp(s: &str) -> IResult<&str, SExp> {
    let rec_sexp = delimited(char('('), many1(sexp), char(')'));
    let ws_rec_sexp = delimited(multispace0, rec_sexp, multispace0);
    let ws_constant = delimited(multispace0, constant, multispace0);
    let ws_symbol = delimited(multispace0, symbol, multispace0);
    alt((
        map(ws_rec_sexp, |e| SExp::Compound(e)),
        map(ws_constant, |c| SExp::Constant(c)),
        map(ws_symbol,   |s| SExp::Symbol(s)),
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
        map(ws_userdef, |s| Sort::UserDef(s)),
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

fn naked_logic(s: &str) -> IResult<&str, &str> {
    let ws_ltag = delimited(multispace0, tag("set-logic"), multispace0);
    let ws_logic = delimited(multispace0, symbol, multispace0);
    preceded(ws_ltag, ws_logic)(s)

}

fn naked_command(s: &str) -> IResult<&str, Command> {
    alt((
        map(naked_assert,      |a| Command::Assert(a)),
        map(naked_csa,         |a| Command::CheckSatAssuming(a)),
        map(tag("check-sat"),  |_| Command::CheckSat()),
        map(tag("get-model"),  |_| Command::GetModel()),
        map(naked_logic,       |_| Command::Logic()),
        map(naked_decl_const,  |(v, s)| Command::DeclConst(v, s)),
    ))(s)
}

fn unknown_balanced(s: &str) -> IResult<&str, Vec<&str>> {
    alt((
        map(tuple((char('('), many0(unknown_balanced), char(')'))), 
            |(_, v, _)| {
                let mut vflat = v.concat();
                vflat.insert(0, "(");
                vflat.push(")");
                vflat
            }),
        // Trim whitespace here
        map(take_while1(|c| !(c == '(') && !(c == ')')), |s| vec![s]),
    ))(s)
}


fn command(s: &str) -> IResult<&str, Command> {
    let ws_ncommand =   delimited(multispace0, naked_command, multispace0);
    let command = delimited(char('('), ws_ncommand, char(')'));
    alt((
        delimited(multispace0, command, multispace0),
        map(unknown_balanced,  |s| Command::Generic(s)),
    ))(s)
}


fn script(s: &str) -> IResult<&str, Script> {
    map(
        many0(delimited(multispace0, command,multispace0)),
        |cmds| Script::Commands(cmds)
    )(s)
}

fn rmv_comments(s: &str) -> IResult<&str, Vec<&str>> {
    let not_comment = take_while1(|c| !(c == ';'));
   let comment = delimited(char(';'), not_line_ending, line_ending);
    many1(alt((
        not_comment,
        map(comment, |_| ""),
    )))(s)
}


pub fn exec() {
    let files = fs::read_dir("samples").expect("error with sample dir");

    for file_res in files {
        let file = file_res.expect("problem with file");
        println!("Starting {:?}", file);
        let filepath = file.path();
        let contents = &fs::read_to_string(filepath).expect("error reading file")[..];
        let contents_ = &rmv_comments(contents).expect("failed to rmv comments").1.join(" ")[..];
        match (script(contents_), script(contents_)) {
            (Ok((_, a)), Ok((_, b))) => assert_eq!(a, 
                                                   script(&b.to_string()[..]).expect("Failed on second parse").1
                                                   ),
            (Err(e), _) |
            (_, Err(e)) => panic!("SMT Parse Error {}", e),
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn quick_test() {
       println!("{:?}", unknown_balanced("((declare-fun sin )Real Real() decl)"));
    }

    #[test]
    fn smoke_test() {
       exec();
    }

    #[test]
    fn visual_test() {
        let contents = &fs::read_to_string("ex.smt2").expect("error reading file")[..];
        println!("Script: {:?}", script(contents));

        match script(contents) {
            Ok((_, script)) => { 
                println!("restrung: {}", script.to_string());
            }
            _ => ()
        };
    }
}
