(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const GEN8 Int)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(define-sort FPN () (_ FloatingPoint 11 53))
(declare-const x FPN)
(declare-const y FPN)
(assert (= x (fp.max (_ +zero 11 53) (_ -zero 11 53))))
(assert (= y (fp.max (_ +zero 11 53) (_ -zero 11 53))))
(assert (not (= x y)))
(assert (= BAV1 (= x (fp.max (_ +zero 11 53) (_ -zero 11 53)))))
(assert (= BAV2 (= y (fp.max (_ +zero 11 53) (_ -zero 11 53)))))
(assert (= BAV3 (= x y)))
(check-sat)
(get-model)
(check-sat-using smt)
(exit)