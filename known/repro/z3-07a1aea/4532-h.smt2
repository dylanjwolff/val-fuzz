(declare-fun a () Real)
(declare-fun b () Real)
(assert (= (* (* 2 a) (* (- 1 b) b)) 1))
(push)
(check-sat)
(get-model)