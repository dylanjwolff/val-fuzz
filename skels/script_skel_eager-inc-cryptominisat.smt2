(set-logic ALL)
(set-option :incremental true)
(get-model)
(declare-fun a () (_ BitVec 16))
(get-model)
(declare-fun b () (_ BitVec 16))
(declare-fun c () (_ BitVec 16))
(assert (bvult a (bvadd b c)))
(check-sat)
(get-model)
(push 1)
(assert (bvult c b))
(check-sat)
(push 1)
(assert (bvugt c b))
(check-sat)
(pop 2)
(check-sat)
(exit)