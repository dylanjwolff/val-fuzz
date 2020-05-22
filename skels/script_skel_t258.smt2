(declare-const BAV1 Bool)
(declare-const GEN1 Real)
(declare-const a Real)
(declare-const b Real)
(assert (< b (- GEN1)))
(apply (and-then (! add-bounds :add-bound-upper 10 :add-bound-lower 0) simplify fm))
(check-sat-using (and-then (! add-bounds :add-bound-upper 10 :add-bound-lower 0) simplify fm))
(check-sat-using (and-then simplify (or-else
                                     (and-then (! add-bounds :add-bound-upper 10 :add-bound-lower 0) fm)
                                     fm)))
(check-sat-using (and-then simplify (or-else
                                     (and-then (! add-bounds :add-bound-upper 10 :add-bound-lower 0) fm fail-if-undecided)
                                     fm)))
(assert (= BAV1 (< b (- GEN1))))