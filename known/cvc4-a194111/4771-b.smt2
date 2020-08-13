(set-logic ALL)
(declare-fun arr-6808665888080864687_6808665888091689937-0 () (Array (_ BitVec 12) (_ BitVec 6)))
(assert (let ((_let_6 (store arr-6808665888080864687_6808665888091689937-0 (concat #b010001 (_ bv0 6)) (_ bv0 6)))) (= (store _let_6 (_ bv0 12) #b010001) (store _let_6 (_ bv0 12) (_ bv0 6)))))
(check-sat)
