(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(define-sort Elt () Int)
(define-sort mySet () (Set Elt))
(declare-fun f (Int) mySet)
(declare-fun x () Int)
(declare-fun y () Int)
(declare-fun S () mySet)
(declare-fun T () mySet)
(assert (= (f x) (union S T)))
(assert (= (f x) (union T (f y))))
(assert (not (= (f y) (union T (f y)))))
(assert (= BAV1 (= (f x) (union S T))))
(assert (= BAV2 (= (f x) (union T (f y)))))
(assert (= BAV3 (= (f y) (union T (f y)))))
(check-sat)