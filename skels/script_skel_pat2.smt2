(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun p (Int) Bool)
(declare-const a Int)
(get-model)
(declare-const b Int)
(set-option :auto-config false)
(set-option :smt.mbqi false)
(assert (forall ((x Int)(y Int)) (! (p (+ x y GEN1)) :pattern (p (+ x y GEN2)))))
(assert (p (+ a GEN3)))
(check-sat)
(get-model)
(reset)
(set-option :auto-config true)
(declare-fun p (Int) Bool)
(declare-const a Int)
(declare-const b Int)
(assert (forall ((x Int)(y Int)) (! (p (+ x y 2)) :pattern (p (+ x y 2)))))
(assert (p (+ a 2)))
(check-sat)