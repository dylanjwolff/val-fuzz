(set-logic ALL)
(declare-const BAV1 Bool)
(declare-sort elem 0)
(declare-datatypes ((list 0)) (((Nil) (Cons (hd elem) (tl list)))))
(define-fun-rec f ((x list)) elem
  (ite ((_ is Nil) x) (f x) (hd x))
)
(declare-fun t () elem)
(assert (= t (f Nil)))
(assert (= BAV1 (= t (f Nil))))
(check-sat)