(set-logic UFBV)
(declare-fun _substvar_210_ () Bool)
(declare-fun _substvar_406_ () Bool)
(declare-fun _substvar_407_ () Bool)
(declare-fun _substvar_408_ () Bool)
(declare-fun _substvar_409_ () Bool)
(declare-fun _substvar_411_ () Bool)
(declare-fun _substvar_412_ () Bool)
(declare-fun _substvar_414_ () Bool)
(declare-fun _substvar_415_ () Bool)
(declare-fun _substvar_416_ () Bool)
(declare-fun _substvar_417_ () (_ BitVec 37))
(declare-fun _substvar_418_ () (_ BitVec 37))
(declare-fun _substvar_509_ () Bool)
(declare-fun _substvar_529_ () Bool)
(declare-fun _substvar_530_ () Bool)
(declare-fun _substvar_552_ () Bool)
(declare-fun _substvar_553_ () Bool)
(declare-fun _substvar_555_ () Bool)
(declare-fun _substvar_556_ () Bool)
(declare-fun _substvar_557_ () Bool)
(declare-fun _substvar_573_ () Bool)
(declare-fun _substvar_574_ () Bool)
(declare-fun _substvar_575_ () (_ BitVec 37))
(declare-fun _substvar_591_ () Bool)
(declare-fun _substvar_592_ () Bool)
(declare-fun _substvar_593_ () Bool)
(declare-fun _substvar_604_ () Bool)
(declare-fun _substvar_606_ () Bool)
(declare-fun _substvar_607_ () Bool)
(declare-fun _substvar_612_ () Bool)
(declare-fun _substvar_615_ () Bool)
(declare-fun _substvar_618_ () Bool)
(declare-fun _substvar_621_ () Bool)
(declare-fun _substvar_622_ () Bool)
(declare-fun _substvar_623_ () Bool)
(declare-fun _substvar_625_ () Bool)
(declare-fun _substvar_629_ () Bool)
(declare-fun _substvar_632_ () Bool)
(declare-fun _substvar_636_ () Bool)
(declare-fun _substvar_639_ () Bool)
(declare-fun _substvar_640_ () Bool)
(declare-fun _substvar_643_ () Bool)
(declare-fun _substvar_644_ () (_ BitVec 29))
(declare-fun _substvar_647_ () Bool)
(declare-fun _substvar_648_ () Bool)
(declare-fun _substvar_649_ () (_ BitVec 37))
(declare-fun _substvar_651_ () Bool)
(declare-fun _substvar_652_ () (_ BitVec 29))
(declare-fun _substvar_653_ () Bool)
(declare-fun _substvar_654_ () Bool)
(declare-fun _substvar_657_ () Bool)
(declare-fun _substvar_660_ () Bool)
(declare-fun _substvar_661_ () Bool)
(declare-fun _substvar_663_ () Bool)
(declare-fun _substvar_665_ () Bool)
(declare-fun _substvar_666_ () Bool)
(declare-fun _substvar_667_ () Bool)
(declare-fun _substvar_669_ () Bool)
(declare-fun _substvar_671_ () Bool)
(declare-fun _substvar_672_ () Bool)
(declare-fun _substvar_674_ () Bool)
(set-option :model_validate true)
(set-option :unsat_core true)
(set-option :parallel.enable true)
(set-option :ackermannization.sat_backend true)
(set-option :smt.arith.auto_config_simplex false)
(set-option :smt.clause_proof true)
(set-option :smt.theory_aware_branching false)
(set-option :smt.dack.eq true)
(set-option :rewriter.local_ctx true)
(set-option :smt.threads 3)
(set-option :smt.phase_selection 4)
(set-option :smt.arith.solver 5)
(declare-const v0 Bool)
(declare-const v1 Bool)
(declare-const v2 Bool)
(declare-const v3 Bool)
(declare-const v4 Bool)
(declare-const v5 Bool)
(declare-const v8 Bool)
(declare-const v9 Bool)
(declare-const v10 Bool)
(check-sat)
(declare-const v11 Bool)
(declare-const bv_29-0 (_ BitVec 29))
(check-sat)
(declare-const v12 Bool)
(check-sat)
(push 1)
(declare-const v13 Bool)
(declare-const v14 Bool)
(declare-const v15 Bool)
(declare-const bv_15-0 (_ BitVec 15))
(assert true)
(declare-sort S0 0)
(declare-const v16 Bool)
(assert (! (distinct true true true true (or true true _substvar_415_ (xor v9 v15 (= (_ bv0 29) bv_29-0 (_ bv0 29)) true true true _substvar_529_ _substvar_618_ v8 v3) (xor _substvar_632_ v8))) :named IP_25))
(declare-const v17 Bool)
(declare-sort S1 0)
(declare-const v18 Bool)
(pop 1)
(assert (! _substvar_661_ :named IP_50))
(declare-const v25 Bool)
(declare-const v26 Bool)
(assert _substvar_573_)
(declare-const v27 Bool)
(declare-const v28 Bool)
(declare-const v29 Bool)
(declare-const v30 Bool)
(assert (! _substvar_607_ :named IP_61))
(declare-const v31 Bool)
(declare-const bv_12-3 (_ BitVec 12))
(assert (! (or (forall ((q56 (_ BitVec 37)) (q57 (_ BitVec 37)) (q58 (_ BitVec 37)) (q59 (_ BitVec 37))) (not (distinct (_ bv0 37) q59 _substvar_575_))) (exists ((q56 (_ BitVec 37)) (q57 (_ BitVec 37)) (q58 (_ BitVec 37)) (q59 (_ BitVec 37))) (not (distinct (_ bv0 37) (bvudiv (_ bv0 37) ((_ sign_extend 8) bv_29-0)) (bvudiv (_ bv0 37) (bvudiv _substvar_417_ _substvar_418_)))))) :named IP_64))
(declare-const v32 Bool)
(declare-const v33 Bool)
(declare-const v34 Bool)
(assert (! (forall ((q71 (_ BitVec 29))) (not (= (_ bv0 29) (_ bv0 29) q71))) :named IP_74))
(declare-const v35 Bool)
(push 1)
(declare-const bv_8-5 (_ BitVec 8))
(declare-const v36 Bool)
(check-sat)
