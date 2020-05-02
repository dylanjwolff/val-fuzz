@IDEAS

1. Feed model back in as constants
2. Record SAT UNSAT Timeout Error Bug results for small iterations ~100 
    Are we getting tons of UNSAT or tons of SAT?
        -> if formulae are consistently either, then maybe they are trivial
3. Use abstract domains as additional model restrictions
4. Measure code coverage
5. Lazily make constants holes only when BAM is UNSAT for the current constants
6. For UNSAT, find minimal unsatisfiable set of BAM, forbid them from future iterations

2nd May:
    Do (2) and maybe start on (1)?
    Alternatively, can do more manual testing...
