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
(declare-const GEN11 Real)
(declare-const GEN10 Real)
(declare-const GEN9 Real)
(declare-const GEN8 Real)
(declare-const GEN7 Real)
(declare-const GEN6 Real)
(declare-const GEN5 Real)
(declare-const GEN4 Real)
(declare-const GEN3 Real)
(declare-const GEN2 Real)
(declare-const GEN1 Real)
(declare-fun time__AT0@0 () Real)
(declare-fun instance.y__AT0@0 () Real)
(declare-fun instance.x__AT0@0 () Real)
(declare-fun instance.x__AT0@1 () Real)
(declare-fun event_is_timed__AT0@1 () Bool)
(declare-fun event_is_timed__AT0@0 () Bool)
(declare-fun instance.EVENT.0__AT0@0 () Bool)
(declare-fun instance.EVENT.1__AT0@0 () Bool)
(declare-fun instance.y__AT0@1 () Real)
(declare-fun time__AT0@1 () Real)
(assert (and (and (= time__AT0@0 GEN11) (and (= instance.y__AT0@0 GEN10) (= instance.x__AT0@0 GEN9))) (and (and (= event_is_timed__AT0@1 (not event_is_timed__AT0@0)) (and (or (not (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))) (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1))) (and (or (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0)) (and (or (not instance.EVENT.1__AT0@0) (and (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))) (<= time__AT0@0 time__AT0@1))) (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)))) (and (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)) (and (or (not instance.EVENT.1__AT0@0) (<= time__AT0@0 time__AT0@1)) (= event_is_timed__AT0@0 instance.EVENT.1__AT0@0)))))) (or (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))) (not (<= GEN1 instance.x__AT0@1))))
(assert (= BAV1 (and (and (= time__AT0@0 GEN11) (and (= instance.y__AT0@0 GEN10) (= instance.x__AT0@0 GEN9))) (and (and (= event_is_timed__AT0@1 (not event_is_timed__AT0@0)) (and (or (not (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))) (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1))) (and (or (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0)) (and (or (not instance.EVENT.1__AT0@0) (and (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))) (<= time__AT0@0 time__AT0@1))) (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)))) (and (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)) (and (or (not instance.EVENT.1__AT0@0) (<= time__AT0@0 time__AT0@1)) (= event_is_timed__AT0@0 instance.EVENT.1__AT0@0)))))) (or (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))) (not (<= GEN1 instance.x__AT0@1)))))
(assert (= BAV2 (and (= time__AT0@0 GEN11) (and (= instance.y__AT0@0 GEN10) (= instance.x__AT0@0 GEN9)))))
(assert (= BAV3 (= time__AT0@0 GEN11)))
(assert (= BAV4 (and (= instance.y__AT0@0 GEN10) (= instance.x__AT0@0 GEN9))))
(assert (= BAV5 (= instance.y__AT0@0 GEN10)))
(assert (= BAV6 (= instance.x__AT0@0 GEN9)))
(assert (= BAV7 (and (and (= event_is_timed__AT0@1 (not event_is_timed__AT0@0)) (and (or (not (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))) (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1))) (and (or (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0)) (and (or (not instance.EVENT.1__AT0@0) (and (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))) (<= time__AT0@0 time__AT0@1))) (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)))) (and (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)) (and (or (not instance.EVENT.1__AT0@0) (<= time__AT0@0 time__AT0@1)) (= event_is_timed__AT0@0 instance.EVENT.1__AT0@0)))))) (or (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0)))))
(assert (= BAV8 (and (= event_is_timed__AT0@1 (not event_is_timed__AT0@0)) (and (or (not (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))) (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1))) (and (or (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0)) (and (or (not instance.EVENT.1__AT0@0) (and (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))) (<= time__AT0@0 time__AT0@1))) (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)))) (and (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)) (and (or (not instance.EVENT.1__AT0@0) (<= time__AT0@0 time__AT0@1)) (= event_is_timed__AT0@0 instance.EVENT.1__AT0@0))))))))
(assert (= BAV9 (= event_is_timed__AT0@1 (not event_is_timed__AT0@0))))
(assert (= BAV10 (and (or (not (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))) (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1))) (and (or (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0)) (and (or (not instance.EVENT.1__AT0@0) (and (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))) (<= time__AT0@0 time__AT0@1))) (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)))) (and (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)) (and (or (not instance.EVENT.1__AT0@0) (<= time__AT0@0 time__AT0@1)) (= event_is_timed__AT0@0 instance.EVENT.1__AT0@0)))))))
(assert (= BAV11 (or (not (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))) (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)))))
(assert (= BAV12 (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))))
(assert (= BAV13 (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1))))
(assert (= BAV14 (= instance.y__AT0@0 instance.y__AT0@1)))
(assert (= BAV15 (= instance.x__AT0@0 instance.x__AT0@1)))
(assert (= BAV16 (and (or (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0)) (and (or (not instance.EVENT.1__AT0@0) (and (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))) (<= time__AT0@0 time__AT0@1))) (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)))) (and (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)) (and (or (not instance.EVENT.1__AT0@0) (<= time__AT0@0 time__AT0@1)) (= event_is_timed__AT0@0 instance.EVENT.1__AT0@0))))))
(assert (= BAV17 (or (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0)) (and (or (not instance.EVENT.1__AT0@0) (and (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))) (<= time__AT0@0 time__AT0@1))) (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1))))))
(assert (= BAV18 (and (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))))
(assert (= BAV19 (and (or (not instance.EVENT.1__AT0@0) (and (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))) (<= time__AT0@0 time__AT0@1))) (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)))))
(assert (= BAV20 (or (not instance.EVENT.1__AT0@0) (and (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))) (<= time__AT0@0 time__AT0@1)))))
(assert (= BAV21 (and (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))) (<= time__AT0@0 time__AT0@1))))
(assert (= BAV22 (and (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1))) (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6)))))
(assert (= BAV23 (or (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1)) (not (= time__AT0@0 time__AT0@1)))))
(assert (= BAV24 (and (= instance.y__AT0@0 instance.y__AT0@1) (= instance.x__AT0@0 instance.x__AT0@1))))
(assert (= BAV25 (= instance.y__AT0@0 instance.y__AT0@1)))
(assert (= BAV26 (= instance.x__AT0@0 instance.x__AT0@1)))
(assert (= BAV27 (= time__AT0@0 time__AT0@1)))
(assert (= BAV28 (and (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8) (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6))))
(assert (= BAV29 (= (+ instance.x__AT0@1 (+ instance.y__AT0@1 (* (- GEN7) (* (+ instance.y__AT0@0 instance.x__AT0@0) (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)))))) GEN8)))
(assert (= BAV30 (= (+ instance.x__AT0@1 (+ (* (- GEN5) instance.y__AT0@1) (* (- GEN4) (* (exp (+ (* (- GEN3) time__AT0@0) time__AT0@1)) (+ instance.x__AT0@0 (* (- GEN2) instance.y__AT0@0)))))) GEN6)))
(assert (= BAV31 (<= time__AT0@0 time__AT0@1)))
(assert (= BAV32 (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1))))
(assert (= BAV33 (= time__AT0@0 time__AT0@1)))
(assert (= BAV34 (and (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1)) (and (or (not instance.EVENT.1__AT0@0) (<= time__AT0@0 time__AT0@1)) (= event_is_timed__AT0@0 instance.EVENT.1__AT0@0)))))
(assert (= BAV35 (or instance.EVENT.1__AT0@0 (= time__AT0@0 time__AT0@1))))
(assert (= BAV36 (= time__AT0@0 time__AT0@1)))
(assert (= BAV37 (and (or (not instance.EVENT.1__AT0@0) (<= time__AT0@0 time__AT0@1)) (= event_is_timed__AT0@0 instance.EVENT.1__AT0@0))))
(assert (= BAV38 (or (not instance.EVENT.1__AT0@0) (<= time__AT0@0 time__AT0@1))))
(assert (= BAV39 (<= time__AT0@0 time__AT0@1)))
(assert (= BAV40 (= event_is_timed__AT0@0 instance.EVENT.1__AT0@0)))
(assert (= BAV41 (or (not instance.EVENT.0__AT0@0) (not instance.EVENT.1__AT0@0))))
(assert (= BAV42 (<= GEN1 instance.x__AT0@1)))
(check-sat)
(get-model)
(exit)