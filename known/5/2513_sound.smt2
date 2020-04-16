(declare-fun a () String)
(declare-fun b () String)
(declare-fun c () String)
(declare-fun d () String)
(assert (= a (str.++ b d))) 
(assert
  (or
    (and
        (= (str.indexof (str.substr a 0 (str.len b)) "=" 0) 0)
        (= (str.indexof b "=" 0) 1) 
    ) 
    (not 
      (= (str.suffixof "A" d) (str.suffixof "A" (str.replace c c d)))
    )
  )
)
(check-sat)
