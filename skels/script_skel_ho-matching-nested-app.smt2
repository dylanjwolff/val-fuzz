(set-logic ALL)
(declare-const GEN1 Int)
(declare-sort U 0)
(declare-fun p (Int) Bool)
(declare-fun g (Int) Int)
(assert (not (p (g GEN1))))
(assert (forall ((f (-> Int Int))(y Int)) (p (f (f (f (f (f y))))))))
(check-sat)
(get-model)
(exit)