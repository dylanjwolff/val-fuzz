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
(declare-const BAV13 Bool)
(declare-const BAV14 Bool)
(declare-const BAV15 Bool)
(declare-const GEN83 Int)
(declare-const GEN82 Int)
(declare-const GEN81 Int)
(declare-const GEN80 Int)
(declare-const GEN79 Int)
(declare-const GEN78 Int)
(declare-const GEN77 Int)
(declare-const GEN76 Int)
(declare-const GEN75 Int)
(declare-const GEN74 Int)
(declare-const GEN73 Int)
(declare-const GEN72 Int)
(declare-const GEN71 Int)
(declare-const GEN70 Int)
(declare-const GEN69 Int)
(declare-const GEN68 Int)
(declare-const GEN67 Int)
(declare-const GEN66 Int)
(declare-const GEN65 Int)
(declare-const GEN64 Int)
(declare-const GEN63 Int)
(declare-const GEN62 Int)
(declare-const GEN61 Int)
(declare-const GEN60 Int)
(declare-const GEN59 Int)
(declare-const GEN58 Int)
(declare-const GEN57 Int)
(declare-const GEN56 Int)
(declare-const GEN55 Int)
(declare-const GEN54 Int)
(declare-const GEN53 Int)
(declare-const GEN52 Int)
(declare-const GEN51 Int)
(declare-const GEN50 Int)
(declare-const GEN49 Int)
(declare-const GEN48 Int)
(declare-const GEN47 Int)
(declare-const GEN46 Int)
(declare-const GEN45 Int)
(declare-const GEN44 Int)
(declare-const GEN43 Int)
(declare-const GEN42 Int)
(declare-const GEN41 Int)
(declare-const GEN40 Int)
(declare-const GEN39 Int)
(declare-const GEN38 Int)
(declare-const GEN37 Int)
(declare-const GEN36 Int)
(declare-const GEN35 Int)
(declare-const GEN34 Int)
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
(declare-fun v5 () (_ BitVec 4))
(declare-fun v0 () (_ BitVec 4))
(declare-fun v8 () (_ BitVec 4))
(declare-fun v3 () (_ BitVec 4))
(declare-fun v2 () (_ BitVec 4))
(declare-fun v6 () (_ BitVec 4))
(declare-fun v1 () (_ BitVec 4))
(assert (= BAV1 (and (bvsge ((_ zero_extend 3) (ite (bvule (ite (= (_ bv1 1) ((_ extract 1 1) v2)) (_ bv4 4) (bvsub (bvadd v1 v6) v6)) v2) (_ bv1 1) (_ bv0 1))) (_ bv1 4)) (or false (bvugt (_ bv1 4) ((_ sign_extend 3) (ite (bvsge (_ bv0 4) ((_ zero_extend 3) (ite (bvugt (bvlshr v1 (bvand (_ bv4 4) v3)) ((_ zero_extend 3) (ite (bvslt (_ bv4 4) v2) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (bvslt ((_ zero_extend 3) (ite (bvslt (_ bv0 4) ((_ zero_extend 3) (ite (bvsle (_ bv0 4) v1) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1))) (_ bv1 4))) (or false (bvule ((_ zero_extend 3) (ite (bvule ((_ sign_extend 3) (ite (bvuge v0 v6) (_ bv1 1) (_ bv0 1))) (_ bv0 4)) (_ bv1 1) (_ bv0 1))) (_ bv0 4)) (not (bvsge (ite (= (_ bv1 1) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1))) (_ bv1 4) (_ bv0 4)) (_ bv0 4)))) (bvule (bvshl (_ bv4 4) v1) (_ bv0 4)) (= (_ bv1 1) (bvlshr (_ bv1 1) (ite (bvult v0 ((_ sign_extend 3) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (or false (bvugt ((_ zero_extend 3) (ite (bvuge (_ bv0 4) (bvshl (_ bv4 4) v1)) (_ bv1 1) (_ bv0 1))) (_ bv0 4)) (distinct (_ bv1 1) (ite (bvslt v6 (_ bv0 4)) (_ bv1 1) (_ bv0 1)))) (or false (= v1 ((_ zero_extend 3) (ite (bvsgt v3 ((_ zero_extend 3) (ite (bvsle (_ bv0 4) v1) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (bvule v6 (bvnot v5))) (or false (bvule (ite (bvule v0 v6) (_ bv1 1) (_ bv0 1)) ((_ extract 2 2) (ite (= (_ bv1 1) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1))) (_ bv1 4) (_ bv0 4)))) (bvsle (_ bv0 4) (bvand (_ bv4 4) v3))) (or false (not (bvsge (_ bv0 4) (bvshl (bvlshr v1 (bvand (_ bv4 4) v3)) (bvadd (_ bv1 4) (_ bv1 4))))) (not (bvuge (_ bv0 4) (bvand (_ bv4 4) (bvnot v5))))) (bvslt (ite (bvugt (bvlshr v1 (bvand (_ bv4 4) v3)) ((_ zero_extend 3) (ite (bvslt (_ bv4 4) v2) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)) (ite (bvuge v6 ((_ sign_extend 3) (_ bv1 1))) (_ bv1 1) (_ bv0 1))) (not (bvsle (bvadd (_ bv1 4) (_ bv1 4)) (bvxnor (_ bv0 4) (bvxnor (_ bv0 4) (bvand v1 v6))))) (distinct (_ bv0 4) (ite (= (_ bv1 1) (bvcomp v2 v3)) v8 ((_ zero_extend 3) (bvcomp v0 v5)))))))
(assert (= BAV2 (= (_ bv1 1) ((_ extract 1 1) v2))))
(assert (= BAV3 (or false (bvugt (_ bv1 4) ((_ sign_extend 3) (ite (bvsge (_ bv0 4) ((_ zero_extend 3) (ite (bvugt (bvlshr v1 (bvand (_ bv4 4) v3)) ((_ zero_extend 3) (ite (bvslt (_ bv4 4) v2) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (bvslt ((_ zero_extend 3) (ite (bvslt (_ bv0 4) ((_ zero_extend 3) (ite (bvsle (_ bv0 4) v1) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1))) (_ bv1 4)))))
(assert (= BAV4 (or false (bvule ((_ zero_extend 3) (ite (bvule ((_ sign_extend 3) (ite (bvuge v0 v6) (_ bv1 1) (_ bv0 1))) (_ bv0 4)) (_ bv1 1) (_ bv0 1))) (_ bv0 4)) (not (bvsge (ite (= (_ bv1 1) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1))) (_ bv1 4) (_ bv0 4)) (_ bv0 4))))))
(assert (= BAV5 (= (_ bv1 1) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1)))))
(assert (= BAV6 (= (_ bv1 1) (bvlshr (_ bv1 1) (ite (bvult v0 ((_ sign_extend 3) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1))))))
(assert (= BAV7 (or false (bvugt ((_ zero_extend 3) (ite (bvuge (_ bv0 4) (bvshl (_ bv4 4) v1)) (_ bv1 1) (_ bv0 1))) (_ bv0 4)) (distinct (_ bv1 1) (ite (bvslt v6 (_ bv0 4)) (_ bv1 1) (_ bv0 1))))))
(assert (= BAV8 (distinct (_ bv1 1) (ite (bvslt v6 (_ bv0 4)) (_ bv1 1) (_ bv0 1)))))
(assert (= BAV9 (or false (= v1 ((_ zero_extend 3) (ite (bvsgt v3 ((_ zero_extend 3) (ite (bvsle (_ bv0 4) v1) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (bvule v6 (bvnot v5)))))
(assert (= BAV10 (= v1 ((_ zero_extend 3) (ite (bvsgt v3 ((_ zero_extend 3) (ite (bvsle (_ bv0 4) v1) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1))))))
(assert (= BAV11 (or false (bvule (ite (bvule v0 v6) (_ bv1 1) (_ bv0 1)) ((_ extract 2 2) (ite (= (_ bv1 1) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1))) (_ bv1 4) (_ bv0 4)))) (bvsle (_ bv0 4) (bvand (_ bv4 4) v3)))))
(assert (= BAV12 (= (_ bv1 1) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1)))))
(assert (= BAV13 (or false (not (bvsge (_ bv0 4) (bvshl (bvlshr v1 (bvand (_ bv4 4) v3)) (bvadd (_ bv1 4) (_ bv1 4))))) (not (bvuge (_ bv0 4) (bvand (_ bv4 4) (bvnot v5)))))))
(assert (= BAV14 (distinct (_ bv0 4) (ite (= (_ bv1 1) (bvcomp v2 v3)) v8 ((_ zero_extend 3) (bvcomp v0 v5))))))
(assert (= BAV15 (= (_ bv1 1) (bvcomp v2 v3))))
(check-sat-assuming ((and (bvsge ((_ zero_extend 3) (ite (bvule (ite (= (_ bv1 1) ((_ extract 1 1) v2)) (_ bv4 4) (bvsub (bvadd v1 v6) v6)) v2) (_ bv1 1) (_ bv0 1))) (_ bv1 4)) (or false (bvugt (_ bv1 4) ((_ sign_extend 3) (ite (bvsge (_ bv0 4) ((_ zero_extend 3) (ite (bvugt (bvlshr v1 (bvand (_ bv4 4) v3)) ((_ zero_extend 3) (ite (bvslt (_ bv4 4) v2) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (bvslt ((_ zero_extend 3) (ite (bvslt (_ bv0 4) ((_ zero_extend 3) (ite (bvsle (_ bv0 4) v1) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1))) (_ bv1 4))) (or false (bvule ((_ zero_extend 3) (ite (bvule ((_ sign_extend 3) (ite (bvuge v0 v6) (_ bv1 1) (_ bv0 1))) (_ bv0 4)) (_ bv1 1) (_ bv0 1))) (_ bv0 4)) (not (bvsge (ite (= (_ bv1 1) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1))) (_ bv1 4) (_ bv0 4)) (_ bv0 4)))) (bvule (bvshl (_ bv4 4) v1) (_ bv0 4)) (= (_ bv1 1) (bvlshr (_ bv1 1) (ite (bvult v0 ((_ sign_extend 3) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (or false (bvugt ((_ zero_extend 3) (ite (bvuge (_ bv0 4) (bvshl (_ bv4 4) v1)) (_ bv1 1) (_ bv0 1))) (_ bv0 4)) (distinct (_ bv1 1) (ite (bvslt v6 (_ bv0 4)) (_ bv1 1) (_ bv0 1)))) (or false (= v1 ((_ zero_extend 3) (ite (bvsgt v3 ((_ zero_extend 3) (ite (bvsle (_ bv0 4) v1) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)))) (bvule v6 (bvnot v5))) (or false (bvule (ite (bvule v0 v6) (_ bv1 1) (_ bv0 1)) ((_ extract 2 2) (ite (= (_ bv1 1) (ite (bvuge v1 v6) (_ bv1 1) (_ bv0 1))) (_ bv1 4) (_ bv0 4)))) (bvsle (_ bv0 4) (bvand (_ bv4 4) v3))) (or false (not (bvsge (_ bv0 4) (bvshl (bvlshr v1 (bvand (_ bv4 4) v3)) (bvadd (_ bv1 4) (_ bv1 4))))) (not (bvuge (_ bv0 4) (bvand (_ bv4 4) (bvnot v5))))) (bvslt (ite (bvugt (bvlshr v1 (bvand (_ bv4 4) v3)) ((_ zero_extend 3) (ite (bvslt (_ bv4 4) v2) (_ bv1 1) (_ bv0 1)))) (_ bv1 1) (_ bv0 1)) (ite (bvuge v6 ((_ sign_extend 3) (_ bv1 1))) (_ bv1 1) (_ bv0 1))) (not (bvsle (bvadd (_ bv1 4) (_ bv1 4)) (bvxnor (_ bv0 4) (bvxnor (_ bv0 4) (bvand v1 v6))))) (distinct (_ bv0 4) (ite (= (_ bv1 1) (bvcomp v2 v3)) v8 ((_ zero_extend 3) (bvcomp v0 v5)))))))
(get-model)