ValFuzz
=======

## Setup and Dependencies:
Rust / Cargo : [https://rustup.rs/](https://rustup.rs/)

Z3: [https://github.com/Z3Prover/z3/](https://github.com/Z3Prover/z3/)

CVC4: [https://github.com/CVC4/CVC4](https://github.com/CVC4/CVC4)

Both Z3 and CVC4 are expected to be on $PATH for now

### Organization
The codebase is split into three modules.

The majority of the code is in the `cswap` library.

A thin CLI wrapper for ValFuzz is in the `cswap-cli` module.

Another thing CLI wrapper *only* for the differential solver used by ValFuzz to detect bugs is located in the `solver-cli` module. This is useful for reproducing bugs after a fuzzing run.

The `scripts` directory contains scripts for the analysis in the paper, and for automatic reproduction and reduction of bugs found.

### Run The Tests

Running the command `cargo test` in the top level directory will run the tests. Note that some tests actually make calls to the Z3 and CVC4 solvers, so these may fail if used with a newer or older version of Z3 or CVC4 than the one with which the tests are created. I believe these tests mostly have something like "run_real" in the name. If the latest version of one of the solvers causes the tests to fail, file an issue on the GitHup repo.

### Build / Install the Fuzzer

From the `cswap-cli` directory do `cargo install --path .` to install the binary CLI to the typical location for Rust programs (for me its `~/.cargo/bin`). Alternatively you can do `cargo build --release` from that directory and copy the binary that gets compiled to the `../target/release` directory wherever you like.

### Running the Fuzzer

The fuzzer CLI has some brief explanations of the options in the -h text, but you can contact me if you have questions. 

I also made a quick bash one-liner in `scripts/shim.sh` that conforms to the OpFuzz interface, along with reasonable options for a bunch of the other knobs that you can twist with the fuzzer. Specifically, you may want to change the number of workers; I don't know how many you will want per process, (I put 1-1-10; the fuzzer works in three stages, but the first two need a much smaller number of workers. The third number is the number of workers solving the intermediate and formulas in parallel). You may also want to reduce the number of solver profiles to get more iterations in.

The fuzzer will do work in and output everything to a new directory with a randomized name in the current working directory. Bug reports will be in `./<random>_<seed>_fuzz_run_out/bugs`. You can grep these reports to filter for soundness bugs or on any other key words (invalid model etc.). The files that cause the corresponding bugs are left in `./<random>_<seed>_fuzz_run_out/scratch`. I have some scripts for bulk reduction and reproduction if you get to that point.

Also, to track progress, I log the lengths of the queues for each stage of the fuzzing process with the `-v` option (enabled in shim.sh), so when the third number hits zero the fuzzer will be done.
Overall, things might still be a bit rough around the edges since I've been the only one using it, so just let me know if something weird comes up.

