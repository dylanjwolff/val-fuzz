(set-option :auto-config false)
(set-option :produce-unsat-cores true)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-fun p () Bool)
(declare-fun q () Bool)
(declare-fun r () Bool)
(declare-fun s () Bool)
(declare-fun t () Bool)
(assert (! (=> p q) :named PQ))
(assert (! (=> q r) :named QR))
(assert (! (=> r s) :named RS))
(assert (! (=> s t) :named ST))
(assert (! (not (=> q s)) :named NQS))
(assert (= BAV1 (=> p q)))
(assert (= BAV2 (=> q r)))
(assert (= BAV3 (=> r s)))
(assert (= BAV4 (=> s t)))
(assert (= BAV5 (=> q s)))
(check-sat)
(get-unsat-core)