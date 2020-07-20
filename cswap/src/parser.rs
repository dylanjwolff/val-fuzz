use super::ast::{
    BitVecConst, BoolOp, Command, Constant, FPConst, FpOp, Logic, NumOp, SExp, SExpBoxRc, SExpRc,
    Script, Sort, SortRc, StrOp, Symbol, SymbolRc,
};

use crate::liftio;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::hex_digit1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
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
use std::io;
use std::path::Path;

macro_rules! op_ws {
    ($x:expr) => {
        delimited(multispace0, $x, alt((multispace1, tag(")"))))
    };
}

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

fn bv(s: &str) -> IResult<&str, BitVecConst> {
    alt((
        map(hex, |h| BitVecConst::Hex(h.to_owned())),
        map(bin, |b| BitVecConst::Bin(b.to_owned())),
    ))(s)
}

fn fp(s: &str) -> IResult<&str, FPConst> {
    let num = preceded(
        ws!(tag("fp")),
        map(tuple((ws!(bv), ws!(bv), ws!(bv))), |(i, e, s)| {
            FPConst::Num(i, e, s)
        }),
    );

    let specials = preceded(
        ws!(tag("_")),
        alt((
            map(
                preceded(ws!(tag("+oo")), tuple((ws!(integer), ws!(integer)))),
                |(m, n)| FPConst::PInf(m.to_owned(), n.to_owned()),
            ),
            map(
                preceded(ws!(tag("-oo")), tuple((ws!(integer), ws!(integer)))),
                |(m, n)| FPConst::NInf(m.to_owned(), n.to_owned()),
            ),
            map(
                preceded(ws!(tag("-zero")), tuple((ws!(integer), ws!(integer)))),
                |(m, n)| FPConst::NZero(m.to_owned(), n.to_owned()),
            ),
            map(
                preceded(ws!(tag("+zero")), tuple((ws!(integer), ws!(integer)))),
                |(m, n)| FPConst::PZero(m.to_owned(), n.to_owned()),
            ),
            map(
                preceded(ws!(tag("NaN")), tuple((ws!(integer), ws!(integer)))),
                |(m, n)| FPConst::Nan(m.to_owned(), n.to_owned()),
            ),
        )),
    );

    brack!(alt((num, specials)))(s)
}

fn constant(s: &str) -> IResult<&str, Constant> {
    alt((
        map(integer, |i| Constant::UInt(i.to_owned())),
        map(decimal, |d| Constant::Dec(d.to_owned())),
        map(hex, |h| Constant::Hex(h.to_owned())),
        map(bin, |b| Constant::Bin(b.to_owned())),
        map(string, |s| Constant::Str(s.to_owned())),
        map(fp, |f| Constant::Fp(f)),
    ))(s)
}

fn symbol(s: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && !(c == '(') && !(c == ')'))(s)
}

fn fp_op(s: &str) -> IResult<&str, FpOp> {
    let naked_fpop_tags = alt((
        map(tag("fp.abs"), |_| FpOp::Abs()),
        map(tag("fp.neg"), |_| FpOp::Neg()),
        map(tag("fp.add"), |_| FpOp::Add()),
        map(tag("fp.sub"), |_| FpOp::Sub()),
        map(tag("fp.mul"), |_| FpOp::Mul()),
        map(tag("fp.div"), |_| FpOp::Div()),
        map(tag("fp.fma"), |_| FpOp::Fma()),
        map(tag("fp.sqrt"), |_| FpOp::Sqrt()),
        map(tag("fp.rem"), |_| FpOp::Rem()),
        map(tag("fp.roundToIntegral"), |_| FpOp::RTI()),
        map(tag("fp.min"), |_| FpOp::Min()),
        map(tag("fp.max"), |_| FpOp::Max()),
        map(
            brack!(tuple((
                ws!(tag("_")),
                ws!(tag("to_fp")),
                ws!(integer),
                integer
            ))),
            |(_, _, eb, sb)| FpOp::ToFp(eb.to_owned(), sb.to_owned()),
        ),
    ));
    op_ws!(naked_fpop_tags)(s)
}

fn fp_sexp(s: &str) -> IResult<&str, SExp> {
    let initial_res = tuple((fp_op, many1(sexp)));
    brack!(map(initial_res, |(op, sexps)| {
        let fp_sort = if let FpOp::ToFp(eb, sb) = &op {
            Some((eb.to_owned(), sb.to_owned()))
        } else {
            sexps
                .iter()
                .map(|sexp| sexp.fp_sort())
                .find(|s| s.is_some())
                .flatten()
        };
        SExp::FPExp(op, fp_sort, sexps.into_iter().map(|s| rccell!(s)).collect())
    }))(s)
}

fn str_op(s: &str) -> IResult<&str, StrOp> {
    let naked_strop_tags = alt((
        map(tag("str.++"), |_| StrOp::Strcat()),
        map(tag("str.at"), |_| StrOp::StrAt()),
        map(tag("str.from_code"), |_| StrOp::StrFromCode()),
        map(tag("str.substr"), |_| StrOp::Substr()),
        map(tag("str.replace_re_all"), |_| StrOp::StrReplaceReAll()),
        map(tag("str.replace_all"), |_| StrOp::StrReplaceAll()),
        map(tag("str.replace_re"), |_| StrOp::StrReplaceRe()),
        map(tag("str.replace"), |_| StrOp::StrReplace()),
    ));

    op_ws!(naked_strop_tags)(s)
}

fn str_sexp(s: &str) -> IResult<&str, SExp> {
    let inner = map(tuple((str_op, many1(sexp))), |(o, v)| {
        SExp::StrExp(o, v.into_iter().map(|s| rccell!(s)).collect())
    });

    brack!(inner)(s)
}

fn num_op(s: &str) -> IResult<&str, NumOp> {
    let naked_numop_tags = alt((
        map(tag("-"), |_| NumOp::Sub()),
        map(tag("+"), |_| NumOp::Add()),
        map(tag("*"), |_| NumOp::Mul()),
        map(tag("/"), |_| NumOp::Div()),
        map(tag("fp.to_real"), |_| NumOp::FpToReal()),
        map(tag("fp.to_int"), |_| NumOp::FpToInt()),
        map(tag("div"), |_| NumOp::IntDiv()),
        map(tag("mod"), |_| NumOp::Mod()),
        map(tag("str.len"), |_| NumOp::StrLen()),
        map(tag("str.to_code"), |_| NumOp::StrToCode()),
        map(tag("str.to_int"), |_| NumOp::StrToInt()),
        map(tag("str.indexof"), |_| NumOp::StrIndexOf()),
    ));

    op_ws!(naked_numop_tags)(s)
}

fn num_sexp(s: &str) -> IResult<&str, SExp> {
    let inner = map(tuple((num_op, many1(sexp))), |(o, v)| {
        SExp::NExp(o, v.into_iter().map(|s| rccell!(s)).collect())
    });

    brack!(inner)(s)
}

fn bool_fp_ops(s: &str) -> IResult<&str, BoolOp> {
    let naked_bop_tags = alt((
        map(tag("fp.leq"), |_| BoolOp::Fpleq()),
        map(tag("fp.lt"), |_| BoolOp::Fplt()),
        map(tag("fp.geq"), |_| BoolOp::Fpgeq()),
        map(tag("fp.gt"), |_| BoolOp::Fpgt()),
        map(tag("fp.eq"), |_| BoolOp::Fpeq()),
        map(tag("fp.isNormal"), |_| BoolOp::Fpnorm()),
        map(tag("fp.isSubnormal"), |_| BoolOp::Fpsubn()),
        map(tag("fp.isZero"), |_| BoolOp::Fpzero()),
        map(tag("fp.isInfinite"), |_| BoolOp::Fpinf()),
        map(tag("fp.isNaN"), |_| BoolOp::Fpnana()),
        map(tag("fp.isNegative"), |_| BoolOp::Fpneg()),
        map(tag("fp.isPositive"), |_| BoolOp::Fppos()),
    ));

    op_ws!(naked_bop_tags)(s)
}

fn bool_str_ops(s: &str) -> IResult<&str, BoolOp> {
    let naked_bop_tags = alt((
        map(tag("str.<="), |_| BoolOp::Strlte()),
        map(tag("str.<"), |_| BoolOp::Strlt()),
        map(tag("str.in_re"), |_| BoolOp::Strre()),
        map(tag("str.prefixof"), |_| BoolOp::Strpref()),
        map(tag("str.suffixof"), |_| BoolOp::Strsuff()),
        map(tag("str.contains"), |_| BoolOp::Strcont()),
        map(tag("str.is_digit"), |_| BoolOp::Strisdig()),
    ));

    op_ws!(naked_bop_tags)(s)
}

fn bool_bv_ops(s: &str) -> IResult<&str, BoolOp> {
    let naked_bop_tags = alt((
        map(tag("bvugt"), |_| BoolOp::Bvgt()),
        map(tag("bvuge"), |_| BoolOp::Bvge()),
        map(tag("bvslt"), |_| BoolOp::Bvslt()),
        map(tag("bvsle"), |_| BoolOp::Bvsle()),
        map(tag("bvsgt"), |_| BoolOp::Bvsgt()),
        map(tag("bvsge"), |_| BoolOp::Bvsge()),
    ));

    op_ws!(naked_bop_tags)(s)
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

    op_ws!(alt((naked_bop_tags, naked_bop_chars)))(s)
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

    op_ws!(alt((naked_bool_tags, naked_eq)))(s)
}

fn bool_sexp(s: &str) -> IResult<&str, SExp> {
    let bool_ops = alt((
        bool_core_ops,
        bool_int_ops,
        bool_bv_ops,
        bool_str_ops,
        bool_fp_ops,
    ));
    let inner = map(tuple((bool_ops, many1(sexp))), |(o, v)| {
        SExp::BExp(rccell!(o), v.into_iter().map(|s| rccell!(s)).collect())
    });

    brack!(inner)(s)
}

fn var_binding(s: &str) -> IResult<&str, (SymbolRc, SortRc)> {
    let mapped_symbol = map(symbol, |s| Symbol::Token(s.to_owned()));
    map(
        brack!(tuple((ws!(mapped_symbol), ws!(sort)))),
        |(sy, se)| (rccell!(sy), rccell!(se)),
    )(s)
}

fn let_sexp(s: &str) -> IResult<&str, (Vec<(SymbolRc, SExpRc)>, SExpBoxRc)> {
    let mapped_symbol = map(symbol, |s| Symbol::Token(s.to_owned()));
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
        num_sexp,
        str_sexp,
        fp_sexp,
        map(let_sexp, |(tbs, sexp)| SExp::Let(tbs, sexp)),
        map(existential_q, |(tbs, sexp)| {
            SExp::QExists(tbs, rccell!(Box::new(sexp)))
        }),
        map(quantifier, |(tbs, sexp)| {
            SExp::QForAll(tbs, rccell!(Box::new(sexp)))
        }),
        map(constant, |c| SExp::Constant(rccell!(c))),
        map(rec_sexp, |es| {
            SExp::Compound(es.into_iter().map(|e| rccell!(e)).collect())
        }),
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

fn naked_decl_generic(s: &str) -> IResult<&str, Command> {
    let decls = alt((
        tag("declare-datatypes"),
        tag("declare-datatype"),
        tag("declare-sort"),
        tag("define-funs-rec"),
        tag("define-fun-rec"),
        tag("define-fun"),
        tag("define-sort"),
    ));
    let inner = tuple((ws!(decls), many0(ws!(unknown_balanced))));
    map(inner, |(decl, rest)| {
        Command::GenericDecl(
            decl.to_owned(),
            rest.into_iter()
                .map(|vin| vin.into_iter().map(|s| s.to_owned()).collect())
                .collect(),
        )
    })(s)
}

fn naked_fn_definition(s: &str) -> IResult<&str, (Symbol, Vec<(SymbolRc, SortRc)>, Sort, SExp)> {
    let mapped_symbol = map(symbol, |s| Symbol::Token(s.to_owned()));
    tuple((
        ws!(mapped_symbol),
        brack!(many0(ws!(var_binding))),
        ws!(sort),
        sexp,
    ))(s)
}

fn mapped_naked_fn_definition(
    s: &str,
) -> IResult<&str, (Symbol, Vec<(SymbolRc, SortRc)>, Sort, SExpRc)> {
    map(naked_fn_definition, |(name, args, rtype, body)| {
        (name, args, rtype, rccell!(body))
    })(s)
}

fn naked_decl_fn(s: &str) -> IResult<&str, Command> {
    let args = delimited(char('('), many0(ws!(sort)), char(')'));
    let pre_map = preceded(
        ws!(tag("declare-fun")),
        tuple((ws!(symbol), ws!(args), ws!(sort))),
    );
    let types = map(pre_map, |(name, args, rtype)| {
        (Symbol::Token(name.to_owned()), args, rtype)
    });
    map(types, |(name, args, rtype)| {
        Command::DeclFn(name, args, rtype)
    })(s)
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
        naked_decl_fn,
        naked_decl_generic,
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

pub fn existential_q(s: &str) -> IResult<&str, (Vec<(SymbolRc, SortRc)>, SExp)> {
    let ws_var_bindings = ws!(brack!(many1(ws!(var_binding))));
    let naked_quant = preceded(tag("exists"), tuple((ws_var_bindings, sexp)));
    brack!(naked_quant)(s)
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

fn define_func(s: &str) -> IResult<&str, Command> {
    map(
        brack!(preceded(
            ws!(tag("define-fun")),
            ws!(mapped_naked_fn_definition)
        )),
        |(name, args, rtype, body)| Command::DefineFun(name, args, rtype, body),
    )(s)
}

fn define_func_rec(s: &str) -> IResult<&str, Command> {
    map(
        brack!(preceded(
            ws!(tag("define-fun-rec")),
            ws!(mapped_naked_fn_definition)
        )),
        |(name, args, rtype, body)| Command::DefineFunRec(name, args, rtype, body),
    )(s)
}

fn define_funcs_rec(s: &str) -> IResult<&str, Command> {
    map(
        brack!(preceded(
            ws!(tag("define-funs-rec")),
            many0(ws!(mapped_naked_fn_definition))
        )),
        |fn_defines| Command::DefineFunsRec(fn_defines),
    )(s)
}

pub fn model(s: &str) -> IResult<&str, Vec<(Symbol, Vec<(SymbolRc, SortRc)>, Sort, SExp)>> {
    let model_result = ws!(brack!(preceded(
        ws!(tag("define-fun")),
        ws!(naked_fn_definition)
    )));
    brack!(preceded(ws!(tag("model")), many0(model_result)))(s)
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
    alt((
        ws!(tag("timeout")),
        ws!(tag("CVC4 interrupted by timeout.")),
    ))(s)
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

pub fn script_from_f_unsanitized(filepath: &Path) -> io::Result<Script> {
    let contents: String = fs::read_to_string(&filepath)?;
    let presult = liftio!(script(&contents[..]))?;
    if presult.0 != "" {
        liftio!(Err("Incomplete Parse!".to_owned()))
    } else {
        Ok(presult.1)
    }
}

pub fn script_from_f(filepath: &Path) -> io::Result<Script> {
    let contents: String = fs::read_to_string(&filepath)?;
    let stripped_contents = &liftio!(rmv_comments(&contents[..]))?.1.join(" ")[..];
    let presult = liftio!(script(&stripped_contents[..]))?;
    if presult.0 != "" {
        liftio!(Err("Incomplete Parse!".to_owned()))
    } else {
        Ok(presult.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_debug_snapshot;
    use insta::assert_display_snapshot;

    #[test]
    fn fp_op_inference_snap() {
        let df = command("(assert (= mpfx (fp.mul roundTowardPositive
            ((_ to_fp 11 53) roundNearestTiesToEven 0.5792861143265499723753464422770775854587554931640625 (- 1022))
            ((_ to_fp 11 53) roundNearestTiesToEven 1.3902774452208657152141313417814671993255615234375 (- 17)))))");
        if let Command::Assert(c) = df.clone().unwrap().1 {
            if let SExp::BExp(_, sexps) = &*c.borrow() {
                if let SExp::FPExp(_, fpsort, _) = &*sexps[1].borrow() {
                    assert_debug_snapshot!(fpsort);
                } else {
                    assert_debug_snapshot!(sexps);
                }
            } else {
                assert_debug_snapshot!(c);
            }
        } else {
            assert_debug_snapshot!(df.unwrap().1);
        }
    }

    #[test]
    fn fn_define_snap() {
        let df = define_func_rec("(define-fun-rec app ((l1 Lst) (l2 Lst)) Lst (ite ((_ is nil) l1) l2 (cons (head l1) (app (tail l1) l2))))");
        assert_display_snapshot!(df.unwrap().1);
    }

    #[test]
    fn identifier_snap() {
        let df = command("(assert (fp.gt ~C1_const_-25497521 (_ +zero 8 24)))");
        assert_debug_snapshot!(df.unwrap().1);
    }

    #[test]
    fn partial_name_fn_decl_snap() {
        let df = command("(assert (forall ((?i Int) (?j Int)) (=> (<= 0 ?j) (= (shr_ ?i (+ ?j 1)) (divide (shr_ ?i ?j) 2)))))");
        assert_display_snapshot!(df.unwrap().1);
    }

    #[test]
    fn decl_snap() {
        let df = command("(declare-datatypes NodeMobile ((Rnode)))");
        assert_debug_snapshot!(df.unwrap().1);
    }

    #[test]
    fn decl_fn_snap() {
        let df = command("(declare-fun x33 () Bool)");
        assert_debug_snapshot!(df.unwrap().1);
    }

    #[test]
    fn decl_fn_with_args_snap() {
        let df = command("(declare-fun f13 (S7 S6) Real)");
        assert_debug_snapshot!(df.unwrap().1);
    }

    #[test]
    fn sort_of_const_special_snap() {
        let c = constant("(_ NaN 2 3)").unwrap().1;
        assert_debug_snapshot!(format!("{}", c.sort()));
    }

    #[test]
    fn sort_of_const_num_snap() {
        let c = constant("(fp #b0 #xf #b10110)").unwrap().1;
        assert_debug_snapshot!(format!("{}", c.sort()));
    }

    #[test]
    fn fp_snap() {
        assert_debug_snapshot!(fp("(fp #b0 #b00000000000 #x0000000000000)").unwrap().1);
    }

    #[test]
    fn fp_special_snap() {
        assert_debug_snapshot!(fp("(_ +oo 2 3)"));
    }

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
    fn timeout_snap() {
        assert_debug_snapshot!(timeout("timeout"));
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

    #[test]
    fn quant_snap() {
        assert_debug_snapshot!(quantifier("(forall ((ah Real)) (= ah 4))"));
    }

    #[test]
    fn equant_snap() {
        assert_debug_snapshot!(existential_q(
            "(exists ((x Int)) (and (< (* 3 x) 2) (< 1 (* 2 x))))"
        )
        .unwrap());
    }

    #[test]
    fn equant() {
        let r = script("(assert (exists ((ah Real)) (= ah 4)))").unwrap();
        assert_debug_snapshot!(r);
    }
}
