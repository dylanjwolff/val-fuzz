(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
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
(set-info :smt-lib-version 2.0)
(set-info :category "crafted")
(declare-fun x () (_ BitVec 3))
(assert (and (= ((_ extract 1 0) x) (concat ((_ extract 1 1) x) ((_ extract 0 0) x))) (= ((_ extract 0 0) x) ((_ extract 1 1) x)) (= ((_ extract 2 2) x) ((_ extract 1 1) x)) (= (_ bv0 1) ((_ extract 0 0) x)) (= x (concat ((_ extract 2 2) x) ((_ extract 1 0) x))) (not (= (_ bv0 3) x))))
(assert (= BAV1 (and (= ((_ extract 1 0) x) (concat ((_ extract 1 1) x) ((_ extract 0 0) x))) (= ((_ extract 0 0) x) ((_ extract 1 1) x)) (= ((_ extract 2 2) x) ((_ extract 1 1) x)) (= (_ bv0 1) ((_ extract 0 0) x)) (= x (concat ((_ extract 2 2) x) ((_ extract 1 0) x))) (not (= (_ bv0 3) x)))))
(assert (= BAV2 (= ((_ extract 1 0) x) (concat ((_ extract 1 1) x) ((_ extract 0 0) x)))))
(assert (= BAV3 (= ((_ extract 0 0) x) ((_ extract 1 1) x))))
(assert (= BAV4 (= ((_ extract 2 2) x) ((_ extract 1 1) x))))
(assert (= BAV5 (= (_ bv0 1) ((_ extract 0 0) x))))
(assert (= BAV6 (= x (concat ((_ extract 2 2) x) ((_ extract 1 0) x)))))
(assert (= BAV7 (= (_ bv0 3) x)))
(check-sat)
(get-model)
(exit)