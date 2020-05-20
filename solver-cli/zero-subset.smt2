(set-logic QF_NRA)
(declare-fun a () Real)
(declare-fun b () Real)
(declare-fun c () Real)
(declare-fun d () Real)
(declare-fun e () Real)

(assert (= (* a b c d) 0))

(assert (not (= (* a b c d e) 0)))

(check-sat)
