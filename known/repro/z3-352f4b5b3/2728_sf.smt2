(declare-fun a () String)
(assert (and and (>= (- (str.indexof a "x" 2) 3) 0) (>= (str.indexof a "y" 0) 0)))
(check-sat)
