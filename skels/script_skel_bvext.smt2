(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(set-info :smt-lib-version 2.0)
(set-info :category "crafted")
(declare-fun v1 () (_ BitVec 10))
(declare-fun v2 () Int)
(declare-fun a2 (Int) (_ BitVec 1024))
(declare-fun v3 () (_ BitVec 1024))
(declare-fun v4 () (_ BitVec 1024))
(declare-fun v5 () (_ BitVec 1024))
(assert (not (= (a2 (ite (= ((_ extract 3 0) v1) ((_ extract 3 0) v3)) v2 GEN5)) (bvudiv (bvudiv v4 v5) (bvudiv (bvudiv v3 v4) (bvudiv v3 v5))))))
(assert (= BAV1 (= (a2 (ite (= ((_ extract 3 0) v1) ((_ extract 3 0) v3)) v2 GEN5)) (bvudiv (bvudiv v4 v5) (bvudiv (bvudiv v3 v4) (bvudiv v3 v5))))))
(assert (= BAV2 (= ((_ extract 3 0) v1) ((_ extract 3 0) v3))))
(check-sat)
(get-model)
(exit)