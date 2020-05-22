(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-const BAV8 Bool)
(declare-const BAV9 Bool)
(declare-const BAV10 Bool)
(declare-const BAV11 Bool)
(declare-const GEN11 Int)
(declare-const GEN10 Int)
(declare-const GEN9 Int)
(declare-const GEN8 Int)
(declare-const GEN7 Int)
(declare-const GEN6 Int)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Int)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(set-info :source |
These benchmarks used in the paper:

  Dejan Jovanovic and Leonardo de Moura.  Solving Non-Linear Arithmetic.
  In IJCAR 2012, published as LNCS volume 7364, pp. 339--354.

The meti-tarski benchmarks are proof obligations extracted from the
Meti-Tarski project, see:

  B. Akbarpour and L. C. Paulson. MetiTarski: An automatic theorem prover
  for real-valued special functions. Journal of Automated Reasoning,
  44(3):175-205, 2010.

Submitted by Dejan Jovanovic for SMT-LIB.


|)
(set-info :smt-lib-version 2.0)
(set-info :category "industrial")
(declare-fun skoC () Real)
(declare-fun skoS () Real)
(declare-fun skoCB () Real)
(declare-fun skoSB () Real)
(declare-fun skoX () Real)
(assert (and (<= skoX (/ GEN1 GEN2)) (and (not (<= (/ GEN3 GEN4) skoX)) (and (= (* skoSB skoSB) (+ GEN5 (* skoCB (* skoCB (- GEN6))))) (and (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8))))) (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX)))))))
(assert (= BAV1 (and (<= skoX (/ GEN1 GEN2)) (and (not (<= (/ GEN3 GEN4) skoX)) (and (= (* skoSB skoSB) (+ GEN5 (* skoCB (* skoCB (- GEN6))))) (and (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8))))) (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX))))))))
(assert (= BAV2 (<= skoX (/ GEN1 GEN2))))
(assert (= BAV3 (and (not (<= (/ GEN3 GEN4) skoX)) (and (= (* skoSB skoSB) (+ GEN5 (* skoCB (* skoCB (- GEN6))))) (and (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8))))) (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX)))))))
(assert (= BAV4 (<= (/ GEN3 GEN4) skoX)))
(assert (= BAV5 (and (= (* skoSB skoSB) (+ GEN5 (* skoCB (* skoCB (- GEN6))))) (and (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8))))) (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX))))))
(assert (= BAV6 (= (* skoSB skoSB) (+ GEN5 (* skoCB (* skoCB (- GEN6)))))))
(assert (= BAV7 (and (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8))))) (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX)))))
(assert (= BAV8 (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8)))))))
(assert (= BAV9 (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX))))
(assert (= BAV10 (<= skoX (/ GEN9 GEN10))))
(assert (= BAV11 (<= GEN11 skoX)))
(check-sat)
(get-model)