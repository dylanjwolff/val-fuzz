(set-logic ALL)
(declare-const GEN1 Int)
(declare-const BAV2 Bool)
(define-fun-rec pow2 ((n Int) (p Int)) Bool (
	or
	(and (= n 0) (= p 1))
	(and (> n 0) (> p 1) (= 0 (mod p 2)) (pow2 (- n 1) (div p 2)))
))
(declare-const n Int)
(declare-const p Int)
(assert (= n GEN1))
(assert (pow2 n p))
(assert (= BAV2 (= n GEN1)))
(check-sat)