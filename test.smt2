(declare-const RL_LET1 Real)
(declare-const c1 Real)
(assert (forall ((u Int)) (> u RL_LET1)))
(assert (= RL_LET1 (forall ((u Int)) (+ u c1))))