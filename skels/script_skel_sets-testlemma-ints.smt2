(set-logic ALL)
(declare-const BAV1 Bool)
(declare-fun x () (Set Int))
(declare-fun y () (Set Int))
(assert (not (= x y)))
(assert (= BAV1 (= x y)))
(check-sat)