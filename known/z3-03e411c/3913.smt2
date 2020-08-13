(define-funs-rec ((f ((a Int)) Int) (g ((b Int)) Int))
 ((ite (= a 0) 1 (ite (= (g (- a 1)) 0) 1 0)) (f (- b 1))))
(assert (= (f 4) 0))
(check-sat)
