(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(set-option :smt.string_solver z3str3)
(declare-fun c0 () String)
(declare-fun c1 () String)
(get-model)
(declare-fun c2 () Int)
(assert (<= GEN1 (str.indexof c0 c1 c2)))
(assert (not (str.contains c0 c1)))
(assert (< c2 (str.len c0)))
(assert (= BAV1 (<= GEN1 (str.indexof c0 c1 c2))))
(assert (= BAV2 (< c2 (str.len c0))))
(assert (= BAV3 (<= 0 (str.indexof c0 c1 c2))))
(assert (= BAV4 (< c2 (str.len c0))))
(check-sat)
(get-model)
(reset)
(set-option :smt.string_solver seq)
(declare-fun c0 () String)
(declare-fun c1 () String)
(declare-fun c2 () Int)
(assert (<= 0 (str.indexof c0 c1 c2)))
(assert (not (str.contains c0 c1)))
(assert (< c2 (str.len c0)))
(check-sat)