
// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;


#[test]
fn nested_parentheses() {
  read_and_parse::test("testdata/expressions.tla", "Nested Parentheses");
}


#[test]
fn tuple_literal() {
  read_and_parse::test("testdata/expressions.tla", "Tuple Literal");
}


#[test]
fn tuple_literal_with_jlist() {
  read_and_parse::test("testdata/expressions.tla", "Tuple Literal with Jlist");
}
