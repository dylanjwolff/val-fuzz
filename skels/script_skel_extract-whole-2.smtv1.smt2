(set-option :incremental false)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun x () (_ BitVec 32))
(assert (= BAV1 (= (concat (concat (concat (concat (concat (concat x (_ bv0 1)) (_ bv1 1)) (_ bv0 1)) (_ bv1 1)) (_ bv0 1)) (_ bv1 1)) (concat x (_ bv21 6)))))
(check-sat-assuming ((not (= (concat (concat (concat (concat (concat (concat x (_ bv0 1)) (_ bv1 1)) (_ bv0 1)) (_ bv1 1)) (_ bv0 1)) (_ bv1 1)) (concat x (_ bv21 6))))))
(get-model)