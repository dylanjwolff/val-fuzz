(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const GEN10 Int)
(declare-const GEN9 Int)
(declare-const GEN8 Int)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(assert (forall ((x Int)(y Int)(a Real)(z Int)) (or (> x (+ a (* (/ GEN1 GEN2) y) (* (/ GEN3 GEN4) z))) (< x (+ GEN5 (* GEN6 a) (* (/ GEN7 GEN8) y) (* (/ GEN9 GEN10) z))))))
(assert (= BAV1 (forall ((x Int)(y Int)(a Real)(z Int)) (or (> x (+ a (* (/ GEN1 GEN2) y) (* (/ GEN3 GEN4) z))) (< x (+ GEN5 (* GEN6 a) (* (/ GEN7 GEN8) y) (* (/ GEN9 GEN10) z)))))))
(assert (= BAV2 (forall ((x Int)(y Int)(a Real)(z Int)) (> x (+ a (* (/ GEN1 GEN2) y) (* (/ GEN3 GEN4) z))))))
(assert (= BAV3 (forall ((x Int)(y Int)(a Real)(z Int)) (< x (+ GEN5 (* GEN6 a) (* (/ GEN7 GEN8) y) (* (/ GEN9 GEN10) z))))))
(check-sat)
(get-model)