#[allow(dead_code)]
#[allow(unused)]
extern crate clap;
extern crate cswap;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use cswap::parser::rmv_comments;
use cswap::parser::script;
use cswap::solver::profiles_solve;
use cswap::solver::profiles_to_string;
use cswap::solver::solve;
use std::time::Duration;

use cswap::solver::ProfileIndex;

use cswap::solver::RSolve;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

const INFILE: &'static str = "File to Solve";
const CREDUCE_SCRIPT: &'static str = "creduce-script";
const PROFILES: &'static str = "profiles";
const FORMAT: &'static str = "format";
const LIST_PROFILES: &'static str = "list-profiles";
const TIMEOUT: &'static str = "timeout";
const SOUNDNESS_ONLY: &'static str = "soundness-only";

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
                Arg::with_name(SOUNDNESS_ONLY)
                    .short("y")
                    .long(SOUNDNESS_ONLY)
                    .help("Only returns 0 if a soundness bug is detected")
            ).arg(
                Arg::with_name(CREDUCE_SCRIPT)
                    .short("s")
                    .help("Outputs a script for use with CReduce to the file named")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(PROFILES)
                    .short("p")
                    .help("Set the solver profiles to use")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(TIMEOUT)
                    .short("t")
                    .long(TIMEOUT)
                    .help("Sets the timeout for the solvers in seconds")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(FORMAT)
                    .short("f")
                    .help("Format the SMTLIB2 file to stdout. Removes all comments"),
            )
            .arg(
                Arg::with_name(LIST_PROFILES)
                    .short("l")
                    .help("List the profiles availaible for use with the -p option"),
            )
            .get_matches();
        let log_level = matches.occurrences_of("verbosity");
        let maybe_cr_script = matches.value_of(CREDUCE_SCRIPT);
        let infile_name = matches.value_of(INFILE).unwrap();

        if matches.is_present(LIST_PROFILES) {
            println!("{}", profiles_to_string());
            return;
        }

        match matches.is_present(FORMAT) {
            true => format(Path::new(infile_name)),
            _ => (),
        };

        let timeout = match matches.value_of(TIMEOUT) {
            Some(to) => Duration::from_secs(
                to.parse::<u8>().expect("Expected small number for timeout") as u64,
            ),
            None => Duration::from_secs(6),
        };

        let results = match matches.value_of(PROFILES) {
            Some(pstr) => profiles_solve(infile_name, &parse_profiles(pstr), timeout),
            None => solve(infile_name, timeout),
        };

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

        let mut ret_val = 1;
        if results.iter().any(|r| r.has_bug_error()) {
            if log_level == 1 {
                results
                    .iter()
                    .find(|r| r.has_bug_error())
                    .map(|r| println!("BUG ERROR in {}: {}", infile_name, r));
            }
            if let Some(crs) = maybe_cr_script {
                print_creduce(&results, crs, infile_name);
            }
            if !matches.is_present(SOUNDNESS_ONLY) {
                ret_val = 0;
            }
        }

        if !RSolve::differential_test(&results).is_ok() {
            if log_level == 1 {
                println!("SOUNDNESS ERROR in {}", infile_name);
            }
            if let Some(crs) = maybe_cr_script {
                print_creduce(&results, crs, infile_name);
            }
            ret_val = 0
        }

        ret_val
    };
    std::process::exit(ecode);
}

fn print_creduce(results: &Vec<RSolve>, crs: &str, infile_name: &str) {
    let eb = results
        .iter()
        .position(|r| r.has_bug_error())
        .map(|p| fs::write(crs, format!("solver-cli -p {} {}", p, infile_name)).unwrap());

    // Only report soundness if other bugs aren't present, as they are typically the cause of
    // false positive soundness issues
    if eb.is_none() && !RSolve::differential_test(results).is_ok() {
        fs::write(crs, format!("solver-cli --soundness-only {}", infile_name)).unwrap();
    }
}

fn parse_profiles(profiles_str: &str) -> HashSet<ProfileIndex> {
    profiles_str
        .split(',')
        .map(|strind| strind.parse::<usize>().unwrap())
        .map(|ind| ProfileIndex::new(ind))
        .collect()
}

fn format(file: &Path) {
    let contents: String = fs::read_to_string(file).unwrap();
    let stripped_contents = &rmv_comments(&contents[..]).unwrap().1.join(" ")[..];
    let script = script(&stripped_contents[..]).unwrap().1;
    println!("{}", script);
}
