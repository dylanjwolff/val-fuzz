(set-logic ALL)
(declare-const GEN1 (_ BitVec 8))
(declare-const GEN2 (_ BitVec 8))
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const p Bool)
(declare-const u (_ BitVec 8))
(declare-const v (_ BitVec 8))
(define-const t (_ BitVec 8) (ite p u v))
(assert (= t GEN1))
(push)
(assert (= t GEN2))
(assert (= BAV3 (= t GEN1)))
(assert (= BAV4 (= t GEN2)))
(check-sat)