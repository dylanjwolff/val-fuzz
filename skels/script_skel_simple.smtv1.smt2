(set-option :incremental false)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-sort U 0)
(declare-fun x0 () Bool)
(declare-fun x1 () Bool)
(declare-fun x2 () Bool)
(declare-fun x3 () Bool)
(assert (= BAV1 (and (or x1 (not x0)) (or x0 (not x3)) (or x3 x2) (not x1) x2 x3)))
(assert (= BAV2 (or x1 (not x0))))
(assert (= BAV3 (or x0 (not x3))))
(assert (= BAV4 (or x3 x2)))
(check-sat-assuming ((and (or x1 (not x0)) (or x0 (not x3)) (or x3 x2) (not x1) x2 x3)))
(get-model)