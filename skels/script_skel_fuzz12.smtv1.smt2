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
(declare-const BAV11 Bool)
(declare-const BAV12 Bool)
(declare-const GEN33 Int)
(declare-const GEN32 Int)
(declare-const GEN31 Int)
(declare-const GEN30 Int)
(declare-const GEN29 Int)
(declare-const GEN28 Int)
(declare-const GEN27 Int)
(declare-const GEN26 Int)
(declare-const GEN25 Int)
(declare-const GEN24 Int)
(declare-const GEN23 Int)
(declare-const GEN22 Int)
(declare-const GEN21 Int)
(declare-const GEN20 Int)
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
(declare-fun v1 () (_ BitVec 9))
(declare-fun v2 () (_ BitVec 10))
(declare-fun v0 () (_ BitVec 3))
(assert (= BAV1 (=> (= (_ bv1 3) v0) (xor (or (= (_ bv0 9) (ite (= (_ bv1 1) ((_ extract 9 9) ((_ sign_extend 2) v2))) (bvneg v1) ((_ zero_extend 8) (ite (bvsgt (_ bv1 11) ((_ zero_extend 8) v0)) (_ bv1 1) (_ bv0 1))))) (= (ite (bvult (_ bv1 12) ((_ zero_extend 3) v1)) (_ bv1 1) (_ bv0 1)) (ite (= ((_ zero_extend 2) (bvlshr v2 ((_ zero_extend 1) v1))) (_ bv0 12)) (_ bv1 1) (_ bv0 1)))) (ite (= (_ bv0 12) ((_ sign_extend 9) v0)) (= (_ bv1 1) (bvcomp (_ bv0 10) (bvmul (bvlshr v2 ((_ zero_extend 1) v1)) ((_ zero_extend 1) ((_ rotate_left 3) (bvneg v1)))))) (= (_ bv0 10) (ite (= (_ bv1 1) (bvnor (ite (bvsge (bvadd v1 (_ bv1 9)) ((_ zero_extend 6) v0)) (_ bv1 1) (_ bv0 1)) (ite (bvsge (bvadd v1 (_ bv1 9)) ((_ zero_extend 6) v0)) (_ bv1 1) (_ bv0 1)))) (_ bv0 10) (bvlshr v2 ((_ zero_extend 1) v1)))))))))
(assert (= BAV2 (= (_ bv1 3) v0)))
(assert (= BAV3 (xor (or (= (_ bv0 9) (ite (= (_ bv1 1) ((_ extract 9 9) ((_ sign_extend 2) v2))) (bvneg v1) ((_ zero_extend 8) (ite (bvsgt (_ bv1 11) ((_ zero_extend 8) v0)) (_ bv1 1) (_ bv0 1))))) (= (ite (bvult (_ bv1 12) ((_ zero_extend 3) v1)) (_ bv1 1) (_ bv0 1)) (ite (= ((_ zero_extend 2) (bvlshr v2 ((_ zero_extend 1) v1))) (_ bv0 12)) (_ bv1 1) (_ bv0 1)))) (ite (= (_ bv0 12) ((_ sign_extend 9) v0)) (= (_ bv1 1) (bvcomp (_ bv0 10) (bvmul (bvlshr v2 ((_ zero_extend 1) v1)) ((_ zero_extend 1) ((_ rotate_left 3) (bvneg v1)))))) (= (_ bv0 10) (ite (= (_ bv1 1) (bvnor (ite (bvsge (bvadd v1 (_ bv1 9)) ((_ zero_extend 6) v0)) (_ bv1 1) (_ bv0 1)) (ite (bvsge (bvadd v1 (_ bv1 9)) ((_ zero_extend 6) v0)) (_ bv1 1) (_ bv0 1)))) (_ bv0 10) (bvlshr v2 ((_ zero_extend 1) v1))))))))
(assert (= BAV4 (or (= (_ bv0 9) (ite (= (_ bv1 1) ((_ extract 9 9) ((_ sign_extend 2) v2))) (bvneg v1) ((_ zero_extend 8) (ite (bvsgt (_ bv1 11) ((_ zero_extend 8) v0)) (_ bv1 1) (_ bv0 1))))) (= (ite (bvult (_ bv1 12) ((_ zero_extend 3) v1)) (_ bv1 1) (_ bv0 1)) (ite (= ((_ zero_extend 2) (bvlshr v2 ((_ zero_extend 1) v1))) (_ bv0 12)) (_ bv1 1) (_ bv0 1))))))
(assert (= BAV5 (= (_ bv0 9) (ite (= (_ bv1 1) ((_ extract 9 9) ((_ sign_extend 2) v2))) (bvneg v1) ((_ zero_extend 8) (ite (bvsgt (_ bv1 11) ((_ zero_extend 8) v0)) (_ bv1 1) (_ bv0 1)))))))
(assert (= BAV6 (= (_ bv1 1) ((_ extract 9 9) ((_ sign_extend 2) v2)))))
(assert (= BAV7 (= (ite (bvult (_ bv1 12) ((_ zero_extend 3) v1)) (_ bv1 1) (_ bv0 1)) (ite (= ((_ zero_extend 2) (bvlshr v2 ((_ zero_extend 1) v1))) (_ bv0 12)) (_ bv1 1) (_ bv0 1)))))
(assert (= BAV8 (= ((_ zero_extend 2) (bvlshr v2 ((_ zero_extend 1) v1))) (_ bv0 12))))
(assert (= BAV9 (= (_ bv0 12) ((_ sign_extend 9) v0))))
(assert (= BAV10 (= (_ bv1 1) (bvcomp (_ bv0 10) (bvmul (bvlshr v2 ((_ zero_extend 1) v1)) ((_ zero_extend 1) ((_ rotate_left 3) (bvneg v1))))))))
(assert (= BAV11 (= (_ bv0 10) (ite (= (_ bv1 1) (bvnor (ite (bvsge (bvadd v1 (_ bv1 9)) ((_ zero_extend 6) v0)) (_ bv1 1) (_ bv0 1)) (ite (bvsge (bvadd v1 (_ bv1 9)) ((_ zero_extend 6) v0)) (_ bv1 1) (_ bv0 1)))) (_ bv0 10) (bvlshr v2 ((_ zero_extend 1) v1))))))
(assert (= BAV12 (= (_ bv1 1) (bvnor (ite (bvsge (bvadd v1 (_ bv1 9)) ((_ zero_extend 6) v0)) (_ bv1 1) (_ bv0 1)) (ite (bvsge (bvadd v1 (_ bv1 9)) ((_ zero_extend 6) v0)) (_ bv1 1) (_ bv0 1))))))
(check-sat-assuming ((=> (= (_ bv1 3) v0) (xor (or (= (_ bv0 9) (ite (= (_ bv1 1) ((_ extract 9 9) ((_ sign_extend 2) v2))) (bvneg v1) ((_ zero_extend 8) (ite (bvsgt (_ bv1 11) ((_ zero_extend 8) v0)) (_ bv1 1) (_ bv0 1))))) (= (ite (bvult (_ bv1 12) ((_ zero_extend 3) v1)) (_ bv1 1) (_ bv0 1)) (ite (= ((_ zero_extend 2) (bvlshr v2 ((_ zero_extend 1) v1))) (_ bv0 12)) (_ bv1 1) (_ bv0 1)))) (ite (= (_ bv0 12) ((_ sign_extend 9) v0)) (= (_ bv1 1) (bvcomp (_ bv0 10) (bvmul (bvlshr v2 ((_ zero_extend 1) v1)) ((_ zero_extend 1) ((_ rotate_left 3) (bvneg v1)))))) (= (_ bv0 10) (ite (= (_ bv1 1) (bvnor (ite (bvsge (bvadd v1 (_ bv1 9)) ((_ zero_extend 6) v0)) (_ bv1 1) (_ bv0 1)) (ite (bvsge (bvadd v1 (_ bv1 9)) ((_ zero_extend 6) v0)) (_ bv1 1) (_ bv0 1)))) (_ bv0 10) (bvlshr v2 ((_ zero_extend 1) v1)))))))))
(get-model)