(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-fun x () Real)
(declare-fun y () Real)
(declare-fun z () Real)
(assert (forall ((t Real)) (or (distinct x y t) (distinct x z t) (distinct y z t))))
(assert (= BAV1 (forall ((t Real)) (or (distinct x y t) (distinct x z t) (distinct y z t)))))
(assert (= BAV2 (forall ((t Real)) (distinct x y t))))
(assert (= BAV3 (forall ((t Real)) (distinct x z t))))
(assert (= BAV4 (forall ((t Real)) (distinct y z t))))
(check-sat)
(get-model)