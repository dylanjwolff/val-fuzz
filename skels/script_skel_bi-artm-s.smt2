(set-option :incremental "false")
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
(declare-const GEN13 String)
(declare-const GEN12 String)
(declare-const GEN11 Int)
(declare-const GEN10 Int)
(declare-const GEN9 Int)
(declare-const GEN8 Int)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 String)
(declare-const GEN1 String)
(declare-fun Y () String)
(set-info :notes "ufP_1 is uf type conv P")
(declare-fun ufP_1 (Int) Int)
(set-info :notes "ufM_2 is uf type conv M")
(declare-fun ufM_2 (Int) Int)
(declare-fun z1_3 () String)
(declare-fun z2_4 () String)
(declare-fun z3_5 () String)
(declare-fun V_253 () String)
(declare-fun V_254 () String)
(assert (or (= Y GEN1) (= Y GEN2)))
(assert (>= (ufP_1 GEN3) GEN4))
(assert (forall ((V_243 Int)) (or (not (and (>= V_243 GEN5) (>= (+ (str.len Y) (* (- GEN6) V_243)) GEN7))) (and (or (not (= (str.len Y) (+ GEN8 V_243))) (= (ufP_1 V_243) (ufM_2 V_243))) (not (>= (ufM_2 V_243) GEN9)) (not (or (not (= (str.len Y) (+ GEN10 V_243 (str.len V_253)))) (not (= Y (str.++ V_253 (ite (= (ufM_2 V_243) GEN11) GEN12 GEN13) V_254)))))))))
(assert (= BAV1 (or (= Y GEN1) (= Y GEN2))))
(assert (= BAV2 (= Y GEN1)))
(assert (= BAV3 (= Y GEN2)))
(assert (= BAV4 (>= (ufP_1 GEN3) GEN4)))
(assert (= BAV5 (forall ((V_243 Int)) (or (not (and (>= V_243 GEN5) (>= (+ (str.len Y) (* (- GEN6) V_243)) GEN7))) (and (or (not (= (str.len Y) (+ GEN8 V_243))) (= (ufP_1 V_243) (ufM_2 V_243))) (not (>= (ufM_2 V_243) GEN9)) (not (or (not (= (str.len Y) (+ GEN10 V_243 (str.len V_253)))) (not (= Y (str.++ V_253 (ite (= (ufM_2 V_243) GEN11) GEN12 GEN13) V_254))))))))))
(assert (= BAV6 (forall ((V_243 Int)) (and (>= V_243 GEN5) (>= (+ (str.len Y) (* (- GEN6) V_243)) GEN7)))))
(assert (= BAV7 (forall ((V_243 Int)) (>= V_243 GEN5))))
(assert (= BAV8 (forall ((V_243 Int)) (>= (+ (str.len Y) (* (- GEN6) V_243)) GEN7))))
(assert (= BAV9 (forall ((V_243 Int)) (and (or (not (= (str.len Y) (+ GEN8 V_243))) (= (ufP_1 V_243) (ufM_2 V_243))) (not (>= (ufM_2 V_243) GEN9)) (not (or (not (= (str.len Y) (+ GEN10 V_243 (str.len V_253)))) (not (= Y (str.++ V_253 (ite (= (ufM_2 V_243) GEN11) GEN12 GEN13) V_254)))))))))
(assert (= BAV10 (forall ((V_243 Int)) (or (not (= (str.len Y) (+ GEN8 V_243))) (= (ufP_1 V_243) (ufM_2 V_243))))))
(assert (= BAV11 (forall ((V_243 Int)) (= (str.len Y) (+ GEN8 V_243)))))
(assert (= BAV12 (forall ((V_243 Int)) (= (ufP_1 V_243) (ufM_2 V_243)))))
(assert (= BAV13 (forall ((V_243 Int)) (>= (ufM_2 V_243) GEN9))))
(assert (= BAV14 (forall ((V_243 Int)) (or (not (= (str.len Y) (+ GEN10 V_243 (str.len V_253)))) (not (= Y (str.++ V_253 (ite (= (ufM_2 V_243) GEN11) GEN12 GEN13) V_254)))))))
(assert (= BAV15 (forall ((V_243 Int)) (= (str.len Y) (+ GEN10 V_243 (str.len V_253))))))
(assert (= BAV16 (forall ((V_243 Int)) (= Y (str.++ V_253 (ite (= (ufM_2 V_243) GEN11) GEN12 GEN13) V_254)))))
(assert (= BAV17 (forall ((V_243 Int)) (= (ufM_2 V_243) GEN11))))
(check-sat)
(get-model)