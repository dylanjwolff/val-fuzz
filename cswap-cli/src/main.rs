#[allow(dead_code)]
#[allow(unused)]
extern crate clap;
extern crate cswap;
extern crate log;
extern crate log4rs;

use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Config as L4rsConfig;
use log4rs::config::{Appender, Logger, Root};

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use cswap::config::Config;
use cswap::config::FileProvider;
use cswap::exec;
use cswap::exec_randomized;
use cswap::from_skels;
use cswap::solver::all_profiles;
use cswap::solver::profiles_to_string;
use cswap::solver::ProfileIndex;
use std::cmp::max;
use std::collections::HashSet;

const KEEP_FILES: &'static str = "keep-files";
const STACK_SIZE: &'static str = "stack-size";
const DIR: &'static str = "Seed/Skeleton Directory";
const FROM_SKELS: &'static str = "from-skels";
const WORKERS: &'static str = "worker-counts";
const MAX_ITER: &'static str = "max-iter";
const SEED: &'static str = "seed";
const PROFILES: &'static str = "profiles";
const LIST_PROFILES: &'static str = "list-profiles";
const NO_MULTITHREAD: &'static str = "no-multithreading";
const VERBOSITY: &'static str = "verbosity";
const RBASE: &'static str = "randomized-baseline";

fn main() {
    let matches: ArgMatches = App::new("Value Constant Mutation Fuzzer for SMTlib2 Solvers")
        .arg(
            Arg::with_name(DIR)
                .help("Location of the seeds (or skeletons) for the run")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name(VERBOSITY)
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity of logging"),
        )
        .arg(
            Arg::with_name(FROM_SKELS)
                .short("f")
                .help("Use skeleton files from pre-processing or previous run"),
        )
        .arg(
            Arg::with_name(RBASE)
                .short("r")
                .help("Use a completely randomized constant value replacement strategy"),
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
        .arg(
            Arg::with_name(PROFILES)
                .short("p")
                .help("Set the solver profiles to use")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(LIST_PROFILES)
                .short("l")
                .help("List the profiles availaible for use with the -p option"),
        )
        .arg(
            Arg::with_name(KEEP_FILES)
                .short("k")
                .help("Keeps intermediate / non-bug inducing generated files"),
        )
        .arg(
            Arg::with_name(NO_MULTITHREAD)
                .short("n")
                .help("Run the fuzzer in a single process/thread for debugging"),
        )
        .arg(
            Arg::with_name(STACK_SIZE)
                .short("z")
                .takes_value(true)
                .help("stack size per thread in MB (default 500MB)"),
        )
        .get_matches();

    setup_logging(matches.occurrences_of(VERBOSITY));
    if matches.is_present(LIST_PROFILES) {
        println!("{}", profiles_to_string());
        return;
    }

    let profiles = match matches.value_of(PROFILES) {
        Some(pstr) => parse_profiles(pstr),
        None => all_profiles(),
    };

    let dir_name = matches.value_of(DIR).unwrap();
    let workers = match matches.value_of(WORKERS) {
        Some(workerstr) => parse_workers(workerstr),
        None => (2, 2, 9),
    };
    info!("Using {:?} worker configuration", workers);

    let max_iter = match matches.value_of(MAX_ITER) {
        Some(iterstr) => iterstr.parse::<u32>().unwrap(),
        None => 1,
    };
    let seed = match matches.value_of(SEED) {
        Some(seedstr) => seedstr.parse::<u64>().unwrap(),
        None => 0,
    };
    info!("Seed is {:?}", seed);
    let stack_size = match matches.value_of(STACK_SIZE) {
        Some(stacksstr) => stacksstr.parse::<usize>().unwrap() * 1024 * 1024,
        None => 500 * 1024 * 1024,
    };
    info!("Stack size is {:?}B", stack_size);

    let fp = FileProvider::new(&(seed.to_string() + "-cswap-fuzz-run-out"));
    let cfg = Config {
        file_provider: fp,
        rng_seed: seed,
        max_iter: max_iter,
        stack_size: stack_size,
        remove_files: !matches.is_present(KEEP_FILES),
        mask_size: 1,
        profiles,
        monitors_in_final: false,
        enforce_on_resub: false,
    };

    assert!(
        !cfg.enforce_on_resub || cfg.monitors_in_final,
        "Must have monitors in final to enforce resub!"
    );

    if matches.is_present(NO_MULTITHREAD) {
        cswap::fuzzer::exec_single_thread(dir_name, cfg);
        return;
    }

    if matches.is_present(RBASE) {
        exec_randomized(dir_name, (max(workers.0, workers.1), workers.2), cfg)
    } else if matches.is_present(FROM_SKELS) {
        from_skels(dir_name, (workers.1, workers.2), cfg)
    } else {
        exec(dir_name, workers, cfg)
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

fn parse_profiles(profiles_str: &str) -> HashSet<ProfileIndex> {
    profiles_str
        .split(',')
        .map(|strind| strind.parse::<usize>().unwrap())
        .map(|ind| ProfileIndex::new(ind))
        .collect()
}

fn setup_logging(vb_occurrences: u64) {
    let verbosity = match vb_occurrences {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 | _ => LevelFilter::Trace,
    };

    let stdout = ConsoleAppender::builder().build();
    let config = L4rsConfig::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("cswap_cli", verbosity))
        .logger(Logger::builder().build("cswap", verbosity))
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();
}
