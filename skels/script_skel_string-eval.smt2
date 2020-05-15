(declare-const s String)
(check-sat)
(get-model)
(get-value (s))