(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(get-model)
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
(declare-const BAV33 Bool)
(declare-const BAV34 Bool)
(declare-const BAV35 Bool)
(declare-const BAV36 Bool)
(declare-const BAV37 Bool)
(declare-const BAV38 Bool)
(declare-const BAV39 Bool)
(declare-const BAV40 Bool)
(declare-const BAV41 Bool)
(declare-const BAV42 Bool)
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
(set-option :auto-config true)
(set-option :produce-models true)
(set-option :smt.mbqi true)
(declare-fun n () Int)
(declare-fun a_1 () Int)
(declare-fun f (Int) Int)
(declare-fun g_1 (Int) Int)
(assert (> n GEN1))
(assert (forall ((i Int)) (=> (and (<= GEN2 i) (<= i n)) (and (= (f GEN3) GEN4) (= (f GEN5) GEN6) (<= GEN7 (f i)) (<= (f i) GEN8) (=> (= (f i) GEN9) (= i n)) (=> (= (f i) GEN10) (or (= (f (+ i GEN11)) GEN12) (= (f (+ i GEN13)) GEN14))) (=> (= (f i) GEN15) (or (= (f (+ i GEN16)) GEN17) (= (f (+ i GEN18)) GEN19))) (= (g_1 GEN20) GEN21) (=> (= (f i) GEN22) (= (g_1 (+ i GEN23)) GEN24)) (=> (= (f i) GEN25) (= (g_1 (+ i GEN26)) (+ (g_1 i) GEN27))) (=> (= (f i) GEN28) (= (g_1 (+ i GEN29)) (g_1 i))) (=> (= (f i) GEN30) (< (g_1 i) a_1)) (=> (= (f i) GEN31) (and (>= (g_1 i) a_1) (> (g_1 i) GEN32)))))))
(assert (= BAV1 (> n GEN1)))
(assert (= BAV2 (forall ((i Int)) (=> (and (<= GEN2 i) (<= i n)) (and (= (f GEN3) GEN4) (= (f GEN5) GEN6) (<= GEN7 (f i)) (<= (f i) GEN8) (=> (= (f i) GEN9) (= i n)) (=> (= (f i) GEN10) (or (= (f (+ i GEN11)) GEN12) (= (f (+ i GEN13)) GEN14))) (=> (= (f i) GEN15) (or (= (f (+ i GEN16)) GEN17) (= (f (+ i GEN18)) GEN19))) (= (g_1 GEN20) GEN21) (=> (= (f i) GEN22) (= (g_1 (+ i GEN23)) GEN24)) (=> (= (f i) GEN25) (= (g_1 (+ i GEN26)) (+ (g_1 i) GEN27))) (=> (= (f i) GEN28) (= (g_1 (+ i GEN29)) (g_1 i))) (=> (= (f i) GEN30) (< (g_1 i) a_1)) (=> (= (f i) GEN31) (and (>= (g_1 i) a_1) (> (g_1 i) GEN32))))))))
(assert (= BAV3 (forall ((i Int)) (and (<= GEN2 i) (<= i n)))))
(assert (= BAV4 (forall ((i Int)) (<= GEN2 i))))
(assert (= BAV5 (forall ((i Int)) (<= i n))))
(assert (= BAV6 (forall ((i Int)) (and (= (f GEN3) GEN4) (= (f GEN5) GEN6) (<= GEN7 (f i)) (<= (f i) GEN8) (=> (= (f i) GEN9) (= i n)) (=> (= (f i) GEN10) (or (= (f (+ i GEN11)) GEN12) (= (f (+ i GEN13)) GEN14))) (=> (= (f i) GEN15) (or (= (f (+ i GEN16)) GEN17) (= (f (+ i GEN18)) GEN19))) (= (g_1 GEN20) GEN21) (=> (= (f i) GEN22) (= (g_1 (+ i GEN23)) GEN24)) (=> (= (f i) GEN25) (= (g_1 (+ i GEN26)) (+ (g_1 i) GEN27))) (=> (= (f i) GEN28) (= (g_1 (+ i GEN29)) (g_1 i))) (=> (= (f i) GEN30) (< (g_1 i) a_1)) (=> (= (f i) GEN31) (and (>= (g_1 i) a_1) (> (g_1 i) GEN32)))))))
(assert (= BAV7 (forall ((i Int)) (= (f GEN3) GEN4))))
(assert (= BAV8 (forall ((i Int)) (= (f GEN5) GEN6))))
(assert (= BAV9 (forall ((i Int)) (<= GEN7 (f i)))))
(assert (= BAV10 (forall ((i Int)) (<= (f i) GEN8))))
(assert (= BAV11 (forall ((i Int)) (=> (= (f i) GEN9) (= i n)))))
(assert (= BAV12 (forall ((i Int)) (= (f i) GEN9))))
(assert (= BAV13 (forall ((i Int)) (= i n))))
(assert (= BAV14 (forall ((i Int)) (=> (= (f i) GEN10) (or (= (f (+ i GEN11)) GEN12) (= (f (+ i GEN13)) GEN14))))))
(assert (= BAV15 (forall ((i Int)) (= (f i) GEN10))))
(assert (= BAV16 (forall ((i Int)) (or (= (f (+ i GEN11)) GEN12) (= (f (+ i GEN13)) GEN14)))))
(assert (= BAV17 (forall ((i Int)) (= (f (+ i GEN11)) GEN12))))
(assert (= BAV18 (forall ((i Int)) (= (f (+ i GEN13)) GEN14))))
(assert (= BAV19 (forall ((i Int)) (=> (= (f i) GEN15) (or (= (f (+ i GEN16)) GEN17) (= (f (+ i GEN18)) GEN19))))))
(assert (= BAV20 (forall ((i Int)) (= (f i) GEN15))))
(assert (= BAV21 (forall ((i Int)) (or (= (f (+ i GEN16)) GEN17) (= (f (+ i GEN18)) GEN19)))))
(assert (= BAV22 (forall ((i Int)) (= (f (+ i GEN16)) GEN17))))
(assert (= BAV23 (forall ((i Int)) (= (f (+ i GEN18)) GEN19))))
(assert (= BAV24 (forall ((i Int)) (= (g_1 GEN20) GEN21))))
(assert (= BAV25 (forall ((i Int)) (=> (= (f i) GEN22) (= (g_1 (+ i GEN23)) GEN24)))))
(assert (= BAV26 (forall ((i Int)) (= (f i) GEN22))))
(assert (= BAV27 (forall ((i Int)) (= (g_1 (+ i GEN23)) GEN24))))
(assert (= BAV28 (forall ((i Int)) (=> (= (f i) GEN25) (= (g_1 (+ i GEN26)) (+ (g_1 i) GEN27))))))
(assert (= BAV29 (forall ((i Int)) (= (f i) GEN25))))
(assert (= BAV30 (forall ((i Int)) (= (g_1 (+ i GEN26)) (+ (g_1 i) GEN27)))))
(assert (= BAV31 (forall ((i Int)) (=> (= (f i) GEN28) (= (g_1 (+ i GEN29)) (g_1 i))))))
(assert (= BAV32 (forall ((i Int)) (= (f i) GEN28))))
(assert (= BAV33 (forall ((i Int)) (= (g_1 (+ i GEN29)) (g_1 i)))))
(assert (= BAV34 (forall ((i Int)) (=> (= (f i) GEN30) (< (g_1 i) a_1)))))
(assert (= BAV35 (forall ((i Int)) (= (f i) GEN30))))
(assert (= BAV36 (forall ((i Int)) (< (g_1 i) a_1))))
(assert (= BAV37 (forall ((i Int)) (=> (= (f i) GEN31) (and (>= (g_1 i) a_1) (> (g_1 i) GEN32))))))
(assert (= BAV38 (forall ((i Int)) (= (f i) GEN31))))
(assert (= BAV39 (forall ((i Int)) (and (>= (g_1 i) a_1) (> (g_1 i) GEN32)))))
(assert (= BAV40 (forall ((i Int)) (>= (g_1 i) a_1))))
(assert (= BAV41 (forall ((i Int)) (> (g_1 i) GEN32))))
(assert (= BAV42 (> n GEN33)))
(check-sat)
(get-model)
(echo "Property does not hold for n > 1")
(assert (> n GEN33))
(check-sat)