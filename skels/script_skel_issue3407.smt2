(set-logic ALL)
(declare-const GEN1 Int)
(declare-const BAV2 Bool)
(declare-fun a () Real)
(declare-fun b () Real)
(assert (= (* a b) GEN1))
(assert (= BAV2 (= (* a b) GEN1)))
(check-sat)