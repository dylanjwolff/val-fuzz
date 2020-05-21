#[allow(dead_code)]
#[allow(unused)]
extern crate clap;
extern crate cswap;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use std::path::Path;
use std::collections::HashSet;
use cswap::solver::solve;
use cswap::solver::solve_cvc4;
use cswap::solver::solve_z3;
use cswap::solver::RSolve;
use cswap::solver::CVC4_Command_Builder;
use cswap::solver::Z3_Command_Builder;
use cswap::solver::CVC4_PROFILES;
use cswap::solver::Z3_PROFILES;
use std::fs;

const INFILE: &'static str = "File to Solve";
const CREDUCE_SCRIPT: &'static str = "creduce-script";
const PROFILES: &'static str = "profiles";

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
            .arg(
                Arg::with_name(PROFILES)
                .short("p")
                .help("Set the solver profiles to use")
                .takes_value(true),
                )
            .get_matches();
        let log_level = matches.occurrences_of("verbosity");
        let maybe_cr_script = matches.value_of(CREDUCE_SCRIPT);
        let infile_name = matches.value_of(INFILE).unwrap();

        let results = match matches.value_of(PROFILES) {
            Some(pstr) => profiles_solve(infile_name, &parse_profiles(pstr)),
            None => solve(infile_name),
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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum ProfileIndex {
    Z3(usize),
    CVC4(usize),
}

impl ProfileIndex {
    fn new(ind : usize) -> Self {
        let num_cvc4_profiles = CVC4_PROFILES.len() ;
        if ind < num_cvc4_profiles {
            Self::CVC4(ind)
        } else {
            Self::Z3(ind - num_cvc4_profiles)
        }
    }
}

fn print_creduce(results : &Vec<RSolve>, crs : &str, infile_name : &str) {
    let eb = results
        .iter()
        .position(|r| r.has_bug_error())
        .map(|p| fs::write(crs, format!("solver-cli -p {} {}", p, infile_name)).unwrap());
    if eb.is_none() && !RSolve::differential_test(results).is_ok() {
        fs::write(crs, format!("solver-cli {}", infile_name)).unwrap();
    }
}

fn parse_profiles(profiles_str : &str) -> HashSet<ProfileIndex> {
    profiles_str.split(',')
        .map(|strind| strind.parse::<usize>().unwrap())
        .map(|ind| ProfileIndex::new(ind))
        .collect()
}

pub fn profiles_solve(filename: &str, pis : &HashSet<ProfileIndex>) -> Vec<RSolve> {
    let filepath = Path::new(filename);

    let mr_cvc4 = CVC4_PROFILES
        .iter()
        .zip(0..CVC4_PROFILES.len()-1)
        .filter(|(_, i)| pis.contains(&ProfileIndex::CVC4(*i)))
        .map(|(p, _ )| p)
        .map(|profile| solve_cvc4(profile, &filepath));

    let mr_z3 = Z3_PROFILES
        .iter()
        .zip(0..Z3_PROFILES.len()-1)
        .filter(|(_, i)| pis.contains(&ProfileIndex::Z3(*i)))
        .map(|(p, _ )| p)
        .map(|profile| solve_z3(profile, &filepath));

    mr_cvc4.chain(mr_z3).collect()
}
