(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-fun c () Real)
(assert (forall ((x Real)) (or (<= x GEN1) (>= x (+ c GEN2)))))
(assert (= BAV3 (forall ((x Real)) (or (<= x GEN1) (>= x (+ c GEN2))))))
(assert (= BAV4 (forall ((x Real)) (<= x GEN1))))
(assert (= BAV5 (forall ((x Real)) (>= x (+ c GEN2)))))
(check-sat)