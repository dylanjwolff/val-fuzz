(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const GEN1 Int)
(set-option :produce-abducts true)
(declare-fun x () Int)
(assert (> x GEN1))
(assert (= BAV1 (> x GEN1)))
(check-sat)
(get-model)