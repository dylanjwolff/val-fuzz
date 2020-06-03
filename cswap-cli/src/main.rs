#[allow(dead_code)]
#[allow(unused)]
extern crate clap;
extern crate cswap;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use cswap::config::Config;
use cswap::config::FileProvider;
use cswap::exec;
use cswap::from_skels;

const DIR: &'static str = "Seed/Skeleton Directory";
const FROM_SKELS: &'static str = "from-skels";
const WORKERS: &'static str = "worker-counts";
const MAX_ITER: &'static str = "max-iter";
const SEED: &'static str = "seed";

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
                .help("Worker threads for each stage of three stages (e.g. 0,1,2)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(MAX_ITER)
                .short("i")
                .help("Maximum iterations (BAM assignments) per seed file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(SEED)
                .short("s")
                .help("Seed for randomization. Runs are repeatable with the same seed")
                .takes_value(true),
        )
        .get_matches();

    let dir_name = matches.value_of(DIR).unwrap();
    let workers = match matches.value_of(WORKERS) {
        Some(workerstr) => parse_workers(workerstr),
        None => (2, 2, 9),
    };
    let max_iter = match matches.value_of(MAX_ITER) {
        Some(iterstr) => iterstr.parse::<u32>().unwrap(),
        None => 1,
    };
    let seed = match matches.value_of(SEED) {
        Some(seedstr) => seedstr.parse::<u64>().unwrap(),
        None => 0,
    };
    println!("Starting with workers {:?}", workers);

    let fp = FileProvider::new(&(seed.to_string() + "-cswap-fuzz-run-out"));
    let cfg = Config {
        file_provider: fp,
        rng_seed: seed,
        max_iter: max_iter,
    };

    match matches.is_present(FROM_SKELS) {
        true => from_skels(dir_name, (workers.1, workers.2), cfg),
        false => exec(dir_name, workers, cfg),
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
