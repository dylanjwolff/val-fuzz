(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const GEN9 Real)
(declare-const GEN8 Real)
(declare-const GEN7 Real)
(declare-const GEN6 Real)
(declare-const GEN5 Real)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-const a Real)
(declare-const b Real)
(assert (= (* (root-obj (+ (^ x 5) (* (- 1) x) (- 1)) 1) a) GEN5))
(assert (= (* (^ GEN6 (/ GEN7 GEN8)) b) GEN9))
(apply (! purify-arith :elim-root-objects false))
(assert (= BAV1 (= (* (root-obj (+ (^ x 5) (* (- 1) x) (- 1)) 1) a) GEN5)))
(assert (= BAV2 (= (* (^ GEN6 (/ GEN7 GEN8)) b) GEN9)))