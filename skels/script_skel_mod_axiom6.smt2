(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-fun x () Int)
(declare-fun y () Int)
(assert (= x (+ y (* (- GEN1) (div x (- GEN2))))))
(assert (not (= y (mod x (- GEN3)))))
(assert (= BAV4 (= x (+ y (* (- GEN1) (div x (- GEN2)))))))
(assert (= BAV5 (= y (mod x (- GEN3)))))
(check-sat)
(get-model)