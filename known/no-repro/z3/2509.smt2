(declare-fun b () Real)
(declare-fun c () Real)
(declare-fun d () Real)
(declare-fun e () Real)
(declare-fun f () Real)
(declare-fun o () Real)
(declare-fun g () Real)
(declare-fun h () Real)
(declare-fun i () Real)
(declare-fun j () Real)
(declare-fun k () Real)
(declare-fun l () Real)
(declare-fun m () Real)
(declare-fun n () Real)
(declare-fun q () Real)
(declare-fun p () Real)
(assert
  (or
    (and
      (not
        (exists
          ((ac Real))
          (=>
            (= c e)
            (=
              (= 0.0 b)
              (= (= (<= ac b) (<= 0 h)) (= o 2.0))
            )
          )
        )
      )
      (not
        (exists
          ((a Real))
          (=>
            (and
              (=>
                (<= l 0)
                (and (<= (* c l) (- d)) (<= l k))
              )
              (= f i)
              (<= 0.0 a)
              (<
                (+ i (/ 0 (* 2.0 g)))
                f
              )
              (<= j d)
              (< 0.0 k)
            )
            (< f 0 (/ 0 0) f)
          )
        )
      )
    )
    (not
      (exists
        ((ae Real))
        (=>
          (and
            (=>
              (<= ae m)
              (>=
                (+ n (* (- 5) (* ae ae)) (* ae q))
                0
              )
            )
            (=
              n
              (+ (* 5 (* p p)) (* q p))
            )
            (>= p 0)
            (<= p (/ 6 5))
          )
          (<= (+ m p) (/ 16 5))
        )
      )
    )
  )
)
(check-sat)

