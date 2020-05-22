(set-option :produce-models true)
(check-sat)
(get-model)
(eval (bv2int (bvadd #x0a #xf0)))