(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-sort S 0)
(declare-sort T 0)
(declare-fun s1 () S)
(declare-fun s2 () S)
(declare-fun t1 () T)
(declare-fun t2 () T)
(declare-fun a () (_ BitVec 4))
(declare-fun b () (_ BitVec 4))
(declare-fun f (S) (_ BitVec 4))
(declare-fun g (S) S)
(declare-fun h (T) S)
(declare-fun i (T) T)
(assert (= (f s1) (bvand a b)))
(assert (= (f s2) (bvand a b)))
(assert (= (f (g s1)) (f (h (i t1)))))
(assert (not (= (f (g (h (i t2)))) (f (h (i t2))))))
(assert (= t1 t2))
(assert (= s1 (h (i t2))))
(assert (= BAV1 (= (f s1) (bvand a b))))
(assert (= BAV2 (= (f s2) (bvand a b))))
(assert (= BAV3 (= (f (g s1)) (f (h (i t1))))))
(assert (= BAV4 (= (f (g (h (i t2)))) (f (h (i t2))))))
(assert (= BAV5 (= t1 t2)))
(assert (= BAV6 (= s1 (h (i t2)))))
(check-sat)
(exit)