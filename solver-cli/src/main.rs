#[allow(dead_code)]
#[allow(unused)]
extern crate clap;
extern crate cswap;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use std::path::Path;

use cswap::solver::solve;
use cswap::solver::RSolve;

const INFILE: &'static str = "File to Solve";
const CREDUCE_SCRIPT: &'static str = "creduce-script";

fn main() {
    let ecode = {
        let matches: ArgMatches = App::new("Multi SMT Solver CLI")
            .arg(
                Arg::with_name(INFILE)
                    .help("Sets the input file to solve")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("verbosity")
                    .short("v")
                    .multiple(true)
                    .help("Sets the level of verbosity"),
            )
            .arg(
                Arg::with_name(CREDUCE_SCRIPT)
                    .short("s")
                    .help("Sets the level of verbosity")
                    .takes_value(true),
            )
            .get_matches();
        let log_level = matches.occurrences_of("verbosity");

        let maybe_cr_script = matches.value_of(CREDUCE_SCRIPT);
        let infile_name = matches.value_of(INFILE).unwrap();
        let results = solve(infile_name);

        if log_level > 1 {
            println!(
                "RESULTS for {}: {}",
                infile_name,
                results
                    .iter()
                    .map(|r| format!("{}", r))
                    .collect::<Vec<String>>()
                    .join("\n\n---------------\n\n")
            );
        }

        if results.iter().any(|r| r.has_bug_error()) {
            if log_level == 1 {
                results
                    .iter()
                    .find(|r| r.has_bug_error())
                    .map(|r| println!("BUG ERROR in {}: {}", infile_name, r));
            }
            if let Some(crs) = maybe_cr_script {
                //TODO needs to use solver-cli
                results
                    .iter()
                    .find(|r| r.has_bug_error())
                    .map(|r| println!("{} {}", r.solver, infile_name));
            }
            0
        } else if !RSolve::differential_test(&results).is_ok() {
            if log_level == 1 {
                println!("SOUNDNESS ERROR in {}", infile_name);
            }
            0
        } else {
            1
        }
    };
    std::process::exit(ecode);
}
