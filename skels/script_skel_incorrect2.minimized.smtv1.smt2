(set-option :incremental false)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-sort Index 0)
(declare-sort Element 0)
(declare-fun v3 () Index)
(declare-fun v4 () Index)
(declare-fun v2 () Index)
(assert (= BAV1 (and (xor true (= v4 v3)) (ite (distinct v4 (ite (distinct v2 v3) (ite (distinct v2 v3) v3 v4) v3)) false true))))
(assert (= BAV2 (xor true (= v4 v3))))
(assert (= BAV3 (= v4 v3)))
(assert (= BAV4 (distinct v4 (ite (distinct v2 v3) (ite (distinct v2 v3) v3 v4) v3))))
(assert (= BAV5 (distinct v2 v3)))
(assert (= BAV6 (distinct v2 v3)))
(check-sat-assuming ((and (xor true (= v4 v3)) (ite (distinct v4 (ite (distinct v2 v3) (ite (distinct v2 v3) v3 v4) v3)) false true))))
(get-model)