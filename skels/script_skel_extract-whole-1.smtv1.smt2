(set-option :incremental false)
(set-logic ALL)
(declare-const BAV1 Bool)
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
(declare-fun x () (_ BitVec 32))
(assert (= BAV1 (= (concat (concat (concat (concat ((_ extract 31 31) x) ((_ extract 30 20) x)) ((_ extract 19 10) x)) ((_ extract 9 1) x)) ((_ extract 0 0) x)) x)))
(check-sat-assuming ((not (= (concat (concat (concat (concat ((_ extract 31 31) x) ((_ extract 30 20) x)) ((_ extract 19 10) x)) ((_ extract 9 1) x)) ((_ extract 0 0) x)) x))))
(get-model)