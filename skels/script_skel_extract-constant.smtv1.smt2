(set-option :incremental false)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(assert (= BAV1 (= ((_ extract 6 2) (_ bv56 9)) (_ bv14 5))))
(check-sat-assuming ((not (= ((_ extract 6 2) (_ bv56 9)) (_ bv14 5)))))
(get-model)