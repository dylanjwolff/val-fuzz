(set-logic QF_S)
(assert (= (re.comp (str.to_re "")) (str.to_re "")))
(check-sat)
