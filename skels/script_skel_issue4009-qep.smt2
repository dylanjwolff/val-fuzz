(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun a () Real)
(declare-fun b () Real)
(assert (forall ((c Real)) (and (xor (> c a) (= b GEN1)) (distinct (+ a b) GEN2))))
(assert (= BAV1 (forall ((c Real)) (and (xor (> c a) (= b GEN1)) (distinct (+ a b) GEN2)))))
(assert (= BAV2 (forall ((c Real)) (xor (> c a) (= b GEN1)))))
(assert (= BAV3 (forall ((c Real)) (> c a))))
(assert (= BAV4 (forall ((c Real)) (= b GEN1))))
(assert (= BAV5 (forall ((c Real)) (distinct (+ a b) GEN2))))
(check-sat)
(get-model)