(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const GEN8 String)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(get-model)
(declare-const GEN5 String)
(declare-const GEN4 String)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 String)
(set-option :model_validate true)
(declare-fun name () String)
(assert (distinct (str.at name (str.indexof GEN1 (str.substr name (str.len name) GEN2) GEN3)) GEN4))
(assert (= BAV1 (distinct (str.at name (str.indexof GEN1 (str.substr name (str.len name) GEN2) GEN3)) GEN4)))
(assert (= BAV2 (= (str.at name (str.indexof " " (str.substr name (str.len name) 1) 2)) "")))
(check-sat)
(get-model)
(reset)
(set-option :model_validate true)
(declare-fun name () String)
(assert (= (str.at name (str.indexof " " (str.substr name (str.len name) 1) 2)) ""))
(check-sat)