(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-const BAV8 Bool)
(declare-const BAV9 Bool)
(set-info :source |Written by D. B. Staple for GitHub issue #629.|)
(declare-fun a0 () Bool)
(declare-fun a1 () Bool)
(declare-fun c0 () Bool)
(declare-fun c1 () Bool)
(declare-fun b0 () Bool)
(declare-fun b1 () Bool)
(declare-fun b2 () Bool)
(assert (or (not (=> (and (and (not b2) (not b1)) (not b0)) (= c0 a0))) (not (=> (and (and (not b2) (not b1)) (not b0)) (= c1 a1)))))
(check-sat-using (and-then (par-or qfbv qfbv)))
(assert (= BAV1 (or (not (=> (and (and (not b2) (not b1)) (not b0)) (= c0 a0))) (not (=> (and (and (not b2) (not b1)) (not b0)) (= c1 a1))))))
(assert (= BAV2 (=> (and (and (not b2) (not b1)) (not b0)) (= c0 a0))))
(assert (= BAV3 (and (and (not b2) (not b1)) (not b0))))
(assert (= BAV4 (and (not b2) (not b1))))
(assert (= BAV5 (= c0 a0)))
(assert (= BAV6 (=> (and (and (not b2) (not b1)) (not b0)) (= c1 a1))))
(assert (= BAV7 (and (and (not b2) (not b1)) (not b0))))
(assert (= BAV8 (and (not b2) (not b1))))
(assert (= BAV9 (= c1 a1)))