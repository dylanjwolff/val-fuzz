(set-logic ALL)
(declare-const GEN1 Int)
(declare-const BAV2 Bool)
(declare-fun v () Real)
(assert (= v GEN1))
(assert (= BAV2 (= v GEN1)))
(check-sat)
(exit)