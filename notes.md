@IDEAS

1. Feed model back in as constants
2. Record SAT UNSAT Timeout Error Bug results for small iterations ~100 
    Are we getting tons of UNSAT or tons of SAT?
        -> if formulae are consistently either, then maybe they are trivial
        -> Looks like mostly UNSAT, so (6) is likely a good option
3. Use abstract domains as additional model restrictions
4. Measure code coverage (!!!!)
5. Lazily make constants holes only when BAM is UNSAT for the current constants
6. For UNSAT, find minimal unsatisfiable set of BAM, forbid them from future iterations
7. Try converting to equisatisfiable CNF, can then focus on driving certain BAs to T or F

2nd May:
    Do (2) and maybe start on (1)?
    Alternatively, can do more manual testing...

@TODO 4th May
Weekly email
Nested Macros
parsing model
more manual

@TODO 7th May
[] Add model parsing to solver.rs
add (get-model) to scripts...
i.e. be able to automatically run the fuzzer and get a model for SAT formulas
(stretch): replug the model back to a second solver call
(stretch): determine why there were so many unsat results

@TODO 10th May
add (get-model) to scripts
do a medium run with the resubstitution
why so many unsat


@TODO 11th May
why so many unsat
1st draft of thesis proposal

@TODO 15th May
[x]NSolv (flags)
pick at code coverage
[x] remove false positives
[x] fix exponential backoff

@TODO 17th May
[] 50% Replacement Errors
or
[] Code Coverage

@TODO 18th May
[x] Check bugs from run
[] 50% Replacement Errors
or
[] error logging specifics

@TODO 20th July
[x] Parse all function/const definitions/declarations
[x] Get sort of var from declarations
[x] Change BAV Assign code to use these variables instead of expressions

@TODO 21st July
[x] Only do abstract domains for functions without args
[] Parse FP and Boolean Sorts (and maybe others?) properly
