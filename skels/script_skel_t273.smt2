(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(set-option :pp.max-depth 100)
(declare-const a Int)
(declare-const b Int)
(assert (= (+ (* GEN1 a) (* GEN2 b)) GEN3))
(apply (and-then simplify nla2bv))
(assert (and (<= GEN4 a) (<= a GEN5)))
(assert (and (<= GEN6 b) (<= b GEN7)))
(apply (and-then simplify nla2bv))
(set-option :produce-models true)
(apply (and-then simplify nla2bv) :print-model-converter true)
(assert (= BAV1 (= (+ (* GEN1 a) (* GEN2 b)) GEN3)))
(assert (= BAV2 (and (<= GEN4 a) (<= a GEN5))))
(assert (= BAV3 (<= GEN4 a)))
(assert (= BAV4 (<= a GEN5)))
(assert (= BAV5 (and (<= GEN6 b) (<= b GEN7))))
(assert (= BAV6 (<= GEN6 b)))
(assert (= BAV7 (<= b GEN7)))