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
(declare-const GEN8 Int)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-const A (Seq Int))
(declare-const m Int)
(assert (<= GEN1 m))
(assert (< m (seq.len A)))
(assert (forall ((i Int)) (<= (seq.nth A i) (seq.nth A m))))
(declare-const f_A (Seq Int))
(assert (= f_A (ite (or (>= (seq.nth A m) (seq.len A)) (< (seq.nth A m) GEN2)) (seq.unit GEN3) (seq.++ (seq.extract A GEN4 m) (seq.extract A (+ m GEN5) (- (- (seq.len A) GEN6) m))))))
(define-fun spec ((in (Seq Int))) Bool
	(let (
		(n (seq.len in)))
	(and
		(forall ((j Int))
			(=>
				(and
					(<= 0 j)
					(<  j 1))
				(and
					(<= 0 (seq.nth in j))
					(<  (seq.nth in j) n))))))
)
(assert (and (spec f_A) (not (spec A))))
(declare-const spec_f_A Bool)
(assert (= (spec f_A) spec_f_A))
(assert (< GEN7 (seq.len A)))
(assert (< (seq.len A) GEN8))
(assert (= BAV1 (<= GEN1 m)))
(assert (= BAV2 (< m (seq.len A))))
(assert (= BAV3 (forall ((i Int)) (<= (seq.nth A i) (seq.nth A m)))))
(assert (= BAV4 (= f_A (ite (or (>= (seq.nth A m) (seq.len A)) (< (seq.nth A m) GEN2)) (seq.unit GEN3) (seq.++ (seq.extract A GEN4 m) (seq.extract A (+ m GEN5) (- (- (seq.len A) GEN6) m)))))))
(assert (= BAV5 (or (>= (seq.nth A m) (seq.len A)) (< (seq.nth A m) GEN2))))
(assert (= BAV6 (>= (seq.nth A m) (seq.len A))))
(assert (= BAV7 (< (seq.nth A m) GEN2)))
(assert (= BAV8 (and (spec f_A) (not (spec A)))))
(assert (= BAV9 (= (spec f_A) spec_f_A)))
(assert (= BAV10 (< GEN7 (seq.len A))))
(assert (= BAV11 (< (seq.len A) GEN8)))
(check-sat)
(get-model)