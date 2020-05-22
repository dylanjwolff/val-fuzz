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
(declare-const BAV12 Bool)
(declare-const BAV13 Bool)
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
(declare-fun skoCOSS () Real)
(declare-fun skoSINS () Real)
(declare-fun skoS () Real)
(declare-fun pi () Real)
(assert (and (= (* skoSINS skoSINS) (+ GEN1 (* skoCOSS (* skoCOSS (- GEN2))))) (and (not (<= (* pi (/ GEN3 GEN4)) skoS)) (and (not (<= pi (/ GEN5 GEN6))) (and (not (<= (/ GEN7 GEN8) pi)) (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS))))))))
(assert (= BAV1 (and (= (* skoSINS skoSINS) (+ GEN1 (* skoCOSS (* skoCOSS (- GEN2))))) (and (not (<= (* pi (/ GEN3 GEN4)) skoS)) (and (not (<= pi (/ GEN5 GEN6))) (and (not (<= (/ GEN7 GEN8) pi)) (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS)))))))))
(assert (= BAV2 (= (* skoSINS skoSINS) (+ GEN1 (* skoCOSS (* skoCOSS (- GEN2)))))))
(assert (= BAV3 (and (not (<= (* pi (/ GEN3 GEN4)) skoS)) (and (not (<= pi (/ GEN5 GEN6))) (and (not (<= (/ GEN7 GEN8) pi)) (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS))))))))
(assert (= BAV4 (<= (* pi (/ GEN3 GEN4)) skoS)))
(assert (= BAV5 (and (not (<= pi (/ GEN5 GEN6))) (and (not (<= (/ GEN7 GEN8) pi)) (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS)))))))
(assert (= BAV6 (<= pi (/ GEN5 GEN6))))
(assert (= BAV7 (and (not (<= (/ GEN7 GEN8) pi)) (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS))))))
(assert (= BAV8 (<= (/ GEN7 GEN8) pi)))
(assert (= BAV9 (and (<= GEN9 skoS) (and (<= GEN10 skoCOSS) (<= skoSINS skoS)))))
(assert (= BAV10 (<= GEN9 skoS)))
(assert (= BAV11 (and (<= GEN10 skoCOSS) (<= skoSINS skoS))))
(assert (= BAV12 (<= GEN10 skoCOSS)))
(assert (= BAV13 (<= skoSINS skoS)))
(check-sat)
(get-model)
(exit)