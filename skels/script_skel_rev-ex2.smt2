(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const GEN2 Int)
(declare-const GEN1 String)
(declare-fun x () String)
(declare-fun y () String)
(declare-fun z () String)
(assert (= x (str.rev (str.++ y GEN1))))
(assert (> (str.len x) (+ (str.len y) GEN2)))
(assert (= BAV1 (= x (str.rev (str.++ y GEN1)))))
(assert (= BAV2 (> (str.len x) (+ (str.len y) GEN2))))
(check-sat)
(get-model)