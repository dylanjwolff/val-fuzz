(set-logic ALL)
(declare-const GEN1 Int)
(define-funs-rec (
(f () Int)
(pred ((y Int)) Bool)) (
0
(ite (< y 0) false (ite (= y 0) true (pred (- y 2))))
))
(assert (pred GEN1))
(check-sat)
(get-model)