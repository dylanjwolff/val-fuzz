(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(set-info :source | 
  Boogie/Spec# benchmarks.
  This benchmark was translated by Michal Moskal.
|)
(set-info :smt-lib-version 2.0)
(set-info :category "industrial")
(declare-fun select2 (Int) Int)
(declare-fun store2 (Int) Int)
(assert (forall ((?A Int)(?o Int)(?f Int)(?p Int)(?g Int)(?v Int)) (=> (not (= ?o ?p)) (= (select2 (store2 ?A)) (select2 ?A)))))
(assert (= BAV1 (forall ((?A Int)(?o Int)(?f Int)(?p Int)(?g Int)(?v Int)) (=> (not (= ?o ?p)) (= (select2 (store2 ?A)) (select2 ?A))))))
(assert (= BAV2 (forall ((?A Int)(?o Int)(?f Int)(?p Int)(?g Int)(?v Int)) (= ?o ?p))))
(assert (= BAV3 (forall ((?A Int)(?o Int)(?f Int)(?p Int)(?g Int)(?v Int)) (= (select2 (store2 ?A)) (select2 ?A)))))
(check-sat)
(get-info :reason-unknown)
(exit)