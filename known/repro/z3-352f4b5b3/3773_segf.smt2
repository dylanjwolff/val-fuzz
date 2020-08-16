(set-option :smt.string_solver z3str3)
(declare-fun a () String)
(assert (distinct (str.++ a "ab") (str.++ "ab" a)))
(check-sat-using unit-subsume-simplify)
