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
(set-info :source | Example extracted from Peter Baumgartner's talk at CADE-21: Logical Engineering with Instance-Based Methods.

It was translated to SMT-LIB by Leonardo de Moura |)
(set-info :smt-lib-version 2.0)
(set-info :category "crafted")
(declare-fun symmetric ((Array Int (Array Int Real)) Int) Bool)
(declare-fun n () Int)
(declare-fun a0 () (Array Int (Array Int Real)))
(declare-fun e0 () Real)
(declare-fun a1 () (Array Int (Array Int Real)))
(declare-fun e1 () Real)
(declare-fun a2 () (Array Int (Array Int Real)))
(declare-fun e2 () Real)
(declare-fun a3 () (Array Int (Array Int Real)))
(declare-fun e3 () Real)
(declare-fun a4 () (Array Int (Array Int Real)))
(declare-fun e4 () Real)
(declare-fun a5 () (Array Int (Array Int Real)))
(declare-fun e5 () Real)
(declare-fun a6 () (Array Int (Array Int Real)))
(declare-fun e6 () Real)
(assert (forall ((?a (Array Int (Array Int Real)))(?n Int)) (= (symmetric ?a ?n) (forall ((?i Int)(?j Int)) (=> (and (<= GEN1 ?i) (<= ?i ?n) (<= GEN2 ?j) (<= ?j ?n)) (= (select (select ?a ?i) ?j) (select (select ?a ?j) ?i)))))))
(assert (symmetric a0 n))
(assert (= a1 (store a0 GEN3 (store (select a0 GEN4) GEN5 e0))))
(assert (= a2 (store a1 GEN6 (store (select a1 GEN7) GEN8 e1))))
(assert (= a3 (store a2 GEN9 (store (select a2 GEN10) GEN11 e2))))
(assert (= a4 (store a3 GEN12 (store (select a3 GEN13) GEN14 e3))))
(assert (= a5 (store a4 GEN15 (store (select a4 GEN16) GEN17 e4))))
(assert (= a6 (store a5 GEN18 (store (select a5 GEN19) GEN20 e5))))
(assert (not (symmetric a6 n)))
(assert (= BAV1 (forall ((?a (Array Int (Array Int Real)))(?n Int)) (= (symmetric ?a ?n) (forall ((?i Int)(?j Int)) (=> (and (<= GEN1 ?i) (<= ?i ?n) (<= GEN2 ?j) (<= ?j ?n)) (= (select (select ?a ?i) ?j) (select (select ?a ?j) ?i))))))))
(assert (= BAV2 (forall ((?a (Array Int (Array Int Real)))(?n Int)(?i Int)(?j Int)) (=> (and (<= GEN1 ?i) (<= ?i ?n) (<= GEN2 ?j) (<= ?j ?n)) (= (select (select ?a ?i) ?j) (select (select ?a ?j) ?i))))))
(assert (= BAV3 (forall ((?a (Array Int (Array Int Real)))(?n Int)(?i Int)(?j Int)) (and (<= GEN1 ?i) (<= ?i ?n) (<= GEN2 ?j) (<= ?j ?n)))))
(assert (= BAV4 (forall ((?a (Array Int (Array Int Real)))(?n Int)(?i Int)(?j Int)) (<= GEN1 ?i))))
(assert (= BAV5 (forall ((?a (Array Int (Array Int Real)))(?n Int)(?i Int)(?j Int)) (<= ?i ?n))))
(assert (= BAV6 (forall ((?a (Array Int (Array Int Real)))(?n Int)(?i Int)(?j Int)) (<= GEN2 ?j))))
(assert (= BAV7 (forall ((?a (Array Int (Array Int Real)))(?n Int)(?i Int)(?j Int)) (<= ?j ?n))))
(assert (= BAV8 (forall ((?a (Array Int (Array Int Real)))(?n Int)(?i Int)(?j Int)) (= (select (select ?a ?i) ?j) (select (select ?a ?j) ?i)))))
(assert (= BAV9 (= a1 (store a0 GEN3 (store (select a0 GEN4) GEN5 e0)))))
(assert (= BAV10 (= a2 (store a1 GEN6 (store (select a1 GEN7) GEN8 e1)))))
(assert (= BAV11 (= a3 (store a2 GEN9 (store (select a2 GEN10) GEN11 e2)))))
(assert (= BAV12 (= a4 (store a3 GEN12 (store (select a3 GEN13) GEN14 e3)))))
(assert (= BAV13 (= a5 (store a4 GEN15 (store (select a4 GEN16) GEN17 e4)))))
(assert (= BAV14 (= a6 (store a5 GEN18 (store (select a5 GEN19) GEN20 e5)))))
(check-sat)
(get-model)
(exit)