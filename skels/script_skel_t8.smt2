(declare-const BAV1 Bool)
(declare-const s1 (Array Int Bool))
(declare-const s2 (Array Int Bool))
(assert (not (= ((_ map and) s1 s2) ((_ map not) ((_ map or) ((_ map not) s1) ((_ map not) s2))))))
(assert (= BAV1 (= ((_ map and) s1 s2) ((_ map not) ((_ map or) ((_ map not) s1) ((_ map not) s2))))))
(check-sat)
(get-model)