(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-fun round2 (Real) Int)
(assert (forall ((x Real)(i Int)) (=> (<= x (to_real i)) (<= (round2 x) i))))
(assert (forall ((x Real)(i Int)) (=> (<= (to_real i) x) (<= i (round2 x)))))
(assert (= BAV1 (forall ((x Real)(i Int)) (=> (<= x (to_real i)) (<= (round2 x) i)))))
(assert (= BAV2 (forall ((x Real)(i Int)) (<= x (to_real i)))))
(assert (= BAV3 (forall ((x Real)(i Int)) (<= (round2 x) i))))
(assert (= BAV4 (forall ((x Real)(i Int)) (=> (<= (to_real i) x) (<= i (round2 x))))))
(assert (= BAV5 (forall ((x Real)(i Int)) (<= (to_real i) x))))
(assert (= BAV6 (forall ((x Real)(i Int)) (<= i (round2 x)))))
(check-sat)
(get-model)