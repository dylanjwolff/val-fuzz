(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
(declare-const GEN4 Int)
(declare-const GEN5 Int)
(declare-const GEN6 Int)
(declare-const BAV7 Bool)
(declare-const BAV8 Bool)
(declare-const BAV9 Bool)
(declare-sort $$unsorted 0)
(declare-fun $$rtu (Real) $$unsorted)
(declare-fun $$utr ($$unsorted) Real)
(declare-fun p ($$unsorted) Bool)
(assert (and (= ($$utr ($$rtu GEN1)) GEN2) (= ($$utr ($$rtu (/ GEN3 GEN4))) (/ GEN5 GEN6))))
(assert (forall ((x $$unsorted)) (p x)))
(assert (= BAV7 (and (= ($$utr ($$rtu GEN1)) GEN2) (= ($$utr ($$rtu (/ GEN3 GEN4))) (/ GEN5 GEN6)))))
(assert (= BAV8 (= ($$utr ($$rtu GEN1)) GEN2)))
(assert (= BAV9 (= ($$utr ($$rtu (/ GEN3 GEN4))) (/ GEN5 GEN6))))
(check-sat)