(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const GEN6 Real)
(declare-const GEN5 Real)
(get-model)
(declare-const GEN4 Real)
(declare-const GEN3 Real)
(declare-const GEN2 Real)
(declare-const GEN1 Real)
(set-option :model-validate false)
(set-option :auto-config true)
(declare-const a Real)
(declare-const b Real)
(assert (not (= (/ a b) (/ a GEN1))))
(assert (<= b GEN2))
(assert (>= b GEN3))
(assert (= BAV1 (= (/ a b) (/ a GEN1))))
(assert (= BAV2 (<= b GEN2)))
(assert (= BAV3 (>= b GEN3)))
(assert (= BAV4 (= (/ a b) (/ a 0.))))
(assert (= BAV5 (<= b 0.)))
(assert (= BAV6 (>= b 0.)))
(check-sat)
(get-model)
(reset)
(set-option :combined_solver.ignore_solver1 true)
(declare-const a Real)
(declare-const b Real)
(assert (not (= (/ a b) (/ a 0.))))
(assert (<= b 0.))
(assert (>= b 0.))
(check-sat)