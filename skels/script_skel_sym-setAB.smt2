(set-logic ALL)
(declare-const BAV1 Bool)
(declare-fun x () Int)
(declare-fun y () Int)
(declare-fun A () (Set Int))
(declare-fun B () (Set Int))
(declare-fun C () (Set Int))
(assert (and (member x A) (member x B) (member x C)))
(assert (member y C))
(assert (= BAV1 (and (member x A) (member x B) (member x C))))
(check-sat)
(get-model)