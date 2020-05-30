use crate::ast::*;
use crate::parser::*;
use std::fmt;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::str::from_utf8;
use std::time::Duration;
use tempfile::tempfile;
use tempfile::Builder;
use tempfile::NamedTempFile;

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

lazy_static! {
    pub static ref Z3_PROFILES: [Z3_Command_Builder; 7] = [
        Z3_Command_Builder::new(),
        Z3_Command_Builder::new().z3str3(),
        Z3_Command_Builder::new().threads3(),
        Z3_Command_Builder::new().threads3().z3str3(),
        Z3_Command_Builder::new().threads3().ematching(false),
        Z3_Command_Builder::new()
            .threads3()
            .ematching(false)
            .z3str3(),
        Z3_Command_Builder::new()
            .threads3()
            .ematching(false)
            .flat_rw(false)
            .z3str3(),
    ];
    pub static ref CVC4_PROFILES: [CVC4_Command_Builder; 5] = [
        CVC4_Command_Builder::new().models(),
        CVC4_Command_Builder::new()
            .models()
            .incremental()
            .strings_exp(),
        CVC4_Command_Builder::new()
            .strings_exp()
            .unconstrained_simp(),
        CVC4_Command_Builder::new()
            .models()
            .incremental()
            .strings_exp()
            .check_unsat_cores(),
        CVC4_Command_Builder::new()
            .models()
            .incremental()
            .strings_exp()
            .check_unsat_cores()
            .dump_all(),
    ];
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CVC4_Command_Builder {
    cmd: Vec<String>,
}

impl CVC4_Command_Builder {
    fn new() -> Self {
        CVC4_Command_Builder {
            cmd: vec!["timeout", "6s", "cvc4"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        }
    }

    fn timeout(&mut self, duration: Duration) -> Self {
        self.cmd[1] = format!("{}s", duration.as_secs());
        self.clone()
    }

    fn models(&mut self) -> Self {
        self.cmd.push("--produce-models".to_owned());
        self.cmd.push("--check-models".to_owned());
        self.clone()
    }

    fn incremental(&mut self) -> Self {
        self.cmd.push("--incremental".to_owned());
        self.clone()
    }

    fn check_unsat_cores(&mut self) -> Self {
        self.cmd.push("--check-unsat-cores".to_owned());
        self.clone()
    }

    fn unconstrained_simp(&mut self) -> Self {
        self.cmd.push("--unconstrained-simp".to_owned());
        self.clone()
    }

    fn strings_exp(&mut self) -> Self {
        self.cmd.push("--strings-exp".to_owned());
        self.clone()
    }

    fn dump_models(&mut self) -> Self {
        self.cmd.push("--dump-models".to_owned());
        self.clone()
    }

    fn dump_unsat_cores(&mut self) -> Self {
        self.cmd.push("--dump-unsat-cores".to_owned());
        self.clone()
    }

    fn dump_unsat_cores_full(&mut self) -> Self {
        self.cmd.push("--dump-unsat-cores-full".to_owned());
        self.clone()
    }

    fn dump_all(&mut self) -> Self {
        self.dump_models()
            .dump_unsat_cores()
            .dump_unsat_cores_full()
    }

    fn run_on(&self, target: &Path) -> Result<std::process::Output, std::io::Error> {
        let mut cmd = self.cmd.clone();
        cmd.push(target.to_str().unwrap().to_owned());
        process::Command::new(&self.cmd[0]).args(&cmd[1..]).output()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Z3_Command_Builder {
    cmd: Vec<String>,
}

impl Z3_Command_Builder {
    fn new() -> Self {
        Z3_Command_Builder {
            cmd: vec!["timeout", "6s", "z3", "model_validate=true"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        }
    }

    fn timeout(&mut self, duration: Duration) -> Self {
        self.cmd[1] = format!("{}s", duration.as_secs());
        self.clone()
    }

    fn ematching(&mut self, should_ematch: bool) -> Self {
        if !should_ematch {
            self.cmd.push("smt.ematching=false".to_owned());
        } else {
            self.cmd.push("smt.ematching=true".to_owned());
        }
        self.clone()
    }

    fn flat_rw(&mut self, should_flat_rw: bool) -> Self {
        if !should_flat_rw {
            self.cmd.push("rewriter.flat=false".to_owned());
        } else {
            self.cmd.push("rewriter.flat=true".to_owned());
        }
        self.clone()
    }

    fn z3str3(&mut self) -> Self {
        self.cmd.push("smt.string_solver=z3str3".to_owned());
        self.clone()
    }

    fn threads3(&mut self) -> Self {
        self.cmd.push("smt.threads=3".to_owned());
        self.clone()
    }

    fn run_on(&self, target: &Path) -> Result<std::process::Output, std::io::Error> {
        let mut cmd = self.cmd.clone();
        cmd.push(target.to_str().unwrap().to_owned());
        process::Command::new(&self.cmd[0]).args(&cmd[1..]).output()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Solver {
    Z3(Z3_Command_Builder),
    CVC4(CVC4_Command_Builder),
    NONE,
}

impl Solver {
    pub fn name(&self) -> String {
        match self {
            Self::Z3(_) => "Z3".to_owned(),
            Self::CVC4(_) => "CVC4".to_owned(),
            Self::NONE => "NONE".to_owned(),
        }
    }
}
impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Solver::Z3(z3c) => write!(f, "{}", z3c.cmd.join(" ")),
            Solver::CVC4(cvc4c) => write!(f, "{}", cvc4c.cmd.join(" ")),
            _ => write!(f, "No Solver Used"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RSolve {
    stdout: String,
    stderr: String,
    lines: Vec<ResultLine>,
    pub solver: Solver,
}

impl fmt::Display for RSolve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: \n\tstdout:\n {} \n\t stderr:\n {}",
            self.solver, self.stdout, self.stderr
        )
    }
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

    fn propogate_diff(a: &Vec<ResultLine>, b: &Vec<ResultLine>) -> Result<Vec<ResultLine>, ()> {
        a.iter()
            .zip(b.iter())
            .map(|pair| match pair {
                (ResultLine::Sat, ResultLine::Unsat) | (ResultLine::Unsat, ResultLine::Sat) => {
                    Err(())
                }
                (ResultLine::Sat, _) | (_, ResultLine::Sat) => Ok(ResultLine::Sat),
                (ResultLine::Unsat, _) | (_, ResultLine::Unsat) => Ok(ResultLine::Unsat),
                _ => Ok(ResultLine::Unknown),
            })
            .collect()
    }

    pub fn differential_test(results: &Vec<Self>) -> Result<Vec<ResultLine>, ()> {
        let is_out_result = |l: &&ResultLine| match l {
            ResultLine::Sat | ResultLine::Unsat | ResultLine::Unknown => true,
            _ => false,
        };

        let propogated = results
            .iter()
            .filter(|r| !(r.has_unrecoverable_error() || r.was_timeout()))
            .map(|r| {
                r.lines
                    .iter()
                    .filter(|l| is_out_result(&l))
                    .map(|l| l.clone()) // TODO
                    .collect::<Vec<ResultLine>>()
            })
            .fold(None, |acc, lines| match acc {
                None => Some(Ok(lines)),
                Some(r_acc_lines) => {
                    Some(r_acc_lines.and_then(|acc_lines| Self::propogate_diff(&acc_lines, &lines)))
                }
            });

        match propogated {
            Some(r) => r,
            None => Ok(vec![]),
        }
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

pub fn solve_cvc4(cvc4_cmd: &CVC4_Command_Builder, target: &Path) -> RSolve {
    let cvc4_res = cvc4_cmd.run_on(target);
    let cvc4mrs = cvc4_res.map(|out| {
        let stderr = from_utf8(&out.stderr[..]).unwrap_or("");
        let stdout = from_utf8(&out.stdout[..]).unwrap_or("");
        let success = out.status.success();
        (success, stderr.to_owned(), stdout.to_owned())
    });

    match cvc4mrs {
        Ok((cvc4_succ, cvc4_out, cvc4_err)) => {
            RSolve::move_new(Solver::CVC4(cvc4_cmd.clone()), cvc4_out, cvc4_err)
        }
        Err(_) => RSolve::process_error(),
    }
}

pub fn solve_z3(z3_command: &Z3_Command_Builder, target: &Path) -> RSolve {
    let z3_res = z3_command.run_on(target);

    let z3mrs = z3_res.map(|out| {
        let stderr = from_utf8(&out.stderr[..]).unwrap_or("");
        let stdout = from_utf8(&out.stdout[..]).unwrap_or("");
        let success = out.status.success();
        (success, stderr.to_owned(), stdout.to_owned())
    });

    match z3mrs {
        Ok((z3_succ, z3_out, z3_err)) => {
            RSolve::move_new(Solver::Z3(z3_command.clone()), z3_out, z3_err)
        }
        Err(_) => RSolve::process_error(),
    }
}
pub fn solve(filename: &str) -> Vec<RSolve> {
    let filepath = Path::new(filename);

    let mr_cvc4 = CVC4_PROFILES
        .iter()
        .map(|profile| solve_cvc4(profile, &filepath));

    let mr_z3 = Z3_PROFILES
        .iter()
        .map(|profile| solve_z3(profile, &filepath));

    mr_cvc4.chain(mr_z3).collect()
}

pub fn check_valid_solve(filename: &str) -> Vec<RSolve> {
    let filepath = Path::new(filename);
    vec![
        solve_z3(
            &Z3_Command_Builder::new().timeout(Duration::from_secs(1)),
            filepath,
        ),
        solve_cvc4(
            &CVC4_Command_Builder::new()
                .timeout(Duration::from_secs(1))
                .incremental(),
            filepath,
        ),
    ]
}

pub fn check_valid_solve_as_temp(script: &Script) -> Result<Vec<RSolve>, String> {
    let s = script.to_string();
    let mut tfile = Builder::new()
        .suffix(".smt2")
        .tempfile()
        .map_err(|e| format!("Temp File Error: {}", e))?;
    tfile
        .as_file_mut()
        .write_all(s.as_bytes())
        .map_err(|e| format!("Temp File Write Error: {}", e))?;
    let filepath = tfile.path();

    let results = vec![
        solve_z3(
            &Z3_Command_Builder::new().timeout(Duration::from_secs(1)),
            filepath,
        ),
        solve_cvc4(
            &CVC4_Command_Builder::new()
                .timeout(Duration::from_secs(1))
                .incremental(),
            filepath,
        ),
    ];

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use walkdir::WalkDir;

    #[test]
    fn tempfile_solve_snap() {
        let s = script("(assert (exists ((ah Real)) (= ah 4)))(check-sat)")
            .unwrap()
            .1;
        assert!(check_valid_solve_as_temp(&s).is_ok());
    }

    #[test]
    fn propagate_snap() {
        assert_debug_snapshot!(RSolve::propogate_diff(
            &vec![ResultLine::Unsat],
            &vec![ResultLine::Unsat]
        ));
    }

    #[test]
    fn run_real_cvc4_snap() {
        assert_debug_snapshot!(CVC4_Command_Builder::new()
            .timeout(Duration::from_secs(3))
            .strings_exp()
            .incremental()
            .dump_all()
            .check_unsat_cores()
            .run_on(Path::new("test/strings20.smt2")));
    }

    #[test]
    fn run_real_z3_snap() {
        assert_debug_snapshot!(Z3_Command_Builder::new()
            .timeout(Duration::from_secs(3))
            .ematching(false)
            .flat_rw(false)
            .threads3()
            .z3str3()
            .run_on(Path::new("test/strings20.smt2")));
    }

    #[test]
    fn Z3_cmd_snap() {
        assert_debug_snapshot!(
            Z3_Command_Builder::new()
                .timeout(Duration::from_secs(3))
                .ematching(false)
                .flat_rw(false)
                .threads3()
                .z3str3()
                .cmd
        );
    }

    #[test]
    fn CVC4_cmd_snap() {
        assert_debug_snapshot!(
            CVC4_Command_Builder::new()
                .timeout(Duration::from_secs(3))
                .models()
                .unconstrained_simp()
                .incremental()
                .strings_exp()
                .dump_all()
                .check_unsat_cores()
                .cmd
        );
    }
    #[test]
    fn unsupp_snap() {
        let stdout="script_skel_segfaults2_minimal.smt2:2.14: No set-logic command was given before this point.
            script_skel_segfaults2_minimal.smt2:2.14: CVC4 will make all theories available.
            script_skel_segfaults2_minimal.smt2:2.14: Consider setting a stricter logic for (likely) better performance.
            script_skel_segfaults2_minimal.smt2:2.14: To suppress this warning in the future use (set-logic ALL).";
        let  stderr= "unsupported
                unsupported
            (error \"Parse Error: script_skel_segfaults2_minimal.smt2:6.65: Symbol = is not declared.

              ...(Array Int Int))) (! (= s (ff s)) :pattern (= s (ff s)))))
                                                 ^
            \")";
        let r = RSolve::new(Solver::NONE, stdout, stderr);
        assert!(!r.has_bug_error());
    }

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
        let r = RSolve::new(Solver::NONE, rstr, "");
        assert_debug_snapshot!(r.extract_const_var_vals(&vec![
            "GEN1".to_owned(),
            "GEN2".to_owned(),
            "BAV3".to_owned()
        ]));
    }

    #[test]
    fn z3_new() {
        let rstring = "sat (model (define-fun b () Int 0) (define-fun a () Int 1))";
        let r = RSolve::new(Solver::NONE, "", rstring);
        assert!(r.has_sat());
        assert!(!r.has_unrecoverable_error());
        assert!(!r.has_bug_error());
    }

    #[test]
    fn diff_self() {
        let rstring = "sat (model (define-fun b () Int 0) (define-fun a () Int 1))";
        let r1 = RSolve::new(Solver::NONE, "", rstring);
        let r2 = RSolve::new(Solver::NONE, rstring, "");
        assert!(RSolve::differential_test(&vec![r1, r2]).is_ok());
    }

    #[test]
    fn diff_difft() {
        let rstring = "sat (model (define-fun b () Int 0) (define-fun a () Int 1))";
        let r1 = RSolve::new(Solver::NONE, "", rstring);
        let r2 = RSolve::new(Solver::NONE, "unsat", "");
        assert!(!RSolve::differential_test(&vec![r1, r2]).is_ok());
    }

    #[test]
    fn diff_difft_same_multiple() {
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
        let r1 = RSolve::new(Solver::NONE, r1str, "");
        let r2 = RSolve::new(Solver::NONE, r2str, "");
        assert!(RSolve::differential_test(&vec![r1, r2]).is_ok());
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
        let r = RSolve::new(Solver::NONE, rstr, "");
        assert!(!r.has_unrecoverable_error());
    }

    #[test]
    fn cvc4_timeout() {
        let rstr = "timeout: sending signal TERM to command ‘cvc4’\nCVC4 interrupted by SIGTERM.\n timeout: the monitored command dumped core";
        let r = RSolve::new(Solver::NONE, rstr, "");
        assert!(!r.has_bug_error());
    }

    #[test]
    fn cvc4_unimpl_not_bug() {
        let rstr = "Fatal failure within CVC4::Node CVC4::theory::fp::FpConverter::convert(CVC4::TNode) at /home/dylan/git/constant-swap/scripts/.solvers/cvc4/src/theory/fp/fp_converter.cpp:1700
        Unimplemented code encounteredConversion is dependent on SymFPU
        timeout: the monitored command dumped core";
        let r = RSolve::new(Solver::NONE, rstr, "");
        assert!(!r.has_bug_error());
    }

    #[test]
    fn difft_diff_w_error() {
        let rstr1 =
            "(error \"line 11 column 79: Sort mismatch between first argument and argument 2\")
                sat";
        let rstr2 = "unsat";
        let r1 = RSolve::new(Solver::NONE, rstr1, "");
        let r2 = RSolve::new(Solver::NONE, rstr2, "");
        assert!(RSolve::differential_test(&vec![r1, r2]).is_ok());
    }

    #[test]
    fn fatal_error() {
        let rstr = "unsat \n (error \"something went wrong\")";
        let r = RSolve::new(Solver::NONE, rstr, "");
        assert!(r.has_unrecoverable_error());
    }
}
