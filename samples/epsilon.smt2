
; Copyright (c) 2015 Microsoft Corporation
(declare-const i_loop Real)
(declare-const loopex Bool)
(declare-fun zot-p1 (Real) Bool)
(declare-fun zot-p2 (Real) Bool)
(declare-const zot-i-eve_zot-p2 Real)
(declare-fun zot-p3 (Real) Bool)
(declare-fun zot-p4 (Real) Bool)
(declare-const zot-i-eve_zot-p4 Real)
(declare-fun zot-p5 (Real) Bool)
(declare-fun y-theta (Real) Real)


(assert (and (zot-p1 1.0)
              (or (not loopex)
                  (or (= i_loop 1.0) (= i_loop 2.0) (= i_loop 3.0)
                      (= i_loop 4.0) (= i_loop 5.0) (= i_loop 6.0)
                      (= i_loop 7.0) (= i_loop 8.0) (= i_loop 9.0)
                      (= i_loop 10.0)))
              (= (and (<= 1.0 i_loop) (<= i_loop 10.0))
                 (and loopex
                      (= (< 0.0 (y-theta (- i_loop 1.0)))
                         (< 0.0 (y-theta 10.0)))
                      (= (= 0.0 (y-theta (- i_loop 1.0)))
                         (= 0.0 (y-theta 10.0)))
                      (= (< (y-theta (- i_loop 1.0)) 0.0)
                         (< (y-theta 10.0) 0.0))
                      (= (= (y-theta (- i_loop 1.0)) 0.0)
                         (= (y-theta 10.0) 0.0))
                      (= (zot-p5 (- i_loop 1.0)) (zot-p5 10.0))
                      (= (zot-p3 (- i_loop 1.0)) (zot-p3 10.0))))
              (= (zot-p1 0.0) (and (zot-p2 0.0) (zot-p4 0.0)))
              (= (zot-p1 1.0) (and (zot-p2 1.0) (zot-p4 1.0)))
              (= (zot-p1 2.0) (and (zot-p2 2.0) (zot-p4 2.0)))
              (= (zot-p1 3.0) (and (zot-p2 3.0) (zot-p4 3.0)))
              (= (zot-p1 4.0) (and (zot-p2 4.0) (zot-p4 4.0)))
              (= (zot-p1 5.0) (and (zot-p2 5.0) (zot-p4 5.0)))
              (= (zot-p1 6.0) (and (zot-p2 6.0) (zot-p4 6.0)))
              (= (zot-p1 7.0) (and (zot-p2 7.0) (zot-p4 7.0)))
              (= (zot-p1 8.0) (and (zot-p2 8.0) (zot-p4 8.0)))
              (= (zot-p1 9.0) (and (zot-p2 9.0) (zot-p4 9.0)))
              (= (zot-p1 10.0) (and (zot-p2 10.0) (zot-p4 10.0)))
              (= (zot-p4 0.0) (and (zot-p5 0.0) (or false (zot-p4 1.0))))
              (= (zot-p2 0.0) (or (zot-p3 0.0) (and true (zot-p2 1.0))))
              (= (zot-p4 1.0) (and (zot-p5 1.0) (or false (zot-p4 2.0))))
              (= (zot-p2 1.0) (or (zot-p3 1.0) (and true (zot-p2 2.0))))
              (= (zot-p4 2.0) (and (zot-p5 2.0) (or false (zot-p4 3.0))))
              (= (zot-p2 2.0) (or (zot-p3 2.0) (and true (zot-p2 3.0))))
              (= (zot-p4 3.0) (and (zot-p5 3.0) (or false (zot-p4 4.0))))
              (= (zot-p2 3.0) (or (zot-p3 3.0) (and true (zot-p2 4.0))))
              (= (zot-p4 4.0) (and (zot-p5 4.0) (or false (zot-p4 5.0))))
              (= (zot-p2 4.0) (or (zot-p3 4.0) (and true (zot-p2 5.0))))
              (= (zot-p4 5.0) (and (zot-p5 5.0) (or false (zot-p4 6.0))))
              (= (zot-p2 5.0) (or (zot-p3 5.0) (and true (zot-p2 6.0))))
              (= (zot-p4 6.0) (and (zot-p5 6.0) (or false (zot-p4 7.0))))
              (= (zot-p2 6.0) (or (zot-p3 6.0) (and true (zot-p2 7.0))))
              (= (zot-p4 7.0) (and (zot-p5 7.0) (or false (zot-p4 8.0))))
              (= (zot-p2 7.0) (or (zot-p3 7.0) (and true (zot-p2 8.0))))
              (= (zot-p4 8.0) (and (zot-p5 8.0) (or false (zot-p4 9.0))))
              (= (zot-p2 8.0) (or (zot-p3 8.0) (and true (zot-p2 9.0))))
              (= (zot-p4 9.0) (and (zot-p5 9.0) (or false (zot-p4 10.0))))
              (= (zot-p2 9.0) (or (zot-p3 9.0) (and true (zot-p2 10.0))))
              (= (zot-p4 10.0) (and (zot-p5 10.0) (or false (zot-p4 11.0))))
              (= (zot-p2 10.0) (or (zot-p3 10.0) (and true (zot-p2 11.0))))
              (or (not (not loopex))
                  (and (not (zot-p5 11.0)) (not (zot-p3 11.0))
                       (not (zot-p1 11.0)) (not (zot-p4 11.0))
                       (not (zot-p2 11.0))))
              (or (not loopex)
                  (and (= (zot-p5 i_loop) (zot-p5 11.0))
                       (= (zot-p3 i_loop) (zot-p3 11.0))
                       (= (zot-p1 i_loop) (zot-p1 11.0))
                       (= (zot-p4 i_loop) (zot-p4 11.0))
                       (= (zot-p2 i_loop) (zot-p2 11.0))))
              (or (not loopex)
                  (and
                   (or (not (not (zot-p4 10.0)))
                       (and (<= i_loop zot-i-eve_zot-p4)
                            (<= zot-i-eve_zot-p4 10.0)
                            (not (zot-p4 zot-i-eve_zot-p4))))
                   (or (not (zot-p2 10.0))
                       (and (<= i_loop zot-i-eve_zot-p2)
                            (<= zot-i-eve_zot-p2 10.0)
                            (zot-p3 zot-i-eve_zot-p2)))))
              (= (zot-p5 0.0) (> (y-theta 0.0) 0.0))
              (= (zot-p3 0.0) (= (y-theta 0.0) 0.0))
              (= (zot-p5 1.0) (> (y-theta 1.0) 0.0))
              (= (zot-p3 1.0) (= (y-theta 1.0) 0.0))
              (= (zot-p5 2.0) (> (y-theta 2.0) 0.0))
              (= (zot-p3 2.0) (= (y-theta 2.0) 0.0))
              (= (zot-p5 3.0) (> (y-theta 3.0) 0.0))
              (= (zot-p3 3.0) (= (y-theta 3.0) 0.0))
              (= (zot-p5 4.0) (> (y-theta 4.0) 0.0))
              (= (zot-p3 4.0) (= (y-theta 4.0) 0.0))
              (= (zot-p5 5.0) (> (y-theta 5.0) 0.0))
              (= (zot-p3 5.0) (= (y-theta 5.0) 0.0))
              (= (zot-p5 6.0) (> (y-theta 6.0) 0.0))
              (= (zot-p3 6.0) (= (y-theta 6.0) 0.0))
              (= (zot-p5 7.0) (> (y-theta 7.0) 0.0))
              (= (zot-p3 7.0) (= (y-theta 7.0) 0.0))
              (= (zot-p5 8.0) (> (y-theta 8.0) 0.0))
              (= (zot-p3 8.0) (= (y-theta 8.0) 0.0))
              (= (zot-p5 9.0) (> (y-theta 9.0) 0.0))
              (= (zot-p3 9.0) (= (y-theta 9.0) 0.0))
              (= (zot-p5 10.0) (> (y-theta 10.0) 0.0))
              (= (zot-p3 10.0) (= (y-theta 10.0) 0.0)) true))

(check-sat)
(eval (and (zot-p1 1.0)
              (or (not loopex)
                  (or (= i_loop 1.0) (= i_loop 2.0) (= i_loop 3.0)
                      (= i_loop 4.0) (= i_loop 5.0) (= i_loop 6.0)
                      (= i_loop 7.0) (= i_loop 8.0) (= i_loop 9.0)
                      (= i_loop 10.0)))
              (= (and (<= 1.0 i_loop) (<= i_loop 10.0))
                 (and loopex
                      (= (< 0.0 (y-theta (- i_loop 1.0)))
                         (< 0.0 (y-theta 10.0)))
                      (= (= 0.0 (y-theta (- i_loop 1.0)))
                         (= 0.0 (y-theta 10.0)))
                      (= (< (y-theta (- i_loop 1.0)) 0.0)
                         (< (y-theta 10.0) 0.0))
                      (= (= (y-theta (- i_loop 1.0)) 0.0)
                         (= (y-theta 10.0) 0.0))
                      (= (zot-p5 (- i_loop 1.0)) (zot-p5 10.0))
                      (= (zot-p3 (- i_loop 1.0)) (zot-p3 10.0))))
              (= (zot-p1 0.0) (and (zot-p2 0.0) (zot-p4 0.0)))
              (= (zot-p1 1.0) (and (zot-p2 1.0) (zot-p4 1.0)))
              (= (zot-p1 2.0) (and (zot-p2 2.0) (zot-p4 2.0)))
              (= (zot-p1 3.0) (and (zot-p2 3.0) (zot-p4 3.0)))
              (= (zot-p1 4.0) (and (zot-p2 4.0) (zot-p4 4.0)))
              (= (zot-p1 5.0) (and (zot-p2 5.0) (zot-p4 5.0)))
              (= (zot-p1 6.0) (and (zot-p2 6.0) (zot-p4 6.0)))
              (= (zot-p1 7.0) (and (zot-p2 7.0) (zot-p4 7.0)))
              (= (zot-p1 8.0) (and (zot-p2 8.0) (zot-p4 8.0)))
              (= (zot-p1 9.0) (and (zot-p2 9.0) (zot-p4 9.0)))
              (= (zot-p1 10.0) (and (zot-p2 10.0) (zot-p4 10.0)))
              (= (zot-p4 0.0) (and (zot-p5 0.0) (or false (zot-p4 1.0))))
              (= (zot-p2 0.0) (or (zot-p3 0.0) (and true (zot-p2 1.0))))
              (= (zot-p4 1.0) (and (zot-p5 1.0) (or false (zot-p4 2.0))))
              (= (zot-p2 1.0) (or (zot-p3 1.0) (and true (zot-p2 2.0))))
              (= (zot-p4 2.0) (and (zot-p5 2.0) (or false (zot-p4 3.0))))
              (= (zot-p2 2.0) (or (zot-p3 2.0) (and true (zot-p2 3.0))))
              (= (zot-p4 3.0) (and (zot-p5 3.0) (or false (zot-p4 4.0))))
              (= (zot-p2 3.0) (or (zot-p3 3.0) (and true (zot-p2 4.0))))
              (= (zot-p4 4.0) (and (zot-p5 4.0) (or false (zot-p4 5.0))))
              (= (zot-p2 4.0) (or (zot-p3 4.0) (and true (zot-p2 5.0))))
              (= (zot-p4 5.0) (and (zot-p5 5.0) (or false (zot-p4 6.0))))
              (= (zot-p2 5.0) (or (zot-p3 5.0) (and true (zot-p2 6.0))))
              (= (zot-p4 6.0) (and (zot-p5 6.0) (or false (zot-p4 7.0))))
              (= (zot-p2 6.0) (or (zot-p3 6.0) (and true (zot-p2 7.0))))
              (= (zot-p4 7.0) (and (zot-p5 7.0) (or false (zot-p4 8.0))))
              (= (zot-p2 7.0) (or (zot-p3 7.0) (and true (zot-p2 8.0))))
              (= (zot-p4 8.0) (and (zot-p5 8.0) (or false (zot-p4 9.0))))
              (= (zot-p2 8.0) (or (zot-p3 8.0) (and true (zot-p2 9.0))))
              (= (zot-p4 9.0) (and (zot-p5 9.0) (or false (zot-p4 10.0))))
              (= (zot-p2 9.0) (or (zot-p3 9.0) (and true (zot-p2 10.0))))
              (= (zot-p4 10.0) (and (zot-p5 10.0) (or false (zot-p4 11.0))))
              (= (zot-p2 10.0) (or (zot-p3 10.0) (and true (zot-p2 11.0))))
              (or (not (not loopex))
                  (and (not (zot-p5 11.0)) (not (zot-p3 11.0))
                       (not (zot-p1 11.0)) (not (zot-p4 11.0))
                       (not (zot-p2 11.0))))
              (or (not loopex)
                  (and (= (zot-p5 i_loop) (zot-p5 11.0))
                       (= (zot-p3 i_loop) (zot-p3 11.0))
                       (= (zot-p1 i_loop) (zot-p1 11.0))
                       (= (zot-p4 i_loop) (zot-p4 11.0))
                       (= (zot-p2 i_loop) (zot-p2 11.0))))
              (or (not loopex)
                  (and
                   (or (not (not (zot-p4 10.0)))
                       (and (<= i_loop zot-i-eve_zot-p4)
                            (<= zot-i-eve_zot-p4 10.0)
                            (not (zot-p4 zot-i-eve_zot-p4))))
                   (or (not (zot-p2 10.0))
                       (and (<= i_loop zot-i-eve_zot-p2)
                            (<= zot-i-eve_zot-p2 10.0)
                            (zot-p3 zot-i-eve_zot-p2)))))
              (= (zot-p5 0.0) (> (y-theta 0.0) 0.0))
              (= (zot-p3 0.0) (= (y-theta 0.0) 0.0))
              (= (zot-p5 1.0) (> (y-theta 1.0) 0.0))
              (= (zot-p3 1.0) (= (y-theta 1.0) 0.0))
              (= (zot-p5 2.0) (> (y-theta 2.0) 0.0))
              (= (zot-p3 2.0) (= (y-theta 2.0) 0.0))
              (= (zot-p5 3.0) (> (y-theta 3.0) 0.0))
              (= (zot-p3 3.0) (= (y-theta 3.0) 0.0))
              (= (zot-p5 4.0) (> (y-theta 4.0) 0.0))
              (= (zot-p3 4.0) (= (y-theta 4.0) 0.0))
              (= (zot-p5 5.0) (> (y-theta 5.0) 0.0))
              (= (zot-p3 5.0) (= (y-theta 5.0) 0.0))
              (= (zot-p5 6.0) (> (y-theta 6.0) 0.0))
              (= (zot-p3 6.0) (= (y-theta 6.0) 0.0))
              (= (zot-p5 7.0) (> (y-theta 7.0) 0.0))
              (= (zot-p3 7.0) (= (y-theta 7.0) 0.0))
              (= (zot-p5 8.0) (> (y-theta 8.0) 0.0))
              (= (zot-p3 8.0) (= (y-theta 8.0) 0.0))
              (= (zot-p5 9.0) (> (y-theta 9.0) 0.0))
              (= (zot-p3 9.0) (= (y-theta 9.0) 0.0))
              (= (zot-p5 10.0) (> (y-theta 10.0) 0.0))
              (= (zot-p3 10.0) (= (y-theta 10.0) 0.0)) true))