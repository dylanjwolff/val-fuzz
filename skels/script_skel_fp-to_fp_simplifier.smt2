(set-logic ALL)
(simplify (( _ to_fp 11 53 ) roundNearestTiesToEven 1.0 0))
(simplify (( _ to_fp 8 24 ) roundNearestTiesToEven 1.0 0))
(simplify ((_ to_fp 11 53) roundNearestTiesToEven
			(( _ to_fp 8 24 ) roundNearestTiesToEven 1.0 0)))