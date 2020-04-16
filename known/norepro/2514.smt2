(declare-fun a () String)
(declare-fun b () Int)
(declare-fun c () String)
(declare-fun d () String)
(declare-fun g () Int)
(declare-fun e () String)
(declare-fun f () Int)
(assert
  (or
    (not
      (=
        (str.prefixof (str.replace c "A" "A") d)
        (str.prefixof (str.substr a 0 (str.len c)) d)
      )
    )
    (not (= (str.substr "A" f f) ""))
  )
)
(assert (= a (str.++ c e)))
(assert (= b (div b f) f (div b g)))
(check-sat)
