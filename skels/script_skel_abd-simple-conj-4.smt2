(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const GEN1 Int)
(set-option :produce-abducts true)
(declare-fun x () Int)
(declare-fun y () Int)
(declare-fun z () Int)
(declare-fun w () Int)
(declare-fun u () Int)
(declare-fun v () Int)
(assert (>= x GEN1))
(get-abduct A (>= (+ x y z w u v) 2))
(assert (= BAV1 (>= x GEN1)))