(set-option :incremental false)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const GEN5 Real)
(declare-const GEN4 Real)
(declare-const GEN3 Real)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun f1 (Real Real) Real)
(assert (= BAV1 (distinct (+ (/ GEN1 (- GEN2)) GEN3) (f1 GEN4 GEN5))))
(check-sat-assuming ((distinct (+ (/ GEN1 (- GEN2)) GEN3) (f1 GEN4 GEN5))))
(get-model)