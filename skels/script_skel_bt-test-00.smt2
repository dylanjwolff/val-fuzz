(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(set-info :smt-lib-version 2.0)
(set-info :category "crafted")
(declare-fun f (Bool) Bool)
(declare-fun g (Bool) Bool)
(declare-fun h (Bool) Bool)
(declare-fun x () Bool)
(declare-fun y () Bool)
(declare-fun z () Bool)
(assert (not (= (f x) (f y))))
(assert (not (= (g y) (g z))))
(assert (not (= (h z) (h x))))
(assert (= BAV1 (= (f x) (f y))))
(assert (= BAV2 (= (g y) (g z))))
(assert (= BAV3 (= (h z) (h x))))
(check-sat)
(exit)