(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-fun x () Int)
(declare-fun y () Int)
(declare-fun z () Int)
(declare-fun a () (Set Int))
(declare-fun b () (Set Int))
(assert (member x (union a b)))
(assert (not (member y a)))
(assert (not (member z b)))
(assert (= z y))
(assert (= x y))
(assert (= BAV1 (= z y)))
(assert (= BAV2 (= x y)))
(check-sat)
(get-model)