(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun x () Int)
(declare-fun y () Int)
(declare-fun z () Int)
(declare-fun w () Int)
(declare-fun u () Int)
(declare-fun v () Int)
(declare-fun p (Int Int) Int)
(declare-fun A () (Set Int))
(assert (or (> (+ x y z) GEN1) (< (p x (+ (* GEN2 y) (* GEN3 z))) GEN4)))
(assert (subset A (insert y (singleton z))))
(assert (= BAV1 (or (> (+ x y z) GEN1) (< (p x (+ (* GEN2 y) (* GEN3 z))) GEN4))))
(assert (= BAV2 (> (+ x y z) GEN1)))
(assert (= BAV3 (< (p x (+ (* GEN2 y) (* GEN3 z))) GEN4)))
(check-sat)
(get-model)