(set-option :model_validate true)
(set-option :rewriter.elim_and true)
(set-option :rewriter.local_ctx true)
(set-option :smt.threads 6)
(declare-const Str19 String)
(declare-const i7 Int)
(assert (>= (str.len (str.++ "" "fnmahu" Str19 (int.to.str (+ 884 518 796 0 i7)))) 38))
(check-sat)

