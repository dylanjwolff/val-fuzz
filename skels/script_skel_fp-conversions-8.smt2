(set-info :source "Handcrafted by C.M. Wintersteiger")
(declare-const GEN1 Real)
(declare-const GEN2 Real)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-fun X () (_ FloatingPoint 11 53))
(declare-fun R () Real)
(assert (= R (fp.to_real X)))
(assert (<= R GEN1))
(assert (>= R GEN2))
(assert (= BAV3 (= R (fp.to_real X))))
(assert (= BAV4 (<= R GEN1)))
(assert (= BAV5 (>= R GEN2)))
(check-sat)
(check-sat-using smt)
(exit)