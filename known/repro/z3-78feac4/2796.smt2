(declare-fun b () String)
(declare-fun a () String)
(assert
 (and
  (= (ite
      (str.contains
       (str.++ (str.replace (str.substr (str.substr a 0 0) 0 0) "R" "r")
        (str.substr (str.substr a 0 (- (str.indexof a "=" 0) 0))
         (- (str.indexof (str.substr a 0 (- (str.indexof a "=" 0) 0)) "R" 0) 1)
         (- (str.len (str.substr a 0 (- (str.indexof a "=" 0) 0)))
          (+ (str.indexof (str.substr a 0 (- (str.indexof a "=" 0) 0)) "R" 0) 1)))) "X")
      1 0)
   0)
  (distinct (ite (str.contains (str.substr a 0 (- (str.indexof a "=" 0) 0)) "R") 1 0) 0)
  (= (ite (str.contains (str.substr a 0 (- (str.indexof a "=" 0))) "J") 1 0)
   (ite (str.contains (str.substr a 0 (- (str.indexof a "=" 0))) "I") 1 0)
   (ite
    (=
     (str.at
      (str.substr a 0 (- (str.indexof a "=" 0) 0))
      (- (str.len (str.substr a 0 (- (str.indexof a "=" 0) 0))) 1))
     "\n") 1 0) 0)
  (distinct (ite (= (str.at (str.substr a 0 (- (str.indexof a "=" 0) 0)) 0) "\v") 1 0) 0)
  (distinct (ite (str.contains a "=") 1 0) 0)
  (= (ite (= (str.len a) 0) 1 0) (ite (str.contains b "H") 1 0))
  (distinct (ite (str.contains b "F") 1 0) 0)
  (distinct (ite (str.contains b "E") 1 0) 0)
  (distinct (ite (str.contains b "D") 1 0) 0)
  (= (ite (str.contains b "C") 1 0) (ite (= (str.at b 0) "") 1 0) 0)))
(check-sat)
