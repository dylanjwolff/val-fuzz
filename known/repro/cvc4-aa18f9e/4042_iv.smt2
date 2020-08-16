(set-logic NRA)
(set-option :check-models true)
(set-option :solve-real-as-int true)
(assert (forall ((q2 Real) (q3 Bool) (q4 Bool) (q5 Real)) (not (= q2 7.22035834))))
(check-sat)
(exit)

