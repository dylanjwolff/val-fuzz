(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(get-model)
(declare-const BAV3 Bool)
(get-model)
(declare-const BAV4 Bool)
(get-model)
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
(declare-fun x0 () Real)
(declare-fun x1 () Real)
(declare-fun x2 () Real)
(assert (= BAV1 (or (not (>= (+ (* GEN1 x1) (* (- GEN2) x2) (* (- GEN3) x1)) (- GEN4))) (not (<= (+ (* (- GEN5) x1) (* GEN6 x2) (* GEN7 x1)) (- GEN8))))))
(assert (= BAV2 (>= (+ (* GEN1 x1) (* (- GEN2) x2) (* (- GEN3) x1)) (- GEN4))))
(assert (= BAV3 (<= (+ (* (- GEN5) x1) (* GEN6 x2) (* GEN7 x1)) (- GEN8))))
(assert (= BAV4 (or (not (> (+ (* GEN9 x2) (* (- GEN10) x1)) (- GEN11))) (not (> (+ (* (- GEN12) x1) (* (- GEN13) x0)) (- GEN14))))))
(assert (= BAV5 (> (+ (* GEN9 x2) (* (- GEN10) x1)) (- GEN11))))
(assert (= BAV6 (> (+ (* (- GEN12) x1) (* (- GEN13) x0)) (- GEN14))))
(assert (= BAV7 (or (< (+ (* (- GEN15) x2) (* (- GEN16) x1) (* (- GEN17) x0) (* (- GEN18) x2)) GEN19) (<= (+ (* GEN20 x1) (* (- GEN21) x2)) (- GEN22)))))
(assert (= BAV8 (< (+ (* (- GEN15) x2) (* (- GEN16) x1) (* (- GEN17) x0) (* (- GEN18) x2)) GEN19)))
(assert (= BAV9 (<= (+ (* GEN20 x1) (* (- GEN21) x2)) (- GEN22))))
(assert (= BAV10 (or (> (+ (* (- GEN23) x1) (* GEN24 x0) (* (- GEN25) x0) (* (- GEN26) x2)) (- GEN27)) (< (+ (* GEN28 x2) (* GEN29 x0) (* GEN30 x2)) GEN31))))
(assert (= BAV11 (> (+ (* (- GEN23) x1) (* GEN24 x0) (* (- GEN25) x0) (* (- GEN26) x2)) (- GEN27))))
(assert (= BAV12 (< (+ (* GEN28 x2) (* GEN29 x0) (* GEN30 x2)) GEN31)))
(assert (= BAV13 (or (< (+ (* GEN32 x2) (* GEN33 x0) (* (- GEN34) x0) (* (- GEN35) x0)) GEN36) (= (+ (* (- GEN37) x1) (* (- GEN38) x1) (* (- GEN39) x0)) GEN40))))
(assert (= BAV14 (< (+ (* GEN32 x2) (* GEN33 x0) (* (- GEN34) x0) (* (- GEN35) x0)) GEN36)))
(assert (= BAV15 (= (+ (* (- GEN37) x1) (* (- GEN38) x1) (* (- GEN39) x0)) GEN40)))
(assert (= BAV16 (or (>= (+ (* GEN41 x0) (* GEN42 x2)) GEN43) (< (+ (* (- GEN44) x2) (* GEN45 x1) (* (- 16) x1)) 3))))
(assert (= BAV17 (>= (+ (* GEN41 x0) (* GEN42 x2)) GEN43)))
(assert (= BAV18 (< (+ (* (- GEN44) x2) (* GEN45 x1) (* (- 16) x1)) 3)))
(assert (= BAV19 (= (+ (* (- 17) x2) (* 34 x1) (* (- 20) x0) (* (- 47) x2)) (- 39))))
(assert (= BAV20 (or (< (+ (* 26 x2) (* (- 29) x2) (* 3 x2) (* (- 42) x0)) (- 10)) (not (<= (+ (* 9 x0) (* (- 31) x1)) (- 6))))))
(assert (= BAV21 (< (+ (* 26 x2) (* (- 29) x2) (* 3 x2) (* (- 42) x0)) (- 10))))
(assert (= BAV22 (<= (+ (* 9 x0) (* (- 31) x1)) (- 6))))
(assert (= BAV23 (or (not (< (+ (* (- 15) x0) (* (- 44) x1) (* (- 5) x2) (* 25 x2)) 38)) (= (+ (* (- 24) x1) (* 20 x0) (* 8 x0)) (- 10)) (not (> (+ (* (- 35) x1) (* (- 14) x2)) 6)))))
(assert (= BAV24 (< (+ (* (- 15) x0) (* (- 44) x1) (* (- 5) x2) (* 25 x2)) 38)))
(assert (= BAV25 (= (+ (* (- 24) x1) (* 20 x0) (* 8 x0)) (- 10))))
(assert (= BAV26 (> (+ (* (- 35) x1) (* (- 14) x2)) 6)))
(assert (= BAV27 (or (> (+ (* (- 13) x0) (* 10 x2)) (- 16)) (= (+ (* 38 x0) (* 38 x2) (* 3 x0)) 8) (not (<= (+ (* 18 x1) (* (- 26) x0)) (- 14))))))
(assert (= BAV28 (> (+ (* (- 13) x0) (* 10 x2)) (- 16))))
(assert (= BAV29 (= (+ (* 38 x0) (* 38 x2) (* 3 x0)) 8)))
(assert (= BAV30 (<= (+ (* 18 x1) (* (- 26) x0)) (- 14))))
(assert (= BAV31 (or (< (+ (* (- 11) x1) (* 42 x1)) (- 50)) (not (> (+ (* (- 24) x0) (* 8 x1) (* 44 x2) (* 27 x0)) 48)) (< (+ (* 2 x0) (* 32 x2)) (- 30)))))
(assert (= BAV32 (< (+ (* (- 11) x1) (* 42 x1)) (- 50))))
(assert (= BAV33 (> (+ (* (- 24) x0) (* 8 x1) (* 44 x2) (* 27 x0)) 48)))
(assert (= BAV34 (< (+ (* 2 x0) (* 32 x2)) (- 30))))
(assert (= BAV35 (>= (+ (* 28 x2) (* 29 x1) (* (- 7) x2) (* 16 x0)) (- 19))))
(assert (= BAV36 (or (not (> (+ (* 1 x1) (* (- 33) x1) (* (- 2) x1)) (- 29))) (>= (+ (* 7 x2) (* 4 x0) (* 49 x2)) (- 17)))))
(assert (= BAV37 (> (+ (* 1 x1) (* (- 33) x1) (* (- 2) x1)) (- 29))))
(assert (= BAV38 (>= (+ (* 7 x2) (* 4 x0) (* 49 x2)) (- 17))))
(check-sat)
(get-model)
(push 1)
(assert (or (not (>= (+ (* GEN1 x1) (* (- GEN2) x2) (* (- GEN3) x1)) (- GEN4))) (not (<= (+ (* (- GEN5) x1) (* GEN6 x2) (* GEN7 x1)) (- GEN8)))))
(assert (or (not (> (+ (* GEN9 x2) (* (- GEN10) x1)) (- GEN11))) (not (> (+ (* (- GEN12) x1) (* (- GEN13) x0)) (- GEN14)))))
(assert (or (< (+ (* (- GEN15) x2) (* (- GEN16) x1) (* (- GEN17) x0) (* (- GEN18) x2)) GEN19) (<= (+ (* GEN20 x1) (* (- GEN21) x2)) (- GEN22))))
(check-sat)
(push 1)
(assert (or (> (+ (* (- GEN23) x1) (* GEN24 x0) (* (- GEN25) x0) (* (- GEN26) x2)) (- GEN27)) (< (+ (* GEN28 x2) (* GEN29 x0) (* GEN30 x2)) GEN31)))
(assert (or (< (+ (* GEN32 x2) (* GEN33 x0) (* (- GEN34) x0) (* (- GEN35) x0)) GEN36) (= (+ (* (- GEN37) x1) (* (- GEN38) x1) (* (- GEN39) x0)) GEN40)))
(assert (or (>= (+ (* GEN41 x0) (* GEN42 x2)) GEN43) (< (+ (* (- GEN44) x2) (* GEN45 x1) (* (- 16) x1)) 3)))
(assert (= (+ (* (- 17) x2) (* 34 x1) (* (- 20) x0) (* (- 47) x2)) (- 39)))
(check-sat)
(push 1)
(assert (or (< (+ (* 26 x2) (* (- 29) x2) (* 3 x2) (* (- 42) x0)) (- 10)) (not (<= (+ (* 9 x0) (* (- 31) x1)) (- 6)))))
(assert (or (not (< (+ (* (- 15) x0) (* (- 44) x1) (* (- 5) x2) (* 25 x2)) 38)) (= (+ (* (- 24) x1) (* 20 x0) (* 8 x0)) (- 10)) (not (> (+ (* (- 35) x1) (* (- 14) x2)) 6))))
(assert (or (> (+ (* (- 13) x0) (* 10 x2)) (- 16)) (= (+ (* 38 x0) (* 38 x2) (* 3 x0)) 8) (not (<= (+ (* 18 x1) (* (- 26) x0)) (- 14)))))
(check-sat)
(push 1)
(assert (or (< (+ (* (- 11) x1) (* 42 x1)) (- 50)) (not (> (+ (* (- 24) x0) (* 8 x1) (* 44 x2) (* 27 x0)) 48)) (< (+ (* 2 x0) (* 32 x2)) (- 30))))
(assert (>= (+ (* 28 x2) (* 29 x1) (* (- 7) x2) (* 16 x0)) (- 19)))
(check-sat)
(pop 1)
(assert (or (not (> (+ (* 1 x1) (* (- 33) x1) (* (- 2) x1)) (- 29))) (>= (+ (* 7 x2) (* 4 x0) (* 49 x2)) (- 17))))
(check-sat)
(push 1)