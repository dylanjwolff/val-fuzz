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
use std::time::Duration;

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
use cswap::utils::MyBackoff;
use std::cmp::max;
use std::collections::HashSet;
use std::thread;
use std::thread::JoinHandle;
use std::time::SystemTime;

const MFINAL: &'static str = "monitors-in-final";
const TIMEOUT: &'static str = "timeout";
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
const MULTIEF: &'static str = "multi-enforce";
const EFFINAL: &'static str = "enforce-final";
const CPOG: &'static str = "copy-original";
const SKOLU: &'static str = "skolemize-universal";
const NOSKOLE: &'static str = "no-skolemize-existential";
const RELC: &'static str = "const-relations";
const ADOMAIN: &'static str = "abstract-domain-vars";
const ADOMAINE: &'static str = "abstract-domain-sub-expressions";
const LEAFOPT: &'static str = "leaf-opt";
const MINCONSTS: &'static str = "min-consts";
const MAXCONSTS: &'static str = "max-consts";

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
            Arg::with_name(TIMEOUT)
                .long(TIMEOUT)
                .help("Sets the default timeout for calls to the solver")
                .short("t")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(MULTIEF)
                .long(MULTIEF)
                .help("Enforce 'n' constraints on the meta-formula")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(MINCONSTS)
                .long(MINCONSTS)
                .help("Attempts to ensure each formula has at least 'n' constant values by concretizing variables to be constants")
                .takes_value(true),
        ).arg(
            Arg::with_name(MAXCONSTS)
                .long(MAXCONSTS)
                .help("Ensures that each formula uses only up to 'n' constants for value mutations")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(EFFINAL)
                .long(EFFINAL)
                .help("Enforce constraints on the final call to solvers"),
        )
        .arg(Arg::with_name(MFINAL).long(MFINAL).help(
            "Include monitor variables in final formula for e.g. boolean sub-exp coverage metrics",
        ))
        .arg(Arg::with_name(CPOG).long(CPOG).help(
            "Copy the original formula and its negation to create two meta-formulas per seed",
        ))
        .arg(
            Arg::with_name(SKOLU)
                .help("Skolemize universal quantifiers")
                .long(SKOLU),
        )
        .arg(
            Arg::with_name(NOSKOLE)
                .help("Don't skolemize existential quantifiers")
                .long(NOSKOLE),
        )
        .arg(
            Arg::with_name(RELC)
                .long(RELC)
                .help("Use 'n' constants to create relational constraints (e.g. c1 == c2)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(LEAFOPT)
                .long(LEAFOPT)
                .help("Ignore non-leaf sub-expressions for creating the meta-formula"),
        )
        .arg(
            Arg::with_name(ADOMAIN)
                .long(ADOMAIN)
                .help("Use abstract domain constraints for variables (e.g. x == 0)"),
        )
        .arg(
            Arg::with_name(ADOMAINE)
                .long(ADOMAINE)
                .help("Use abstract domain constraints for sub-expressions (e.g. (+ x 3) == 0)"),
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
    let seeds = match matches.value_of(SEED) {
        Some(seedstr) => seedstr
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect(),
        None => vec![0],
    };
    info!("Seeds are {:?}", seeds);
    let stack_size = match matches.value_of(STACK_SIZE) {
        Some(stacksstr) => stacksstr.parse::<usize>().unwrap() * 1024 * 1024,
        None => 500 * 1024 * 1024,
    };
    info!("Stack size is {:?}B", stack_size);

    let enforce_mask_size = match matches.value_of(MULTIEF) {
        Some(s) => s.parse::<usize>().unwrap(),
        None => 1,
    };

    let min_consts = match matches.value_of(MINCONSTS) {
        Some(s) => s.parse::<usize>().unwrap(),
        None => 0,
    };

    let max_consts = match matches.value_of(MAXCONSTS) {
        Some(s) => {
            let mc = s.parse::<usize>().unwrap();
            assert!(
                mc >= min_consts,
                "Max Constants must be greater than the minimum number of constants"
            );
            Some(mc)
        }

        None => None,
    };

    let rel_cs = match matches.value_of(RELC) {
        Some(s) => s.parse::<u8>().unwrap(),
        None => 0,
    };

    let timeout = match matches.value_of(TIMEOUT) {
        Some(s) => {
            Duration::from_secs(s.parse::<u8>().expect("Timeout should be small number") as u64)
        }
        None => Duration::from_secs(6),
    };

    assert!(
        !matches.is_present(ADOMAINE) || matches.is_present(SKOLU),
        "Expr Adomains can't have universal quantifiers"
    );

    let cfg = Config {
        max_iter: max_iter,
        stack_size: stack_size,
        remove_files: !matches.is_present(KEEP_FILES),
        mask_size: enforce_mask_size,
        max_const_relations_to_monitor: rel_cs,
        dont_skolemize_existential: matches.is_present(NOSKOLE),
        monitors_in_final: matches.is_present(EFFINAL) || matches.is_present(MFINAL),
        use_bdom_vs: matches.is_present(ADOMAIN),
        adomain_exprs: matches.is_present(ADOMAINE),
        skolemize_universal: matches.is_present(SKOLU),
        leaf_opt: matches.is_present(LEAFOPT),
        cp_og: matches.is_present(CPOG),
        enforce_on_resub: matches.is_present(EFFINAL),
        max_consts,
        min_consts,
        timeout,
        profiles,
        ..Config::default()
    };

    assert!(
        !cfg.enforce_on_resub || cfg.monitors_in_final,
        "Must have monitors in final to enforce resub!"
    );

    if matches.is_present(NO_MULTITHREAD) {
        cswap::fuzzer::exec_single_thread(dir_name, cfg);
        return;
    }

    let start = SystemTime::now();

    let use_random_base = matches.is_present(RBASE);
    let use_from_skels = matches.is_present(FROM_SKELS);
    let handles = seeds
        .iter()
        .map(|seed| {
            let dir_name = dir_name.to_owned();
            let fp = FileProvider::new_unique(&(seed.to_string() + "-cswap-fuzz-run-out"));
            let cfg = Config {
                rng_seed: *seed,
                file_provider: fp,
                ..cfg.clone()
            };
            thread::Builder::new()
                .stack_size(cfg.stack_size)
                .spawn(move || {
                    if use_random_base {
                        exec_randomized(&dir_name, (max(workers.0, workers.1), workers.2), cfg);
                    } else if use_from_skels {
                        from_skels(&dir_name, (workers.1, workers.2), cfg);
                    } else {
                        exec(&dir_name, workers, cfg);
                    }
                })
        })
        .collect::<Vec<std::io::Result<JoinHandle<()>>>>();

    let mut backoff = MyBackoff::new();
    for h in handles {
        h.unwrap().join().unwrap();
        backoff.snooze();
        info!("Seed finished");
    }
    let end = SystemTime::now();
    info!(
        "Total Elapsed Time:{}",
        end.duration_since(start).unwrap().as_secs()
    );
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
