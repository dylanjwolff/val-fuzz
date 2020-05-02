(set-logic ALL)
(declare-fun x () (_ FloatingPoint 3 5))
(assert (fp.isNegative (fp.abs (fp.neg x))))
(check-sat)