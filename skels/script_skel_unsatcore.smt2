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
(declare-const BAV18 Bool)
(declare-const BAV19 Bool)
(declare-const BAV20 Bool)
(declare-const BAV21 Bool)
(declare-const BAV22 Bool)
(declare-const BAV23 Bool)
(declare-const BAV24 Bool)
(declare-const BAV25 Bool)
(declare-const BAV26 Bool)
(declare-const BAV27 Bool)
(declare-const BAV28 Bool)
(declare-const BAV29 Bool)
(declare-const BAV30 Bool)
(declare-const BAV31 Bool)
(declare-const BAV32 Bool)
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
(set-option :produce-unsat-cores true)
(set-option :produce-models false)
(declare-const start25 Bool)
(declare-const bf07 Bool)
(declare-const bf19 Bool)
(declare-const lt06 Int)
(declare-const ef08 Int)
(declare-const ef110 Int)
(declare-fun whileM4 (Int) Int)
(assert start25)
(assert (=> start25 (distinct lt06 GEN1)))
(assert (=> start25 (= lt06 (whileM4 GEN2))))
(assert (=> (not bf07) (= ef08 GEN3)))
(assert (=> bf07 (= ef08 (whileM4 (+ GEN4 GEN5)))))
(assert (=> start25 (not (< (whileM4 GEN6) GEN7))))
(assert (=> start25 (= (whileM4 GEN8) ef08)))
(assert (=> start25 (and (=> bf07 (< GEN9 GEN10)) (=> (< GEN11 GEN12) bf07))))
(assert (=> (not bf19) (= ef110 (+ GEN13 GEN14))))
(assert (=> bf19 (= ef110 (whileM4 (+ (+ GEN15 GEN16) GEN17)))))
(assert (=> bf07 (not (< (whileM4 (+ GEN18 GEN19)) GEN20))))
(assert (=> bf07 (= (whileM4 (+ GEN21 GEN22)) ef110)))
(assert (=> bf07 (and (=> bf19 (< (+ GEN23 GEN24) GEN25)) (=> (< (+ GEN26 GEN27) GEN28) bf19))))
(check-sat (not bf19))
(assert (= BAV1 (=> start25 (distinct lt06 GEN1))))
(assert (= BAV2 (distinct lt06 GEN1)))
(assert (= BAV3 (=> start25 (= lt06 (whileM4 GEN2)))))
(assert (= BAV4 (= lt06 (whileM4 GEN2))))
(assert (= BAV5 (=> (not bf07) (= ef08 GEN3))))
(assert (= BAV6 (= ef08 GEN3)))
(assert (= BAV7 (=> bf07 (= ef08 (whileM4 (+ GEN4 GEN5))))))
(assert (= BAV8 (= ef08 (whileM4 (+ GEN4 GEN5)))))
(assert (= BAV9 (=> start25 (not (< (whileM4 GEN6) GEN7)))))
(assert (= BAV10 (< (whileM4 GEN6) GEN7)))
(assert (= BAV11 (=> start25 (= (whileM4 GEN8) ef08))))
(assert (= BAV12 (= (whileM4 GEN8) ef08)))
(assert (= BAV13 (=> start25 (and (=> bf07 (< GEN9 GEN10)) (=> (< GEN11 GEN12) bf07)))))
(assert (= BAV14 (and (=> bf07 (< GEN9 GEN10)) (=> (< GEN11 GEN12) bf07))))
(assert (= BAV15 (=> bf07 (< GEN9 GEN10))))
(assert (= BAV16 (< GEN9 GEN10)))
(assert (= BAV17 (=> (< GEN11 GEN12) bf07)))
(assert (= BAV18 (< GEN11 GEN12)))
(assert (= BAV19 (=> (not bf19) (= ef110 (+ GEN13 GEN14)))))
(assert (= BAV20 (= ef110 (+ GEN13 GEN14))))
(assert (= BAV21 (=> bf19 (= ef110 (whileM4 (+ (+ GEN15 GEN16) GEN17))))))
(assert (= BAV22 (= ef110 (whileM4 (+ (+ GEN15 GEN16) GEN17)))))
(assert (= BAV23 (=> bf07 (not (< (whileM4 (+ GEN18 GEN19)) GEN20)))))
(assert (= BAV24 (< (whileM4 (+ GEN18 GEN19)) GEN20)))
(assert (= BAV25 (=> bf07 (= (whileM4 (+ GEN21 GEN22)) ef110))))
(assert (= BAV26 (= (whileM4 (+ GEN21 GEN22)) ef110)))
(assert (= BAV27 (=> bf07 (and (=> bf19 (< (+ GEN23 GEN24) GEN25)) (=> (< (+ GEN26 GEN27) GEN28) bf19)))))
(assert (= BAV28 (and (=> bf19 (< (+ GEN23 GEN24) GEN25)) (=> (< (+ GEN26 GEN27) GEN28) bf19))))
(assert (= BAV29 (=> bf19 (< (+ GEN23 GEN24) GEN25))))
(assert (= BAV30 (< (+ GEN23 GEN24) GEN25)))
(assert (= BAV31 (=> (< (+ GEN26 GEN27) GEN28) bf19)))
(assert (= BAV32 (< (+ GEN26 GEN27) GEN28)))
(check-sat)
(get-model)