(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const GEN4 Real)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(set-option :pp.max-depth 100)
(declare-fun p (Int) Bool)
(declare-fun f (Int Real) Bool)
(declare-fun q (Int Bool) Bool)
(push)
(assert (or (= (p GEN1) (p GEN2)) (q GEN3 (forall ((x Int)) (f x GEN4)))))
(apply nnf)
(echo "----")
(apply snf)
(pop)
(assert (= BAV1 (or (= (p GEN1) (p GEN2)) (q GEN3 (forall ((x Int)) (f x GEN4))))))
(assert (= BAV2 (= (p GEN1) (p GEN2))))