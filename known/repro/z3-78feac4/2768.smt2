(declare-fun value2 () String)
(declare-fun key1 () String)
(declare-fun value1 () String)
(declare-fun key2 () String)                                  
                                                                                
(assert                                                                         
(and                                                                            
    (= (str.at (str.substr value1 0 1) 0) "\r")                                 
    (not                                                                        
        (= (str.at (str.substr value1 (+ (str.indexof value1 "=" 0) 1)          
        (- 1 (+ (str.indexof value1 "=" 0) 1))) 0) "\f")                        
    )                                                                           
    (= (ite (= (str.len (str.substr value1 0 (- (str.indexof value1 "=" 0) 0))) 0) 1 0) 0)) 
)                                                                               
(check-sat)                                                                     
(get-model)  
