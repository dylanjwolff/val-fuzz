(set-logic ALL)
(declare-const GEN1 Real)
(declare-const GEN2 Real)
(declare-const GEN3 Real)
(declare-const GEN4 Real)
(declare-const GEN5 Real)
(declare-const GEN6 Real)
(declare-const GEN7 Real)
(declare-const BAV8 Bool)
(declare-const BAV9 Bool)
(declare-const BAV10 Bool)
(declare-const BAV11 Bool)
(declare-const BAV12 Bool)
(declare-const BAV13 Bool)
(declare-const BAV14 Bool)
( set-info :source | SMT-COMP'06 organizers |)
( set-info :smt-lib-version 2.0)
( set-info :category "check")
( declare-datatypes ()
  ( ( Even ( ECons ( EElem Real) ( OTl Odd)) ( ENil))
    ( Odd ( OCons ( OElem Real) ( ETl Even)) ( ONil))))
( declare-fun SumOdd ( Odd) Real)
( declare-fun SumEven ( Even) Real)
( declare-fun v1 () Real)
( declare-fun v2 () Real)
( declare-fun e1 () Even)
( declare-fun e2 () Even)
( declare-fun e3 () Even)
( declare-fun o1 () Odd)
( declare-fun o2 () Odd)
( declare-fun o3 () Odd)
(assert (= e1 (ECons GEN1 o2)))
(assert (= e2 (ECons GEN2 o3)))
(assert (= o1 (OCons GEN3 e2)))
(assert (= o2 (OCons GEN4 e3)))
(assert (= (SumEven e3) GEN5))
(assert (= (SumOdd o3) GEN6))
(assert (= (SumEven e1) GEN7))
(assert (= BAV8 (= e1 (ECons GEN1 o2))))
(assert (= BAV9 (= e2 (ECons GEN2 o3))))
(assert (= BAV10 (= o1 (OCons GEN3 e2))))
(assert (= BAV11 (= o2 (OCons GEN4 e3))))
(assert (= BAV12 (= (SumEven e3) GEN5)))
(assert (= BAV13 (= (SumOdd o3) GEN6)))
(assert (= BAV14 (= (SumEven e1) GEN7)))
(check-sat)
(exit)