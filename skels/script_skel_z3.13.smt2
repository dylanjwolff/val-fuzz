(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(get-model)
(declare-const GEN1 Int)
(set-option :auto-config true)
(set-option :produce-models true)
(declare-datatypes (T1 T2) ((Pair (mk-pair (first T1) (second T2)))))
(declare-const p1 (Pair Int Int))
(declare-const p2 (Pair Int Int))
(assert (= p1 p2))
(assert (> (second p1) GEN1))
(assert (= BAV1 (= p1 p2)))
(assert (= BAV2 (> (second p1) GEN1)))
(assert (= BAV3 (= (first p1) (first p2))))
(check-sat)
(get-model)
(assert (not (= (first p1) (first p2))))
(check-sat)