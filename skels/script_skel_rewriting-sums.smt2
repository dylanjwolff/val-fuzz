(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
(declare-const GEN4 Int)
(declare-const GEN5 Int)
(declare-const GEN6 Int)
(declare-const GEN7 Int)
(declare-const GEN8 Int)
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
(declare-fun x () Int)
(declare-fun y () Int)
(declare-fun z () Int)
(assert (or (= x GEN1) (= x GEN2) (= x GEN3)))
(assert (or (= y (+ x GEN4)) (= y (+ x GEN5))))
(assert (or (= z (+ y GEN6)) (= z (+ y GEN7))))
(assert (> (* z z) GEN8))
(assert (= BAV9 (or (= x GEN1) (= x GEN2) (= x GEN3))))
(assert (= BAV10 (= x GEN1)))
(assert (= BAV11 (= x GEN2)))
(assert (= BAV12 (= x GEN3)))
(assert (= BAV13 (or (= y (+ x GEN4)) (= y (+ x GEN5)))))
(assert (= BAV14 (= y (+ x GEN4))))
(assert (= BAV15 (= y (+ x GEN5))))
(assert (= BAV16 (or (= z (+ y GEN6)) (= z (+ y GEN7)))))
(assert (= BAV17 (= z (+ y GEN6))))
(assert (= BAV18 (= z (+ y GEN7))))
(assert (= BAV19 (> (* z z) GEN8)))
(check-sat)