(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const GEN6 String)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 String)
(declare-const GEN2 String)
(declare-const GEN1 String)
(declare-fun x () String)
(declare-fun y () String)
(assert (and (= (str.replace x x GEN1) GEN2) (= (str.replace (str.++ x y) x GEN3) (str.++ x y)) (= (str.replace (str.++ x y) (str.substr x GEN4 GEN5) GEN6) y)))
(assert (= BAV1 (and (= (str.replace x x GEN1) GEN2) (= (str.replace (str.++ x y) x GEN3) (str.++ x y)) (= (str.replace (str.++ x y) (str.substr x GEN4 GEN5) GEN6) y))))
(assert (= BAV2 (= (str.replace x x GEN1) GEN2)))
(assert (= BAV3 (= (str.replace (str.++ x y) x GEN3) (str.++ x y))))
(assert (= BAV4 (= (str.replace (str.++ x y) (str.substr x GEN4 GEN5) GEN6) y)))
(check-sat)
(get-model)