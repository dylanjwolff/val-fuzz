(declare-fun a () Int)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-fun b () Int)
(declare-fun z () Int)
(assert (= a z))
(assert (not (= (* a b) (* z b))))
(assert (= BAV1 (= a z)))
(assert (= BAV2 (= (* a b) (* z b))))
(check-sat)