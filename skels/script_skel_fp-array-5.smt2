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
(declare-const GEN30 Int)
(declare-const GEN29 Int)
(declare-const GEN28 Int)
(declare-const GEN27 Int)
(declare-const GEN26 Int)
(declare-const GEN25 Int)
(declare-const GEN24 Int)
(declare-const GEN23 Int)
(declare-const GEN22 Int)
(declare-const GEN21 Int)
(declare-const GEN20 Int)
(declare-const GEN19 Int)
(declare-const GEN18 Int)
(declare-const GEN17 Int)
(declare-const GEN16 Int)
(declare-const GEN15 Int)
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
(set-info :source |Handcrafted by Christoph M. Wintersteiger (cwinter@microsoft.com).|)
(set-option :model_validate true)
(set-option :smt.relevancy 2)
(set-option :rewriter.hi_fp_unspecified false)
(declare-fun A () (Array (_ FloatingPoint 8 24) (_ FloatingPoint 11 53)))
(declare-fun A2 () (Array (_ FloatingPoint 8 24) (_ FloatingPoint 11 53)))
(assert (not (= A A2)))
(assert (= (select A (_ +oo 8 24)) (select A (_ -oo 8 24))))
(assert (not (= (select A (_ +oo 8 24)) (_ NaN 11 53))))
(assert (not (= (select A (_ +oo 8 24)) (_ +zero 11 53))))
(declare-fun B () (Array Int (_ FloatingPoint 11 53)))
(assert (not (= (select B GEN13) (select B GEN14))))
(declare-fun C () (Array (_ FloatingPoint 11 53) Int))
(assert (not (= (select C (_ +oo 11 53)) GEN17)))
(assert (not (= (select C (_ -oo 11 53)) GEN20)))
(declare-fun D () (Array Int (Array Int (_ FloatingPoint 11 53))))
(assert (not (= (select (select D GEN21) GEN22) (_ -oo 11 53))))
(assert (not (= (select (select D GEN25) GEN26) (_ +oo 11 53))))
(declare-fun AP () (_ FloatingPoint 11 53))
(assert (= AP (select A (_ +oo 8 24))))
(check-sat-using (then simplify fpa2bv smt))
(get-value (C))
(exit)
(assert (= BAV1 (= A A2)))
(assert (= BAV2 (= (select A (_ +oo 8 24)) (select A (_ -oo 8 24)))))
(assert (= BAV3 (= (select A (_ +oo 8 24)) (_ NaN 11 53))))
(assert (= BAV4 (= (select A (_ +oo 8 24)) (_ +zero 11 53))))
(assert (= BAV5 (= (select B GEN13) (select B GEN14))))
(assert (= BAV6 (= (select C (_ +oo 11 53)) GEN17)))
(assert (= BAV7 (= (select C (_ -oo 11 53)) GEN20)))
(assert (= BAV8 (= (select (select D GEN21) GEN22) (_ -oo 11 53))))
(assert (= BAV9 (= (select (select D GEN25) GEN26) (_ +oo 11 53))))
(assert (= BAV10 (= AP (select A (_ +oo 8 24)))))