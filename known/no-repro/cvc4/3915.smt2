(set-logic UFNIA)
(set-option :incremental true)
(set-option :check-models true)
(set-option :solve-real-as-int true)
(declare-const v0 Bool)
(declare-const v1 Bool)
(declare-const v2 Bool)
(declare-const v3 Bool)
(declare-const v4 Bool)
(declare-const v5 Bool)
(declare-const v6 Bool)
(declare-const v7 Bool)
(declare-const v8 Bool)
(declare-const v9 Bool)
(declare-const v10 Bool)
(declare-const v11 Bool)
(declare-const v12 Bool)
(declare-const v13 Bool)
(declare-const v14 Bool)
(declare-const i1 Int)
(assert (forall ((q0 Int) (q1 Int) (q2 Int) (q3 Bool)) (=> (= v7 q3 v7 q3 v0 q3 q3 q3 q3 v3) (> q0 59))))
(push 1)
(declare-const v15 Bool)
(declare-sort S0 0)
(declare-sort S1 0)
(declare-const i2 Int)
(assert v13)
(push 1)
(declare-const S1-0 S1)
(assert (forall ((q4 Int)) (not (distinct q4 q4))))
(check-sat)
(push 1)
(declare-const v16 Bool)
(assert v13)
(assert v9)
(declare-const v17 Bool)
(assert (> (abs i1) (div 238 59)))
(pop 1)
(assert (not (< 59 i1)))
(declare-const v18 Bool)
(assert (=> (< 59 i1) v2))
(declare-const v19 Bool)
(assert (or (xor v13 v13 v12) (< 59 i1) (not (< 59 i1)) v0 v6 v19 v9 v1))
(pop 1)
(declare-const S0-0 S0)
(assert (forall ((q5 Bool)) v12))
(assert v3)
(push 1)
(declare-sort S2 0)
(assert (> (abs i1) (div 238 59)))
(pop 1)
(assert (or v3 (xor v13 v13 v12) (distinct v3 v12 v6 v14 (distinct i1 i1) v2) (< 59 i1) v10 v1 v5 v3 v12))
(declare-const v20 Bool)
(assert v14)
(declare-const v21 Bool)
(pop 1)
(declare-const i3 Int)
(assert v3)
(assert (= v14 (< 59 i1) v0 v6 v0 v3 v8 v12 v4 v3))
(declare-sort S0 0)
(declare-const v22 Bool)
(declare-const S0-0 S0)
(declare-const v23 Bool)
(assert (exists ((q6 S0) (q7 Int) (q8 Bool)) (= (= v22 v10 (=> v11 v1) q8 v5 q8) (> i1 i3) (= S0-0 S0-0 q6 S0-0))))
(declare-const v24 Bool)
(assert (and v0 v23 v10 v22))
(assert v9)
(declare-const i4 Int)
(declare-const i5 Int)
(assert (distinct (xor v13 v13 v12) v4 v8 (= (distinct i1 i1) v10 v6 v6) v22))
(declare-sort S1 0)
(declare-const v25 Bool)
(declare-const v26 Bool)
(assert (=> v11 v11))
(push 1)
(declare-sort S2 0)
(assert (and (and v0 v23 v10 v22) v0 (distinct i1 i1) v22 (xor v13 v13 v12)))
(declare-const S2-0 S2)
(declare-const v27 Bool)
(declare-const i6 Int)
(pop 1)
(declare-const v28 Bool)
(assert v10)
(push 1)
(assert v9)
(declare-const i7 Int)
(declare-const i8 Int)
(assert (forall ((q9 Bool) (q10 S0)) (not (and q9 q9 q9 q9 (= v4 (=> v11 v11) (distinct S0-0 S0-0) (distinct i1 i1) (xor v13 v13 v12) (and v0 v23 v10 v22) (=> v11 v11)) q9 q9 v5))))
(declare-const i9 Int)
(assert (and (= (distinct i1 i1) v10 v6 v6) (< 59 i3) (<= 711 734) v26 (= v14 (< 59 i1) v0 v6 v0 v3 v8 v12 v4 v3) v23 (distinct i1 i1) (distinct (and v0 v23 v10 v22) (xor v13 v13 v12) v8 (xor v13 v13 v12) (< 59 i1) v26 v4 (distinct i1 i1) (<= 711 734) (distinct v8 (< 59 i1) (distinct i1 i1) v12 (distinct (xor v13 v13 v12) v4 v8 (= (distinct i1 i1) v10 v6 v6) v22) (= 573 59) v25 v6) (<= 711 734)) v3))
(pop 1)
(declare-const v29 Bool)
(assert (distinct (xor v13 v13 v12) (< 59 i3) v3 v0))
(declare-const v30 Bool)
(assert (or v7 v25 v14 (xor v13 v13 v12) v22 v11 v29 v29 v23))
(assert (= v26 (= (- i1) i5)))
(declare-const v31 Bool)
(assert (= v26 (= (- i1) i5)))
(declare-const i10 Int)
(declare-const v32 Bool)
(assert v22)
(push 1)
(pop 1)
(declare-const v33 Bool)
(check-sat)
(declare-const v34 Bool)
(assert (=> v13 v12))
(assert (or (=> v13 v12) v0 (< 59 i3)))
(declare-const v35 Bool)
(declare-const v36 Bool)
(declare-const v37 Bool)
(check-sat)
(push 1)
(declare-const v38 Bool)
(assert (distinct (<= (div i1 59) 59) (> 879 888) (or (=> v13 v12) v0 (< 59 i3)) (distinct S0-0 S0-0 S0-0 S0-0) v24 v28 v11 v37 v9 v4))
(pop 1)
(declare-const S1-0 S1)
(push 1)
(declare-const v39 Bool)
(check-sat)
(assert (distinct (distinct (xor v13 v13 v12) (< 59 i3) v3 v0) (< (div i1 59) (abs 59)) v35 (<= (div i1 59) 59) v28 (or v7 v25 v14 (xor v13 v13 v12) v22 v11 v29 v29 v23) v35 v12))
(declare-const v40 Bool)
(declare-const i11 Int)
(push 1)
(assert (= v14 (< 59 i1) v0 v6 v0 v3 v8 v12 v4 v3))
(assert v36)
(declare-const S0-1 S0)
(assert v11)
(check-sat)
(declare-const v41 Bool)
(push 1)
(assert v6)
(declare-const S0-2 S0)
(assert (= S0-0 S0-0 S0-0 S0-0 S0-0))
(push 1)
(declare-const v42 Bool)
(assert (not (or (=> v13 v12) v0 (< 59 i3))))
(declare-const i12 Int)
(declare-const v43 Bool)
(assert (= (= v14 (< 59 i1) v0 v6 v0 v3 v8 v12 v4 v3) v42 (=> v13 v12) v41 (= v14 (< 59 i1) v0 v6 v0 v3 v8 v12 v4 v3) (not v34) v26 (= v14 (< 59 i1) v0 v6 v0 v3 v8 v12 v4 v3) (and v13 (distinct (xor v13 v13 v12) (< 59 i3) v3 v0) (= v26 (= (- i1) i5)) (= v26 (= (- i1) i5)) (distinct S0-0 S0-0) (distinct v14 v2 v7) (=> v11 v1)) (or (=> v13 v12) v0 (< 59 i3)) v3))
(declare-const S1-1 S1)
(assert (or v7 v25 v14 (xor v13 v13 v12) v22 v11 v29 v29 v23))
(pop 1)
(assert (= (not (= (distinct i1 i1) v10 v6 v6)) v40 (or v7 v25 v14 (xor v13 v13 v12) v22 v11 v29 v29 v23)))
(declare-const i13 Int)
(declare-const i14 Int)
(assert (and (or (=> v13 v12) v0 (< 59 i3)) (= v14 (< 59 i1) v0 v6 v0 v3 v8 v12 v4 v3) (and (< (div i1 59) (abs 59)) (< 59 i3) v31 (distinct S0-0 S0-0 S0-0 S0-0) v26 (not (= (distinct i1 i1) v10 v6 v6)) v6 v5 (= (- i1) i5) v6) v14 v40 (=> v13 v12) (distinct S0-0 S0-0) (=> v11 v11) v24))
(declare-const v44 Bool)
(assert (and v13 (distinct (xor v13 v13 v12) (< 59 i3) v3 v0) (= v26 (= (- i1) i5)) (= v26 (= (- i1) i5)) (distinct S0-0 S0-0) (distinct v14 v2 v7) (=> v11 v1)))
(pop 1)
(assert (not (distinct S0-0 S0-0)))
(assert (forall ((q11 Bool) (q12 Int)) (=> (> 402 (- (div i1 59))) (distinct v35 v2 v3 q11 q11 v1 q11 q11 (or v7 v25 v14 (xor v13 v13 v12) v22 v11 v29 v29 v23) q11))))
(declare-const v45 Bool)
(declare-const i15 Int)
(declare-const i16 Int)
(assert (= (- i1) i5))
(check-sat)
(exit)