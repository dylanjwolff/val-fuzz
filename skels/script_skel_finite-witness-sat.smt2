(set-logic ALL)
(declare-sort Loc 0)
(declare-const l Loc)
(assert (not (_ emp Loc Loc)))
(assert (forall ((x Loc)(y Loc)) (not (pto x y))))
(check-sat)
(get-model)