(set-logic ALL)
(declare-const GEN1 (_ BitVec 32))
(declare-const GEN2 (_ BitVec 32))
(declare-const GEN3 (_ BitVec 32))
(declare-const GEN4 (_ BitVec 32))
(declare-const BAV5 Bool)
(declare-const BAV6 Bool)
(declare-const BAV7 Bool)
(declare-const BAV8 Bool)
(declare-const BAV9 Bool)
(declare-const BAV10 Bool)
(declare-fun _substvar_245_ () Bool)
(declare-fun _substvar_246_ () Bool)
(declare-fun _substvar_247_ () Bool)
(declare-fun group_size_x () (_ BitVec 32))
(declare-fun group_id_x$2 () (_ BitVec 32))
(declare-fun local_id_x$2 () (_ BitVec 32))
(assert (= _substvar_245_ _substvar_246_))
(assert (and (bvsge group_id_x$2 GEN1) (bvsge local_id_x$2 GEN2) (bvslt local_id_x$2 group_size_x) (or (not (bvsgt group_size_x GEN3)) (not (and (=> _substvar_247_ (bvsge group_id_x$2 GEN4)) (= _substvar_245_ _substvar_246_))))))
(assert (= BAV5 (= _substvar_245_ _substvar_246_)))
(assert (= BAV6 (and (bvsge group_id_x$2 GEN1) (bvsge local_id_x$2 GEN2) (bvslt local_id_x$2 group_size_x) (or (not (bvsgt group_size_x GEN3)) (not (and (=> _substvar_247_ (bvsge group_id_x$2 GEN4)) (= _substvar_245_ _substvar_246_)))))))
(assert (= BAV7 (or (not (bvsgt group_size_x GEN3)) (not (and (=> _substvar_247_ (bvsge group_id_x$2 GEN4)) (= _substvar_245_ _substvar_246_))))))
(assert (= BAV8 (and (=> _substvar_247_ (bvsge group_id_x$2 GEN4)) (= _substvar_245_ _substvar_246_))))
(assert (= BAV9 (=> _substvar_247_ (bvsge group_id_x$2 GEN4))))
(assert (= BAV10 (= _substvar_245_ _substvar_246_)))
(check-sat)