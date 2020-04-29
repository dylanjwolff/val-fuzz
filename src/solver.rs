use std::process;
use std::str::from_utf8;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SolveResult {
    SoundnessBug,
    ErrorBug,
    Error,
    ProcessError,
    Sat,
    Unsat,
    Unknown,
    Timeout,
}

const TO: &str = "timeout: sending signal TERM";

const SF: &str = "segfault";
const SF_L: &str = "segmentation fault";
const SF_DC: &str = "dumped core";
const AV: &str = "ASSERTION VIOLATION";
const AF: &str = "Assertion Failure";
const IE: &str = "Internal error detected";
const IVM: &str = "invalid model";
const BUG_ERRORS: [&str; 7] = [SF, SF_L, SF_DC, AV, AF, IE, IVM];

fn is_bug_error(stdout: &str, stderr: &str) -> bool {
    for bug_error in BUG_ERRORS.iter() {
        if stdout.contains(bug_error) || stderr.contains(bug_error) {
            return true;
        }
    }
    return false;
}

fn is_timeout(stdout: &str, stderr: &str) -> bool {
    stdout.contains(TO) || stderr.contains(TO)
}

fn is_unrecoverable(stdout: &str, stderr: &str) -> bool {
    stderr.contains("(error ") || stdout.contains("(error ")
}

fn get_z3_result(stdout: &str, stderr: &str) -> SolveResult {
    if stdout.contains("unsat") || stderr.contains("unsat\n") {
        SolveResult::Unsat
    } else if stdout.contains("sat") || stderr.contains("sat\n") {
        SolveResult::Sat
    } else {
        SolveResult::Unknown
    }
}

fn solve_z3(z3path: &str, filename: &str) -> SolveResult {
    let z3_res = process::Command::new("timeout")
        .args(&[
            "-v",
            "6s",
            z3path,
            "smt.string_solver=z3str3",
            "model_validate=true",
            filename,
        ])
        .output();

    let z3mrs = z3_res.map(|out| {
        let stderr = from_utf8(&out.stderr[..]).unwrap_or("");
        let stdout = from_utf8(&out.stdout[..]).unwrap_or("");
        let success = out.status.success();
        (success, stderr.to_owned(), stdout.to_owned())
    });

    match z3mrs {
        Ok((z3_succ, z3_out, z3_err)) => {
            println!("s: {} o: {} e: {}", z3_succ, z3_out, z3_err);
            if is_timeout(&z3_out, &z3_err) {
                SolveResult::Timeout
            } else if is_bug_error(&z3_out, &z3_err) {
                SolveResult::ErrorBug
            } else if !z3_succ && is_unrecoverable(&z3_out, &z3_err) {
                SolveResult::Error
            } else {
                get_z3_result(&z3_err, &z3_err)
            }
        }
        Err(_) => SolveResult::ProcessError,
    }
}

fn get_cvc4_result(stdout: &str, stderr: &str) -> SolveResult {
    if stdout.contains("unsat") || stderr.contains("unsat\n") {
        SolveResult::Unsat
    } else if stdout.contains("sat") || stderr.contains("sat\n") {
        SolveResult::Sat
    } else {
        SolveResult::Unknown
    }
}

fn solve_cvc4(cvc4path: &str, filename: &str) -> SolveResult {
    let cvc4_res = process::Command::new("timeout")
        .args(&[
            "-v",
            "6s",
            "cvc4",
            "--incremental",
            "--produce-model",
            filename,
        ])
        .output();

    let cvc4mrs = cvc4_res.map(|out| {
        let stderr = from_utf8(&out.stderr[..]).unwrap_or("");
        let stdout = from_utf8(&out.stdout[..]).unwrap_or("");
        let success = out.status.success();
        (success, stderr.to_owned(), stdout.to_owned())
    });

    match cvc4mrs {
        Ok((cvc4_succ, cvc4_out, cvc4_err)) => {
            if is_timeout(&cvc4_out, &cvc4_err) {
                SolveResult::Timeout
            } else if is_bug_error(&cvc4_out, &cvc4_err) {
                SolveResult::ErrorBug
            } else if !cvc4_succ && is_unrecoverable(&cvc4_out, &cvc4_err) {
                SolveResult::Error
            } else {
                get_cvc4_result(&cvc4_err, &cvc4_err)
            }
        }
        Err(_) => SolveResult::ProcessError,
    }
}

pub fn solve(filename: &str) -> SolveResult {
    match (solve_z3("z3", filename), solve_cvc4("cvc4", filename)) {
        (SolveResult::ErrorBug, _) | (_, SolveResult::ErrorBug) => SolveResult::ErrorBug,
        (SolveResult::Sat, SolveResult::Unsat) | (SolveResult::Unsat, SolveResult::Sat) => {
            SolveResult::SoundnessBug
        }
        (SolveResult::ProcessError, _) | (_, SolveResult::ProcessError) => {
            SolveResult::ProcessError
        }
        (SolveResult::Error, _) | (_, SolveResult::Error) => SolveResult::Error,
        (SolveResult::Sat, _) | (_, SolveResult::Sat) => SolveResult::Sat,
        (SolveResult::Unsat, _) | (_, SolveResult::Unsat) => SolveResult::Unsat,
        _ => SolveResult::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use walkdir::WalkDir;

    const STACK_SIZE: usize = 20 * 1024 * 1024;

    #[ignore]
    #[test]
    fn to_detect() {
        // need to manually shorten timeout for this one, hence ignored for now
        let segf_file = "test/prp-13-24.smt2";
        assert!(solve(segf_file) == SolveResult::Timeout);
    }

    #[test]
    fn segf_detect() {
        let segf_file = "known/8/352f4b5b3/3773_segf.smt2";
        assert!(solve(segf_file) == SolveResult::ErrorBug);
    }

    #[test]
    fn solver_test() {
        for entry in WalkDir::new("known/8")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            let filepath = entry.path();
            println!("starting file {:?}", filepath);
            solve(filepath.to_str().unwrap());
        }
    }
}
