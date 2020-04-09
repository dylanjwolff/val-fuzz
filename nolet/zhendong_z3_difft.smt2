(set-option :smt.arith.solver 6)
(set-option :rewriter.push_to_real false)
(declare-fun a () Int)
(declare-fun b () Int)
(declare-fun c () Int)
(assert (forall ((d Int)) (= (/ 1 (- 2 d (ite (= d a) b c))) 0)))
(check-sat)

