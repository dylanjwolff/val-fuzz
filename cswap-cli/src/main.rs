#[allow(dead_code)]
#[allow(unused)]
extern crate clap;
extern crate cswap;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use cswap::exec;
use cswap::from_skels;

const DIR: &'static str = "Seed/Skeleton Directory";
const FROM_SKELS: &'static str = "from-skels";
const WORKERS: &'static str = "worker-counts";

fn main() {
    let matches: ArgMatches = App::new("Value Constant Mutation Fuzzer for SMTlib2 Solvers")
        .arg(
            Arg::with_name(DIR)
                .help("Location of the seeds (or skeletons) for the run")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name(FROM_SKELS)
                .short("f")
                .help("Use skeleton files from pre-processing or previous run"),
        )
        .arg(
            Arg::with_name(WORKERS)
                .short("w")
                .help("Worker threads for each stage (e.g. 0,1,2)")
                .takes_value(true),
        )
        .get_matches();

    let dir_name = matches.value_of(DIR).unwrap();
    let workers = match matches.value_of(WORKERS) {
        Some(workerstr) => parse_workers(workerstr),
        None => (2, 2, 9),
    };

    match matches.value_of(FROM_SKELS) {
        Some(_) => from_skels(dir_name, (workers.0, workers.1)),
        None => exec(dir_name, workers),
    }
}

fn parse_workers(workern_str: &str) -> (u8, u8, u8) {
    let mut workers = workern_str
        .split(',')
        .map(|strind| strind.parse::<u8>().unwrap());

    (
        workers.next().unwrap(),
        workers.next().unwrap(),
        workers.next().unwrap(),
    )
}
