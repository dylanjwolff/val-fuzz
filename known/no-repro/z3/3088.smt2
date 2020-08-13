(assert
 (and
  (and
   (and
    (and
     (=
      (str.len
       (str.substr
        (str.substr url (str.indexof url "#" 62) (- (str.indexof url "#" 2)))
        (+ 0 1)
        (- (str.len (str.substr url (str.indexof url "#" 2) 0))
         (+ (str.indexof (str.substr url (str.indexof url "#" 2)
                          (- 0 (str.indexof url "#" 2))) "" 0) 1)))) 0)
     (not (= (str.substr url 0 0) "http")))
    (> (str.indexof url ":" 0) 0))
   (>= (- (str.indexof url "#" 2) 2) 0))
  (>= (str.indexof url ":" 0) 0)))
(check-sat)

