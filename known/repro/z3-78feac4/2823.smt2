(set-logic QF_NRA)
(declare-fun a () Real)      
(declare-fun b () Real)    
(declare-fun c () Real)    
(assert    
    (let ((d 0)) (let ((e 0))    
    (distinct true (= true (= 0 a))    
    (distinct true (not (<= c 0))    
    (= false  (distinct false    
              (distinct false (= a b)))))))))    
(check-sat)
