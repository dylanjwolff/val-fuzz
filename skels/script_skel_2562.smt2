(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const GEN11 Int)
(declare-const GEN10 Int)
(declare-const GEN9 Int)
(declare-const GEN8 Int)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 String)
(declare-const GEN4 String)
(declare-const GEN3 String)
(declare-const GEN2 String)
(declare-const GEN1 Int)
(declare-fun a () String)
(declare-fun b () String)
(declare-fun c () String)
(assert (= (str.substr a GEN1 (str.len b)) GEN2))
(assert (= (ite (= (str.len (str.substr (str.replace (str.replace a b GEN3) GEN4 GEN5) GEN6 (- (str.len c) GEN7))) GEN8) GEN9 GEN10) GEN11))
(assert (= BAV1 (= (str.substr a GEN1 (str.len b)) GEN2)))
(assert (= BAV2 (= (ite (= (str.len (str.substr (str.replace (str.replace a b GEN3) GEN4 GEN5) GEN6 (- (str.len c) GEN7))) GEN8) GEN9 GEN10) GEN11)))
(assert (= BAV3 (= (str.len (str.substr (str.replace (str.replace a b GEN3) GEN4 GEN5) GEN6 (- (str.len c) GEN7))) GEN8)))
(check-sat)
(get-model)