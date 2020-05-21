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
        .get_matches();

    let dir_name = matches.value_of(DIR).unwrap();
    match matches.value_of(FROM_SKELS) {
        Some(_) => from_skels(dir_name),
        None => exec(dir_name),
    }
}
