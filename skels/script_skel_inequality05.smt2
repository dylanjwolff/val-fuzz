(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(set-info :smt-lib-version 2.0)
(set-info :category "crafted")
(declare-fun v0 () (_ BitVec 16))
(declare-fun v1 () (_ BitVec 16))
(declare-fun v2 () (_ BitVec 16))
(declare-fun v3 () (_ BitVec 16))
(declare-fun v4 () (_ BitVec 16))
(declare-fun v5 () (_ BitVec 16))
(declare-fun v6 () (_ BitVec 16))
(declare-fun v7 () (_ BitVec 16))
(declare-fun v8 () (_ BitVec 16))
(declare-fun v9 () (_ BitVec 16))
(assert (and (bvult v6 v5) (bvule v7 v8) (bvugt v7 v1) (bvuge v4 v1) (bvuge v8 v0) (bvugt v1 v0) (bvuge v1 (_ bv60094 16)) (bvule v3 v0) (bvuge (_ bv47327 16) v6) (bvugt v3 v6)))
(assert (= BAV1 (and (bvult v6 v5) (bvule v7 v8) (bvugt v7 v1) (bvuge v4 v1) (bvuge v8 v0) (bvugt v1 v0) (bvuge v1 (_ bv60094 16)) (bvule v3 v0) (bvuge (_ bv47327 16) v6) (bvugt v3 v6))))
(check-sat)
(get-model)
(exit)