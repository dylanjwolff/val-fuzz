(set-logic ALL)
(declare-const BAV1 Bool)
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
(declare-fun i0 () Index)
(declare-fun i1 () Index)
(assert (not (= (store (store (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0 (select (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0)) i0 (select (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0)) (store (store (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0 (select (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0)) i0 (select (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0)))))
(assert (= BAV1 (= (store (store (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0 (select (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0)) i0 (select (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0)) (store (store (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0 (select (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0)) i0 (select (store (store a1 i1 (select a1 i1)) i1 (select a1 i1)) i0)))))
(check-sat)
(get-model)
(exit)