(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-const BAV8 Bool)
(declare-const BAV9 Bool)
(declare-const BAV10 Bool)
(declare-const BAV11 Bool)
(declare-const BAV12 Bool)
(declare-const BAV13 Bool)
(declare-const BAV14 Bool)
(declare-const BAV15 Bool)
(declare-const BAV16 Bool)
(declare-const BAV17 Bool)
(declare-const BAV18 Bool)
(declare-const BAV19 Bool)
(declare-sort var 0)
(declare-sort reg 0)
(declare-fun var5_1 () var)
(declare-fun b_1 () var)
(declare-fun a_1 () var)
(declare-fun r0 () reg)
(declare-fun r1 () reg)
(declare-fun r2 () reg)
(declare-fun r3 () reg)
(assert (not (= r0 r1)))
(assert (not (= r0 r2)))
(assert (not (= r0 r3)))
(assert (not (= r1 r2)))
(assert (not (= r1 r3)))
(assert (not (= r2 r3)))
(declare-fun assign (var) reg)
(assert (or (= (assign var5_1) r0) (= (assign var5_1) r1) (= (assign var5_1) r2) (= (assign var5_1) r3)))
(assert (or (= (assign b_1) r1)))
(assert (or (= (assign a_1) r0)))
(assert (not (= (assign b_1) (assign a_1))))
(assert (= (assign var5_1) r0))
(assert (= (assign b_1) r1))
(assert (= (assign a_1) r0))
(assert (= BAV1 (= r0 r1)))
(assert (= BAV2 (= r0 r2)))
(assert (= BAV3 (= r0 r3)))
(assert (= BAV4 (= r1 r2)))
(assert (= BAV5 (= r1 r3)))
(assert (= BAV6 (= r2 r3)))
(assert (= BAV7 (or (= (assign var5_1) r0) (= (assign var5_1) r1) (= (assign var5_1) r2) (= (assign var5_1) r3))))
(assert (= BAV8 (= (assign var5_1) r0)))
(assert (= BAV9 (= (assign var5_1) r1)))
(assert (= BAV10 (= (assign var5_1) r2)))
(assert (= BAV11 (= (assign var5_1) r3)))
(assert (= BAV12 (or (= (assign b_1) r1))))
(assert (= BAV13 (= (assign b_1) r1)))
(assert (= BAV14 (or (= (assign a_1) r0))))
(assert (= BAV15 (= (assign a_1) r0)))
(assert (= BAV16 (= (assign b_1) (assign a_1))))
(assert (= BAV17 (= (assign var5_1) r0)))
(assert (= BAV18 (= (assign b_1) r1)))
(assert (= BAV19 (= (assign a_1) r0)))
(check-sat)
(exit)