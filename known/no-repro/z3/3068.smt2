(declare-fun A () Bool)

(assert (not A))
(assert (or (not A) (not A)))

(check-sat-using (then dom-simplify smt))
(check-sat)
(apply dom-simplify)
