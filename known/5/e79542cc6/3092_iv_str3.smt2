(declare-fun s () String)
(assert (distinct s "<a></a>"))
(assert (< (str.indexof s "<a>" 4) (str.indexof s "</a>" 7)))
(check-sat)
