(set-logic ALL)
(declare-const BAV1 Bool)
(get-model)
(get-model)
(get-model)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-const BAV8 Bool)
(get-model)
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
(declare-fun x0 () Real)
(declare-fun x1 () Real)
(declare-fun x2 () Real)
(assert (not (<= (+ (* (- GEN1) x0) (* GEN2 x0) (* GEN3 x1)) (- GEN4))))
(assert (or (not (> (+ (* (- GEN5) x0) (* GEN6 x1)) (- GEN7))) (not (>= (+ (* GEN8 x0) (* GEN9 x0) (* (- GEN10) x1) (* (- GEN11) x0)) (- GEN12)))))
(assert (= BAV1 (<= (+ (* (- GEN1) x0) (* GEN2 x0) (* GEN3 x1)) (- GEN4))))
(assert (= BAV2 (or (not (> (+ (* (- GEN5) x0) (* GEN6 x1)) (- GEN7))) (not (>= (+ (* GEN8 x0) (* GEN9 x0) (* (- GEN10) x1) (* (- GEN11) x0)) (- GEN12))))))
(assert (= BAV3 (> (+ (* (- GEN5) x0) (* GEN6 x1)) (- GEN7))))
(assert (= BAV4 (>= (+ (* GEN8 x0) (* GEN9 x0) (* (- GEN10) x1) (* (- GEN11) x0)) (- GEN12))))
(assert (= BAV5 (> (+ (* (- GEN13) x2) (* GEN14 x2)) GEN15)))
(assert (= BAV6 (= (+ (* GEN16 x1) (* (- GEN17) x2)) GEN18)))
(assert (= BAV7 (<= (+ (* GEN19 x0) (* (- GEN20) x1) (* GEN21 x1)) GEN22)))
(assert (= BAV8 (or (<= (+ (* GEN23 x0) (* (- GEN24) x0) (* GEN25 x1) (* GEN26 x1)) GEN27) (not (>= (+ (* GEN28 x1) (* (- GEN29) x1) (* (- GEN30) x1)) GEN31)) (not (>= (+ (* (- GEN32) x2) (* GEN33 x2) (* GEN34 x0) (* GEN35 x2)) GEN36)))))
(assert (= BAV9 (<= (+ (* GEN23 x0) (* (- GEN24) x0) (* GEN25 x1) (* GEN26 x1)) GEN27)))
(assert (= BAV10 (>= (+ (* GEN28 x1) (* (- GEN29) x1) (* (- GEN30) x1)) GEN31)))
(assert (= BAV11 (>= (+ (* (- GEN32) x2) (* GEN33 x2) (* GEN34 x0) (* GEN35 x2)) GEN36)))
(assert (= BAV12 (or (= (+ (* (- GEN37) x1) (* GEN38 x0) (* (- GEN39) x0)) GEN40) (not (<= (+ (* (- GEN41) x2) (* GEN42 x0) (* GEN43 x2)) GEN44)))))
(assert (= BAV13 (= (+ (* (- GEN37) x1) (* GEN38 x0) (* (- GEN39) x0)) GEN40)))
(assert (= BAV14 (<= (+ (* (- GEN41) x2) (* GEN42 x0) (* GEN43 x2)) GEN44)))
(assert (= BAV15 (or (<= (+ (* GEN45 x2) (* (- GEN46) x1)) (- GEN47)) (= (+ (* (- GEN48) x2) (* (- GEN49) x2) (* (- GEN50) x2) (* (- GEN51) x2)) (- 39)))))
(assert (= BAV16 (<= (+ (* GEN45 x2) (* (- GEN46) x1)) (- GEN47))))
(assert (= BAV17 (= (+ (* (- GEN48) x2) (* (- GEN49) x2) (* (- GEN50) x2) (* (- GEN51) x2)) (- 39))))
(assert (= BAV18 (or (= (+ (* 33 x2) (* 44 x0) (* (- 4) x1)) 5) (not (< (+ (* 27 x2) (* (- 45) x0) (* 43 x2) (* 40 x0)) 17)) (not (<= (+ (* (- 40) x2) (* 3 x0) (* 16 x2) (* (- 37) x1)) 29)))))
(assert (= BAV19 (= (+ (* 33 x2) (* 44 x0) (* (- 4) x1)) 5)))
(assert (= BAV20 (< (+ (* 27 x2) (* (- 45) x0) (* 43 x2) (* 40 x0)) 17)))
(assert (= BAV21 (<= (+ (* (- 40) x2) (* 3 x0) (* 16 x2) (* (- 37) x1)) 29)))
(assert (= BAV22 (or (= (+ (* (- 21) x0) (* 5 x2)) (- 27)) (not (<= (+ (* (- 20) x0) (* 19 x0) (* (- 50) x1) (* (- 24) x0)) (- 32))))))
(assert (= BAV23 (= (+ (* (- 21) x0) (* 5 x2)) (- 27))))
(assert (= BAV24 (<= (+ (* (- 20) x0) (* 19 x0) (* (- 50) x1) (* (- 24) x0)) (- 32))))
(assert (= BAV25 (<= (+ (* 9 x2) (* 0 x0) (* (- 40) x0) (* 49 x2)) (- 11))))
(assert (= BAV26 (or (not (< (+ (* (- 2) x0) (* 2 x2)) 19)) (= (+ (* (- 28) x1) (* (- 1) x2) (* (- 4) x1)) 38))))
(assert (= BAV27 (< (+ (* (- 2) x0) (* 2 x2)) 19)))
(assert (= BAV28 (= (+ (* (- 28) x1) (* (- 1) x2) (* (- 4) x1)) 38)))
(check-sat)
(get-model)
(push 1)
(assert (> (+ (* (- GEN13) x2) (* GEN14 x2)) GEN15))
(assert (not (= (+ (* GEN16 x1) (* (- GEN17) x2)) GEN18)))
(assert (not (<= (+ (* GEN19 x0) (* (- GEN20) x1) (* GEN21 x1)) GEN22)))
(assert (or (<= (+ (* GEN23 x0) (* (- GEN24) x0) (* GEN25 x1) (* GEN26 x1)) GEN27) (not (>= (+ (* GEN28 x1) (* (- GEN29) x1) (* (- GEN30) x1)) GEN31)) (not (>= (+ (* (- GEN32) x2) (* GEN33 x2) (* GEN34 x0) (* GEN35 x2)) GEN36))))
(assert (or (= (+ (* (- GEN37) x1) (* GEN38 x0) (* (- GEN39) x0)) GEN40) (not (<= (+ (* (- GEN41) x2) (* GEN42 x0) (* GEN43 x2)) GEN44))))
(assert (or (<= (+ (* GEN45 x2) (* (- GEN46) x1)) (- GEN47)) (= (+ (* (- GEN48) x2) (* (- GEN49) x2) (* (- GEN50) x2) (* (- GEN51) x2)) (- 39))))
(assert (or (= (+ (* 33 x2) (* 44 x0) (* (- 4) x1)) 5) (not (< (+ (* 27 x2) (* (- 45) x0) (* 43 x2) (* 40 x0)) 17)) (not (<= (+ (* (- 40) x2) (* 3 x0) (* 16 x2) (* (- 37) x1)) 29))))
(check-sat)
(push 1)
(check-sat)
(push 1)
(check-sat)
(push 1)
(assert (or (= (+ (* (- 21) x0) (* 5 x2)) (- 27)) (not (<= (+ (* (- 20) x0) (* 19 x0) (* (- 50) x1) (* (- 24) x0)) (- 32)))))
(check-sat)
(pop 1)
(assert (not (<= (+ (* 9 x2) (* 0 x0) (* (- 40) x0) (* 49 x2)) (- 11))))
(assert (or (not (< (+ (* (- 2) x0) (* 2 x2)) 19)) (= (+ (* (- 28) x1) (* (- 1) x2) (* (- 4) x1)) 38)))
(check-sat)
(push 1)