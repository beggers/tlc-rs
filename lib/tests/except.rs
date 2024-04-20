
// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;


#[test]
fn basic_record_update() {
  read_and_parse::test("testdata/except.tla", "Basic record update");
}


#[test]
fn single_nested_record_update() {
  read_and_parse::test("testdata/except.tla", "Single nested record update");
}


#[test]
fn single_nested_record_update_with_dot_syntax() {
  read_and_parse::test("testdata/except.tla", "Single nested record update with dot syntax");
}


#[test]
fn multiple_nested_record_updates() {
  read_and_parse::test("testdata/except.tla", "Multiple nested record updates");
}


#[test]
fn multiple_nested_record_updates_with_dot_syntax() {
  read_and_parse::test("testdata/except.tla", "Multiple nested record updates with dot syntax");
}
