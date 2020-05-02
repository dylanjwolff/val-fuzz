(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(set-info :source |
Benchmarks used in the followin paper:
Big proof engines as little proof engines: new results on rewrite-based satisfiability procedure
Alessandro Armando, Maria Paola Bonacina, Silvio Ranise, Stephan Schulz. 
PDPAR'05
http://www.ai.dist.unige.it/pdpar05/


|)
(set-info :smt-lib-version 2.0)
(set-info :category "crafted")
(declare-sort Index 0)
(declare-sort Element 0)
(declare-fun a1 () (Array Index Element))
(declare-fun a2 () (Array Index Element))
(declare-fun i1 () Index)
(declare-fun i2 () Index)
(assert (= (store (store a1 i1 (select a2 i1)) i2 (select (store a2 i1 (select a1 i1)) i2)) (store (store a2 i1 (select a1 i1)) i2 (select (store a1 i1 (select a2 i1)) i2))))
(assert (not (= a1 a2)))
(assert (= BAV1 (= (store (store a1 i1 (select a2 i1)) i2 (select (store a2 i1 (select a1 i1)) i2)) (store (store a2 i1 (select a1 i1)) i2 (select (store a1 i1 (select a2 i1)) i2)))))
(assert (= BAV2 (= a1 a2)))
(check-sat)
(exit)