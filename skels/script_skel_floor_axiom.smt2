(set-logic ALL)
(declare-const GEN1 Int)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-fun x () Real)
(declare-fun y () Int)
(assert (and (<= y x) (< x (+ GEN1 y)) (distinct y (to_int x))))
(assert (= BAV2 (and (<= y x) (< x (+ GEN1 y)) (distinct y (to_int x)))))
(assert (= BAV3 (<= y x)))
(assert (= BAV4 (< x (+ GEN1 y))))
(assert (= BAV5 (distinct y (to_int x))))
(check-sat)
(get-model)