#![feature(try_blocks)]
#![feature(rustc_private)]
extern crate antlr_rust;
extern crate itertools;
#[macro_use]
extern crate lazy_static;

pub mod smtlibv2lexer;
pub mod smtlibv2listener;
pub mod smtlibv2parser;

use crate::smtlibv2parser::CommandContext;
use crate::smtlibv2parser::Qual_identiferContext;
use crate::smtlibv2parser::Spec_constantContextAttrs;
use crate::smtlibv2parser::TermContext;
use crate::smtlibv2parser::TermContextAttrs;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::parser_rule_context::ParserRuleContextType;
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use antlr_rust::tree::Tree;
use antlr_rust::tree::{ParseTree, ParseTreeListener, TerminalNodeCtx};
use itertools::Itertools;
use rsmt2::SmtConf;
use smtlibv2lexer::SMTLIBv2Lexer;
use smtlibv2listener::SMTLIBv2Listener;
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

use smtlibv2parser::CommandContextAttrs;
use smtlibv2parser::IdentifierContextAttrs;
use smtlibv2parser::Qual_identiferContextAttrs;
use smtlibv2parser::SMTLIBv2Parser;
use smtlibv2parser::SimpleSymbolContextAttrs;
use smtlibv2parser::SymbolContextAttrs;

use std::collections::BTreeMap;
use std::fs;
use std::rc::Rc;

pub fn exec() {
    let files = fs::read_dir("samples").expect("error with sample dir");

    for file_res in files {
        match file_res {
            Ok(file) => {
                let filepath = file.path();
                test_file(&filepath);
            }
            Err(_) => (),
        }
    }
}

pub fn test_file(source_filepath: &PathBuf) {
    let smt_ex: String =
        fs::read_to_string(source_filepath).expect("Something went wrong reading the file");
    let mut _lexer = SMTLIBv2Lexer::new(Box::new(InputStream::new(smt_ex.into())));
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = SMTLIBv2Parser::new(Box::new(token_source));
    let listener = Box::new(Listener {
        vng: VarNameGenerator {
            basename: "GEN".to_string(),
            counter: 0,
        },
        new_vars: BTreeMap::new(),
        bexps: vec![],
    });
    let lid = parser.add_parse_listener(listener);
    let result = parser.script();

    let listener = parser.remove_parse_listener(lid);
    let mut bav_gen = VarNameGenerator {
        basename: "BAV".to_string(),
        counter: 0,
    };

    let (bav_names, bav_inits) = listener
        .bexps
        .into_iter()
        .map(|term| {
            let t: ParserRuleContextType = Rc::new(term);
            let name = bav_gen.get_name();
            let cmd = cmd_from(format!("(assert (= {} {}))", name, ast_string(&t)));
            (name, cmd)
        })
        .unzip::<String, ParserRuleContextType, Vec<String>, Vec<ParserRuleContextType>>();

    let bool_type = vec![SMTlibConst::Bool()];
    let new_bavs = bav_names.iter().zip(bool_type.iter().cycle());

    let decls = listener
        .new_vars
        .iter()
        .chain(new_bavs)
        .map(|(k, v)| format!("(declare-const {} {})", k, v.typestr()))
        .map(|ds| cmd_from(ds))
        .collect::<Vec<Rc<dyn ParserRuleContext + 'static>>>();

    let script: Rc<dyn ParserRuleContext> = result.unwrap();

    let maybe_sl_pos = script.get_children().iter().position(|pctx| {
        match pctx
            .upcast_any()
            .downcast_ref::<CommandContext>()
            .and_then(|cmd| cmd.cmd_setLogic())
        {
            Some(_) => true,
            None => false,
        }
    });

    let start_insertion_point = match maybe_sl_pos {
        Some(sl_pos) => sl_pos + 1,
        None => 0,
    };

    for decl in decls {
        script
            .get_children_full()
            .borrow_mut()
            .insert(start_insertion_point, decl);
    }

    let maybe_cs_pos = script.get_children().iter().position(|pctx| {
        match pctx
            .upcast_any()
            .downcast_ref::<CommandContext>()
            .and_then(|cmd| cmd.cmd_checkSat())
        {
            Some(_) => true,
            None => false,
        }
    });

    let end_insertion_point = match maybe_cs_pos {
        Some(cs_pos) => cs_pos,
        None => script.get_children().len(),
    };

    for bav_init in bav_inits {
        script
            .get_children_full()
            .borrow_mut()
            .insert(end_insertion_point, bav_init);
    }

    script
        .get_children_full()
        .borrow_mut()
        .insert(end_insertion_point, cmd_from("(assert true)".to_owned()));

    let mut iterations = 0;
    let num_bavs = bav_names.len();
    for truths in 0..num_bavs + 1 {
        let mut unordered_tvs = vec![true; truths];
        unordered_tvs.extend(vec![false; num_bavs - truths]);
        let truth_value_assigments = unordered_tvs.iter().permutations(num_bavs).unique();
        for truth_values in truth_value_assigments {
            let cmd_string = format!(
                "(assert {})",
                bam_string(&mut bav_names.iter(), &mut truth_values.iter())
            );

            let cmd = cmd_from(cmd_string);
            let mut kids = script.get_children_full().borrow_mut();
            kids[end_insertion_point] = cmd;
            drop(kids); // Not sure why, but borrow checker needs help here
            let source_filename = match source_filepath.file_name().and_then(|n| n.to_str()) {
                Some(name) => name,
                None => "unknown",
            };
            let filename = (iterations).to_string() + "_" + source_filename;
            fs::write(&filename, ast_string(&script));
            solve(&filename);
            iterations = iterations + 1;
        }
    }
}

fn solve(filename: &str) {
    let cvc4_res = Command::new("cvc4")
        .args(&[filename, "--produce-model"])
        .output();

    let z3_res = Command::new("z3").args(&[filename]).output();

    let cvc4_stdout_res = cvc4_res
        .and_then(|out| {
            if !out.status.success() && out.stderr.len() > 0 {
                println!("cvc4 error on file {}", filename);
                Err(std::io::Error::last_os_error()) // really sloppy hack for now, needs to be fixed
            } else {
                Ok(out)
            }
        })
        .map(|out| from_utf8(&out.stdout.clone()[..]).map(|s| s.to_string()));

    let z3_stdout_res = z3_res
        .and_then(|out| {
            if !out.status.success() && out.stderr.len() > 0 {
                println!("z3 error on file {}", filename);
                Err(std::io::Error::last_os_error()) // really sloppy hack for now, needs to be fixed
            } else {
                Ok(out)
            }
        })
        .map(|out| from_utf8(&out.stdout.clone()[..]).map(|s| s.to_string()));

    match (cvc4_stdout_res, z3_stdout_res) {
        (Ok(Ok(cvc4_stdout)), Ok(Ok(z3_stdout))) => {
            // also sloppy hack above
            if cvc4_stdout.contains("unsat") && !z3_stdout.contains("unsat") {
                println!("file {} has soundness problem!!!", filename);
            } else if cvc4_stdout.contains("sat") && !z3_stdout.contains("sat") {
                println!("file {} has soundness problem!!!", filename);
            } else {
                fs::remove_file(filename);
            }
            ()
        }
        _ => println!("Error with file {}", filename),
    }
}

fn bam_string(
    names: &mut std::slice::Iter<String>,
    truth_vals: &mut std::slice::Iter<&bool>,
) -> String {
    match (names.next(), truth_vals.next()) {
        (Some(name), Some(truth_val)) => {
            let sub = bam_string(names, truth_vals);
            format!("(and (= {} {}) {})", name, truth_val, sub)
        }
        _ => "".to_string(),
    }
}

fn ast_string(ast: &ParserRuleContextType) -> String {
    match ast
        .upcast_any()
        .downcast_ref::<BaseParserRuleContext<TerminalNodeCtx>>()
    {
        Some(tn) => tn.get_text() + " ",
        None => {
            let mut s: String = "".to_owned();
            for child in ast.get_children().iter() {
                s.push_str(&ast_string(child)[..]);
            }
            s
        }
    }
}

fn cmd_from(cmd: String) -> Rc<dyn ParserRuleContext + 'static> {
    let mut _lexer = SMTLIBv2Lexer::new(Box::new(InputStream::new(cmd)));
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = SMTLIBv2Parser::new(Box::new(token_source));
    parser.command().unwrap()
}

#[derive(Debug, Clone)]
enum SMTlibConst {
    Num(i64),
    Dec(f64),
    Str(String),
    Bool(),
    Bin(), // Will add support later
    Hex(), // ditto
}

impl SMTlibConst {
    fn typestr(&self) -> &str {
        match self {
            SMTlibConst::Num(_) => "Int",
            SMTlibConst::Bool() => "Bool",
            _ => panic!("Unimplemented for non-ints"),
        }
    }
}

struct VarNameGenerator {
    basename: String,
    counter: u32,
}

impl VarNameGenerator {
    fn get_name(&mut self) -> String {
        self.counter = self.counter + 1;
        format!("{}{}", self.basename, self.counter)
    }
}

struct Listener {
    vng: VarNameGenerator,
    new_vars: BTreeMap<String, SMTlibConst>,
    bexps: Vec<TermContext>,
}

fn qual_id_from(var: String) -> Rc<Qual_identiferContext> {
    let mut _lexer = SMTLIBv2Lexer::new(Box::new(InputStream::new(var)));
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = SMTLIBv2Parser::new(Box::new(token_source));
    parser.qual_identifer().unwrap()
}

impl SMTLIBv2Listener for Listener {
    // should add some assertions that these are single parent nodes
    fn exit_term(&mut self, term: &TermContext) {
        match term.spec_constant() {
            Some(sc) => {
                let name = self.vng.get_name();
                sc.numeral()
                    .and_then(|n| n.get_text().parse::<i64>().ok())
                    .map(|n| self.new_vars.insert(name.clone(), SMTlibConst::Num(n)));
                term.get_children_full().replace(vec![qual_id_from(name)]);
                ()
            }
            None => (),
        };

        // Assume all ops are wrapped in qi's like this
        term.qual_identifer()
            .and_then(|qi| qi.identifier())
            .and_then(|i| i.symbol())
            .and_then(|s| s.simpleSymbol())
            .and_then(|ss| ss.UndefinedSymbol())
            .map(|us| us.get_text())
            .map(|op| match &op[..] {
                "<" | "=" | "or" | "and" => self
                    .bexps
                    .push(BaseParserRuleContext::copy_from(term, (*term).clone())),
                _ => (),
            });
    }
}

impl ParseTreeListener for Listener {
    fn enter_every_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
    fn exit_every_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        exec();
    }
}
