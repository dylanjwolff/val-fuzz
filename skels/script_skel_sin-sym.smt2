(set-logic ALL)
(declare-const GEN1 Real)
(declare-const GEN2 Real)
(declare-const GEN3 Real)
(declare-const BAV4 Bool)
(declare-fun x () Real)
(assert (not (= (+ (sin GEN1) (sin (- GEN2))) GEN3)))
(assert (= BAV4 (= (+ (sin GEN1) (sin (- GEN2))) GEN3)))
(check-sat)