(set-logic ALL)
(declare-const GEN1 Int)
(declare-const GEN2 Int)
(declare-const GEN3 Int)
(declare-const GEN4 Int)
(declare-const GEN5 Int)
(declare-const GEN6 Int)
(declare-const GEN7 Int)
(declare-const GEN8 Int)
(declare-const GEN9 Int)
(declare-const GEN10 Int)
(declare-const GEN11 Int)
(declare-const BAV12 Bool)
(declare-const BAV13 Bool)
(declare-const BAV14 Bool)
(declare-const BAV15 Bool)
(declare-const BAV16 Bool)
(declare-const BAV17 Bool)
(declare-const BAV18 Bool)
(declare-const BAV19 Bool)
(declare-const BAV20 Bool)
(declare-const BAV21 Bool)
(declare-const BAV22 Bool)
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
(assert (= BAV12 (and (<= skoX (/ GEN1 GEN2)) (and (not (<= (/ GEN3 GEN4) skoX)) (and (= (* skoSB skoSB) (+ GEN5 (* skoCB (* skoCB (- GEN6))))) (and (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8))))) (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX))))))))
(assert (= BAV13 (<= skoX (/ GEN1 GEN2))))
(assert (= BAV14 (and (not (<= (/ GEN3 GEN4) skoX)) (and (= (* skoSB skoSB) (+ GEN5 (* skoCB (* skoCB (- GEN6))))) (and (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8))))) (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX)))))))
(assert (= BAV15 (<= (/ GEN3 GEN4) skoX)))
(assert (= BAV16 (and (= (* skoSB skoSB) (+ GEN5 (* skoCB (* skoCB (- GEN6))))) (and (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8))))) (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX))))))
(assert (= BAV17 (= (* skoSB skoSB) (+ GEN5 (* skoCB (* skoCB (- GEN6)))))))
(assert (= BAV18 (and (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8))))) (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX)))))
(assert (= BAV19 (= (* skoS skoS) (+ GEN7 (* skoC (* skoC (- GEN8)))))))
(assert (= BAV20 (and (<= skoX (/ GEN9 GEN10)) (<= GEN11 skoX))))
(assert (= BAV21 (<= skoX (/ GEN9 GEN10))))
(assert (= BAV22 (<= GEN11 skoX)))
(check-sat)
(get-model)