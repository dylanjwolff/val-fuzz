(set-info :source |fuzzsmt|)
(set-info :smt-lib-version 2.0)
(set-info :category "random")
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
(declare-const BAV11 Bool)
(declare-const BAV12 Bool)
(declare-const BAV13 Bool)
(declare-const BAV14 Bool)
(declare-const BAV15 Bool)
(declare-const BAV16 Bool)
(declare-const BAV17 Bool)
(declare-const BAV18 Bool)
(declare-const BAV19 Bool)
(declare-const BAV20 Bool)
(declare-const BAV21 Bool)
(declare-const BAV22 Bool)
(declare-const BAV23 Bool)
(declare-const BAV24 Bool)
(declare-const BAV25 Bool)
(declare-const BAV26 Bool)
(declare-const BAV27 Bool)
(declare-const BAV28 Bool)
(declare-const BAV29 Bool)
(declare-const BAV30 Bool)
(declare-const BAV31 Bool)
(declare-const BAV32 Bool)
(declare-const BAV33 Bool)
(declare-const BAV34 Bool)
(declare-const BAV35 Bool)
(declare-const BAV36 Bool)
(declare-const BAV37 Bool)
(declare-const BAV38 Bool)
(declare-const BAV39 Bool)
(declare-const BAV40 Bool)
(declare-const BAV41 Bool)
(declare-const BAV42 Bool)
(declare-const BAV43 Bool)
(declare-const BAV44 Bool)
(declare-const BAV45 Bool)
(declare-const BAV46 Bool)
(declare-const BAV47 Bool)
(declare-const BAV48 Bool)
(declare-const BAV49 Bool)
(declare-const BAV50 Bool)
(declare-const BAV51 Bool)
(declare-const BAV52 Bool)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(define-sort Element () Int)
(declare-fun f0 ( Int) Int)
(declare-fun f1 ( (Set Element) (Set Element) (Set Element)) (Set Element))
(declare-fun p0 ( Int Int) Bool)
(declare-fun p1 ( (Set Element)) Bool)
(declare-fun v0 () Int)
(declare-fun v1 () (Set Element))
(declare-fun v2 () (Set Element))
(declare-fun v3 () (Set Element))
(declare-fun v4 () (Set Element))
(assert (= (= (not (and (=> (>= v0 (f0 v0)) (not (member v0 (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1))))) (= (member (f0 v0) (f1 (setminus v2 v2) v1 v4)) (= (xor (member (f0 (* GEN1 v0)) (setminus v2 v2)) (member (f0 (* GEN1 v0)) v1)) (p0 (* v0 (- GEN1)) (* v0 (- GEN1))))))) (and (member (* v0 GEN1) (union v3 v4)) (member (* v0 GEN1) (union v3 v4)))) (and (and (and (not (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))) (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0)))) (or (ite (not (member v0 (union v3 v4))) (= (* (- GEN1) (f0 v0)) (* v0 GEN1)) (not (member v0 (union v3 v4)))) (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4))))) (and (and (not (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))) (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0)))) (or (ite (not (member v0 (union v3 v4))) (= (* (- GEN1) (f0 v0)) (* v0 GEN1)) (not (member v0 (union v3 v4)))) (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4))))))))
(assert (= BAV1 (= (= (not (and (=> (>= v0 (f0 v0)) (not (member v0 (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1))))) (= (member (f0 v0) (f1 (setminus v2 v2) v1 v4)) (= (xor (member (f0 (* GEN1 v0)) (setminus v2 v2)) (member (f0 (* GEN1 v0)) v1)) (p0 (* v0 (- GEN1)) (* v0 (- GEN1))))))) (and (member (* v0 GEN1) (union v3 v4)) (member (* v0 GEN1) (union v3 v4)))) (and (and (and (not (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))) (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0)))) (or (ite (not (member v0 (union v3 v4))) (= (* (- GEN1) (f0 v0)) (* v0 GEN1)) (not (member v0 (union v3 v4)))) (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4))))) (and (and (not (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))) (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0)))) (or (ite (not (member v0 (union v3 v4))) (= (* (- GEN1) (f0 v0)) (* v0 GEN1)) (not (member v0 (union v3 v4)))) (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4)))))))))
(assert (= BAV2 (= (not (and (=> (>= v0 (f0 v0)) (not (member v0 (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1))))) (= (member (f0 v0) (f1 (setminus v2 v2) v1 v4)) (= (xor (member (f0 (* GEN1 v0)) (setminus v2 v2)) (member (f0 (* GEN1 v0)) v1)) (p0 (* v0 (- GEN1)) (* v0 (- GEN1))))))) (and (member (* v0 GEN1) (union v3 v4)) (member (* v0 GEN1) (union v3 v4))))))
(assert (= BAV3 (and (=> (>= v0 (f0 v0)) (not (member v0 (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1))))) (= (member (f0 v0) (f1 (setminus v2 v2) v1 v4)) (= (xor (member (f0 (* GEN1 v0)) (setminus v2 v2)) (member (f0 (* GEN1 v0)) v1)) (p0 (* v0 (- GEN1)) (* v0 (- GEN1))))))))
(assert (= BAV4 (=> (>= v0 (f0 v0)) (not (member v0 (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))))))
(assert (= BAV5 (>= v0 (f0 v0))))
(assert (= BAV6 (= (member (f0 v0) (f1 (setminus v2 v2) v1 v4)) (= (xor (member (f0 (* GEN1 v0)) (setminus v2 v2)) (member (f0 (* GEN1 v0)) v1)) (p0 (* v0 (- GEN1)) (* v0 (- GEN1)))))))
(assert (= BAV7 (= (xor (member (f0 (* GEN1 v0)) (setminus v2 v2)) (member (f0 (* GEN1 v0)) v1)) (p0 (* v0 (- GEN1)) (* v0 (- GEN1))))))
(assert (= BAV8 (xor (member (f0 (* GEN1 v0)) (setminus v2 v2)) (member (f0 (* GEN1 v0)) v1))))
(assert (= BAV9 (and (member (* v0 GEN1) (union v3 v4)) (member (* v0 GEN1) (union v3 v4)))))
(assert (= BAV10 (and (and (and (not (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))) (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0)))) (or (ite (not (member v0 (union v3 v4))) (= (* (- GEN1) (f0 v0)) (* v0 GEN1)) (not (member v0 (union v3 v4)))) (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4))))) (and (and (not (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))) (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0)))) (or (ite (not (member v0 (union v3 v4))) (= (* (- GEN1) (f0 v0)) (* v0 GEN1)) (not (member v0 (union v3 v4)))) (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4))))))))
(assert (= BAV11 (and (and (not (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))) (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0)))) (or (ite (not (member v0 (union v3 v4))) (= (* (- GEN1) (f0 v0)) (* v0 GEN1)) (not (member v0 (union v3 v4)))) (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4)))))))
(assert (= BAV12 (and (not (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))) (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0))))))
(assert (= BAV13 (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))))
(assert (= BAV14 (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0)))))
(assert (= BAV15 (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0))))))
(assert (= BAV16 (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0)))))
(assert (= BAV17 (> (* GEN1 v0) (* (- GEN1) (f0 v0)))))
(assert (= BAV18 (< (* v0 GEN1) (f0 v0))))
(assert (= BAV19 (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))))
(assert (= BAV20 (<= (* (- GEN1) (f0 v0)) (f0 v0))))
(assert (= BAV21 (or (ite (not (member v0 (union v3 v4))) (= (* (- GEN1) (f0 v0)) (* v0 GEN1)) (not (member v0 (union v3 v4)))) (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4))))))
(assert (= BAV22 (= (* (- GEN1) (f0 v0)) (* v0 GEN1))))
(assert (= BAV23 (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4)))))
(assert (= BAV24 (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4))))
(assert (= BAV25 (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2))))))
(assert (= BAV26 (< (f0 v0) (* (- GEN1) (f0 v0)))))
(assert (= BAV27 (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))))
(assert (= BAV28 (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1))))))
(assert (= BAV29 (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)))))
(assert (= BAV30 (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))))
(assert (= BAV31 (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))))
(assert (= BAV32 (and (and (not (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))) (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0)))) (or (ite (not (member v0 (union v3 v4))) (= (* (- GEN1) (f0 v0)) (* v0 GEN1)) (not (member v0 (union v3 v4)))) (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4)))))))
(assert (= BAV33 (and (not (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))) (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0))))))
(assert (= BAV34 (= (member (f0 (* GEN1 v0)) (f1 (setminus v2 v2) v1 v4)) (member (* v0 GEN1) (f1 (setminus v2 v2) v1 v4)))))
(assert (= BAV35 (xor (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))) (<= (* (- GEN1) (f0 v0)) (f0 v0)))))
(assert (= BAV36 (xor (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0))) (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0))))))
(assert (= BAV37 (and (> (* GEN1 v0) (* (- GEN1) (f0 v0))) (< (* v0 GEN1) (f0 v0)))))
(assert (= BAV38 (> (* GEN1 v0) (* (- GEN1) (f0 v0)))))
(assert (= BAV39 (< (* v0 GEN1) (f0 v0))))
(assert (= BAV40 (< (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) (f0 (* GEN1 v0)))))
(assert (= BAV41 (<= (* (- GEN1) (f0 v0)) (f0 v0))))
(assert (= BAV42 (or (ite (not (member v0 (union v3 v4))) (= (* (- GEN1) (f0 v0)) (* v0 GEN1)) (not (member v0 (union v3 v4)))) (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4))))))
(assert (= BAV43 (= (* (- GEN1) (f0 v0)) (* v0 GEN1))))
(assert (= BAV44 (=> (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4)) (member (f0 v0) (union v3 v4)))))
(assert (= BAV45 (and (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))) (member (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3) v4))))
(assert (= BAV46 (= (< (f0 v0) (* (- GEN1) (f0 v0))) (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2))))))
(assert (= BAV47 (< (f0 v0) (* (- GEN1) (f0 v0)))))
(assert (= BAV48 (and (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1)))) (member (* v0 (- GEN1)) (setminus v2 v2)))))
(assert (= BAV49 (or (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))) (member (f0 (* GEN1 v0)) (intersection (f1 (setminus v2 v2) v1 v4) (f1 v1 v4 v1))))))
(assert (= BAV50 (xor (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)) (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3)))))
(assert (= BAV51 (= (* (- GEN1) (f0 v0)) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))))
(assert (= BAV52 (= (* GEN1 v0) (ite (p0 (* v0 GEN1) (* GEN1 v0)) GEN2 GEN3))))
(check-sat)
(get-model)