

(declare-fun a () String)
(declare-fun b () Int)
(assert (distinct (str.replace "A" (int.to.str b) a)
         (str.replace "" (int.to.str b) a)))
(assert (= (str.replace a (str.at a b) "") a))
(check-sat)
(get-model)

