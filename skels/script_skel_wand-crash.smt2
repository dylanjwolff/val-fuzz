(set-logic ALL)
(assert (wand (_ emp Int Int) (_ emp Int Int)))
(check-sat)
(get-model)