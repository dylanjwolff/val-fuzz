(set-option :incremental false)
(set-info :source "Generated by Roberto Bruttomesso")
(set-info :category "crafted")
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
(declare-const BAV18 Bool)
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
(declare-fun a () (_ BitVec 1024))
(declare-fun dummy () (_ BitVec 256))
(declare-fun v1 () (_ BitVec 1024))
(declare-fun v2 () (_ BitVec 1024))
(declare-fun v3 () (_ BitVec 1024))
(declare-fun v4 () (_ BitVec 1024))
(assert (= BAV1 (and (not (= ((_ extract 767 512) v1) ((_ extract 511 256) v1))) (not (= ((_ extract 767 512) v2) ((_ extract 511 256) v2))) (not (= ((_ extract 767 512) v3) ((_ extract 511 256) v3))) (not (= ((_ extract 767 512) v4) ((_ extract 511 256) v4))) (or (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v1) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v1)))) (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v2) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v2)))) (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v3) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v3)))) (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v4) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v4))))))))
(assert (= BAV2 (= ((_ extract 767 512) v1) ((_ extract 511 256) v1))))
(assert (= BAV3 (= ((_ extract 767 512) v2) ((_ extract 511 256) v2))))
(assert (= BAV4 (= ((_ extract 767 512) v3) ((_ extract 511 256) v3))))
(assert (= BAV5 (= ((_ extract 767 512) v4) ((_ extract 511 256) v4))))
(assert (= BAV6 (or (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v1) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v1)))) (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v2) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v2)))) (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v3) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v3)))) (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v4) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v4)))))))
(assert (= BAV7 (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v1) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v1))))))
(assert (= BAV8 (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v1) dummy))))
(assert (= BAV9 (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v1)))))
(assert (= BAV10 (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v2) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v2))))))
(assert (= BAV11 (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v2) dummy))))
(assert (= BAV12 (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v2)))))
(assert (= BAV13 (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v3) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v3))))))
(assert (= BAV14 (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v3) dummy))))
(assert (= BAV15 (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v3)))))
(assert (= BAV16 (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v4) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v4))))))
(assert (= BAV17 (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v4) dummy))))
(assert (= BAV18 (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v4)))))
(check-sat-assuming ((and (not (= ((_ extract 767 512) v1) ((_ extract 511 256) v1))) (not (= ((_ extract 767 512) v2) ((_ extract 511 256) v2))) (not (= ((_ extract 767 512) v3) ((_ extract 511 256) v3))) (not (= ((_ extract 767 512) v4) ((_ extract 511 256) v4))) (or (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v1) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v1)))) (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v2) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v2)))) (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v3) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v3)))) (and (= ((_ extract 1023 256) a) (concat ((_ extract 1023 512) v4) dummy)) (= ((_ extract 767 0) a) (concat dummy ((_ extract 511 0) v4))))))))
(get-model)