(declare-const _yy Real)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
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
(declare-const BAV20 Bool)
(declare-const BAV21 Bool)
(declare-const BAV22 Bool)
(declare-const BAV23 Bool)
(declare-const _xx Real)
(declare-const _Y Real)
(declare-const _X Real)
(declare-const _x Real)
(declare-const _y Real)
(declare-const _z Real)
(assert (not (=> (and (and (and (<= _xx _x) (<= _x _X)) (and (<= _yy _y) (<= _y _Y))) (and (or (< _Y GEN1) (< GEN2 _yy)) (and (<= (/ _xx _yy) _z) (and (<= (/ _xx _Y) _z) (and (<= (/ _X _yy) _z) (<= (/ _X _Y) _z)))))) (<= (/ _x _y) _z))))
(assert (= BAV3 (=> (and (and (and (<= _xx _x) (<= _x _X)) (and (<= _yy _y) (<= _y _Y))) (and (or (< _Y GEN1) (< GEN2 _yy)) (and (<= (/ _xx _yy) _z) (and (<= (/ _xx _Y) _z) (and (<= (/ _X _yy) _z) (<= (/ _X _Y) _z)))))) (<= (/ _x _y) _z))))
(assert (= BAV4 (and (and (and (<= _xx _x) (<= _x _X)) (and (<= _yy _y) (<= _y _Y))) (and (or (< _Y GEN1) (< GEN2 _yy)) (and (<= (/ _xx _yy) _z) (and (<= (/ _xx _Y) _z) (and (<= (/ _X _yy) _z) (<= (/ _X _Y) _z))))))))
(assert (= BAV5 (and (and (<= _xx _x) (<= _x _X)) (and (<= _yy _y) (<= _y _Y)))))
(assert (= BAV6 (and (<= _xx _x) (<= _x _X))))
(assert (= BAV7 (<= _xx _x)))
(assert (= BAV8 (<= _x _X)))
(assert (= BAV9 (and (<= _yy _y) (<= _y _Y))))
(assert (= BAV10 (<= _yy _y)))
(assert (= BAV11 (<= _y _Y)))
(assert (= BAV12 (and (or (< _Y GEN1) (< GEN2 _yy)) (and (<= (/ _xx _yy) _z) (and (<= (/ _xx _Y) _z) (and (<= (/ _X _yy) _z) (<= (/ _X _Y) _z)))))))
(assert (= BAV13 (or (< _Y GEN1) (< GEN2 _yy))))
(assert (= BAV14 (< _Y GEN1)))
(assert (= BAV15 (< GEN2 _yy)))
(assert (= BAV16 (and (<= (/ _xx _yy) _z) (and (<= (/ _xx _Y) _z) (and (<= (/ _X _yy) _z) (<= (/ _X _Y) _z))))))
(assert (= BAV17 (<= (/ _xx _yy) _z)))
(assert (= BAV18 (and (<= (/ _xx _Y) _z) (and (<= (/ _X _yy) _z) (<= (/ _X _Y) _z)))))
(assert (= BAV19 (<= (/ _xx _Y) _z)))
(assert (= BAV20 (and (<= (/ _X _yy) _z) (<= (/ _X _Y) _z))))
(assert (= BAV21 (<= (/ _X _yy) _z)))
(assert (= BAV22 (<= (/ _X _Y) _z)))
(assert (= BAV23 (<= (/ _x _y) _z)))
(check-sat)