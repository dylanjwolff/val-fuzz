(set-logic ALL)
(assert (not (_ emp Int Int)))
(check-sat)
(get-model)