(set-logic ALL)
(declare-const GEN1 Real)
(declare-const GEN2 Int)
(declare-const GEN3 Real)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-fun a () Real)
(declare-fun b () Real)
(declare-fun c () Real)
(declare-fun d () Real)
(declare-fun e () Real)
(assert (not (exists ((g Real)) (= (< GEN1 (* b d)) (distinct (* (to_real GEN2) a c) GEN3)))))
(assert (= a (* c e)))
(assert (= BAV4 (= (< GEN1 (* b d)) (distinct (* (to_real GEN2) a c) GEN3))))
(assert (= BAV5 (< GEN1 (* b d))))
(assert (= BAV6 (distinct (* (to_real GEN2) a c) GEN3)))
(assert (= BAV7 (= a (* c e))))
(check-sat)