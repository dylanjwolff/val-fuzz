(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-fun a () Int)
(declare-fun b () Int)
(declare-fun c () Int)
(declare-fun d () Int)
(assert (= c d))
(assert (> (+ a (div GEN1 b)) c))
(assert (= b (- GEN2)))
(assert (= BAV3 (= c d)))
(assert (= BAV4 (> (+ a (div GEN1 b)) c)))
(assert (= BAV5 (= b (- GEN2))))
(check-sat)