(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const GEN9 Real)
(declare-const GEN8 Real)
(declare-const GEN7 Real)
(declare-const GEN6 Real)
(declare-const GEN5 Real)
(declare-const GEN4 Real)
(declare-const GEN3 Real)
(declare-const GEN2 Real)
(declare-const GEN1 Real)
(set-option :auto-config true)
(declare-const x Real)
(assert (>= x GEN1))
(assert (not (= (^ (* GEN2 x) (/ GEN3 GEN4)) (* (^ GEN5 (/ GEN6 GEN7)) (^ x (/ GEN8 GEN9))))))
(assert (= BAV1 (>= x GEN1)))
(assert (= BAV2 (= (^ (* GEN2 x) (/ GEN3 GEN4)) (* (^ GEN5 (/ GEN6 GEN7)) (^ x (/ GEN8 GEN9))))))
(check-sat)
(get-model)