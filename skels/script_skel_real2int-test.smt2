(set-info :smt-lib-version 2.6)
(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
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
(set-info :category "industrial")
(declare-fun skoX () Real)
(declare-fun skoS3 () Real)
(declare-fun skoSX () Real)
(assert (and (not (<= (* skoX (+ (+ (* skoS3 (/ GEN1 GEN2)) (* skoSX (/ GEN3 GEN4))) (* skoX (* skoS3 (- GEN5))))) (+ (* skoS3 GEN6) skoSX))) (and (not (<= skoX GEN7)) (and (not (<= skoSX GEN8)) (not (<= skoS3 GEN9))))))
(assert (= BAV1 (and (not (<= (* skoX (+ (+ (* skoS3 (/ GEN1 GEN2)) (* skoSX (/ GEN3 GEN4))) (* skoX (* skoS3 (- GEN5))))) (+ (* skoS3 GEN6) skoSX))) (and (not (<= skoX GEN7)) (and (not (<= skoSX GEN8)) (not (<= skoS3 GEN9)))))))
(assert (= BAV2 (<= (* skoX (+ (+ (* skoS3 (/ GEN1 GEN2)) (* skoSX (/ GEN3 GEN4))) (* skoX (* skoS3 (- GEN5))))) (+ (* skoS3 GEN6) skoSX))))
(assert (= BAV3 (and (not (<= skoX GEN7)) (and (not (<= skoSX GEN8)) (not (<= skoS3 GEN9))))))
(assert (= BAV4 (<= skoX GEN7)))
(assert (= BAV5 (and (not (<= skoSX GEN8)) (not (<= skoS3 GEN9)))))
(assert (= BAV6 (<= skoSX GEN8)))
(assert (= BAV7 (<= skoS3 GEN9)))
(check-sat)
(get-model)
(exit)