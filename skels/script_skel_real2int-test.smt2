(set-info :smt-lib-version 2.6)
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
(declare-const BAV10 Bool)
(declare-const BAV11 Bool)
(declare-const BAV12 Bool)
(declare-const BAV13 Bool)
(declare-const BAV14 Bool)
(declare-const BAV15 Bool)
(declare-const BAV16 Bool)
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
(set-info :category "industrial")
(declare-fun skoX () Real)
(declare-fun skoS3 () Real)
(declare-fun skoSX () Real)
(assert (and (not (<= (* skoX (+ (+ (* skoS3 (/ GEN1 GEN2)) (* skoSX (/ GEN3 GEN4))) (* skoX (* skoS3 (- GEN5))))) (+ (* skoS3 GEN6) skoSX))) (and (not (<= skoX GEN7)) (and (not (<= skoSX GEN8)) (not (<= skoS3 GEN9))))))
(assert (= BAV10 (and (not (<= (* skoX (+ (+ (* skoS3 (/ GEN1 GEN2)) (* skoSX (/ GEN3 GEN4))) (* skoX (* skoS3 (- GEN5))))) (+ (* skoS3 GEN6) skoSX))) (and (not (<= skoX GEN7)) (and (not (<= skoSX GEN8)) (not (<= skoS3 GEN9)))))))
(assert (= BAV11 (<= (* skoX (+ (+ (* skoS3 (/ GEN1 GEN2)) (* skoSX (/ GEN3 GEN4))) (* skoX (* skoS3 (- GEN5))))) (+ (* skoS3 GEN6) skoSX))))
(assert (= BAV12 (and (not (<= skoX GEN7)) (and (not (<= skoSX GEN8)) (not (<= skoS3 GEN9))))))
(assert (= BAV13 (<= skoX GEN7)))
(assert (= BAV14 (and (not (<= skoSX GEN8)) (not (<= skoS3 GEN9)))))
(assert (= BAV15 (<= skoSX GEN8)))
(assert (= BAV16 (<= skoS3 GEN9)))
(check-sat)
(exit)