(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-fun f (Int) Int)
(declare-fun g (Int) Int)
(declare-fun a () Int)
(declare-fun b () Int)
(declare-fun P (Int) Int)
(declare-fun Q (Int) Int)
(assert (or (= (f a) GEN1) (= (g a) GEN2)))
(assert (= BAV3 (or (= (f a) GEN1) (= (g a) GEN2))))
(assert (= BAV4 (= (f a) GEN1)))
(assert (= BAV5 (= (g a) GEN2)))
(check-sat)