(set-logic ALL)
(declare-const GEN1 String)
(declare-const GEN2 String)
(declare-const GEN3 String)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-fun x () String)
(declare-fun y () String)
(declare-fun z () String)
(assert (= GEN1 x))
(assert (= GEN2 y))
(assert (= GEN3 z))
(assert (= x z))
(assert (= BAV4 (= GEN1 x)))
(assert (= BAV5 (= GEN2 y)))
(assert (= BAV6 (= GEN3 z)))
(assert (= BAV7 (= x z)))
(check-sat)