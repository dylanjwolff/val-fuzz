(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
(declare-const GEN4 Int)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-const BAV8 Bool)
(declare-fun x () Int)
(declare-fun y () Int)
(assert (and (<= (* GEN1 y) x) (< x (+ (* GEN2 y) GEN3)) (not (= y (div x GEN4)))))
(assert (= BAV5 (and (<= (* GEN1 y) x) (< x (+ (* GEN2 y) GEN3)) (not (= y (div x GEN4))))))
(assert (= BAV6 (<= (* GEN1 y) x)))
(assert (= BAV7 (< x (+ (* GEN2 y) GEN3))))
(assert (= BAV8 (= y (div x GEN4))))
(check-sat)
(get-model)