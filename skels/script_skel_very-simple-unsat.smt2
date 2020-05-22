(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const GEN1 Int)
(set-info :source |
Harald Roman Zankl <Harald.Zankl@uibk.ac.at>

|)
(set-info :smt-lib-version 2.0)
(set-info :category "crafted")
(declare-fun a () Real)
(assert (= (* a a) (- GEN1)))
(assert (= BAV1 (= (* a a) (- GEN1))))
(check-sat)
(get-model)
(exit)