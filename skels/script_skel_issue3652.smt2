(set-logic ALL)
(declare-const GEN1 Int)
(declare-const BAV2 Bool)
(declare-fun a () Real)
(assert (= (* a a) GEN1))
(assert (= BAV2 (= (* a a) GEN1)))
(check-sat)