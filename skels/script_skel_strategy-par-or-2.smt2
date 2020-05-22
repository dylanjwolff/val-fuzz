(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(set-info :source |Written by D. B. Staple for GitHub issue #625.|)
(declare-fun x () Int)
(declare-fun y () Int)
(assert (= y (abs x)))
(assert (distinct y (abs x)))
(check-sat-using (par-or
    smt smt smt smt smt
    smt smt smt smt smt
    smt smt smt smt smt
))
(assert (= BAV1 (= y (abs x))))
(assert (= BAV2 (distinct y (abs x))))