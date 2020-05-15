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
(declare-const BAV11 Bool)
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
(declare-const BAV23 Bool)
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
(declare-fun skoCOSS () Real)
(declare-fun skoSINS () Real)
(declare-fun skoS () Real)
(declare-fun pi () Real)
(assert (and (= (* skoSINS skoSINS) (+ GEN1 (* skoCOSS (* skoCOSS (- GEN2))))) (and (not (<= (* pi (/ GEN3 GEN4)) skoS)) (and (not (<= pi (/ GEN5 GEN6))) (and (not (<= (/ GEN7 GEN8) pi)) (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS))))))))
(assert (= BAV11 (and (= (* skoSINS skoSINS) (+ GEN1 (* skoCOSS (* skoCOSS (- GEN2))))) (and (not (<= (* pi (/ GEN3 GEN4)) skoS)) (and (not (<= pi (/ GEN5 GEN6))) (and (not (<= (/ GEN7 GEN8) pi)) (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS)))))))))
(assert (= BAV12 (= (* skoSINS skoSINS) (+ GEN1 (* skoCOSS (* skoCOSS (- GEN2)))))))
(assert (= BAV13 (and (not (<= (* pi (/ GEN3 GEN4)) skoS)) (and (not (<= pi (/ GEN5 GEN6))) (and (not (<= (/ GEN7 GEN8) pi)) (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS))))))))
(assert (= BAV14 (<= (* pi (/ GEN3 GEN4)) skoS)))
(assert (= BAV15 (and (not (<= pi (/ GEN5 GEN6))) (and (not (<= (/ GEN7 GEN8) pi)) (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS)))))))
(assert (= BAV16 (<= pi (/ GEN5 GEN6))))
(assert (= BAV17 (and (not (<= (/ GEN7 GEN8) pi)) (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS))))))
(assert (= BAV18 (<= (/ GEN7 GEN8) pi)))
(assert (= BAV19 (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS)))))
(assert (= BAV20 (<= GEN9 skoS)))
(assert (= BAV21 (and (<= GEN10 skoCOSS) (<= skoSINS skoS))))
(assert (= BAV22 (<= GEN10 skoCOSS)))
(assert (= BAV23 (<= skoSINS skoS)))
(check-sat)
(get-model)
(exit)