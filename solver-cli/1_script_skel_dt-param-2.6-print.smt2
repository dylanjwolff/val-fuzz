(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Real)
(declare-const BAV3 Bool)
(set-option :produce-models true)
(declare-datatypes ( (Pair 2) ) (
(par ( X Y ) ( (mkPair (first X) (second Y))))
))
(declare-fun x () (Pair Int Real))
(assert (= x (mkPair GEN1 GEN2)))
(assert (= BAV3 (= x (mkPair GEN1 GEN2))))
(assert (= BAV3 true))
(check-sat)
(get-model)