(set-option :incremental false)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const GEN19 Int)
(declare-const GEN18 Int)
(declare-const GEN17 Int)
(declare-const GEN16 Int)
(declare-const GEN15 Int)
(declare-const GEN14 Int)
(declare-const GEN13 Int)
(declare-const GEN12 Int)
(declare-const GEN11 Int)
(declare-const GEN10 Int)
(declare-const GEN9 Int)
(declare-const GEN8 Int)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun v6 () (_ BitVec 32))
(declare-fun a1 () (Array (_ BitVec 32) (_ BitVec 8)))
(declare-fun v15 () (_ BitVec 32))
(assert (= BAV1 (= (_ bv0 1) (bvand (ite (= ((_ extract 1 0) v6) (_ bv0 2)) (_ bv1 1) (_ bv0 1)) (bvnot (ite (= (store (store (store a1 v15 (_ bv0 8)) (bvadd (_ bv1 32) v15) (_ bv0 8)) (bvadd (_ bv3 32) v15) (_ bv0 8)) (store (store (store a1 v6 ((_ extract 7 0) v6)) (bvadd (_ bv1 32) v6) (_ bv1 8)) (bvadd (_ bv3 32) v6) (_ bv1 8))) (_ bv1 1) (_ bv0 1)))))))
(assert (= BAV2 (= ((_ extract 1 0) v6) (_ bv0 2))))
(assert (= BAV3 (= (store (store (store a1 v15 (_ bv0 8)) (bvadd (_ bv1 32) v15) (_ bv0 8)) (bvadd (_ bv3 32) v15) (_ bv0 8)) (store (store (store a1 v6 ((_ extract 7 0) v6)) (bvadd (_ bv1 32) v6) (_ bv1 8)) (bvadd (_ bv3 32) v6) (_ bv1 8)))))
(check-sat-assuming ((not (= (_ bv0 1) (bvand (ite (= ((_ extract 1 0) v6) (_ bv0 2)) (_ bv1 1) (_ bv0 1)) (bvnot (ite (= (store (store (store a1 v15 (_ bv0 8)) (bvadd (_ bv1 32) v15) (_ bv0 8)) (bvadd (_ bv3 32) v15) (_ bv0 8)) (store (store (store a1 v6 ((_ extract 7 0) v6)) (bvadd (_ bv1 32) v6) (_ bv1 8)) (bvadd (_ bv3 32) v6) (_ bv1 8))) (_ bv1 1) (_ bv0 1))))))))
(get-model)