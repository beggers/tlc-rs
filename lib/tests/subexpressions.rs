
// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;


#[test]
fn basic_submodule_reference() {
  read_and_parse::test("testdata/subexpressions.tla", "Basic Submodule Reference");
}

#[test]
fn subexpression_tree_navigation() {
  read_and_parse::test("testdata/subexpressions.tla", "Subexpression Tree Navigation");
}

#[test]
fn proof_step_id_subexpression_tree_navigation() {
  read_and_parse::test("testdata/subexpressions.tla", "Proof Step ID Subexpression Tree Navigation");
}
