(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
(declare-const GEN4 Int)
(declare-const GEN5 Int)
(declare-const GEN6 Int)
(declare-const GEN7 Int)
(declare-const GEN8 Int)
(declare-const GEN9 Int)
(declare-const GEN10 Int)
(declare-const GEN11 Int)
(declare-const GEN12 Int)
(declare-const BAV13 Bool)
(declare-const BAV14 Bool)
(declare-const BAV15 Bool)
(declare-const BAV16 Bool)
(declare-const BAV17 Bool)
(declare-const BAV18 Bool)
(declare-const BAV19 Bool)
(declare-const BAV20 Bool)
(declare-const BAV21 Bool)
(declare-const BAV22 Bool)
(declare-const BAV23 Bool)
(declare-const BAV24 Bool)
(declare-const BAV25 Bool)
(declare-const BAV26 Bool)
(declare-const BAV27 Bool)
(assert (or (forall ((H Int)(G Int)) (= (= G GEN1) (= H GEN2))) (forall ((C Int)(D Int)(E Int)) (or (= (= D GEN3) (= C GEN4)) (and (not (forall ((G Int)) (= (= E GEN5) (= G GEN6)))) (not (forall ((A Int)) (not (= (ite (= A GEN7) GEN8 GEN9) (ite (= C GEN10) GEN11 GEN12))))))))))
(assert (= BAV13 (or (forall ((H Int)(G Int)) (= (= G GEN1) (= H GEN2))) (forall ((C Int)(D Int)(E Int)) (or (= (= D GEN3) (= C GEN4)) (and (not (forall ((G Int)) (= (= E GEN5) (= G GEN6)))) (not (forall ((A Int)) (not (= (ite (= A GEN7) GEN8 GEN9) (ite (= C GEN10) GEN11 GEN12)))))))))))
(assert (= BAV14 (forall ((H Int)(G Int)) (= (= G GEN1) (= H GEN2)))))
(assert (= BAV15 (forall ((H Int)(G Int)) (= G GEN1))))
(assert (= BAV16 (forall ((H Int)(G Int)) (= H GEN2))))
(assert (= BAV17 (forall ((C Int)(D Int)(E Int)) (or (= (= D GEN3) (= C GEN4)) (and (not (forall ((G Int)) (= (= E GEN5) (= G GEN6)))) (not (forall ((A Int)) (not (= (ite (= A GEN7) GEN8 GEN9) (ite (= C GEN10) GEN11 GEN12))))))))))
(assert (= BAV18 (forall ((C Int)(D Int)(E Int)) (= (= D GEN3) (= C GEN4)))))
(assert (= BAV19 (forall ((C Int)(D Int)(E Int)) (= D GEN3))))
(assert (= BAV20 (forall ((C Int)(D Int)(E Int)) (= C GEN4))))
(assert (= BAV21 (forall ((C Int)(D Int)(E Int)) (and (not (forall ((G Int)) (= (= E GEN5) (= G GEN6)))) (not (forall ((A Int)) (not (= (ite (= A GEN7) GEN8 GEN9) (ite (= C GEN10) GEN11 GEN12)))))))))
(assert (= BAV22 (forall ((C Int)(D Int)(E Int)(G Int)) (= (= E GEN5) (= G GEN6)))))
(assert (= BAV23 (forall ((C Int)(D Int)(E Int)(G Int)) (= E GEN5))))
(assert (= BAV24 (forall ((C Int)(D Int)(E Int)(G Int)) (= G GEN6))))
(assert (= BAV25 (forall ((C Int)(D Int)(E Int)(A Int)) (= (ite (= A GEN7) GEN8 GEN9) (ite (= C GEN10) GEN11 GEN12)))))
(assert (= BAV26 (forall ((C Int)(D Int)(E Int)(A Int)) (= A GEN7))))
(assert (= BAV27 (forall ((C Int)(D Int)(E Int)(A Int)) (= C GEN10))))
(check-sat)