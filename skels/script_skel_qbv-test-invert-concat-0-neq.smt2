(set-logic ALL)
(declare-const BAV1 Bool)
(declare-fun a () (_ BitVec 8))
(declare-fun b () (_ BitVec 16))
(assert (forall ((x (_ BitVec 8))) (= (concat x a) b)))
(assert (= BAV1 (forall ((x (_ BitVec 8))) (= (concat x a) b))))
(check-sat)