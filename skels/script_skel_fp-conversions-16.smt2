(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const GEN6 (_ BitVec 52))
(declare-const GEN5 (_ BitVec 11))
(declare-const GEN4 (_ BitVec 1))
(declare-const GEN3 (_ BitVec 64))
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(set-info :source "Handcrafted by C.M. Wintersteiger")
(declare-fun X ()  (_ FloatingPoint 11 53))
(assert (= X ((_ to_fp 11 53) RTZ GEN3)))
(assert (= X (fp GEN4 GEN5 GEN6)))
(assert (= BAV1 (= X ((_ to_fp 11 53) RTZ GEN3))))
(assert (= BAV2 (= X (fp GEN4 GEN5 GEN6))))
(check-sat)
(get-model)
(check-sat-using smt)
(exit)