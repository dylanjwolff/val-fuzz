(set-logic ALL)
(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const GEN6 Real)
(declare-const GEN5 Int)
(declare-const GEN4 Int)
(declare-const GEN3 Real)
(declare-const GEN2 Int)
(declare-const GEN1 Int)
(declare-fun j () (_ FloatingPoint 3 5))
(declare-fun i () (_ FloatingPoint 3 5))
(assert (not (or (not (fp.gt (fp.sub roundNearestTiesToEven i j) ((_ to_fp 3 5) roundNearestTiesToEven GEN3))) (and (fp.leq (fp.sub roundNearestTiesToEven (fp.sub roundNearestTiesToEven (fp.add roundNearestTiesToEven j i) (fp.sub roundNearestTiesToEven (fp.add roundNearestTiesToEven j i) j)) (fp.sub roundNearestTiesToEven (fp.add roundNearestTiesToEven j i) j)) ((_ to_fp 3 5) roundNearestTiesToEven GEN6)) true))))
(assert (= BAV1 (or (not (fp.gt (fp.sub roundNearestTiesToEven i j) ((_ to_fp 3 5) roundNearestTiesToEven GEN3))) (and (fp.leq (fp.sub roundNearestTiesToEven (fp.sub roundNearestTiesToEven (fp.add roundNearestTiesToEven j i) (fp.sub roundNearestTiesToEven (fp.add roundNearestTiesToEven j i) j)) (fp.sub roundNearestTiesToEven (fp.add roundNearestTiesToEven j i) j)) ((_ to_fp 3 5) roundNearestTiesToEven GEN6)) true))))
(assert (= BAV2 (and (fp.leq (fp.sub roundNearestTiesToEven (fp.sub roundNearestTiesToEven (fp.add roundNearestTiesToEven j i) (fp.sub roundNearestTiesToEven (fp.add roundNearestTiesToEven j i) j)) (fp.sub roundNearestTiesToEven (fp.add roundNearestTiesToEven j i) j)) ((_ to_fp 3 5) roundNearestTiesToEven GEN6)) true)))
(check-sat)
(get-model)