(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
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
(declare-const BAV20 Bool)
(declare-const BAV21 Bool)
(define-fun findIdx ((y1 Int)(y2 Int)(k1 Int)) Int (div (* (- 7) (mod y1 (- 5))) 2))
(declare-fun x1 () Int)
(declare-fun x2 () Int)
(declare-fun k () Int)
(assert (not (and (=> (< x1 x2) (=> (< k x1) (= (findIdx x1 x2 k) GEN1))) (=> (< x1 x2) (=> (> k x2) (= (findIdx x1 x2 k) GEN2))) (=> (< x1 x2) (=> (and (> k x1) (< k x2)) (= (findIdx x1 x2 k) GEN3))))))
(assert (= BAV4 (and (=> (< x1 x2) (=> (< k x1) (= (findIdx x1 x2 k) GEN1))) (=> (< x1 x2) (=> (> k x2) (= (findIdx x1 x2 k) GEN2))) (=> (< x1 x2) (=> (and (> k x1) (< k x2)) (= (findIdx x1 x2 k) GEN3))))))
(assert (= BAV5 (=> (< x1 x2) (=> (< k x1) (= (findIdx x1 x2 k) GEN1)))))
(assert (= BAV6 (< x1 x2)))
(assert (= BAV7 (=> (< k x1) (= (findIdx x1 x2 k) GEN1))))
(assert (= BAV8 (< k x1)))
(assert (= BAV9 (= (findIdx x1 x2 k) GEN1)))
(assert (= BAV10 (=> (< x1 x2) (=> (> k x2) (= (findIdx x1 x2 k) GEN2)))))
(assert (= BAV11 (< x1 x2)))
(assert (= BAV12 (=> (> k x2) (= (findIdx x1 x2 k) GEN2))))
(assert (= BAV13 (> k x2)))
(assert (= BAV14 (= (findIdx x1 x2 k) GEN2)))
(assert (= BAV15 (=> (< x1 x2) (=> (and (> k x1) (< k x2)) (= (findIdx x1 x2 k) GEN3)))))
(assert (= BAV16 (< x1 x2)))
(assert (= BAV17 (=> (and (> k x1) (< k x2)) (= (findIdx x1 x2 k) GEN3))))
(assert (= BAV18 (and (> k x1) (< k x2))))
(assert (= BAV19 (> k x1)))
(assert (= BAV20 (< k x2)))
(assert (= BAV21 (= (findIdx x1 x2 k) GEN3)))
(check-sat)