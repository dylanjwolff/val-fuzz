(set-logic ALL)
(declare-const GEN1 Int)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(set-info :smt-lib-version 2.0)
(declare-fun n () Int)
(declare-fun x () Int)
(assert (>= n GEN1))
(assert (< (mod x n) n))
(assert (= BAV2 (>= n GEN1)))
(assert (= BAV3 (< (mod x n) n)))
(check-sat)