(set-option :auto-config true)
(set-logic ALL)
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
(declare-const BAV22 Bool)
(declare-const BAV23 Bool)
(declare-const BAV24 Bool)
(declare-const BAV25 Bool)
(declare-const BAV26 Bool)
(declare-const BAV27 Bool)
(declare-const BAV28 Bool)
(declare-const BAV29 Bool)
(declare-const BAV30 Bool)
(declare-const BAV31 Bool)
(declare-const BAV32 Bool)
(declare-const BAV33 Bool)
(declare-const BAV34 Bool)
(declare-const BAV35 Bool)
(declare-const BAV36 Bool)
(declare-const BAV37 Bool)
(declare-const BAV38 Bool)
(declare-const GEN24 Int)
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
(declare-fun var1 () Int)
(assert (and (>= var1 GEN1) (<= var1 GEN2)))
(declare-fun var1_post () Int)
(assert (and (>= var1_post GEN3) (<= var1_post GEN4)))
(declare-fun var2 () Int)
(assert (and (>= var2 GEN5) (<= var2 GEN6)))
(declare-fun var2_post () Int)
(assert (and (>= var2_post GEN7) (<= var2_post GEN8)))
(declare-fun var3 () Int)
(assert (and (>= var3 GEN9) (<= var3 GEN10)))
(declare-fun var3_post () Int)
(assert (and (>= var3_post GEN11) (<= var3_post GEN12)))
(declare-fun var4 () Int)
(assert (and (>= var4 GEN13) (<= var4 GEN14)))
(declare-fun var4_post () Int)
(assert (and (>= var4_post GEN15) (<= var4_post GEN16)))
(assert (not (and (= var1 GEN17) (= var2 GEN18) (= var1_post GEN19) (= var4_post GEN20) (= var2_post var2) (= var3_post var3))))
(assert (and (= var1 GEN21) (= var2 GEN22) (= var1_post GEN23) (= var4_post GEN24) (= var2_post var2) (= var3_post var3)))
(assert (= BAV1 (and (>= var1 GEN1) (<= var1 GEN2))))
(assert (= BAV2 (>= var1 GEN1)))
(assert (= BAV3 (<= var1 GEN2)))
(assert (= BAV4 (and (>= var1_post GEN3) (<= var1_post GEN4))))
(assert (= BAV5 (>= var1_post GEN3)))
(assert (= BAV6 (<= var1_post GEN4)))
(assert (= BAV7 (and (>= var2 GEN5) (<= var2 GEN6))))
(assert (= BAV8 (>= var2 GEN5)))
(assert (= BAV9 (<= var2 GEN6)))
(assert (= BAV10 (and (>= var2_post GEN7) (<= var2_post GEN8))))
(assert (= BAV11 (>= var2_post GEN7)))
(assert (= BAV12 (<= var2_post GEN8)))
(assert (= BAV13 (and (>= var3 GEN9) (<= var3 GEN10))))
(assert (= BAV14 (>= var3 GEN9)))
(assert (= BAV15 (<= var3 GEN10)))
(assert (= BAV16 (and (>= var3_post GEN11) (<= var3_post GEN12))))
(assert (= BAV17 (>= var3_post GEN11)))
(assert (= BAV18 (<= var3_post GEN12)))
(assert (= BAV19 (and (>= var4 GEN13) (<= var4 GEN14))))
(assert (= BAV20 (>= var4 GEN13)))
(assert (= BAV21 (<= var4 GEN14)))
(assert (= BAV22 (and (>= var4_post GEN15) (<= var4_post GEN16))))
(assert (= BAV23 (>= var4_post GEN15)))
(assert (= BAV24 (<= var4_post GEN16)))
(assert (= BAV25 (and (= var1 GEN17) (= var2 GEN18) (= var1_post GEN19) (= var4_post GEN20) (= var2_post var2) (= var3_post var3))))
(assert (= BAV26 (= var1 GEN17)))
(assert (= BAV27 (= var2 GEN18)))
(assert (= BAV28 (= var1_post GEN19)))
(assert (= BAV29 (= var4_post GEN20)))
(assert (= BAV30 (= var2_post var2)))
(assert (= BAV31 (= var3_post var3)))
(assert (= BAV32 (and (= var1 GEN21) (= var2 GEN22) (= var1_post GEN23) (= var4_post GEN24) (= var2_post var2) (= var3_post var3))))
(assert (= BAV33 (= var1 GEN21)))
(assert (= BAV34 (= var2 GEN22)))
(assert (= BAV35 (= var1_post GEN23)))
(assert (= BAV36 (= var4_post GEN24)))
(assert (= BAV37 (= var2_post var2)))
(assert (= BAV38 (= var3_post var3)))
(check-sat)
(get-model)
(eval     (and
      (>= var1 0)
      (<= var1 7)
    ))
(eval     (and
      (>= var1_post 0)
      (<= var1_post 7)
    ))
(eval     (and
      (>= var2 0)
      (<= var2 4)
    ))
(eval     (and
      (>= var2_post 0)
      (<= var2_post 4)
    ))
(eval     (and
      (>= var3 0)
      (<= var3 4)
    ))
(eval     (and
      (>= var3_post 0)
      (<= var3_post 4)
    ))
(eval     (and
      (>= var4 0)
      (<= var4 1)
    ))
(eval     (and
      (>= var4_post 0)
      (<= var4_post 1)
    ))
(eval (not
(and
  (=
    var1
    7
  )
  (=
    var2
    0
  )
  (=
    var1_post
    2
  )
  (=
    var4_post
    1
  )
  (=
    var2_post
      var2
  )
  (=
    var3_post
      var3
  )
)
))
(eval 
  (and
    (=
      var1
      0
    )
    (=
	var2
      0
    ) 
    (=
      var1_post
      2
    )
    (=
      var4_post
      1
    )
    (=
      var2_post
	var2
    )
    (=
      var3_post
	var3
    )

  )
)