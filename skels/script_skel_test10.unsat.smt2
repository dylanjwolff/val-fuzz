(set-logic ALL)
(declare-const BAV1 Bool)
(declare-fun x () Real)
(declare-fun y () Real)
(declare-fun z () Real)
(assert (forall ((t Real)) (distinct x y z t)))
(assert (= BAV1 (forall ((t Real)) (distinct x y z t))))
(check-sat)
(get-model)