(set-info :smt-lib-version 2.6)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const GEN14 Int)
(declare-const GEN13 Int)
(declare-const GEN12 Int)
(declare-const GEN11 Int)
(declare-const GEN10 Int)
(declare-const GEN9 Int)
(declare-const GEN8 Int)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun a () (_ BitVec 64))
(declare-fun b () (_ BitVec 64))
(assert (fp.leq ((_ to_fp 11 53) a) ((_ to_fp 11 53) (_ bv4626322717216342016 64))))
(assert (not (fp.isNaN ((_ to_fp 11 53) b))))
(declare-fun k2 () (_ BitVec 64))
(assert (or (= k2 b) (= k2 a)))
(assert (or (fp.isNaN ((_ to_fp 11 53) k2)) (fp.gt ((_ to_fp 11 53) k2) ((_ to_fp 11 53) (_ bv4626322717216342016 64)))))
(assert (= BAV1 (or (= k2 b) (= k2 a))))
(assert (= BAV2 (= k2 b)))
(assert (= BAV3 (= k2 a)))
(assert (= BAV4 (or (fp.isNaN ((_ to_fp 11 53) k2)) (fp.gt ((_ to_fp 11 53) k2) ((_ to_fp 11 53) (_ bv4626322717216342016 64))))))
(check-sat)
(get-model)