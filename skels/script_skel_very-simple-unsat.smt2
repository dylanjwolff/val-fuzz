(set-logic ALL)
(declare-const GEN1 Int)
(declare-const BAV2 Bool)
(set-info :source |
Harald Roman Zankl <Harald.Zankl@uibk.ac.at>

|)
(set-info :smt-lib-version 2.0)
(set-info :category "crafted")
(declare-fun a () Real)
(assert (= (* a a) (- GEN1)))
(assert (= BAV2 (= (* a a) (- GEN1))))
(check-sat)
(exit)