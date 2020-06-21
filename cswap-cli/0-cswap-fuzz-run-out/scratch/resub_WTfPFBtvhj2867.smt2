



































(declare-fun a () Real)
(declare-fun c () Real)
(declare-fun b () Real)
(declare-fun j () Real)
(declare-fun d () Real)
(declare-fun k () Real)
(declare-fun e () Real)
(declare-fun f () Real)
(declare-fun i () Real)
(declare-fun g () Real)
(declare-fun h () Real)
(declare-fun o () Real)
(declare-fun q () Real)
(declare-fun l () Real)
(declare-fun m () Real)
(declare-fun n () Real)
(declare-fun p () Real)
(declare-fun u () Real)
(declare-fun t () Real)
(declare-fun r () Real)
(assert (forall ((s Real)) (and (or (= i o (/ 0 r)) (>= e o) (<= 0.0 g)) (> 0.0 o) (or (< 0.0 s) (>= s c)) (or (< 0 o) (< 0 (* k p)) (= f 0.0)))))
(assert (or (and (or (> 0.0 q) (>= (/ 0 a) u) (< l n)) (>= (- (/ (* o (/ d o)) m)) 0.0 (/ 0 j h) u b)) (>= l (* 0 (+ 0 0) (- 0 (- (+ 0 (/ 0 j h) (* 0 q q)) (- 0 (/ 0 0.0 q) (+ 0 a i))) (+ 0 0.0 n)))) (>= (/ 0 (* (+ 0 (+ (/ 0 (+ 0 j h) (- 0 q q)) (/ (* 0.0 q) (/ 0 a i))) (- 0 0.0 n))) (* (/ (* 0 (/ 0 (- j h) q i)) (* (+ (/ 0 j h) a i))))) l)))
(assert (= a (+ i t)))
(check-sat)
