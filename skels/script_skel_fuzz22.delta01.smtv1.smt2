(set-option :incremental false)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-const BAV8 Bool)
(declare-const BAV9 Bool)
(declare-const BAV10 Bool)
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
(declare-fun v3 () (_ BitVec 4))
(declare-fun v0 () (_ BitVec 4))
(declare-fun v1 () (_ BitVec 4))
(declare-fun v2 () (_ BitVec 4))
(declare-fun v4 () (_ BitVec 4))
(assert (= BAV1 (and (not (bvuge (_ bv12 4) v2)) (or false (= (_ bv0 4) (bvneg (bvlshr v1 (bvnot v0)))) (not (= (_ bv0 4) (bvneg (bvlshr v1 (bvnot v0)))))) (or false (distinct v2 (bvxnor (bvnot v2) ((_ zero_extend 3) (bvcomp (_ bv12 4) v0)))) (not (bvsge (bvlshr v1 (bvnot v0)) (ite (= (_ bv1 1) (ite (bvsgt v0 (bvnot v2)) (_ bv1 1) (_ bv0 1))) ((_ zero_extend 3) (ite (bvsge v1 (_ bv0 4)) (_ bv1 1) (_ bv0 1))) (bvnot v2))))) (not (bvsle ((_ sign_extend 3) (ite (bvule (_ bv12 4) (bvneg (bvlshr v1 (bvnot v0)))) (_ bv1 1) (_ bv0 1))) (bvxnor (bvnot v2) ((_ zero_extend 3) (bvcomp (_ bv12 4) v0))))) (bvugt (bvlshr v1 (bvnot v0)) (bvxor v0 v4)) (or false (distinct (ite (bvsge v1 (_ bv0 4)) (_ bv1 1) (_ bv0 1)) (ite (bvslt (bvneg (bvnot v2)) v4) (_ bv1 1) (_ bv0 1))) (bvslt (bvlshr (bvlshr v1 (bvnot v0)) v3) (bvneg (bvlshr v1 (bvnot v0))))) (or false (bvsle (bvnot v0) (bvnot v2)) (bvsle (_ bv0 4) (bvlshr (bvlshr v1 (bvnot v0)) v3))))))
(assert (= BAV2 (or false (= (_ bv0 4) (bvneg (bvlshr v1 (bvnot v0)))) (not (= (_ bv0 4) (bvneg (bvlshr v1 (bvnot v0))))))))
(assert (= BAV3 (= (_ bv0 4) (bvneg (bvlshr v1 (bvnot v0))))))
(assert (= BAV4 (= (_ bv0 4) (bvneg (bvlshr v1 (bvnot v0))))))
(assert (= BAV5 (or false (distinct v2 (bvxnor (bvnot v2) ((_ zero_extend 3) (bvcomp (_ bv12 4) v0)))) (not (bvsge (bvlshr v1 (bvnot v0)) (ite (= (_ bv1 1) (ite (bvsgt v0 (bvnot v2)) (_ bv1 1) (_ bv0 1))) ((_ zero_extend 3) (ite (bvsge v1 (_ bv0 4)) (_ bv1 1) (_ bv0 1))) (bvnot v2)))))))
(assert (= BAV6 (distinct v2 (bvxnor (bvnot v2) ((_ zero_extend 3) (bvcomp (_ bv12 4) v0))))))
(assert (= BAV7 (= (_ bv1 1) (ite (bvsgt v0 (bvnot v2)) (_ bv1 1) (_ bv0 1)))))
(assert (= BAV8 (or false (distinct (ite (bvsge v1 (_ bv0 4)) (_ bv1 1) (_ bv0 1)) (ite (bvslt (bvneg (bvnot v2)) v4) (_ bv1 1) (_ bv0 1))) (bvslt (bvlshr (bvlshr v1 (bvnot v0)) v3) (bvneg (bvlshr v1 (bvnot v0)))))))
(assert (= BAV9 (distinct (ite (bvsge v1 (_ bv0 4)) (_ bv1 1) (_ bv0 1)) (ite (bvslt (bvneg (bvnot v2)) v4) (_ bv1 1) (_ bv0 1)))))
(assert (= BAV10 (or false (bvsle (bvnot v0) (bvnot v2)) (bvsle (_ bv0 4) (bvlshr (bvlshr v1 (bvnot v0)) v3)))))
(check-sat-assuming ((and (not (bvuge (_ bv12 4) v2)) (or false (= (_ bv0 4) (bvneg (bvlshr v1 (bvnot v0)))) (not (= (_ bv0 4) (bvneg (bvlshr v1 (bvnot v0)))))) (or false (distinct v2 (bvxnor (bvnot v2) ((_ zero_extend 3) (bvcomp (_ bv12 4) v0)))) (not (bvsge (bvlshr v1 (bvnot v0)) (ite (= (_ bv1 1) (ite (bvsgt v0 (bvnot v2)) (_ bv1 1) (_ bv0 1))) ((_ zero_extend 3) (ite (bvsge v1 (_ bv0 4)) (_ bv1 1) (_ bv0 1))) (bvnot v2))))) (not (bvsle ((_ sign_extend 3) (ite (bvule (_ bv12 4) (bvneg (bvlshr v1 (bvnot v0)))) (_ bv1 1) (_ bv0 1))) (bvxnor (bvnot v2) ((_ zero_extend 3) (bvcomp (_ bv12 4) v0))))) (bvugt (bvlshr v1 (bvnot v0)) (bvxor v0 v4)) (or false (distinct (ite (bvsge v1 (_ bv0 4)) (_ bv1 1) (_ bv0 1)) (ite (bvslt (bvneg (bvnot v2)) v4) (_ bv1 1) (_ bv0 1))) (bvslt (bvlshr (bvlshr v1 (bvnot v0)) v3) (bvneg (bvlshr v1 (bvnot v0))))) (or false (bvsle (bvnot v0) (bvnot v2)) (bvsle (_ bv0 4) (bvlshr (bvlshr v1 (bvnot v0)) v3))))))
(get-model)