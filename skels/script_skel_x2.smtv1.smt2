(set-option :incremental false)
(set-info :status unknown)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-sort Index 0)
(declare-sort Element 0)
(declare-fun a () Index)
(declare-fun S () (Array Index Element))
(declare-fun SS () (Array Index Element))
(assert (= BAV1 (= S SS)))
(assert (= BAV2 (= S (store SS a (select S a)))))
(check-sat-assuming ((not (ite (= S SS) (= S (store SS a (select S a))) true))))
(get-model)