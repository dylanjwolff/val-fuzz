(declare-fun a () String)
(declare-fun b () String)
(declare-fun c () String)
(declare-fun d () String)

(assert
  (or
    ;; unsat
    (not
      ;; sat
      (=
        (str.replace
          (str.++ (str.substr a 0 (str.len b)) (str.substr a 0 (str.len b)))
          "B"
          c
        )
        (str.++ (str.replace b "B" c) (str.substr a 0 (str.len b)))
      )
    )
    ;; unsat
    (and
      false ;; MARKER
      (=
        (ite
          (=
            (str.at (str.substr a (str.len b) (str.len d)) (- (str.len d) 1))
            ""
          )
          1
          0
        )
        0
      )
      (=
        (ite
          (=
            (str.at (str.substr a (str.len b) (str.len d)) (- (str.len d) 1))
            "
            "
          )
          1
          0
        )
        0
      )
    )
  )
)
(assert (= a (str.++ b d)))
(check-sat)
