(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-sort I 0)
(declare-fun a () I)
(declare-fun b () I)
(declare-fun c () I)
(declare-fun f (I) I)
(assert (not (= (f a) (f b))))
(assert (not (= (f a) (f c))))
(assert (not (and (not (= a b)) (not (= a c)))))
(assert (= BAV1 (= (f a) (f b))))
(assert (= BAV2 (= (f a) (f c))))
(assert (= BAV3 (and (not (= a b)) (not (= a c)))))
(assert (= BAV4 (= a b)))
(assert (= BAV5 (= a c)))
(check-sat)