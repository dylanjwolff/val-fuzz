(set-info :source |fuzzsmt|)
(set-info :smt-lib-version 2.0)
(set-info :category "random")
(set-info :status unknown)
(set-logic ALL)
(declare-const GEN1 (_ BitVec 7))
(declare-const BAV2 Bool)
(declare-fun v1 () (_ BitVec 7))
(assert (not (= (bvsub (bvnot (bvadd v1 v1)) (bvadd v1 v1)) GEN1)))
(assert (= BAV2 (= (bvsub (bvnot (bvadd v1 v1)) (bvadd v1 v1)) GEN1)))
(check-sat)
(get-model)