(set-logic ALL)
(declare-const GEN1 String)
(declare-const GEN2 String)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const action String)
(declare-const example_key String)
(assert (str.in.re action (re.++ (str.to.re GEN1) (re.* re.allchar))))
(declare-const action_1 String)
(declare-const action_2 String)
(assert (and (= action (str.++ action_1 example_key action_2)) (= action_1 GEN2)))
(assert (= BAV3 (and (= action (str.++ action_1 example_key action_2)) (= action_1 GEN2))))
(assert (= BAV4 (= action (str.++ action_1 example_key action_2))))
(assert (= BAV5 (= action_1 GEN2)))
(check-sat)