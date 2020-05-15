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
const SIGTERM_TO: &str = "interrupted by SIGTERM";
const UNIMPL: &str = "Unimplemented code";
const NON_BUG_ERRORS: [&str; 2] = [SIGTERM_TO, UNIMPL];

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Solver {
    Z3,
    CVC4,
    NONE,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RSolve {
    stdout: String,
    stderr: String,
    lines: Vec<ResultLine>,
    pub solver: Solver,
}

impl RSolve {
    pub fn process_error() -> Self {
        RSolve {
            stdout: "".to_owned(),
            stderr: "".to_owned(),
            lines: vec![ResultLine::Error("Process Error".to_owned())],
            solver: Solver::NONE,
        }
    }

    pub fn move_new(solver: Solver, stdout: String, stderr: String) -> Self {
        let mut v = z3o(&stdout).unwrap().1;
        v.extend(z3o(&stderr).unwrap().1);
        RSolve {
            // Following should never panic, as parser should never throw an error
            lines: {
                let mut v = z3o(&stdout).unwrap().1;
                v.extend(z3o(&stderr).unwrap().1);
                v
            },
            stdout: stdout,
            stderr: stderr,
            solver: solver,
        }
    }

    pub fn new(solver: Solver, stdout: &str, stderr: &str) -> Self {
        let mut v = z3o(&stdout).unwrap().1;
        v.extend(z3o(&stderr).unwrap().1);
        RSolve {
            stdout: stdout.to_owned(),
            stderr: stderr.to_owned(),
            // Following should never panic, as parser should never throw an error
            lines: {
                let mut v = z3o(&stdout).unwrap().1;
                v.extend(z3o(&stderr).unwrap().1);
                v
            },
            solver: solver,
        }
    }

    fn bug_is_negated(&self, bug_error: &str) -> bool {
        if bug_error == SF_DC {
            for nbe in NON_BUG_ERRORS.iter() {
                if self.stdout.contains(nbe) || self.stderr.contains(nbe) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn has_bug_error(&self) -> bool {
        for bug_error in BUG_ERRORS.iter() {
            if (self.stdout.contains(bug_error) || self.stderr.contains(bug_error))
                && !self.bug_is_negated(bug_error)
            {
                return true;
            }
        }
        return false;
    }

    pub fn has_unrecoverable_error(&self) -> bool {
        self.lines
            .iter()
            .filter_map(|l| match l {
                ResultLine::Error(s) => Some(s),
                _ => None,
            })
            // any error line that has no non-fatal errors is a fatal error
            .any(|s| NON_FATAL_ERRORS.iter().all(|e| !s.contains(e)))
    }

    pub fn was_timeout(&self) -> bool {
        self.lines.iter().any(|l| match l {
            ResultLine::Timeout => true,
            _ => false,
        })
    }

    pub fn differential(a: &Self, b: &Self) -> bool {
        if a.has_unrecoverable_error()
            || b.has_unrecoverable_error()
            || a.was_timeout()
            || b.was_timeout()
        {
            return false;
        }

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

    pub fn has_sat(&self) -> bool {
        self.lines.iter().any(|l| match l {
            ResultLine::Sat => true,
            _ => false,
        })
    }

    pub fn has_unsat(&self) -> bool {
        self.lines.iter().any(|l| match l {
            ResultLine::Unsat => true,
            _ => false,
        })
    }

    pub fn has_unknown(&self) -> bool {
        self.lines.iter().any(|l| match l {
            ResultLine::Unknown => true,
            _ => false,
        })
    }

    pub fn extract_const_var_vals(&self, varnames: &Vec<String>) -> Vec<(&Symbol, &SExp)> {
        self.lines
            .iter()
            .filter_map(|l| match l {
                ResultLine::Model(m) => Some(
                    m.iter()
                        .filter(|(name, _, _, _)| varnames.contains(&name.to_string()))
                        .map(|(name, _, _, val)| (name, val))
                        .collect::<Vec<(&Symbol, &SExp)>>(),
                ),
                _ => None,
            })
            .flatten()
            .collect::<Vec<(&Symbol, &SExp)>>()
    }
}

fn solve_cvc4(cvc4path: &str, filename: &str) -> RSolve {
    let cvc4_res = process::Command::new("timeout")
        .args(&[
            "-v",
            "6s",
            cvc4path,
            "--produce-models",
            "--incremental",
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
        Ok((cvc4_succ, cvc4_out, cvc4_err)) => RSolve::move_new(Solver::CVC4, cvc4_out, cvc4_err),
        Err(_) => RSolve::process_error(),
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
        Ok((z3_succ, z3_out, z3_err)) => RSolve::move_new(Solver::Z3, z3_out, z3_err),
        Err(_) => RSolve::process_error(),
    }
}
pub fn solve(filename: &str) -> (RSolve, RSolve) {
    (solve_cvc4("cvc4", filename), solve_z3("z3", filename))
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
        let r = RSolve::new(Solver::Z3, rstr, "");
        assert_debug_snapshot!(r.extract_const_var_vals(&vec![
            "GEN1".to_owned(),
            "GEN2".to_owned(),
            "BAV3".to_owned()
        ]));
    }

    #[test]
    fn z3_new() {
        let rstring = "sat (model (define-fun b () Int 0) (define-fun a () Int 1))";
        let r = RSolve::new(Solver::Z3, "", rstring);
        assert!(r.has_sat());
        assert!(!r.has_unrecoverable_error());
        assert!(!r.has_bug_error());
    }

    #[test]
    fn diff_self() {
        let rstring = "sat (model (define-fun b () Int 0) (define-fun a () Int 1))";
        let r1 = RSolve::new(Solver::Z3, "", rstring);
        let r2 = RSolve::new(Solver::Z3, rstring, "");
        assert!(!RSolve::differential(&r1, &r2))
    }

    #[test]
    fn diff_difft() {
        let rstring = "sat (model (define-fun b () Int 0) (define-fun a () Int 1))";
        let r1 = RSolve::new(Solver::Z3, "", rstring);
        let r2 = RSolve::new(Solver::Z3, "unsat", "");
        assert!(RSolve::differential(&r1, &r2))
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
        let r1 = RSolve::new(Solver::Z3, r1str, "");
        let r2 = RSolve::new(Solver::Z3, r2str, "");
        assert!(!RSolve::differential(&r1, &r2));
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
        let r = RSolve::new(Solver::Z3, rstr, "");
        assert!(!r.has_unrecoverable_error());
    }

    #[test]
    fn cvc4_timeout() {
        let rstr = "timeout: sending signal TERM to command ‘cvc4’\nCVC4 interrupted by SIGTERM.\n timeout: the monitored command dumped core";
        let r = RSolve::new(Solver::Z3, rstr, "");
        assert!(!r.has_bug_error());
    }

    #[test]
    fn cvc4_unimpl_not_bug() {
        let rstr = "Fatal failure within CVC4::Node CVC4::theory::fp::FpConverter::convert(CVC4::TNode) at /home/dylan/git/constant-swap/scripts/.solvers/cvc4/src/theory/fp/fp_converter.cpp:1700
        Unimplemented code encounteredConversion is dependent on SymFPU
        timeout: the monitored command dumped core";
        let r = RSolve::new(Solver::Z3, rstr, "");
        assert!(!r.has_bug_error());
    }

    #[test]
    fn difft_diff_w_error() {
        let rstr1 =
            "(error \"line 11 column 79: Sort mismatch between first argument and argument 2\")
                sat";
        let rstr2 = "unsat";
        let r1 = RSolve::new(Solver::Z3, rstr1, "");
        let r2 = RSolve::new(Solver::Z3, rstr2, "");
        assert!(!RSolve::differential(&r1, &r2));
    }

    #[test]
    fn fatal_error() {
        let rstr = "unsat \n (error \"something went wrong\")";
        let r = RSolve::new(Solver::Z3, rstr, "");
        assert!(r.has_unrecoverable_error());
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
