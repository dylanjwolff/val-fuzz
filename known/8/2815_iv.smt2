(declare-fun str () String)
(declare-fun str1 () String)

(define-fun-rec sanitize ((name String)) String
(let ((a!1 (re.union (str.to.re "@")
                                        (re.union (str.to.re ".")
                                                  (re.union (str.to.re "[")
                                                            (str.to.re "]"))))))
                                                            
                                                            
(let ((a!2 (re.union (str.to.re "}")
                                        (re.union (str.to.re "|")
                                                  (re.union (str.to.re "~") a!1)))))                                                            
(let ((a!3 (re.union (str.to.re "_")
                                        (re.union (str.to.re "`")
                                                  (re.union (str.to.re "{") a!2)))))
(let ((a!4 (re.union (str.to.re "=")
                                        (re.union (str.to.re "?")
                                                  (re.union (str.to.re "^") a!3)))))
(let ((a!5 (re.union (str.to.re "*")
                                        (re.union (str.to.re "+")
                                                  (re.union (str.to.re "-") a!4)))))
(let ((a!6 (re.union (re.range "0" "9")
                                        (re.union (re.range "#" "'")
                                                  (re.union (str.to.re "!") a!5)))))

(let ((a!7 (str.in.re (str.substr name 0 1)
                                         (re.union (re.range "A" "Z")
                                                   (re.union (re.range "a" "z")
                                                             a!6)))))
(ite  (= 0 (str.len name))  name
(ite a!7 (str.++ (str.substr name 0 1) (sanitize (str.substr name 1 (str.len name))) ) (sanitize (str.substr name 1 (str.len name))))
                                                             
)))))))))

(assert (= str (sanitize str1)))
(assert (str.contains str "test''''1@2343" ))

(check-sat)
(get-model)
