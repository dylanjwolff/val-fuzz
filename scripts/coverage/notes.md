1. Compile solvers with "--coverage" flag in debug mode (important since otherwise optimizations etc. may affect the coverage measurement)

For Z3:
```
$CXXFLAGS="--coverage" LDFLAGS="--coverage" ./configure -d
```
... (regular compilation steps)

For CVC4:
```
$./configure.sh debug --coverage
```
... (regular compilation steps)

2. Measuring coverage increments of formula1.smt2 and mutant1.smt2

```
$cd /home/windomin/dylan_coverage/z3/build  # goto build folder of the solver   
$find . -name "*.gcda" -exec rm {} \;       # remove old coverage related files  
$./z3 formula1.smt2; ./z3 formula2.smt2     # execute z3 on all seeds  
$find . -name "*.gcda"  | wc -l             # now some gcda files should have been created
681
```

3. Generate an info file for the coverage summary later.
Both can take a long time (~7-15 minutes total)

```
$lcov -q -t 'result' -o ex_test.info -c -d --rc lcov_branch_coverage=1 . 
$lcov --summary --rc lcov_branch_coverage=1 ex_test.info
Reading tracefile ex_test.info
Summary coverage rate:
  lines......: 9.8% (27368 of 278295 lines)
  functions..: 16.8% (16355 of 97538 functions)
  branches...: 2.9% (24315 of 829859 branches)
```
4. Execute the mutants now. The additional lines, functions and branches will be added to the .gcda files
```
$ ./z3 mutant1.smt2; ./z3 mutant2.smt2 
$ lcov -q -t 'result' -o ex_test.info -c -d --rc lcov_branch_coverage=1 . 
Reading tracefile ex_test.info
Summary coverage rate:
  lines......: 9.8% (27368 of 278295 lines)
  functions..: 16.8% (16355 of 97538 functions)
  branches...: 2.9% (24316 of 829859 branches)
```
4.5 It seems that the mutants have only triggered an additional branch being covered. However, no additional lines and functions. Coverage increment of calling z3 without files ./z3 gives:
```
Summary coverage rate:
  lines......: 9.8% (27373 of 278295 lines)
  functions..: 16.8% (16356 of 97538 functions)
  branches...: 2.9% (24319 of 829859 branches)
```
i.e. a bunch of additional lines, one function and 3 additionally covered branches 
