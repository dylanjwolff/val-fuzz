(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(set-option :incremental true)
(declare-fun P (Int) Bool)
(declare-fun ten () Int)
(assert (forall ((x Int)) (=> (<= GEN1 x ten) (P x))))
(push)
(assert (= ten GEN2))
(assert (= BAV1 (forall ((x Int)) (=> (<= GEN1 x ten) (P x)))))
(assert (= BAV2 (forall ((x Int)) (<= GEN1 x ten))))
(assert (= BAV3 (= ten GEN2)))
(check-sat)
(get-model)
(pop)