
// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;


#[test]
fn infix_operator_definition() {
  read_and_parse::test("testdata/infix_op.tla", "Infix Operator Definition");
}

#[test]
fn infix_operator_declaration_as_parameter() {
  read_and_parse::test("testdata/infix_op.tla", "Infix Operator Declaration as Parameter");
}

#[test]
fn infix_operator_application() {
  read_and_parse::test("testdata/infix_op.tla", "Infix Operator Application");
}

#[test]
fn infix_operators_as_parameters() {
  read_and_parse::test("testdata/infix_op.tla", "Infix Operators as Parameters");
}

#[test]
fn nonfix_infix_operators() {
  read_and_parse::test("testdata/infix_op.tla", "Nonfix Infix Operators");
}
