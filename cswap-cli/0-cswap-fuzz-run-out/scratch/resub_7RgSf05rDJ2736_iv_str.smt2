

(declare-fun i () Int)
(assert (< i 0))
(assert (= (int.to.str i) "\x00"))
(check-sat)
