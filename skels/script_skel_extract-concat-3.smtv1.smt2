(set-option :incremental false)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun x () (_ BitVec 32))
(declare-fun y () (_ BitVec 32))
(assert (= BAV1 (= ((_ extract 60 35) (concat x y)) ((_ extract 28 3) x))))
(check-sat-assuming ((not (= ((_ extract 60 35) (concat x y)) ((_ extract 28 3) x)))))
(get-model)