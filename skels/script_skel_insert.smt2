(set-option :produce-models true)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun X () (Set Int))
(assert (= X (insert GEN1 GEN2 (singleton GEN3))))
(assert (= BAV1 (= X (insert GEN1 GEN2 (singleton GEN3)))))
(check-sat)
(get-model)