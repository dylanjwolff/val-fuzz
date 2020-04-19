use std::process;
use std::str::from_utf8;


pub fn solve(filename: &str) -> bool {
    let cvc4_res = process::Command::new("timeout")
        .args(&[
            "6s",
            "cvc4",
            "--incremental",
            "--produce-model",
            filename,
        ])
        .output();

    
    let z3_res = process::Command::new("timeout")
        .args(&["6s", "z3", "smt.string_solver=z3str3", "model_validate=true", filename ])
        .output();


    let cvc4mrs = cvc4_res.map(|out| {
        let stderr = from_utf8(&out.stderr[..]).unwrap_or("");
        let stdout = from_utf8(&out.stdout[..]).unwrap_or("");
        let success = out.status.success();
        (success, stderr.to_owned(), stdout.to_owned())
    });

    let z3mrs = z3_res.map(|out| {
        let stderr = from_utf8(&out.stderr[..]).unwrap_or("");
        let stdout = from_utf8(&out.stdout[..]).unwrap_or("");
        let success = out.status.success();
        (success, stderr.to_owned(), stdout.to_owned())
    });

    let mut rmv_file = false;
    match (cvc4mrs, z3mrs) {
        (Ok((cvc4_succ, cvc4_out, cvc4_err)), Ok((z3_succ, z3_out, z3_err))) => {
            // TODO This isn't correct for incremental formulas... nor is it a good way to parse
            // these responses... leaving for now until we have firmly proved the concept
            let z3_unsat = z3_out.contains("unsat") || z3_err.contains("unsat\n");
            let z3_sat = !z3_unsat && (z3_out.contains("sat") || z3_err.contains("sat\n"));
            let z3_unknown = !z3_unsat && !z3_sat && (z3_out.contains("unknown")
                                                      || z3_err.contains("unknown\n"));
            let cvc4_unsat = cvc4_out.contains("unsat") || cvc4_err.contains("unsat\n");
            let cvc4_sat = !cvc4_unsat && (cvc4_out.contains("sat") || cvc4_err.contains("sat\n"));
            let cvc4_unknown = !cvc4_unsat && !cvc4_sat && (cvc4_out.contains("unknown")
                                                      || cvc4_err.contains("unknown\n"));

            if z3_out.contains("ASSERTION VIOLATION") {
               println!("file {} has assertion violation problem!!!", filename);
            } else if z3_err.contains("invalid model") {
               println!("file {} has invalid model problem!!!", filename);
            } else if z3_out.contains("dumped core") || cvc4_out.contains("dumped core") {
               println!("file {} cause segfault!!!", filename);
            } else if !z3_succ && z3_err.len() > 0 && !z3_sat && !z3_unsat && !z3_unknown {
               println!("z3 unsuccessful on file {} : {}", filename, z3_err);
            } else if !cvc4_succ && cvc4_err.len() > 0 && !cvc4_sat && !cvc4_unsat && !cvc4_unknown {
               println!("cvc4 unsuccessful on file {} : {}", filename, cvc4_err);
            } else if z3_unknown || cvc4_unknown {
               println!("unknown result for file {}", filename);
               rmv_file = true;
            } else if cvc4_sat && z3_unsat {
               println!("file {} has soundness problem!!!", filename);
            } else if cvc4_unsat && z3_sat && !z3_err.contains("unknown function/constant")
                && !z3_err.contains("unknown constant")
                && !z3_err.contains("unknown parameter") {
               println!("file {} has soundness problem!!!", filename);
            } else if cvc4_out.contains("timeout") || z3_out.contains("timeout") {
                // TODO look for timeouts in stderr in robust manner (shouldn't happen, but good to
                // be safe). Can't because some error messages have timeout in them that aren't
                // timeouts
               println!("timeout on file {}", filename);
            } else {
               rmv_file = true;
            }

            if cvc4_succ && z3_succ {
                println!("parse success for file :{}", filename);
            }


        },
        (Err(e), _) => println!("cvc4 process error on file {} : {}", filename, e),
        (_, Err(e)) => println!("z3 process error on file {} : {}", filename, e),
    };
    return rmv_file;
}

#[cfg(test)]
mod tests {
    use super::*;
    use walkdir::WalkDir;

    const STACK_SIZE: usize = 20 * 1024 * 1024;

    #[test]
    fn solver_test() {
        for entry in WalkDir::new("known/8")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
                let filepath = entry.path();
                println!("starting file {:?}", filepath);
                solve(filepath.to_str().unwrap());
        }
    }

}
