(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-fun x () Int)
(declare-fun y () Int)
(declare-fun z () Int)
(declare-fun f (Int) Int)
(assert (= (f x) GEN1))
(assert (or (> z GEN2) (> y GEN3)))
(assert (= BAV4 (= (f x) GEN1)))
(assert (= BAV5 (or (> z GEN2) (> y GEN3))))
(assert (= BAV6 (> z GEN2)))
(assert (= BAV7 (> y GEN3)))
(check-sat)