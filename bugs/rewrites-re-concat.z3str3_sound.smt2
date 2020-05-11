(set-info :smt-lib-version 2.5)
(set-logic ALL)



















(declare-const BAV20 Bool)
(declare-const BAV21 Bool)
(declare-fun x () String)
(assert (str.in.re x (re.++ (str.to.re "") (re.* (str.to.re "")) (re.* (str.to.re "")))))
(declare-fun y () String)
(assert (< (str.len y) 4))
(assert (or (not (str.in.re x (re.++ (str.to.re "") (re.* (str.to.re "")) (re.* (str.to.re "")) (re.* (str.to.re ""))))) (not (str.in.re x (re.++ (str.to.re "") (str.to.re "") (re.* (str.to.re "")) (str.to.re "") (re.* (str.to.re ""))))) (not (str.in.re x (re.++ (str.to.re "") (re.* (str.to.re "")) (str.to.re "") (re.* (str.to.re "")) (str.to.re "") (re.* (str.to.re ""))))) (str.in.re y (re.++ re.allchar re.allchar (re.* re.allchar) re.allchar)) (str.in.re y (re.++ re.allchar re.allchar re.allchar))))
(assert (= BAV20 (< (str.len y) 4)))
(assert (= BAV21 (or (not (str.in.re x (re.++ (str.to.re "") (re.* (str.to.re "")) (re.* (str.to.re "")) (re.* (str.to.re ""))))) (not (str.in.re x (re.++ (str.to.re "") (str.to.re "") (re.* (str.to.re "")) (str.to.re "") (re.* (str.to.re ""))))) (not (str.in.re x (re.++ (str.to.re "") (re.* (str.to.re "")) (str.to.re "") (re.* (str.to.re "")) (str.to.re "") (re.* (str.to.re ""))))) (str.in.re y (re.++ re.allchar re.allchar (re.* re.allchar) re.allchar)) (str.in.re y (re.++ re.allchar re.allchar re.allchar)))))
(assert (and (= BAV20 true) (= BAV21 true)))
(check-sat)
(get-model)