(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
(declare-const BAV4 Bool)
(declare-fun a () Real)
(declare-fun b () Real)
(assert (= (/ GEN1 (+ GEN2 (* a a b))) GEN3))
(assert (= BAV4 (= (/ GEN1 (+ GEN2 (* a a b))) GEN3)))
(check-sat)