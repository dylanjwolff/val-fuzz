(set-option :print-success false)
(set-logic QF_FP)
(set-info :status unknown)
(declare-fun y () (_ FloatingPoint 11 53))
(declare-fun r () (_ FloatingPoint 11 53))
(declare-fun x () (_ FloatingPoint 11 53))

(push 2)
  (assert
   (let (($x5213 (= y (fp (_ bv1 1) (_ bv1352 11) (_ bv2776630467343723 52)))))
   (let (($x14703 (not $x5213)))
   (let (($x7370 (not $x14703)))
   (let (($x14839 (= (fp.add roundTowardPositive x y) r)))
   (let (($x12091 (and $x14703 $x14839)))
   (let (($x14494 (and $x12091 $x7370)))
   (let (($x3307 (not $x14494)))
   (let (($x1490 (= x (fp (_ bv0 1) (_ bv980 11) (_ bv770027620894247 52)))))
   (let (($x15510 (not $x1490)))
   (let (($x6859 (and $x14839 $x12091)))
   (let (($x4431 (not $x6859)))
   (let (($x15679 (and $x4431 $x15510)))
   (let (($x6984 (and $x15679 $x3307)))
   (let (($x15737 (not $x6984)))
   (let (($x11325 (and $x1490 $x12091)))
   (let (($x10080 (not $x11325)))
   (let (($x16883 (and $x7370 $x10080)))
   (let (($x567 (not $x16883)))
   (let (($x7991 (and $x567 $x15737)))
   (not $x7991)))))))))))))))))))))
  (assert
   (let (($x1490 (= x (fp (_ bv0 1) (_ bv980 11) (_ bv770027620894247 52)))))
   (let (($x15510 (not $x1490)))
   (let (($x14839 (= (fp.add roundTowardPositive x y) r)))
   (let (($x5213 (= y (fp (_ bv1 1) (_ bv1352 11) (_ bv2776630467343723 52)))))
   (let (($x14703 (not $x5213)))
   (let (($x12091 (and $x14703 $x14839)))
   (let (($x6859 (and $x14839 $x12091)))
   (let (($x4431 (not $x6859)))
   (let (($x15679 (and $x4431 $x15510)))
   (let (($x7572 (and $x15679 $x1490)))
   (let (($x7370 (not $x14703)))
   (let (($x12728 (not $x15510)))
   (let (($x6743 (and $x12728 $x7370)))
   (let (($x1559 (and $x6743 $x7572)))
   (not $x1559))))))))))))))))
(check-sat)
(pop 0)

(push 2)
  (assert
   (let (($x1490 (= x (fp (_ bv0 1) (_ bv980 11) (_ bv770027620894247 52)))))
   (let (($x15510 (not $x1490)))
   (not $x15510))))
  (assert
   (let (($x5213 (= y (fp (_ bv1 1) (_ bv1352 11) (_ bv2776630467343723 52)))))
   (let (($x14703 (not $x5213)))
   (let (($x7370 (not $x14703)))
   (let (($x1490 (= x (fp (_ bv0 1) (_ bv980 11) (_ bv770027620894247 52)))))
   (let (($x15510 (not $x1490)))
   (let (($x12728 (not $x15510)))
   (let (($x6743 (and $x12728 $x7370)))
   (let (($x11681 (not $x6743)))
   (let (($x9311 (and $x11681 $x6743)))
   (not $x9311)))))))))))
(check-sat)