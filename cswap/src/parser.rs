use super::ast::{
    BoolOp, Command, Constant, Logic, SExp, SExpBoxRc, SExpRc, Script, Sort, SortRc, Symbol,
    SymbolRc,
};
use nom::branch::alt;
use nom::bytes::complete::take_until;
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
use nom::sequence::terminated;
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use std::fs;
use std::path::Path;

macro_rules! ws {
    ($x:expr) => {
        delimited(multispace0, $x, multispace0)
    };
}

macro_rules! brack {
    ($x:expr) => {
        delimited(char('('), $x, char(')'))
    };
}

macro_rules! brack_ws {
    ($x:expr) => {
        delimited(char('('), ws!($x), char(')'))
    };
}

macro_rules! ws_brack {
    ($x:expr) => {
        ws!(delimited(char('('), $x, char(')')))
    };
}

macro_rules! ws_brack_ws {
    ($x:expr) => {
        ws!(delimited(char('('), ws!($x), char(')')))
    };
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

    ws!(alt((naked_bop_tags, naked_bop_chars)))(s)
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

    ws!(alt((naked_bool_tags, naked_eq)))(s)
}

fn bool_sexp(s: &str) -> IResult<&str, SExp> {
    let inner_int = map(tuple((bool_int_ops, many1(sexp))), |(o, v)| {
        SExp::BExp(rccell!(o), v.into_iter().map(|s| rccell!(s)).collect())
    });
    let inner_core = map(tuple((bool_core_ops, many1(sexp))), |(o, v)| {
        SExp::BExp(rccell!(o), v.into_iter().map(|s| rccell!(s)).collect())
    });

    let naked_b = alt((inner_int, inner_core));
    brack!(naked_b)(s)
}

fn var_binding(s: &str) -> IResult<&str, (SymbolRc, SortRc)> {
    let mapped_symbol = map(symbol, |s| Symbol::Var(s.to_owned()));
    map(
        brack!(tuple((ws!(mapped_symbol), ws!(sort)))),
        |(sy, se)| (rccell!(sy), rccell!(se)),
    )(s)
}

fn let_sexp(s: &str) -> IResult<&str, (Vec<(SymbolRc, SExpRc)>, SExpBoxRc)> {
    let mapped_symbol = map(symbol, |s| Symbol::Var(s.to_owned()));
    let var_binding = brack!(tuple((ws!(mapped_symbol), ws!(sexp))));
    let var_bs = brack!(many1(ws!(var_binding)));
    let inner = preceded(tag("let"), tuple((ws!(var_bs), ws!(sexp))));
    let mapped = map(brack_ws!(inner), |(a, b)| {
        (
            a.into_iter()
                .map(|(x, y)| (rccell!(x), rccell!(y)))
                .collect(),
            rccell!(Box::new(b)),
        )
    });
    mapped(s)
}

pub fn sexp(s: &str) -> IResult<&str, SExp> {
    let rec_sexp = brack!(many1(sexp));
    ws!(alt((
        bool_sexp,
        map(let_sexp, |(tbs, sexp)| SExp::Let(tbs, sexp)),
        map(quantifier, |(tbs, sexp)| {
            SExp::QForAll(tbs, rccell!(Box::new(sexp)))
        }),
        map(rec_sexp, |es| {
            SExp::Compound(es.into_iter().map(|e| rccell!(e)).collect())
        }),
        map(constant, |c| SExp::Constant(rccell!(c))),
        map(symbol, |s| {
            SExp::Symbol(rccell!(Symbol::Token(s.to_owned())))
        }),
    )))(s)
}

fn sort(s: &str) -> IResult<&str, Sort> {
    ws!(alt((
        map(tag("Int"), |_| Sort::UInt()),
        map(tag("Real"), |_| Sort::Dec()),
        map(symbol, |s| Sort::UserDef(s.to_owned())),
        map(brack!(many1(sort)), |ss| {
            Sort::Compound(ss.into_iter().map(|s| rccell!(s)).collect())
        }),
    )))(s)
}

fn naked_decl_const(s: &str) -> IResult<&str, (&str, Sort)> {
    let ws_decl = ws!(tag("declare-const"));
    preceded(ws_decl, tuple((ws!(symbol), ws!(sort))))(s)
}

fn naked_assert(s: &str) -> IResult<&str, SExp> {
    preceded(ws!(tag("assert")), ws!(sexp))(s)
}

fn naked_csa(s: &str) -> IResult<&str, SExp> {
    let ws_csatag = ws!(tag("check-sat-assuming"));
    let ws_sexp = ws!(sexp);
    preceded(ws_csatag, ws_sexp)(s)
}

fn naked_logic(s: &str) -> IResult<&str, Logic> {
    let ws_ltag = ws!(tag("set-logic"));
    let qslia = map(tag("QF_SLIA"), |_| Logic::QF_SLIA());
    let other = map(symbol, |s| Logic::Other(s.to_owned()));
    let ws_l = ws!(alt((qslia, other)));
    preceded(ws_ltag, ws_l)(s)
}

fn set_info_status(s: &str) -> IResult<&str, (&str, &str)> {
    let ws_val = ws!(alt((tag("sat"), tag("unsat"))));
    let ws_status = ws!(tag(":status"));
    let naked_si = preceded(tag("set-info"), tuple((ws_status, ws_val)));

    ws_brack_ws!(naked_si)(s)
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
        map(brack!(many0(unknown_balanced)), |v| {
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
    alt((
        preceded(set_info_status, command), // just drop for now
        ws_brack_ws!(naked_command),
        map(unknown_balanced, |v| {
            Command::Generic(v.into_iter().map(|g| g.to_owned()).collect::<String>())
        }),
    ))(s)
}

pub fn quantifier(s: &str) -> IResult<&str, (Vec<(SymbolRc, SortRc)>, SExp)> {
    let ws_var_bindings = ws!(brack!(many1(ws!(var_binding))));
    let naked_quant = preceded(tag("forall"), tuple((ws_var_bindings, sexp)));
    brack!(naked_quant)(s)
}

pub fn script(s: &str) -> IResult<&str, Script> {
    map(many0(ws!(command)), |cmds| {
        Script::Commands(cmds.into_iter().map(|cmd| rccell!(cmd)).collect())
    })(s)
}

pub fn rmv_comments(s: &str) -> IResult<&str, Vec<&str>> {
    let not_comment = take_while1(|c| !(c == ';'));
    let comment = delimited(char(';'), not_line_ending, line_ending);
    many1(alt((not_comment, map(comment, |_| ""))))(s)
}

pub fn define_func(s: &str) -> IResult<&str, (Symbol, Vec<(SymbolRc, SortRc)>, Sort, SExp)> {
    let mapped_symbol = map(symbol, |s| Symbol::Token(s.to_owned()));
    brack!(preceded(
        ws!(tag("define-fun")),
        tuple((
            ws!(mapped_symbol),
            brack!(many0(ws!(var_binding))),
            ws!(sort),
            ws!(sexp)
        ))
    ))(s)
}

pub fn model(s: &str) -> IResult<&str, Vec<(Symbol, Vec<(SymbolRc, SortRc)>, Sort, SExp)>> {
    brack!(preceded(ws!(tag("model")), many0(ws!(define_func))))(s)
}

pub fn z3_oerror(s: &str) -> IResult<&str, &str> {
    brack!(preceded(ws!(tag("error")), ws!(string)))(s)
}
type FuncDefine = (Symbol, Vec<(SymbolRc, SortRc)>, Sort, SExp);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ResultLine {
    Sat,
    Unsat,
    Unknown,
    Unsupported,
    Comment,
    Timeout,
    Generic(String),
    SegF,
    AssertionViolation,
    Error(String),
    Model(Vec<FuncDefine>),
}

fn timeout(s: &str) -> IResult<&str, &str> {
    preceded(
        ws!(tag("timeout: sending signal TERM to command")),
        ws!(symbol),
    )(s)
}

fn segf(s: &str) -> IResult<&str, &str> {
    tag("timeout: the monitored command dumped core")(s)
}

pub fn z3o(s: &str) -> IResult<&str, Vec<ResultLine>> {
    many0(ws!(alt((
        map(tag("sat"), |_| ResultLine::Sat),
        map(tag("unsat"), |_| ResultLine::Unsat),
        map(tag("unknown"), |_| ResultLine::Unknown),
        map(tag("unsupported"), |_| ResultLine::Unsupported),
        map(delimited(char(';'), not_line_ending, line_ending), |_| {
            ResultLine::Comment
        }),
        map(z3_oerror, |e| ResultLine::Error(e.to_owned())),
        map(model, |m| ResultLine::Model(m)),
        map(timeout, |_| ResultLine::Timeout),
        map(segf, |_| ResultLine::SegF),
        map(av, |_| ResultLine::AssertionViolation),
        map(generic, |s| ResultLine::Generic(s.to_owned())),
    ))))(s)
    .map(|(i, mut o)| {
        if i != "" {
            o.push(ResultLine::Generic(i.to_string()));
        }
        (i, o)
    })
}

fn iv_model_err(s: &str) -> IResult<&str, &str> {
    brack!(preceded(
        ws!(tag("error")),
        delimited(
            char('"'),
            preceded(
                take_until("invalid model was generated"),
                tag("invalid model was generated")
            ),
            char('"')
        )
    ))(s)
}

fn av(s: &str) -> IResult<&str, &str> {
    delimited(
        ws!(tag("ASSERTION VIOLATION")),
        take_until("(G)DB"),
        tag("(G)DB"),
    )(s)
}

fn generic(s: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, line_ending)(s)
}

pub fn script_from_f_unsanitized(filepath: &Path) -> Result<Script, String> {
    let contents: String = fs::read_to_string(&filepath).map_err(|e| e.to_string())?;
    let presult = script(&contents[..]).map_err(|e| e.to_string())?;
    if presult.0 != "" {
        Err("Incomplete Parse!".to_owned())
    } else {
        Ok(presult.1)
    }
}

pub fn script_from_f(filepath: &Path) -> Result<Script, String> {
    let contents: String = fs::read_to_string(&filepath).map_err(|e| e.to_string())?;
    let stripped_contents = &rmv_comments(&contents[..])
        .map_err(|e| e.to_string())?
        .1
        .join(" ")[..];
    let presult = script(&stripped_contents[..]).map_err(|e| e.to_string())?;
    if presult.0 != "" {
        Err("Incomplete Parse!".to_owned())
    } else {
        Ok(presult.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use std::fs;

    #[test]
    fn generic_snap() {
        assert_debug_snapshot!(generic("line \n break"));
    }

    #[test]
    fn generic_w_eof_snap() {
        assert_debug_snapshot!(z3o("no linebreak before end"));
    }

    #[test]
    fn av_snap() {
        assert_debug_snapshot!(av("ASSERTION VIOLATION
                File: ../src/qe/qsat.cpp
                Line: 631
                validate_defs(\"check_sat\")
                (C)ontinue, (A)bort, (S)top, (T)hrow exception, Invoke (G)DB"));
    }

    #[test]
    fn iv_snap() {
        assert_debug_snapshot!(iv_model_err(
            "(error \"line 5 column 10: an invalid model was generated\")"
        ));
    }

    #[test]
    fn segf_snap() {
        assert_debug_snapshot!(segf("timeout: the monitored command dumped core"));
    }

    #[test]
    fn timeout_snap() {
        assert_debug_snapshot!(timeout("timeout: sending signal TERM to command ‘z3’"));
    }

    #[test]
    fn z3o_model_snap() {
        let response = "sat
                        (model 
                          (define-fun b () Int
                            0)
                          (define-fun a () Int
                            1)
                          (define-fun e () Real
                            1.0)
                          (define-fun GEN1 () Real
                            (- 1.0))
                          (define-fun BAV5 () Bool
                            true)
                          (define-fun BAV4 () Bool
                            true)
                          (define-fun BAV3 () Bool
                            true)
                          (define-fun c () Int
                            0)
                          (define-fun GEN2 () Real
                            0.0)
                          (define-fun d () Real
                            0.0)
                        )";
        assert_debug_snapshot!(z3o(response));
    }

    #[test]
    fn z3o_errors_snap() {
        let response = "unsupported
                        ; ignoring unsupported logic QF_ALL_SUPPORTED line: 2 position: 1
                        (error \"line 5 column 52: unknown constant emptyset\")
                        (error \"line 6 column 60: unknown function/constant member\")
                        (error \"line 7 column 71: unknown function/constant singleton\")
                        (error \"line 168 column 52: unknown function/constant smt_set_mem\")
                        sat";
        assert_debug_snapshot!(z3o(response));
    }

    #[test]
    fn cvc4_modle_snap() {
        let response = "(model
            (define-fun f ((BOUND_VARIABLE_397 Int)) (Set Int) (ite (= BOUND_VARIABLE_397 (- 1)) (singleton 0) (as emptyset (Set Int))))
            (define-fun x () Int (- 1))
            (define-fun y () Int 0)
            (define-fun S () (Set Int) (as emptyset (Set Int)))
            (define-fun T () (Set Int) (singleton 0))
            )";
        assert_debug_snapshot!(model(response));
    }

    #[test]
    fn cvc4o_err_snap() {
        let response =
            "(error \"Parse Error: ../samples/z3.44.produced.smt2:3.3: Unexpected token: 'sat'.

              sat
              ^
            \")";

        assert_debug_snapshot!(z3_oerror(response));
    }

    #[test]
    fn z3oerr_snap() {
        assert_debug_snapshot!(z3_oerror(
            "(error \"line 6 column 52: unknown constant emptyset\")"
        ));
    }

    #[test]
    fn model_snap() {
        assert_debug_snapshot!(model("(model (define-fun f () Int 7))"));
    }

    #[test]
    fn func_snap() {
        assert_debug_snapshot!(define_func("(define-fun foo ((a Real) (b String)) Int 7)"));
    }

    #[test]
    fn func_noargs_snap() {
        assert_debug_snapshot!(define_func("(define-fun foo () Int 7)"));
    }

    fn parse_file(f: &str) -> Script {
        let contents = &fs::read_to_string(f).expect("error reading file")[..];
        let contents_sans_comments = &rmv_comments(contents)
            .expect("failed to rmv comments")
            .1
            .join(" ")[..];

        script(contents_sans_comments).expect("parser error").1
    }

    #[test]
    fn quant_snap() {
        assert_debug_snapshot!(quantifier("(forall ((ah Real)) (= ah 4))"));
    }

    #[test]
    fn equant() {
        let r = script("(assert (exists ((ah Real)) (= ah 4)))").unwrap();
        assert_debug_snapshot!(r);
    }
}
