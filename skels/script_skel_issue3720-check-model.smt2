(set-logic ALL)
(assert (_ emp Int Int))
(check-sat)
(get-model)