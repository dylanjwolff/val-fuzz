(set-option :solver.enforce_model_conversion true) 
 (declare-fun x0 () Real) 
 (declare-fun x1 () Real) 
 (declare-fun x2 () Real) 
 (assert (or (not (>= (+ (* (- 32) x2) (* (- 10) x0) (* (- 33) x0) (* 16 x1)) (- 35))) (> (+ (* (- 46) x1) (* 44 x1) (* (- 15) x1) (* (- 22) x2)) (- 20)))) 
 (check-sat) 
 (push 1) 
 (assert (or (not (< (+ (* (- 61) x2) (* (/ 60 26) x0)) 114)) (> (* (* 39 x1) (* 31 x1) (* (- 16) x0)) 27))) 
 (check-sat) 
