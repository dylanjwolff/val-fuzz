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
(declare-fun a () Real)
(declare-fun h () Real)
(declare-fun R () Real)
(declare-fun b () Real)
(assert (and (not (= b GEN1)) (< GEN2 R) (< GEN3 b) (< GEN4 h) (= (+ (+ (+ (- (- (- (* GEN5 (* (* a a) (* (* h h) (* (* b b) (* b b))))) (* GEN6 (* (* a a) (* (* b b) (* (* b b) (* b b)))))) (* GEN7 (* (* (* a a) (* R R)) (* (* h h) (* b b))))) (* GEN8 (* (* R R) (* (* h h) (* (* b b) (* b b)))))) (* (* (* b b) (* b b)) (* (* b b) (* b b)))) (* (* (* a a) (* a a)) (* (* b b) (* b b)))) (* GEN9 (* (* (* R R) (* R R)) (* (* h h) (* h h))))) GEN10) (> GEN11 (* (- (- (* GEN12 (* R h)) (* a b)) (* b b)) b)) (< GEN13 (* R (* h b))) (< GEN14 (* (- (+ (* GEN15 (* R h)) (* a b)) (* b b)) b)) (< GEN16 (* (- (+ (* b b) (* GEN17 (* R h))) (* a b)) b))))
(eliminate-quantifiers (exists b))
(exit)
(assert (= BAV1 (and (not (= b GEN1)) (< GEN2 R) (< GEN3 b) (< GEN4 h) (= (+ (+ (+ (- (- (- (* GEN5 (* (* a a) (* (* h h) (* (* b b) (* b b))))) (* GEN6 (* (* a a) (* (* b b) (* (* b b) (* b b)))))) (* GEN7 (* (* (* a a) (* R R)) (* (* h h) (* b b))))) (* GEN8 (* (* R R) (* (* h h) (* (* b b) (* b b)))))) (* (* (* b b) (* b b)) (* (* b b) (* b b)))) (* (* (* a a) (* a a)) (* (* b b) (* b b)))) (* GEN9 (* (* (* R R) (* R R)) (* (* h h) (* h h))))) GEN10) (> GEN11 (* (- (- (* GEN12 (* R h)) (* a b)) (* b b)) b)) (< GEN13 (* R (* h b))) (< GEN14 (* (- (+ (* GEN15 (* R h)) (* a b)) (* b b)) b)) (< GEN16 (* (- (+ (* b b) (* GEN17 (* R h))) (* a b)) b)))))
(assert (= BAV2 (= b GEN1)))
(assert (= BAV3 (< GEN2 R)))
(assert (= BAV4 (< GEN3 b)))
(assert (= BAV5 (< GEN4 h)))
(assert (= BAV6 (= (+ (+ (+ (- (- (- (* GEN5 (* (* a a) (* (* h h) (* (* b b) (* b b))))) (* GEN6 (* (* a a) (* (* b b) (* (* b b) (* b b)))))) (* GEN7 (* (* (* a a) (* R R)) (* (* h h) (* b b))))) (* GEN8 (* (* R R) (* (* h h) (* (* b b) (* b b)))))) (* (* (* b b) (* b b)) (* (* b b) (* b b)))) (* (* (* a a) (* a a)) (* (* b b) (* b b)))) (* GEN9 (* (* (* R R) (* R R)) (* (* h h) (* h h))))) GEN10)))
(assert (= BAV7 (> GEN11 (* (- (- (* GEN12 (* R h)) (* a b)) (* b b)) b))))
(assert (= BAV8 (< GEN13 (* R (* h b)))))
(assert (= BAV9 (< GEN14 (* (- (+ (* GEN15 (* R h)) (* a b)) (* b b)) b))))
(assert (= BAV10 (< GEN16 (* (- (+ (* b b) (* GEN17 (* R h))) (* a b)) b))))