(set-info :source "Handcrafted by C.M. Wintersteiger")
(declare-fun X () (_ FloatingPoint 11 53))
(assert (fp.eq X X))
(check-sat)
(check-sat-using smt)
(exit)