(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(define-fun-rec int-and ((n Int) (n1 Int) (n2 Int)) Bool (
    or
    (= n1 n 0)
    (= n2 n 0)
    (
        and
        (> n1 0)
        (> n2 0)
        (>= n 0)
        (= (not (= (mod n 2 ) 0)) (and (not (= (mod n1 2 ) 0)) (not (= (mod n2 2) 0))))
        (int-and (div n 2) (div n1 2) (div n2 2))
    )
))
(declare-const x Int)
(declare-const y Int)
(declare-const z Int)
(assert (= x GEN1))
(assert (= y GEN2))
(assert (= z GEN3))
(assert (int-and z x y))
(assert (= BAV4 (= x GEN1)))
(assert (= BAV5 (= y GEN2)))
(assert (= BAV6 (= z GEN3)))
(check-sat)