(set-logic ALL)
(declare-const BAV1 Bool)
(set-info :smt-lib-version 2.0)
(set-info :category "crafted")
(declare-fun v0 () (_ BitVec 4))
(declare-fun f ((_ BitVec 4)) (_ BitVec 4))
(declare-fun g ((_ BitVec 4)) (_ BitVec 4))
(assert (= (f (f v0)) (g (f v0))))
(assert (= BAV1 (= (f (f v0)) (g (f v0)))))
(check-sat)
(exit)