(declare-const BAV1 Bool)
(declare-const BAV2 Bool)
(declare-const BAV3 Bool)
(declare-const BAV4 Bool)
(set-info :source |Written by D. B. Staple for GitHub issue #680.|)
(declare-fun x () Real)
(declare-fun y () Real)
(declare-fun z () Real)
(declare-fun r () Real)
(declare-fun theta () Real)
(declare-fun phi () Real)
(assert (= x (* r (* (sin theta) (cos phi)))))
(assert (= y (* r (* (sin theta) (sin phi)))))
(assert (= z (* r (cos theta))))
(assert (distinct (* r r) (+ (* x x) (+ (* y y) (* z z)))))
(check-sat-using qfnra-nlsat)
(assert (= BAV1 (= x (* r (* (sin theta) (cos phi))))))
(assert (= BAV2 (= y (* r (* (sin theta) (sin phi))))))
(assert (= BAV3 (= z (* r (cos theta)))))
(assert (= BAV4 (distinct (* r r) (+ (* x x) (+ (* y y) (* z z))))))