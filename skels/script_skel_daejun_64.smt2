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
(declare-const BAV20 Bool)
(declare-const BAV21 Bool)
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
(define-fun pow64 () Int 18446744073709551616)
(define-fun chop ((x Int)) Int (mod x pow64))
(define-fun add ((x Int) (y Int)) Int (chop (+ x y)))
(define-fun mul ((x Int) (y Int)) Int (chop (* x y)))
(declare-fun a () Int)
(assert (and (<= GEN1 a) (< a pow64)))
(declare-fun a07 () Int)
(declare-fun a06 () Int)
(declare-fun a05 () Int)
(declare-fun a04 () Int)
(declare-fun a03 () Int)
(declare-fun a02 () Int)
(declare-fun a01 () Int)
(declare-fun a00 () Int)
(declare-fun b07 () Int)
(declare-fun b06 () Int)
(declare-fun b05 () Int)
(declare-fun b04 () Int)
(declare-fun b03 () Int)
(declare-fun b02 () Int)
(declare-fun b01 () Int)
(declare-fun b00 () Int)
(assert (= b00 a))
(assert (= b01 (div b00 GEN2)))
(assert (= b02 (div b01 GEN3)))
(assert (= b03 (div b02 GEN4)))
(assert (= b04 (div b03 GEN5)))
(assert (= b05 (div b04 GEN6)))
(assert (= b06 (div b05 GEN7)))
(assert (= b07 (div b06 GEN8)))
(assert (= a00 (mod b00 GEN9)))
(assert (= a01 (mod b01 GEN10)))
(assert (= a02 (mod b02 GEN11)))
(assert (= a03 (mod b03 GEN12)))
(assert (= a04 (mod b04 GEN13)))
(assert (= a05 (mod b05 GEN14)))
(assert (= a06 (mod b06 GEN15)))
(assert (= a07 (mod b07 GEN16)))
(declare-fun aprime () Int)
(assert (= aprime (add (mul (add (mul (add (mul (add (mul (add (mul (add (mul (add (mul a07 GEN17) a06) GEN18) a05) GEN19) a04) GEN20) a03) GEN21) a02) GEN22) a01) GEN23) a00)))
(assert (not (= a aprime)))
(assert (= BAV1 (and (<= GEN1 a) (< a pow64))))
(assert (= BAV2 (<= GEN1 a)))
(assert (= BAV3 (< a pow64)))
(assert (= BAV4 (= b00 a)))
(assert (= BAV5 (= b01 (div b00 GEN2))))
(assert (= BAV6 (= b02 (div b01 GEN3))))
(assert (= BAV7 (= b03 (div b02 GEN4))))
(assert (= BAV8 (= b04 (div b03 GEN5))))
(assert (= BAV9 (= b05 (div b04 GEN6))))
(assert (= BAV10 (= b06 (div b05 GEN7))))
(assert (= BAV11 (= b07 (div b06 GEN8))))
(assert (= BAV12 (= a00 (mod b00 GEN9))))
(assert (= BAV13 (= a01 (mod b01 GEN10))))
(assert (= BAV14 (= a02 (mod b02 GEN11))))
(assert (= BAV15 (= a03 (mod b03 GEN12))))
(assert (= BAV16 (= a04 (mod b04 GEN13))))
(assert (= BAV17 (= a05 (mod b05 GEN14))))
(assert (= BAV18 (= a06 (mod b06 GEN15))))
(assert (= BAV19 (= a07 (mod b07 GEN16))))
(assert (= BAV20 (= aprime (add (mul (add (mul (add (mul (add (mul (add (mul (add (mul (add (mul a07 GEN17) a06) GEN18) a05) GEN19) a04) GEN20) a03) GEN21) a02) GEN22) a01) GEN23) a00))))
(assert (= BAV21 (= a aprime)))
(check-sat)
(get-model)