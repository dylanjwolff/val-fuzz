(assert (forall ((x Real))
        (exists ((y Real))
                (and (<= 0.0 y) (<= y 1.0) (<= x (* y y))))))
(check-sat)