(set-logic ALL)
(declare-const BAV1 Bool)
(declare-datatypes ((DNat 0) (Nat 0)) (((dnat (data Nat)))
                                       ((succ (pred DNat)) (zero))))
(declare-fun x () Nat)
(assert (not (= x zero)))
(assert (= BAV1 (= x zero)))
(check-sat)