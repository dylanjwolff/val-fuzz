(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const GEN4 Real)
(declare-const GEN3 Real)
(declare-const GEN2 Real)
(declare-const GEN1 Real)
(declare-const X Real)
(declare-const K Real)
(declare-const W Real)
(declare-const Z Real)
(define-fun grant-poly () Real
(+ (* X (* X (* X X)))
(+ (* (- 2.) (* K (* X (* X X))))
(+ (* (* K K) (* X X))
(+ (* 6. (* W (* X (* X X))))
(+ (* (- 6.) (* W (* K (* X X))))
(+ (* 9. (* (* W W) (* X X)))
(+ (* (- 4.) (* Z (* X (* X X))))
(+ (* 4. (* Z (* K (* X X))))
(+ (* (- 12.) (* Z (* W (* X X))))
(* 4. (* (* Z Z) (* X X)))))))))))))
(push)
(assert (> grant-poly GEN1))
(apply factor)
(pop)
(push)
(assert (>= grant-poly GEN2))
(apply factor)
(pop)
(push)
(assert (<= grant-poly GEN3))
(apply factor)
(pop)
(push)
(assert (< grant-poly GEN4))
(apply factor)
(pop)
(assert (= BAV1 (> grant-poly GEN1)))
(assert (= BAV2 (>= grant-poly GEN2)))
(assert (= BAV3 (<= grant-poly GEN3)))
(assert (= BAV4 (< grant-poly GEN4)))