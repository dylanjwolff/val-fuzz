(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const BAV3 Bool)
(define-fun-rec sumr ((x Int)) Int 
    (+ x (ite (> x 0) (sumr (- x 1)) 0)))
(assert (= (sumr GEN1) GEN2))
(assert (= BAV3 (= (sumr GEN1) GEN2)))
(check-sat)