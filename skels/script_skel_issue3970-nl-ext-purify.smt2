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
(declare-const BAV16 Bool)
(declare-const BAV17 Bool)
(declare-const GEN34 Real)
(declare-const GEN33 Int)
(declare-const GEN32 Real)
(declare-const GEN31 Int)
(declare-const GEN30 Int)
(declare-const GEN29 Real)
(declare-const GEN28 Real)
(declare-const GEN27 Real)
(declare-const GEN26 Int)
(declare-const GEN25 Real)
(declare-const GEN24 Int)
(declare-const GEN23 Real)
(declare-const GEN22 Real)
(declare-const GEN21 Int)
(declare-const GEN20 Real)
(declare-const GEN19 Int)
(declare-const GEN18 Real)
(declare-const GEN17 Int)
(declare-const GEN16 Real)
(declare-const GEN15 Real)
(declare-const GEN14 Int)
(declare-const GEN13 Int)
(declare-const GEN12 Real)
(declare-const GEN11 Int)
(declare-const GEN10 Real)
(declare-const GEN9 Real)
(declare-const GEN8 Int)
(declare-const GEN7 Real)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Real)
(declare-const GEN3 Int)
(declare-const GEN2 Real)
(declare-const GEN1 Int)
(set-option :nl-ext-purify true)
(set-option :sygus-inference true)
(declare-const v0 Bool)
(declare-const v1 Bool)
(declare-const v2 Bool)
(declare-const v3 Bool)
(declare-const v4 Bool)
(declare-const v5 Bool)
(declare-const v6 Bool)
(declare-const v7 Bool)
(declare-const v8 Bool)
(declare-const v9 Bool)
(declare-const r3 Real)
(declare-const r5 Real)
(assert v8)
(declare-const v10 Bool)
(assert (= 169 r5))
(declare-const v11 Bool)
(assert v5)
(declare-const v12 Bool)
(declare-const r7 Real)
(assert (not v1))
(assert v10)
(declare-const v13 Bool)
(declare-const r8 Real)
(assert (not (distinct r7 (- (/ 0.816577 (+ r5 569703))) 0.0 r3)))
(assert v4)
(assert (and (distinct v3 v4 (= 169 r5) (= (+ r5 569703) (/ 0.816577 (+ r5 569703)) 0.816577 0.26205) v11 v7 (= 169 r5)) (> 0.816577 169 569703 0.26205) (distinct r7 (- (/ 0.816577 (+ r5 569703))) 0.0 r3) v3 (= (+ r5 569703) (/ 0.816577 (+ r5 569703)) 0.816577 0.26205) v5 v10))
(declare-const v14 Bool)
(assert (or (= (+ r5 569703) (/ 0.816577 (+ r5 569703)) 0.816577 0.26205) v6 (or (<= 0.26205 r7 353 569703) v7 v8 v13 v3) v5 v6 (distinct r7 (- (/ 0.816577 (+ r5 569703))) 0.0 r3)))
(declare-const v15 Bool)
(assert (xor v3 v11 v9 v3))
(declare-const v16 Bool)
(assert (xor v2 v8))
(assert (= BAV1 (= 169 r5)))
(assert (= BAV2 (distinct r7 (- (/ 0.816577 (+ r5 569703))) 0.0 r3)))
(assert (= BAV3 (and (distinct v3 v4 (= 169 r5) (= (+ r5 569703) (/ 0.816577 (+ r5 569703)) 0.816577 0.26205) v11 v7 (= 169 r5)) (> 0.816577 169 569703 0.26205) (distinct r7 (- (/ 0.816577 (+ r5 569703))) 0.0 r3) v3 (= (+ r5 569703) (/ 0.816577 (+ r5 569703)) 0.816577 0.26205) v5 v10)))
(assert (= BAV4 (distinct v3 v4 (= 169 r5) (= (+ r5 569703) (/ 0.816577 (+ r5 569703)) 0.816577 0.26205) v11 v7 (= 169 r5))))
(assert (= BAV5 (= 169 r5)))
(assert (= BAV6 (= (+ r5 569703) (/ 0.816577 (+ r5 569703)) 0.816577 0.26205)))
(assert (= BAV7 (= 169 r5)))
(assert (= BAV8 (> 0.816577 169 569703 0.26205)))
(assert (= BAV9 (distinct r7 (- (/ 0.816577 (+ r5 569703))) 0.0 r3)))
(assert (= BAV10 (= (+ r5 569703) (/ 0.816577 (+ r5 569703)) 0.816577 0.26205)))
(assert (= BAV11 (or (= (+ r5 569703) (/ 0.816577 (+ r5 569703)) 0.816577 0.26205) v6 (or (<= 0.26205 r7 353 569703) v7 v8 v13 v3) v5 v6 (distinct r7 (- (/ 0.816577 (+ r5 569703))) 0.0 r3))))
(assert (= BAV12 (= (+ r5 569703) (/ 0.816577 (+ r5 569703)) 0.816577 0.26205)))
(assert (= BAV13 (or (<= 0.26205 r7 353 569703) v7 v8 v13 v3)))
(assert (= BAV14 (<= 0.26205 r7 353 569703)))
(assert (= BAV15 (distinct r7 (- (/ 0.816577 (+ r5 569703))) 0.0 r3)))
(assert (= BAV16 (xor v3 v11 v9 v3)))
(assert (= BAV17 (xor v2 v8)))
(check-sat)
(get-model)