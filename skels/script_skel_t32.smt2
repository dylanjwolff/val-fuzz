(set-option :auto-config false)
(get-option :auto-config)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-fun v2 () Int)
(declare-fun v1 () Int)
(push 1)
(assert (= v2 v1))
(push 1)
(assert (not (= v2 v1)))
(assert (= BAV1 (= v2 v1)))
(assert (= BAV2 (= v2 v1)))
(assert (= BAV3 (= v2 v1)))
(assert (= BAV4 (= v2 v1)))
(assert (= BAV5 (= v2 v1)))
(assert (= BAV6 (= v2 v1)))
(check-sat)
(pop 1)
(pop 1)
(assert (not (= v2 v1)))
(check-sat)
(reset)
(set-option :auto-config true)
(get-option :auto-config)
(set-logic QF_UFLIA)
(declare-fun v2 () Int)
(declare-fun v1 () Int)
(push 1)
(assert (= v2 v1))
(push 1)
(assert (not (= v2 v1)))
(check-sat)
(pop 1)
(pop 1)
(assert (not (= v2 v1)))
(check-sat)