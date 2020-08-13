(set-logic NRA)
(declare-fun a () Real)
(declare-fun b () Real)
(declare-fun c () Real)
(declare-fun d () Real)
(declare-fun e () Real)
(declare-fun f () Real)
(declare-fun g () Real)
(declare-fun h () Real)
(declare-fun i () Real)
(declare-fun j () Real)
(declare-fun k () Real)
(declare-fun l () Real)
(declare-fun m () Real)
(declare-fun n () Real)
(declare-fun o () Real)
(declare-fun p () Real)
(declare-fun q () Real)
(declare-fun r () Real)
(declare-fun s () Real)
(assert
  (not
    (exists
      ((t Real))
      (=>
        (and
          (or
            (and
              (=>
                (< 0 j)
                (and
                  (or
                    (<= 0 (+ (* n s) m))
                    (<= 0 p)
                  )
                  (> s 0)
                )
              )
              (<
                (*
                  o
                  (* (* n h) (/ o (- b h)))
                )
                0
              )
            )
            (< 0 n)
          )
          (< 0 p)
          (< 0 (- b h))
        )
        (<
          (/
            (* (+ (* n j) m) (+ (* n j) m))
            (* 2 o)
          )
          (* a e)
        )
      )
    )
  )
)
(assert
  (not
    (exists
      ((u Real))
      (=>
        (and
          (or (= (* c r) 0) (= l 0))
          (= e 2)
          (< h i (/ d r))
        )
        (=> (= 0 k) (not (<= u k 0 r)))
      )
    )
  )
)
(assert (= a (+ f o) (+ g o)))
(assert (= b (/ h q)))
(check-sat)
(get-model)
