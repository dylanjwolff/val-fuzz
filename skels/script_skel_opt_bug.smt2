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
(declare-fun |ku'0'| () Int)
(declare-fun |ku'1'| () Int)
(declare-fun |ku'4'| () Int)
(declare-fun |ku'3'| () Int)
(declare-fun |ku'2'| () Int)
(assert (and (>= |ku'0'| GEN1) (<= |ku'0'| GEN2)))
(assert (= |ku'0'| GEN3))
(assert (and (>= |ku'1'| GEN4) (<= |ku'1'| GEN5)))
(assert (>= (+ |ku'1'| |ku'2'| |ku'3'| |ku'4'|) GEN6))
(assert (and (>= |ku'2'| GEN7) (<= |ku'2'| GEN8)))
(assert (>= (+ |ku'2'| |ku'3'|) GEN9))
(assert (and (>= |ku'3'| GEN10) (<= |ku'3'| GEN11)))
(assert (>= (+ |ku'3'| |ku'2'|) GEN12))
(assert (and (>= |ku'4'| GEN13) (<= |ku'4'| GEN14)))
(assert (>= (+ |ku'4'| |ku'2'| |ku'3'|) GEN15))
(minimize (+ |ku'0'| |ku'1'| |ku'2'| |ku'3'| |ku'4'|))
(assert (= BAV1 (and (>= |ku'0'| GEN1) (<= |ku'0'| GEN2))))
(assert (= BAV2 (>= |ku'0'| GEN1)))
(assert (= BAV3 (<= |ku'0'| GEN2)))
(assert (= BAV4 (= |ku'0'| GEN3)))
(assert (= BAV5 (and (>= |ku'1'| GEN4) (<= |ku'1'| GEN5))))
(assert (= BAV6 (>= |ku'1'| GEN4)))
(assert (= BAV7 (<= |ku'1'| GEN5)))
(assert (= BAV8 (>= (+ |ku'1'| |ku'2'| |ku'3'| |ku'4'|) GEN6)))
(assert (= BAV9 (and (>= |ku'2'| GEN7) (<= |ku'2'| GEN8))))
(assert (= BAV10 (>= |ku'2'| GEN7)))
(assert (= BAV11 (<= |ku'2'| GEN8)))
(assert (= BAV12 (>= (+ |ku'2'| |ku'3'|) GEN9)))
(assert (= BAV13 (and (>= |ku'3'| GEN10) (<= |ku'3'| GEN11))))
(assert (= BAV14 (>= |ku'3'| GEN10)))
(assert (= BAV15 (<= |ku'3'| GEN11)))
(assert (= BAV16 (>= (+ |ku'3'| |ku'2'|) GEN12)))
(assert (= BAV17 (and (>= |ku'4'| GEN13) (<= |ku'4'| GEN14))))
(assert (= BAV18 (>= |ku'4'| GEN13)))
(assert (= BAV19 (<= |ku'4'| GEN14)))
(assert (= BAV20 (>= (+ |ku'4'| |ku'2'| |ku'3'|) GEN15)))
(check-sat)
(get-model)