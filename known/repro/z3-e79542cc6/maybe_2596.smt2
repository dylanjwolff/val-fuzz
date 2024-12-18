(declare-fun %fb () (_ FloatingPoint 8 24))

(assert
  (forall ((undef_1 (_ FloatingPoint 8 24))
           (undef_3 (_ FloatingPoint 8 24)))
    (let ((a!1 (bvnot (bvor
                        (bvnot ((_ fp.to_sbv 32) roundNearestTiesToEven %fb))
                        ((_ fp.to_sbv 32) roundNearestTiesToEven undef_3))))
          (a!2 (bvnot (bvor
                        (bvnot ((_ fp.to_sbv 32) roundNearestTiesToEven undef_1))
                        ((_ fp.to_sbv 32) roundNearestTiesToEven %fb)))))

          (not (= (bvnot (bvor a!1 a!2))
                  (bvxor #xffffffff
                         #x00000000
                         ((_ fp.to_sbv 32) roundNearestTiesToEven %fb)))))))

(check-sat)
(get-model)

; this is the model of %fb
(assert (= %fb (fp #b0 #x81 #b00101000000010001001000)))
(check-sat)
