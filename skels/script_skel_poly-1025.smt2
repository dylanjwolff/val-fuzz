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
(declare-const GEN12 Int)
(declare-const GEN13 Int)
(declare-const GEN14 Int)
(declare-const GEN15 Int)
(declare-const GEN16 Int)
(declare-const GEN17 Int)
(declare-const GEN18 Int)
(declare-const GEN19 Int)
(declare-const GEN20 Int)
(declare-const GEN21 Int)
(declare-const GEN22 Int)
(declare-const GEN23 Int)
(declare-const GEN24 Int)
(declare-const GEN25 Int)
(declare-const GEN26 Int)
(declare-const GEN27 Int)
(declare-const GEN28 Int)
(declare-const GEN29 Int)
(declare-const GEN30 Int)
(declare-const GEN31 Int)
(declare-const GEN32 Int)
(declare-const BAV33 Bool)
(declare-const BAV34 Bool)
(declare-const BAV35 Bool)
(declare-const BAV36 Bool)
(declare-const BAV37 Bool)
(declare-const BAV38 Bool)
(declare-const BAV39 Bool)
(declare-const BAV40 Bool)
(declare-const BAV41 Bool)
(declare-const BAV42 Bool)
(declare-const BAV43 Bool)
(declare-const BAV44 Bool)
(declare-const BAV45 Bool)
(declare-const BAV46 Bool)
(declare-const BAV47 Bool)
(declare-const BAV48 Bool)
(declare-const BAV49 Bool)
(declare-const BAV50 Bool)
(declare-const BAV51 Bool)
(declare-const BAV52 Bool)
(declare-const BAV53 Bool)
(declare-const BAV54 Bool)
(declare-const BAV55 Bool)
(declare-const BAV56 Bool)
(declare-const BAV57 Bool)
(declare-const BAV58 Bool)
(declare-const BAV59 Bool)
(declare-const BAV60 Bool)
(declare-const BAV61 Bool)
(declare-const BAV62 Bool)
(declare-const BAV63 Bool)
(declare-const BAV64 Bool)
(declare-const BAV65 Bool)
(declare-const BAV66 Bool)
(declare-const BAV67 Bool)
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
(declare-fun skoX () Real)
(declare-fun skoY () Real)
(declare-fun skoZ () Real)
(assert (and (not (<= skoX GEN14)) (and (or (not (<= (* skoZ (* skoY (* skoY (+ (* skoX (* skoX (+ (/ (- GEN15) GEN16) (* skoX (/ GEN4 GEN5))))) (* skoY (* skoX (* skoX (+ (/ GEN17 GEN18) (* skoX (/ GEN4 GEN5)))))))))) (* skoY (+ (* skoX (+ (/ GEN19 GEN20) (* skoX (/ (- GEN6) GEN7)))) (* skoY (* skoX (+ (/ (- GEN21) GEN22) (* skoX (/ (- GEN6) GEN7))))))))) (not (<= skoZ GEN1))) (and (or (not (<= skoY GEN2)) (not (<= skoZ GEN1))) (and (or (not (<= skoX GEN3)) (or (not (<= skoY GEN2)) (not (<= skoZ GEN1)))) (and (<= skoZ GEN23) (and (<= skoY GEN24) (and (<= skoX GEN25) (and (<= GEN26 skoZ) (and (<= GEN27 skoY) (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1)))))))))))))))
(assert (= BAV33 (and (not (<= skoX GEN14)) (and (or (not (<= (* skoZ (* skoY (* skoY (+ (* skoX (* skoX (+ (/ (- GEN15) GEN16) (* skoX (/ GEN4 GEN5))))) (* skoY (* skoX (* skoX (+ (/ GEN17 GEN18) (* skoX (/ GEN4 GEN5)))))))))) (* skoY (+ (* skoX (+ (/ GEN19 GEN20) (* skoX (/ (- GEN6) GEN7)))) (* skoY (* skoX (+ (/ (- GEN21) GEN22) (* skoX (/ (- GEN6) GEN7))))))))) (not (<= skoZ GEN1))) (and (or (not (<= skoY GEN2)) (not (<= skoZ GEN1))) (and (or (not (<= skoX GEN3)) (or (not (<= skoY GEN2)) (not (<= skoZ GEN1)))) (and (<= skoZ GEN23) (and (<= skoY GEN24) (and (<= skoX GEN25) (and (<= GEN26 skoZ) (and (<= GEN27 skoY) (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1))))))))))))))))
(assert (= BAV34 (<= skoX GEN14)))
(assert (= BAV35 (and (or (not (<= (* skoZ (* skoY (* skoY (+ (* skoX (* skoX (+ (/ (- GEN15) GEN16) (* skoX (/ GEN4 GEN5))))) (* skoY (* skoX (* skoX (+ (/ GEN17 GEN18) (* skoX (/ GEN4 GEN5)))))))))) (* skoY (+ (* skoX (+ (/ GEN19 GEN20) (* skoX (/ (- GEN6) GEN7)))) (* skoY (* skoX (+ (/ (- GEN21) GEN22) (* skoX (/ (- GEN6) GEN7))))))))) (not (<= skoZ GEN1))) (and (or (not (<= skoY GEN2)) (not (<= skoZ GEN1))) (and (or (not (<= skoX GEN3)) (or (not (<= skoY GEN2)) (not (<= skoZ GEN1)))) (and (<= skoZ GEN23) (and (<= skoY GEN24) (and (<= skoX GEN25) (and (<= GEN26 skoZ) (and (<= GEN27 skoY) (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1)))))))))))))))
(assert (= BAV36 (or (not (<= (* skoZ (* skoY (* skoY (+ (* skoX (* skoX (+ (/ (- GEN15) GEN16) (* skoX (/ GEN4 GEN5))))) (* skoY (* skoX (* skoX (+ (/ GEN17 GEN18) (* skoX (/ GEN4 GEN5)))))))))) (* skoY (+ (* skoX (+ (/ GEN19 GEN20) (* skoX (/ (- GEN6) GEN7)))) (* skoY (* skoX (+ (/ (- GEN21) GEN22) (* skoX (/ (- GEN6) GEN7))))))))) (not (<= skoZ GEN1)))))
(assert (= BAV37 (<= (* skoZ (* skoY (* skoY (+ (* skoX (* skoX (+ (/ (- GEN15) GEN16) (* skoX (/ GEN4 GEN5))))) (* skoY (* skoX (* skoX (+ (/ GEN17 GEN18) (* skoX (/ GEN4 GEN5)))))))))) (* skoY (+ (* skoX (+ (/ GEN19 GEN20) (* skoX (/ (- GEN6) GEN7)))) (* skoY (* skoX (+ (/ (- GEN21) GEN22) (* skoX (/ (- GEN6) GEN7))))))))))
(assert (= BAV38 (<= skoZ GEN1)))
(assert (= BAV39 (and (or (not (<= skoY GEN2)) (not (<= skoZ GEN1))) (and (or (not (<= skoX GEN3)) (or (not (<= skoY GEN2)) (not (<= skoZ GEN1)))) (and (<= skoZ GEN23) (and (<= skoY GEN24) (and (<= skoX GEN25) (and (<= GEN26 skoZ) (and (<= GEN27 skoY) (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1))))))))))))))
(assert (= BAV40 (or (not (<= skoY GEN2)) (not (<= skoZ GEN1)))))
(assert (= BAV41 (<= skoY GEN2)))
(assert (= BAV42 (<= skoZ GEN1)))
(assert (= BAV43 (and (or (not (<= skoX GEN3)) (or (not (<= skoY GEN2)) (not (<= skoZ GEN1)))) (and (<= skoZ GEN23) (and (<= skoY GEN24) (and (<= skoX GEN25) (and (<= GEN26 skoZ) (and (<= GEN27 skoY) (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1)))))))))))))
(assert (= BAV44 (or (not (<= skoX GEN3)) (or (not (<= skoY GEN2)) (not (<= skoZ GEN1))))))
(assert (= BAV45 (<= skoX GEN3)))
(assert (= BAV46 (or (not (<= skoY GEN2)) (not (<= skoZ GEN1)))))
(assert (= BAV47 (<= skoY GEN2)))
(assert (= BAV48 (<= skoZ GEN1)))
(assert (= BAV49 (and (<= skoZ GEN23) (and (<= skoY GEN24) (and (<= skoX GEN25) (and (<= GEN26 skoZ) (and (<= GEN27 skoY) (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1))))))))))))
(assert (= BAV50 (<= skoZ GEN23)))
(assert (= BAV51 (and (<= skoY GEN24) (and (<= skoX GEN25) (and (<= GEN26 skoZ) (and (<= GEN27 skoY) (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1)))))))))))
(assert (= BAV52 (<= skoY GEN24)))
(assert (= BAV53 (and (<= skoX GEN25) (and (<= GEN26 skoZ) (and (<= GEN27 skoY) (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1))))))))))
(assert (= BAV54 (<= skoX GEN25)))
(assert (= BAV55 (and (<= GEN26 skoZ) (and (<= GEN27 skoY) (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1)))))))))
(assert (= BAV56 (<= GEN26 skoZ)))
(assert (= BAV57 (and (<= GEN27 skoY) (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1))))))))
(assert (= BAV58 (<= GEN27 skoY)))
(assert (= BAV59 (and (<= GEN28 skoX) (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1)))))))
(assert (= BAV60 (<= GEN28 skoX)))
(assert (= BAV61 (and (or (not (<= skoX GEN3)) (not (<= skoZ GEN1))) (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1))))))
(assert (= BAV62 (or (not (<= skoX GEN3)) (not (<= skoZ GEN1)))))
(assert (= BAV63 (<= skoX GEN3)))
(assert (= BAV64 (<= skoZ GEN1)))
(assert (= BAV65 (or (not (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))) (not (<= skoZ GEN1)))))
(assert (= BAV66 (<= (* skoZ (* skoY (+ (* skoX (+ (/ (- GEN29) GEN30) (* skoX (/ GEN8 GEN9)))) (* skoY (* skoX (+ (/ GEN31 GEN32) (* skoX (/ GEN8 GEN9)))))))) (+ (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))) (* skoY (+ (/ (- GEN10) GEN11) (* skoX (/ (- GEN12) GEN13))))))))
(assert (= BAV67 (<= skoZ GEN1)))
(check-sat)
(exit)