(set-info :smt-lib-version 2.0)
(set-logic ALL)
(declare-const GEN1 String)
(set-option :strings-exp true)
(define-fun strinre ((?s String)) Bool (str.in.re ?s (re.union re.nostr (re.++ (str.to.re "") (str.to.re "") (re.union re.nostr (re.range "*" ",") (str.to.re "\t") (re.range "*" "|") ) (re.+ (re.union re.nostr (re.++ (str.to.re "") (str.to.re "") ((_ re.^ 6) re.allchar) (re.opt (re.union re.nostr (re.++ (str.to.re "") (str.to.re "") ) ) ) ) ) ) ) )  ) )
(assert (not (strinre GEN1)))
(check-sat)
(get-model)