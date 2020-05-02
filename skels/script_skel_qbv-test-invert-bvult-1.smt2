(set-logic ALL)
(declare-fun a () (_ BitVec 8))
(assert (forall ((x (_ BitVec 8))) (not (bvult a x))))
(check-sat)