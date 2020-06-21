




(set-option :smt.arith.solver 4)
(set-option :smt.phase_selection 5)
(set-option :rewriter.eq2ineq true)
(declare-fun a () Int)
(declare-fun b () Int)
(declare-fun c () Int)
(declare-fun d () Int)
(assert (= 1 c 1))
(assert (or (= d 1) (= d 0)))
(assert (= (div b a) (+ d c) 1))
(check-sat)
