(set-info :smt-lib-version 2.6)
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
(declare-fun v () (_ BitVec 32))
(declare-fun g () (_ BitVec 32))
(declare-fun t () (_ BitVec 32))
(declare-fun V () (_ BitVec 32))
(declare-fun vuscore2dollarskuscore0 () (_ BitVec 32))
(declare-fun huscore2dollarskuscore0 () (_ BitVec 32))
(declare-fun c () (_ BitVec 32))
(declare-fun t1uscore0dollarskuscore0 () (_ BitVec 32))
(declare-fun tuscore2dollarskuscore0 () (_ BitVec 32))
(declare-fun ts1uscore0 () (_ BitVec 32))
(declare-fun h () (_ BitVec 32))
(assert (not (exists ((ts1uscore0 (_ BitVec 32))) (=> (and (and (and (and (and (and (and (and (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32))) (bvsle vuscore2dollarskuscore0 (bvadd (bvmul (bvneg g) tuscore2dollarskuscore0) V))) (bvsgt g (_ bv0 32))) (bvsle (_ bv0 32) c)) (bvslt c (_ bv1 32))) (= h (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul t t)) (bvmul v t)))) (bvsge h (_ bv0 32))) (bvsge t (_ bv0 32))) (bvsle v (bvadd (bvmul (bvneg g) t) V))) (or (bvsgt (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0) (_ bv0 32)) (= (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul t1uscore0dollarskuscore0 t1uscore0dollarskuscore0))) (bvmul (bvmul (_ bv2 32) t1uscore0dollarskuscore0) vuscore2dollarskuscore0))) (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0) (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0))) (bvmul (bvadd (bvmul (bvmul (bvneg (_ bv1 32)) g) t1uscore0dollarskuscore0) vuscore2dollarskuscore0) (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0)))))))))
(assert (= BAV1 (=> (and (and (and (and (and (and (and (and (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32))) (bvsle vuscore2dollarskuscore0 (bvadd (bvmul (bvneg g) tuscore2dollarskuscore0) V))) (bvsgt g (_ bv0 32))) (bvsle (_ bv0 32) c)) (bvslt c (_ bv1 32))) (= h (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul t t)) (bvmul v t)))) (bvsge h (_ bv0 32))) (bvsge t (_ bv0 32))) (bvsle v (bvadd (bvmul (bvneg g) t) V))) (or (bvsgt (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0) (_ bv0 32)) (= (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul t1uscore0dollarskuscore0 t1uscore0dollarskuscore0))) (bvmul (bvmul (_ bv2 32) t1uscore0dollarskuscore0) vuscore2dollarskuscore0))) (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0) (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0))) (bvmul (bvadd (bvmul (bvmul (bvneg (_ bv1 32)) g) t1uscore0dollarskuscore0) vuscore2dollarskuscore0) (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0))))))))
(assert (= BAV2 (and (and (and (and (and (and (and (and (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32))) (bvsle vuscore2dollarskuscore0 (bvadd (bvmul (bvneg g) tuscore2dollarskuscore0) V))) (bvsgt g (_ bv0 32))) (bvsle (_ bv0 32) c)) (bvslt c (_ bv1 32))) (= h (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul t t)) (bvmul v t)))) (bvsge h (_ bv0 32))) (bvsge t (_ bv0 32))) (bvsle v (bvadd (bvmul (bvneg g) t) V)))))
(assert (= BAV3 (and (and (and (and (and (and (and (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32))) (bvsle vuscore2dollarskuscore0 (bvadd (bvmul (bvneg g) tuscore2dollarskuscore0) V))) (bvsgt g (_ bv0 32))) (bvsle (_ bv0 32) c)) (bvslt c (_ bv1 32))) (= h (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul t t)) (bvmul v t)))) (bvsge h (_ bv0 32))) (bvsge t (_ bv0 32)))))
(assert (= BAV4 (and (and (and (and (and (and (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32))) (bvsle vuscore2dollarskuscore0 (bvadd (bvmul (bvneg g) tuscore2dollarskuscore0) V))) (bvsgt g (_ bv0 32))) (bvsle (_ bv0 32) c)) (bvslt c (_ bv1 32))) (= h (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul t t)) (bvmul v t)))) (bvsge h (_ bv0 32)))))
(assert (= BAV5 (and (and (and (and (and (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32))) (bvsle vuscore2dollarskuscore0 (bvadd (bvmul (bvneg g) tuscore2dollarskuscore0) V))) (bvsgt g (_ bv0 32))) (bvsle (_ bv0 32) c)) (bvslt c (_ bv1 32))) (= h (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul t t)) (bvmul v t))))))
(assert (= BAV6 (and (and (and (and (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32))) (bvsle vuscore2dollarskuscore0 (bvadd (bvmul (bvneg g) tuscore2dollarskuscore0) V))) (bvsgt g (_ bv0 32))) (bvsle (_ bv0 32) c)) (bvslt c (_ bv1 32)))))
(assert (= BAV7 (and (and (and (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32))) (bvsle vuscore2dollarskuscore0 (bvadd (bvmul (bvneg g) tuscore2dollarskuscore0) V))) (bvsgt g (_ bv0 32))) (bvsle (_ bv0 32) c))))
(assert (= BAV8 (and (and (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32))) (bvsle vuscore2dollarskuscore0 (bvadd (bvmul (bvneg g) tuscore2dollarskuscore0) V))) (bvsgt g (_ bv0 32)))))
(assert (= BAV9 (and (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32))) (bvsle vuscore2dollarskuscore0 (bvadd (bvmul (bvneg g) tuscore2dollarskuscore0) V)))))
(assert (= BAV10 (and (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32))) (bvsge tuscore2dollarskuscore0 (_ bv0 32)))))
(assert (= BAV11 (and (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))) (bvsge huscore2dollarskuscore0 (_ bv0 32)))))
(assert (= BAV12 (and (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32))) (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0))))))
(assert (= BAV13 (and (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32))) (bvsge t1uscore0dollarskuscore0 (_ bv0 32)))))
(assert (= BAV14 (=> (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0)) (bvsge (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul ts1uscore0 ts1uscore0))) (bvmul (bvmul (_ bv2 32) ts1uscore0) vuscore2dollarskuscore0))) (_ bv0 32)))))
(assert (= BAV15 (and (bvsle (_ bv0 32) ts1uscore0) (bvsle ts1uscore0 t1uscore0dollarskuscore0))))
(assert (= BAV16 (= huscore2dollarskuscore0 (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul tuscore2dollarskuscore0 tuscore2dollarskuscore0)) (bvmul vuscore2dollarskuscore0 tuscore2dollarskuscore0)))))
(assert (= BAV17 (= h (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul t t)) (bvmul v t)))))
(assert (= BAV18 (or (bvsgt (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0) (_ bv0 32)) (= (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul t1uscore0dollarskuscore0 t1uscore0dollarskuscore0))) (bvmul (bvmul (_ bv2 32) t1uscore0dollarskuscore0) vuscore2dollarskuscore0))) (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0) (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0))) (bvmul (bvadd (bvmul (bvmul (bvneg (_ bv1 32)) g) t1uscore0dollarskuscore0) vuscore2dollarskuscore0) (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0)))))))
(assert (= BAV19 (= (bvmul (bvsdiv (_ bv1 32) (_ bv2 32)) (bvadd (bvadd (bvmul (_ bv2 32) huscore2dollarskuscore0) (bvmul (bvmul (bvneg (_ bv1 32)) g) (bvmul t1uscore0dollarskuscore0 t1uscore0dollarskuscore0))) (bvmul (bvmul (_ bv2 32) t1uscore0dollarskuscore0) vuscore2dollarskuscore0))) (bvadd (bvmul (bvsdiv g (_ bv2 32)) (bvmul (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0) (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0))) (bvmul (bvadd (bvmul (bvmul (bvneg (_ bv1 32)) g) t1uscore0dollarskuscore0) vuscore2dollarskuscore0) (bvadd t1uscore0dollarskuscore0 tuscore2dollarskuscore0))))))
(check-sat)
(get-model)
(exit)