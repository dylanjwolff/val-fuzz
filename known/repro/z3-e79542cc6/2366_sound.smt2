(declare-fun a () String)     
(declare-fun b () String)      
(declare-fun d () String)     
(declare-fun i () String)    
(declare-fun e () String)          
(declare-fun h () String)            
(assert  (= 0 (str.len d)))            
(assert (= "" ( str.substr a 0 (str.len ( str.substr b (str.len e) (str.len h))))))     
(assert (not (= d h )))     
(assert (= a ( str.++ i h )))     
(assert (= b ( str.++ e h )))          
(check-sat)    

