(set-logic ALL)
(declare-const BAV1 Bool)
(declare-sort Atom 0)
(declare-fun j ((Set Atom)) Atom)
(declare-fun kk (Atom (Set Atom)) (Set Atom))
(declare-fun n () (Set Atom))
(assert (forall ((b Atom)) (= (as emptyset (Set Atom)) (kk (j (singleton b)) n))))
(assert (= BAV1 (forall ((b Atom)) (= (as emptyset (Set Atom)) (kk (j (singleton b)) n)))))
(check-sat)
(get-model)