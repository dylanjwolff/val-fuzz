(set-option :produce-models true)
(set-logic ALL)
(declare-sort U 0)
(declare-fun x () U)
(declare-fun a () (Array U Bool))
(assert (select a x))
(check-sat)
(get-value ((not (select a x))))