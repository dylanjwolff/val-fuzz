(set-option :auto-config true)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
(declare-const GEN4 Int)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-const BAV8 Bool)
(set-option :produce-models true)
(declare-const a Int)
(declare-const b Int)
(declare-const c Int)
(declare-const d Real)
(declare-const e Real)
(assert (> a (+ b GEN1)))
(assert (= a (+ (* GEN2 c) GEN3)))
(assert (<= (+ c b) GEN4))
(assert (>= d e))
(assert (= BAV5 (> a (+ b GEN1))))
(assert (= BAV6 (= a (+ (* GEN2 c) GEN3))))
(assert (= BAV7 (<= (+ c b) GEN4)))
(assert (= BAV8 (>= d e)))
(check-sat)
(get-model)