(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(set-option :auto-config true)
(set-option :produce-models true)
(declare-sort A)
(declare-const x A)
(declare-const y A)
(declare-fun f (A) A)
(assert (= (f (f x)) x))
(assert (= (f x) y))
(assert (not (= x y)))
(assert (= BAV1 (= (f (f x)) x)))
(assert (= BAV2 (= (f x) y)))
(assert (= BAV3 (= x y)))
(check-sat)
(get-model)