(set-option :strings-print-ascii true)
(declare-fun a () String)
(assert (= (str.code a) 128))
(check-sat)
