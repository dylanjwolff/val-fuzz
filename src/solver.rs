use crate::ast::*;
use crate::parser::*;
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
const MNA_Z3: &str = "model is not available";
const MNA_CVC4: &str = "Cannot get model unless";
const NON_FATAL_ERRORS: [&str; 2] = [MNA_CVC4, MNA_Z3];

#[allow(dead_code)]
struct RSolve<'a> {
    stdout: String,
    stderr: String,
    lines: Vec<ResultLine<'a>>,
}

#[allow(dead_code)]
impl<'a> RSolve<'a> {
    fn new(stdout: String, stderr: String) -> Self {
        RSolve {
            stdout: stdout,
            stderr: stderr,
            // Following should never panic, as parser should never throw an error
            lines: {
                let mut v = z3o(stdout).unwrap().1;
                v.extend(z3o(stderr).unwrap().1);
                v
            },
        }
    }

    fn has_bug_error(&self) -> bool {
        for bug_error in BUG_ERRORS.iter() {
            if self.stdout.contains(bug_error) || self.stderr.contains(bug_error) {
                return true;
            }
        }
        return false;
    }

    fn has_unrecoverable_error(&self) -> bool {
        self.lines
            .iter()
            .filter_map(|l| match l {
                ResultLine::Error(s) => Some(s),
                _ => None,
            })
            // any error line that has no non-fatal errors is a fatal error
            .any(|s| NON_FATAL_ERRORS.iter().all(|e| !s.contains(e)))
    }

    fn was_timeout(&self) -> bool {
        self.lines.iter().any(|l| match l {
            ResultLine::Timeout => true,
            _ => false,
        })
    }

    fn differential(a: Self, b: Self) -> bool {
        let is_out_result = |l: &&ResultLine| match l {
            ResultLine::Sat | ResultLine::Unsat | ResultLine::Unknown => true,
            _ => false,
        };
        a.lines
            .iter()
            .filter(|l| is_out_result(l))
            .zip(b.lines.iter().filter(|l| is_out_result(l)))
            .any(|r| match r {
                (ResultLine::Sat, ResultLine::Unsat) | (ResultLine::Unsat, ResultLine::Sat) => true,
                _ => false,
            })
    }

    fn has_sat(&self) -> bool {
        self.lines.iter().any(|l| match l {
            ResultLine::Sat => true,
            _ => false,
        })
    }

    fn extract_const_var_vals(&self, varnames: Vec<&str>) -> Vec<(&Symbol, &SExp)> {
        self.lines
            .iter()
            .filter_map(|l| match l {
                ResultLine::Model(m) => Some(
                    m.iter()
                        .filter(|(name, _, _, _)| varnames.contains(&&name.to_string()[..]))
                        .map(|(name, _, _, val)| (name, val))
                        .collect::<Vec<(&Symbol, &SExp)>>(),
                ),
                _ => None,
            })
            .flatten()
            .collect::<Vec<(&Symbol, &SExp)>>()
    }
}

fn solve_z3(z3path: &str, filename: &str) -> RSolve {
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
        Ok((z3_succ, z3_out, z3_err)) => RSolve::new(z3_out, z3_err),
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

fn solve_cvc4(_cvc4path: &str, filename: &str) -> SolveResult {
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
        (SolveResult::Timeout, _) | (_, SolveResult::Timeout) => SolveResult::Timeout,
        _ => SolveResult::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use walkdir::WalkDir;

    #[test]
    fn get_var_vals_snap() {
        let rstr = "sat
                        (model 
                          (define-fun b () Int
                            0)
                          (define-fun a () Int
                            1)
                          (define-fun e () Real
                            1.0)
                          (define-fun GEN1 () Real
                            (- 1.0))
                          (define-fun BAV5 () Bool
                            true)
                          (define-fun BAV4 () Bool
                            true)
                          (define-fun BAV3 () Bool
                            true)
                          (define-fun c () Int
                            0)
                          (define-fun GEN2 () Real
                            0.0)
                          (define-fun d () Real
                            0.0)
                        )";
        let r = RSolve::new(rstr, "");
        assert_debug_snapshot!(r.extract_const_var_vals(vec!["GEN1", "GEN2", "BAV3"]));
    }

    #[test]
    fn z3_new() {
        let rstring = "sat (model (define-fun b () Int 0) (define-fun a () Int 1))";
        let r = RSolve::new("", rstring);
        assert!(r.has_sat());
        assert!(!r.has_unrecoverable_error());
        assert!(!r.has_bug_error());
    }

    #[test]
    fn diff_self() {
        let rstring = "sat (model (define-fun b () Int 0) (define-fun a () Int 1))";
        let r1 = RSolve::new("", rstring);
        let r2 = RSolve::new(rstring, "");
        assert!(!RSolve::differential(r1, r2))
    }

    #[test]
    fn diff_difft() {
        let rstring = "sat (model (define-fun b () Int 0) (define-fun a () Int 1))";
        let r1 = RSolve::new("", rstring);
        let r2 = RSolve::new("unsat", "");
        assert!(RSolve::differential(r1, r2))
    }

    #[test]
    fn diff_difft_multiple() {
        let r1str = "unsupported
            samples/z3.11.smt2:6.14: No set-logic command was given before this point.
            samples/z3.11.smt2:6.14: CVC4 will make all theories available.
            samples/z3.11.smt2:6.14: Consider setting a stricter logic for (likely) better performance.
            samples/z3.11.smt2:6.14: To suppress this warning in the future use (set-logic ALL).
            sat
            (model
            (define-fun a () Real 0.0)
            )
            unsat";
        let r2str = "sat
            (model 
              (define-fun a () Real
                38.0)
              (define-fun /0 ((x!0 Real) (x!1 Real)) Real
                10.0)
            )
            unsat";
        let r1 = RSolve::new(r1str, "");
        let r2 = RSolve::new(r2str, "");
        assert!(!RSolve::differential(r1, r2));
    }

    #[test]
    fn nonfatal_error() {
        let rstr = "unsupported
                unsupported
                unsupported
                testfile.smt2:8.12: No set-logic command was given before this point.
                testfile.smt2:8.12: CVC4 will make all theories available.
                testfile.smt2:8.12: Consider setting a stricter logic for (likely) better performance.
                testfile.smt2:8.12: To suppress this warning in the future use (set-logic ALL).
                unsat
                (error \"Cannot get model unless immediately preceded by SAT/NOT_ENTAILED or UNKNOWN response.\")";
        let r = RSolve::new(rstr, "");
        assert!(!r.has_unrecoverable_error());
    }

    #[test]
    fn fatal_error() {
        let rstr = "unsat \n (error \"something went wrong\")";
        let r = RSolve::new(rstr, "");
        assert!(r.has_unrecoverable_error());
    }

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
