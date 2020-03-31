(set-logic ALL)
(declare-const x Int)
(declare-const y Int)
(assert (and (= (+ x y) 4) (< x (- 1))))
(check-sat)
(get-model)

