#![allow(non_snake_case)]
// Generated from /home/dylan/git/smtlibv2-grammar/src/main/resources/SMTLIBv2.g4 by ANTLR 4.8
use super::smtlibv2parser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait SMTLIBv2Listener: ParseTreeListener {
    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#start}.
     * @param ctx the parse tree
     */
    fn enter_start(&mut self, _ctx: &StartContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#start}.
     * @param ctx the parse tree
     */
    fn exit_start(&mut self, _ctx: &StartContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#response}.
     * @param ctx the parse tree
     */
    fn enter_response(&mut self, _ctx: &ResponseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#response}.
     * @param ctx the parse tree
     */
    fn exit_response(&mut self, _ctx: &ResponseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#generalReservedWord}.
     * @param ctx the parse tree
     */
    fn enter_generalReservedWord(&mut self, _ctx: &GeneralReservedWordContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#generalReservedWord}.
     * @param ctx the parse tree
     */
    fn exit_generalReservedWord(&mut self, _ctx: &GeneralReservedWordContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#simpleSymbol}.
     * @param ctx the parse tree
     */
    fn enter_simpleSymbol(&mut self, _ctx: &SimpleSymbolContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#simpleSymbol}.
     * @param ctx the parse tree
     */
    fn exit_simpleSymbol(&mut self, _ctx: &SimpleSymbolContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#quotedSymbol}.
     * @param ctx the parse tree
     */
    fn enter_quotedSymbol(&mut self, _ctx: &QuotedSymbolContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#quotedSymbol}.
     * @param ctx the parse tree
     */
    fn exit_quotedSymbol(&mut self, _ctx: &QuotedSymbolContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#predefSymbol}.
     * @param ctx the parse tree
     */
    fn enter_predefSymbol(&mut self, _ctx: &PredefSymbolContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#predefSymbol}.
     * @param ctx the parse tree
     */
    fn exit_predefSymbol(&mut self, _ctx: &PredefSymbolContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#predefKeyword}.
     * @param ctx the parse tree
     */
    fn enter_predefKeyword(&mut self, _ctx: &PredefKeywordContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#predefKeyword}.
     * @param ctx the parse tree
     */
    fn exit_predefKeyword(&mut self, _ctx: &PredefKeywordContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#symbol}.
     * @param ctx the parse tree
     */
    fn enter_symbol(&mut self, _ctx: &SymbolContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#symbol}.
     * @param ctx the parse tree
     */
    fn exit_symbol(&mut self, _ctx: &SymbolContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#numeral}.
     * @param ctx the parse tree
     */
    fn enter_numeral(&mut self, _ctx: &NumeralContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#numeral}.
     * @param ctx the parse tree
     */
    fn exit_numeral(&mut self, _ctx: &NumeralContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#decimal}.
     * @param ctx the parse tree
     */
    fn enter_decimal(&mut self, _ctx: &DecimalContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#decimal}.
     * @param ctx the parse tree
     */
    fn exit_decimal(&mut self, _ctx: &DecimalContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#hexadecimal}.
     * @param ctx the parse tree
     */
    fn enter_hexadecimal(&mut self, _ctx: &HexadecimalContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#hexadecimal}.
     * @param ctx the parse tree
     */
    fn exit_hexadecimal(&mut self, _ctx: &HexadecimalContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#binary}.
     * @param ctx the parse tree
     */
    fn enter_binary(&mut self, _ctx: &BinaryContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#binary}.
     * @param ctx the parse tree
     */
    fn exit_binary(&mut self, _ctx: &BinaryContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#string}.
     * @param ctx the parse tree
     */
    fn enter_string(&mut self, _ctx: &StringContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#string}.
     * @param ctx the parse tree
     */
    fn exit_string(&mut self, _ctx: &StringContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#keyword}.
     * @param ctx the parse tree
     */
    fn enter_keyword(&mut self, _ctx: &KeywordContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#keyword}.
     * @param ctx the parse tree
     */
    fn exit_keyword(&mut self, _ctx: &KeywordContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#spec_constant}.
     * @param ctx the parse tree
     */
    fn enter_spec_constant(&mut self, _ctx: &Spec_constantContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#spec_constant}.
     * @param ctx the parse tree
     */
    fn exit_spec_constant(&mut self, _ctx: &Spec_constantContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#s_expr}.
     * @param ctx the parse tree
     */
    fn enter_s_expr(&mut self, _ctx: &S_exprContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#s_expr}.
     * @param ctx the parse tree
     */
    fn exit_s_expr(&mut self, _ctx: &S_exprContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#index}.
     * @param ctx the parse tree
     */
    fn enter_index(&mut self, _ctx: &IndexContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#index}.
     * @param ctx the parse tree
     */
    fn exit_index(&mut self, _ctx: &IndexContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#identifier}.
     * @param ctx the parse tree
     */
    fn enter_identifier(&mut self, _ctx: &IdentifierContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#identifier}.
     * @param ctx the parse tree
     */
    fn exit_identifier(&mut self, _ctx: &IdentifierContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#attribute_value}.
     * @param ctx the parse tree
     */
    fn enter_attribute_value(&mut self, _ctx: &Attribute_valueContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#attribute_value}.
     * @param ctx the parse tree
     */
    fn exit_attribute_value(&mut self, _ctx: &Attribute_valueContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#attribute}.
     * @param ctx the parse tree
     */
    fn enter_attribute(&mut self, _ctx: &AttributeContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#attribute}.
     * @param ctx the parse tree
     */
    fn exit_attribute(&mut self, _ctx: &AttributeContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#sort}.
     * @param ctx the parse tree
     */
    fn enter_sort(&mut self, _ctx: &SortContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#sort}.
     * @param ctx the parse tree
     */
    fn exit_sort(&mut self, _ctx: &SortContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#qual_identifer}.
     * @param ctx the parse tree
     */
    fn enter_qual_identifer(&mut self, _ctx: &Qual_identiferContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#qual_identifer}.
     * @param ctx the parse tree
     */
    fn exit_qual_identifer(&mut self, _ctx: &Qual_identiferContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#var_binding}.
     * @param ctx the parse tree
     */
    fn enter_var_binding(&mut self, _ctx: &Var_bindingContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#var_binding}.
     * @param ctx the parse tree
     */
    fn exit_var_binding(&mut self, _ctx: &Var_bindingContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#sorted_var}.
     * @param ctx the parse tree
     */
    fn enter_sorted_var(&mut self, _ctx: &Sorted_varContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#sorted_var}.
     * @param ctx the parse tree
     */
    fn exit_sorted_var(&mut self, _ctx: &Sorted_varContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#pattern}.
     * @param ctx the parse tree
     */
    fn enter_pattern(&mut self, _ctx: &PatternContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#pattern}.
     * @param ctx the parse tree
     */
    fn exit_pattern(&mut self, _ctx: &PatternContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#match_case}.
     * @param ctx the parse tree
     */
    fn enter_match_case(&mut self, _ctx: &Match_caseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#match_case}.
     * @param ctx the parse tree
     */
    fn exit_match_case(&mut self, _ctx: &Match_caseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#term}.
     * @param ctx the parse tree
     */
    fn enter_term(&mut self, _ctx: &TermContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#term}.
     * @param ctx the parse tree
     */
    fn exit_term(&mut self, _ctx: &TermContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#sort_symbol_decl}.
     * @param ctx the parse tree
     */
    fn enter_sort_symbol_decl(&mut self, _ctx: &Sort_symbol_declContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#sort_symbol_decl}.
     * @param ctx the parse tree
     */
    fn exit_sort_symbol_decl(&mut self, _ctx: &Sort_symbol_declContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#meta_spec_constant}.
     * @param ctx the parse tree
     */
    fn enter_meta_spec_constant(&mut self, _ctx: &Meta_spec_constantContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#meta_spec_constant}.
     * @param ctx the parse tree
     */
    fn exit_meta_spec_constant(&mut self, _ctx: &Meta_spec_constantContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#fun_symbol_decl}.
     * @param ctx the parse tree
     */
    fn enter_fun_symbol_decl(&mut self, _ctx: &Fun_symbol_declContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#fun_symbol_decl}.
     * @param ctx the parse tree
     */
    fn exit_fun_symbol_decl(&mut self, _ctx: &Fun_symbol_declContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#par_fun_symbol_decl}.
     * @param ctx the parse tree
     */
    fn enter_par_fun_symbol_decl(&mut self, _ctx: &Par_fun_symbol_declContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#par_fun_symbol_decl}.
     * @param ctx the parse tree
     */
    fn exit_par_fun_symbol_decl(&mut self, _ctx: &Par_fun_symbol_declContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#theory_attribute}.
     * @param ctx the parse tree
     */
    fn enter_theory_attribute(&mut self, _ctx: &Theory_attributeContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#theory_attribute}.
     * @param ctx the parse tree
     */
    fn exit_theory_attribute(&mut self, _ctx: &Theory_attributeContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#theory_decl}.
     * @param ctx the parse tree
     */
    fn enter_theory_decl(&mut self, _ctx: &Theory_declContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#theory_decl}.
     * @param ctx the parse tree
     */
    fn exit_theory_decl(&mut self, _ctx: &Theory_declContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#logic_attribue}.
     * @param ctx the parse tree
     */
    fn enter_logic_attribue(&mut self, _ctx: &Logic_attribueContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#logic_attribue}.
     * @param ctx the parse tree
     */
    fn exit_logic_attribue(&mut self, _ctx: &Logic_attribueContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#logic}.
     * @param ctx the parse tree
     */
    fn enter_logic(&mut self, _ctx: &LogicContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#logic}.
     * @param ctx the parse tree
     */
    fn exit_logic(&mut self, _ctx: &LogicContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#sort_dec}.
     * @param ctx the parse tree
     */
    fn enter_sort_dec(&mut self, _ctx: &Sort_decContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#sort_dec}.
     * @param ctx the parse tree
     */
    fn exit_sort_dec(&mut self, _ctx: &Sort_decContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#selector_dec}.
     * @param ctx the parse tree
     */
    fn enter_selector_dec(&mut self, _ctx: &Selector_decContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#selector_dec}.
     * @param ctx the parse tree
     */
    fn exit_selector_dec(&mut self, _ctx: &Selector_decContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#constructor_dec}.
     * @param ctx the parse tree
     */
    fn enter_constructor_dec(&mut self, _ctx: &Constructor_decContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#constructor_dec}.
     * @param ctx the parse tree
     */
    fn exit_constructor_dec(&mut self, _ctx: &Constructor_decContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#datatype_dec}.
     * @param ctx the parse tree
     */
    fn enter_datatype_dec(&mut self, _ctx: &Datatype_decContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#datatype_dec}.
     * @param ctx the parse tree
     */
    fn exit_datatype_dec(&mut self, _ctx: &Datatype_decContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#function_dec}.
     * @param ctx the parse tree
     */
    fn enter_function_dec(&mut self, _ctx: &Function_decContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#function_dec}.
     * @param ctx the parse tree
     */
    fn exit_function_dec(&mut self, _ctx: &Function_decContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#function_def}.
     * @param ctx the parse tree
     */
    fn enter_function_def(&mut self, _ctx: &Function_defContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#function_def}.
     * @param ctx the parse tree
     */
    fn exit_function_def(&mut self, _ctx: &Function_defContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#prop_literal}.
     * @param ctx the parse tree
     */
    fn enter_prop_literal(&mut self, _ctx: &Prop_literalContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#prop_literal}.
     * @param ctx the parse tree
     */
    fn exit_prop_literal(&mut self, _ctx: &Prop_literalContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#script}.
     * @param ctx the parse tree
     */
    fn enter_script(&mut self, _ctx: &ScriptContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#script}.
     * @param ctx the parse tree
     */
    fn exit_script(&mut self, _ctx: &ScriptContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_assert}.
     * @param ctx the parse tree
     */
    fn enter_cmd_assert(&mut self, _ctx: &Cmd_assertContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_assert}.
     * @param ctx the parse tree
     */
    fn exit_cmd_assert(&mut self, _ctx: &Cmd_assertContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_checkSat}.
     * @param ctx the parse tree
     */
    fn enter_cmd_checkSat(&mut self, _ctx: &Cmd_checkSatContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_checkSat}.
     * @param ctx the parse tree
     */
    fn exit_cmd_checkSat(&mut self, _ctx: &Cmd_checkSatContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_checkSatAssuming}.
     * @param ctx the parse tree
     */
    fn enter_cmd_checkSatAssuming(&mut self, _ctx: &Cmd_checkSatAssumingContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_checkSatAssuming}.
     * @param ctx the parse tree
     */
    fn exit_cmd_checkSatAssuming(&mut self, _ctx: &Cmd_checkSatAssumingContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_declareConst}.
     * @param ctx the parse tree
     */
    fn enter_cmd_declareConst(&mut self, _ctx: &Cmd_declareConstContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareConst}.
     * @param ctx the parse tree
     */
    fn exit_cmd_declareConst(&mut self, _ctx: &Cmd_declareConstContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_declareDatatype}.
     * @param ctx the parse tree
     */
    fn enter_cmd_declareDatatype(&mut self, _ctx: &Cmd_declareDatatypeContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareDatatype}.
     * @param ctx the parse tree
     */
    fn exit_cmd_declareDatatype(&mut self, _ctx: &Cmd_declareDatatypeContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_declareDatatypes}.
     * @param ctx the parse tree
     */
    fn enter_cmd_declareDatatypes(&mut self, _ctx: &Cmd_declareDatatypesContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareDatatypes}.
     * @param ctx the parse tree
     */
    fn exit_cmd_declareDatatypes(&mut self, _ctx: &Cmd_declareDatatypesContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_declareFun}.
     * @param ctx the parse tree
     */
    fn enter_cmd_declareFun(&mut self, _ctx: &Cmd_declareFunContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareFun}.
     * @param ctx the parse tree
     */
    fn exit_cmd_declareFun(&mut self, _ctx: &Cmd_declareFunContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_declareSort}.
     * @param ctx the parse tree
     */
    fn enter_cmd_declareSort(&mut self, _ctx: &Cmd_declareSortContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_declareSort}.
     * @param ctx the parse tree
     */
    fn exit_cmd_declareSort(&mut self, _ctx: &Cmd_declareSortContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFun}.
     * @param ctx the parse tree
     */
    fn enter_cmd_defineFun(&mut self, _ctx: &Cmd_defineFunContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFun}.
     * @param ctx the parse tree
     */
    fn exit_cmd_defineFun(&mut self, _ctx: &Cmd_defineFunContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFunRec}.
     * @param ctx the parse tree
     */
    fn enter_cmd_defineFunRec(&mut self, _ctx: &Cmd_defineFunRecContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFunRec}.
     * @param ctx the parse tree
     */
    fn exit_cmd_defineFunRec(&mut self, _ctx: &Cmd_defineFunRecContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFunsRec}.
     * @param ctx the parse tree
     */
    fn enter_cmd_defineFunsRec(&mut self, _ctx: &Cmd_defineFunsRecContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineFunsRec}.
     * @param ctx the parse tree
     */
    fn exit_cmd_defineFunsRec(&mut self, _ctx: &Cmd_defineFunsRecContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_defineSort}.
     * @param ctx the parse tree
     */
    fn enter_cmd_defineSort(&mut self, _ctx: &Cmd_defineSortContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_defineSort}.
     * @param ctx the parse tree
     */
    fn exit_cmd_defineSort(&mut self, _ctx: &Cmd_defineSortContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_echo}.
     * @param ctx the parse tree
     */
    fn enter_cmd_echo(&mut self, _ctx: &Cmd_echoContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_echo}.
     * @param ctx the parse tree
     */
    fn exit_cmd_echo(&mut self, _ctx: &Cmd_echoContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_exit}.
     * @param ctx the parse tree
     */
    fn enter_cmd_exit(&mut self, _ctx: &Cmd_exitContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_exit}.
     * @param ctx the parse tree
     */
    fn exit_cmd_exit(&mut self, _ctx: &Cmd_exitContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getAssertions}.
     * @param ctx the parse tree
     */
    fn enter_cmd_getAssertions(&mut self, _ctx: &Cmd_getAssertionsContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getAssertions}.
     * @param ctx the parse tree
     */
    fn exit_cmd_getAssertions(&mut self, _ctx: &Cmd_getAssertionsContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getAssignment}.
     * @param ctx the parse tree
     */
    fn enter_cmd_getAssignment(&mut self, _ctx: &Cmd_getAssignmentContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getAssignment}.
     * @param ctx the parse tree
     */
    fn exit_cmd_getAssignment(&mut self, _ctx: &Cmd_getAssignmentContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getInfo}.
     * @param ctx the parse tree
     */
    fn enter_cmd_getInfo(&mut self, _ctx: &Cmd_getInfoContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getInfo}.
     * @param ctx the parse tree
     */
    fn exit_cmd_getInfo(&mut self, _ctx: &Cmd_getInfoContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getModel}.
     * @param ctx the parse tree
     */
    fn enter_cmd_getModel(&mut self, _ctx: &Cmd_getModelContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getModel}.
     * @param ctx the parse tree
     */
    fn exit_cmd_getModel(&mut self, _ctx: &Cmd_getModelContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getOption}.
     * @param ctx the parse tree
     */
    fn enter_cmd_getOption(&mut self, _ctx: &Cmd_getOptionContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getOption}.
     * @param ctx the parse tree
     */
    fn exit_cmd_getOption(&mut self, _ctx: &Cmd_getOptionContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getProof}.
     * @param ctx the parse tree
     */
    fn enter_cmd_getProof(&mut self, _ctx: &Cmd_getProofContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getProof}.
     * @param ctx the parse tree
     */
    fn exit_cmd_getProof(&mut self, _ctx: &Cmd_getProofContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getUnsatAssumptions}.
     * @param ctx the parse tree
     */
    fn enter_cmd_getUnsatAssumptions(&mut self, _ctx: &Cmd_getUnsatAssumptionsContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getUnsatAssumptions}.
     * @param ctx the parse tree
     */
    fn exit_cmd_getUnsatAssumptions(&mut self, _ctx: &Cmd_getUnsatAssumptionsContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getUnsatCore}.
     * @param ctx the parse tree
     */
    fn enter_cmd_getUnsatCore(&mut self, _ctx: &Cmd_getUnsatCoreContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getUnsatCore}.
     * @param ctx the parse tree
     */
    fn exit_cmd_getUnsatCore(&mut self, _ctx: &Cmd_getUnsatCoreContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_getValue}.
     * @param ctx the parse tree
     */
    fn enter_cmd_getValue(&mut self, _ctx: &Cmd_getValueContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_getValue}.
     * @param ctx the parse tree
     */
    fn exit_cmd_getValue(&mut self, _ctx: &Cmd_getValueContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_pop}.
     * @param ctx the parse tree
     */
    fn enter_cmd_pop(&mut self, _ctx: &Cmd_popContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_pop}.
     * @param ctx the parse tree
     */
    fn exit_cmd_pop(&mut self, _ctx: &Cmd_popContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_push}.
     * @param ctx the parse tree
     */
    fn enter_cmd_push(&mut self, _ctx: &Cmd_pushContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_push}.
     * @param ctx the parse tree
     */
    fn exit_cmd_push(&mut self, _ctx: &Cmd_pushContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_reset}.
     * @param ctx the parse tree
     */
    fn enter_cmd_reset(&mut self, _ctx: &Cmd_resetContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_reset}.
     * @param ctx the parse tree
     */
    fn exit_cmd_reset(&mut self, _ctx: &Cmd_resetContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_resetAssertions}.
     * @param ctx the parse tree
     */
    fn enter_cmd_resetAssertions(&mut self, _ctx: &Cmd_resetAssertionsContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_resetAssertions}.
     * @param ctx the parse tree
     */
    fn exit_cmd_resetAssertions(&mut self, _ctx: &Cmd_resetAssertionsContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_setInfo}.
     * @param ctx the parse tree
     */
    fn enter_cmd_setInfo(&mut self, _ctx: &Cmd_setInfoContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_setInfo}.
     * @param ctx the parse tree
     */
    fn exit_cmd_setInfo(&mut self, _ctx: &Cmd_setInfoContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_setLogic}.
     * @param ctx the parse tree
     */
    fn enter_cmd_setLogic(&mut self, _ctx: &Cmd_setLogicContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_setLogic}.
     * @param ctx the parse tree
     */
    fn exit_cmd_setLogic(&mut self, _ctx: &Cmd_setLogicContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#cmd_setOption}.
     * @param ctx the parse tree
     */
    fn enter_cmd_setOption(&mut self, _ctx: &Cmd_setOptionContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#cmd_setOption}.
     * @param ctx the parse tree
     */
    fn exit_cmd_setOption(&mut self, _ctx: &Cmd_setOptionContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#command}.
     * @param ctx the parse tree
     */
    fn enter_command(&mut self, _ctx: &CommandContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#command}.
     * @param ctx the parse tree
     */
    fn exit_command(&mut self, _ctx: &CommandContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#b_value}.
     * @param ctx the parse tree
     */
    fn enter_b_value(&mut self, _ctx: &B_valueContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#b_value}.
     * @param ctx the parse tree
     */
    fn exit_b_value(&mut self, _ctx: &B_valueContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#option}.
     * @param ctx the parse tree
     */
    fn enter_option(&mut self, _ctx: &OptionContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#option}.
     * @param ctx the parse tree
     */
    fn exit_option(&mut self, _ctx: &OptionContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#info_flag}.
     * @param ctx the parse tree
     */
    fn enter_info_flag(&mut self, _ctx: &Info_flagContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#info_flag}.
     * @param ctx the parse tree
     */
    fn exit_info_flag(&mut self, _ctx: &Info_flagContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#error_behaviour}.
     * @param ctx the parse tree
     */
    fn enter_error_behaviour(&mut self, _ctx: &Error_behaviourContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#error_behaviour}.
     * @param ctx the parse tree
     */
    fn exit_error_behaviour(&mut self, _ctx: &Error_behaviourContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#reason_unknown}.
     * @param ctx the parse tree
     */
    fn enter_reason_unknown(&mut self, _ctx: &Reason_unknownContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#reason_unknown}.
     * @param ctx the parse tree
     */
    fn exit_reason_unknown(&mut self, _ctx: &Reason_unknownContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#model_response}.
     * @param ctx the parse tree
     */
    fn enter_model_response(&mut self, _ctx: &Model_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#model_response}.
     * @param ctx the parse tree
     */
    fn exit_model_response(&mut self, _ctx: &Model_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#info_response}.
     * @param ctx the parse tree
     */
    fn enter_info_response(&mut self, _ctx: &Info_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#info_response}.
     * @param ctx the parse tree
     */
    fn exit_info_response(&mut self, _ctx: &Info_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#valuation_pair}.
     * @param ctx the parse tree
     */
    fn enter_valuation_pair(&mut self, _ctx: &Valuation_pairContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#valuation_pair}.
     * @param ctx the parse tree
     */
    fn exit_valuation_pair(&mut self, _ctx: &Valuation_pairContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#t_valuation_pair}.
     * @param ctx the parse tree
     */
    fn enter_t_valuation_pair(&mut self, _ctx: &T_valuation_pairContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#t_valuation_pair}.
     * @param ctx the parse tree
     */
    fn exit_t_valuation_pair(&mut self, _ctx: &T_valuation_pairContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#check_sat_response}.
     * @param ctx the parse tree
     */
    fn enter_check_sat_response(&mut self, _ctx: &Check_sat_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#check_sat_response}.
     * @param ctx the parse tree
     */
    fn exit_check_sat_response(&mut self, _ctx: &Check_sat_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#echo_response}.
     * @param ctx the parse tree
     */
    fn enter_echo_response(&mut self, _ctx: &Echo_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#echo_response}.
     * @param ctx the parse tree
     */
    fn exit_echo_response(&mut self, _ctx: &Echo_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#get_assertions_response}.
     * @param ctx the parse tree
     */
    fn enter_get_assertions_response(&mut self, _ctx: &Get_assertions_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#get_assertions_response}.
     * @param ctx the parse tree
     */
    fn exit_get_assertions_response(&mut self, _ctx: &Get_assertions_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#get_assignment_response}.
     * @param ctx the parse tree
     */
    fn enter_get_assignment_response(&mut self, _ctx: &Get_assignment_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#get_assignment_response}.
     * @param ctx the parse tree
     */
    fn exit_get_assignment_response(&mut self, _ctx: &Get_assignment_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#get_info_response}.
     * @param ctx the parse tree
     */
    fn enter_get_info_response(&mut self, _ctx: &Get_info_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#get_info_response}.
     * @param ctx the parse tree
     */
    fn exit_get_info_response(&mut self, _ctx: &Get_info_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#get_model_response}.
     * @param ctx the parse tree
     */
    fn enter_get_model_response(&mut self, _ctx: &Get_model_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#get_model_response}.
     * @param ctx the parse tree
     */
    fn exit_get_model_response(&mut self, _ctx: &Get_model_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#get_option_response}.
     * @param ctx the parse tree
     */
    fn enter_get_option_response(&mut self, _ctx: &Get_option_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#get_option_response}.
     * @param ctx the parse tree
     */
    fn exit_get_option_response(&mut self, _ctx: &Get_option_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#get_proof_response}.
     * @param ctx the parse tree
     */
    fn enter_get_proof_response(&mut self, _ctx: &Get_proof_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#get_proof_response}.
     * @param ctx the parse tree
     */
    fn exit_get_proof_response(&mut self, _ctx: &Get_proof_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#get_unsat_assump_response}.
     * @param ctx the parse tree
     */
    fn enter_get_unsat_assump_response(&mut self, _ctx: &Get_unsat_assump_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#get_unsat_assump_response}.
     * @param ctx the parse tree
     */
    fn exit_get_unsat_assump_response(&mut self, _ctx: &Get_unsat_assump_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#get_unsat_core_response}.
     * @param ctx the parse tree
     */
    fn enter_get_unsat_core_response(&mut self, _ctx: &Get_unsat_core_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#get_unsat_core_response}.
     * @param ctx the parse tree
     */
    fn exit_get_unsat_core_response(&mut self, _ctx: &Get_unsat_core_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#get_value_response}.
     * @param ctx the parse tree
     */
    fn enter_get_value_response(&mut self, _ctx: &Get_value_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#get_value_response}.
     * @param ctx the parse tree
     */
    fn exit_get_value_response(&mut self, _ctx: &Get_value_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#specific_success_response}.
     * @param ctx the parse tree
     */
    fn enter_specific_success_response(&mut self, _ctx: &Specific_success_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#specific_success_response}.
     * @param ctx the parse tree
     */
    fn exit_specific_success_response(&mut self, _ctx: &Specific_success_responseContext) {}

    /**
     * Enter a parse tree produced by {@link SMTLIBv2Parser#general_response}.
     * @param ctx the parse tree
     */
    fn enter_general_response(&mut self, _ctx: &General_responseContext) {}
    /**
     * Exit a parse tree produced by {@link SMTLIBv2Parser#general_response}.
     * @param ctx the parse tree
     */
    fn exit_general_response(&mut self, _ctx: &General_responseContext) {}
}
