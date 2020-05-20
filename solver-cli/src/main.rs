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
            .get_matches();
        let log_level = matches.occurrences_of("verbosity");

        let infile_name = matches.value_of(INFILE).unwrap();
        let results = solve(infile_name);

        if results.iter().any(|r| r.has_bug_error()) {
            if log_level > 0 {
                results
                    .iter()
                    .find(|r| r.has_bug_error())
                    .map(|r| println!("BUG ERROR in {}: {:?}", infile_name, r));
            }
            0
        } else if !RSolve::differential_test(&results).is_ok() {
            0
        } else {
            1
        }
    };
    std::process::exit(ecode);
}
