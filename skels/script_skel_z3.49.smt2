(set-option :auto-config true)
(declare-const GEN1 (_ BitVec 8))
(declare-const GEN2 (_ BitVec 8))
(declare-const GEN3 (_ BitVec 8))
(declare-const GEN4 (_ BitVec 8))
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-const BAV8 Bool)
(set-option :produce-models true)
(set-option :smt.mbqi true)
(define-sort Char () (_ BitVec 8))
(declare-fun f  (Char) Char)
(declare-fun f1 (Char) Char)
(declare-const a Char)
(assert (bvugt a GEN1))
(assert (= (f1 (bvadd a GEN2)) GEN3))
(assert (forall ((x Char)) (or (= x (bvadd a GEN4)) (= (f1 x) (f x)))))
(assert (= BAV5 (= (f1 (bvadd a GEN2)) GEN3)))
(assert (= BAV6 (forall ((x Char)) (or (= x (bvadd a GEN4)) (= (f1 x) (f x))))))
(assert (= BAV7 (forall ((x Char)) (= x (bvadd a GEN4)))))
(assert (= BAV8 (forall ((x Char)) (= (f1 x) (f x)))))
(check-sat)
(get-model)