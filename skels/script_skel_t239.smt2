(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const GEN2 Real)
(declare-const GEN1 Real)
(set-option :produce-models true)
(declare-const x Real)
(declare-const y Real)
(declare-const z Real)
(assert (= (/ GEN1 x) y))
(assert (not (= x GEN2)))
(apply purify-arith :print-model-converter true)
(apply (and-then purify-arith propagate-values) :print-model-converter true)
(assert (= BAV1 (= (/ GEN1 x) y)))
(assert (= BAV2 (= x GEN2)))