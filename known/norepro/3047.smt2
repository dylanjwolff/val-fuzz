(assert (forall ((a (_ BitVec 2)))
  (exists ((b (_ BitVec 2))) (and (= a #b00) (distinct a b)))))
(check-sat-using qe)
