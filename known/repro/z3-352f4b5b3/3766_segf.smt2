(set-option :smt.string_solver z3str3)
(declare-const Str0 String)
(declare-const Str1 String)
(declare-const Str6 String)
(declare-const Str10 String)
(assert (= true true true true true true (= Str1 (str.++ Str6 Str0) (str.++ Str0 "" Str10 "")) true true))
(push 1)
