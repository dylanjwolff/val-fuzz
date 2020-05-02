(set-logic ALL)
(declare-fun a () Int)
(declare-fun b () Int)
(get-qe (exists ((x Int)) (and (<= a x) (<= x b))))