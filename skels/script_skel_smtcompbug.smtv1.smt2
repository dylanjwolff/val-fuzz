(set-option :incremental false)
(set-info :category "unknown")
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun x781 () (_ BitVec 32))
(declare-fun x803 () (_ BitVec 8))
(declare-fun x804 () (_ BitVec 8))
(declare-fun x791 () (_ BitVec 8))
(assert (= BAV1 (and (= x804 (bvxor (bvxor ((_ extract 7 0) (bvadd (_ bv1 32) x781)) x791) x803)) (= (bvnot ((_ extract 0 0) x804)) (_ bv0 1)) (= x781 (_ bv0 32)))))
(assert (= BAV2 (= x804 (bvxor (bvxor ((_ extract 7 0) (bvadd (_ bv1 32) x781)) x791) x803))))
(assert (= BAV3 (= (bvnot ((_ extract 0 0) x804)) (_ bv0 1))))
(assert (= BAV4 (= x781 (_ bv0 32))))
(check-sat-assuming ((and (= x804 (bvxor (bvxor ((_ extract 7 0) (bvadd (_ bv1 32) x781)) x791) x803)) (= (bvnot ((_ extract 0 0) x804)) (_ bv0 1)) (= x781 (_ bv0 32)))))
(get-model)