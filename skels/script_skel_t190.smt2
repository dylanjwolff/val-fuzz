(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
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
(set-option :pp.max-depth 100)
(declare-fun p (Int) Bool)
(declare-fun f (Int Real) Bool)
(declare-fun q (Int Bool) Bool)
(declare-fun a () Int)
(push)
(assert (= (ite (> (ite (! (p GEN1) :lblneg baa) GEN2 GEN3) a) GEN4 GEN5) a))
(apply snf)
(echo "----")
(pop)
(push)
(assert (= (ite (> (ite (! (p GEN6) :lblpos baa) GEN7 GEN8) a) GEN9 GEN10) a))
(apply snf)
(echo "----")
(pop)
(assert (= BAV1 (= (ite (> (ite (! (p GEN1) :lblneg baa) GEN2 GEN3) a) GEN4 GEN5) a)))
(assert (= BAV2 (> (ite (! (p GEN1) :lblneg baa) GEN2 GEN3) a)))
(assert (= BAV3 (= (ite (> (ite (! (p GEN6) :lblpos baa) GEN7 GEN8) a) GEN9 GEN10) a)))
(assert (= BAV4 (> (ite (! (p GEN6) :lblpos baa) GEN7 GEN8) a)))