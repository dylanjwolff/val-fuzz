(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-sort i_ 0)
(declare-fun i2_ () i_)
(declare-fun i1_ () i_)
(declare-fun i3_ () i_)
(assert (and (not (= i2_ i3_)) (not (= i1_ i2_)) (not (= i1_ i3_))))
(declare-fun i4_ () i_)
(declare-fun i5_ () i_)
(declare-fun i6_ () i_)
(assert (distinct i4_ i5_ i6_))
(declare-fun P (i_) Bool)
(declare-fun f (i_) i_)
(assert (forall ((x i_)) (P (f x))))
(assert (= BAV1 (and (not (= i2_ i3_)) (not (= i1_ i2_)) (not (= i1_ i3_)))))
(assert (= BAV2 (= i2_ i3_)))
(assert (= BAV3 (= i1_ i2_)))
(assert (= BAV4 (= i1_ i3_)))
(assert (= BAV5 (distinct i4_ i5_ i6_)))
(check-sat)