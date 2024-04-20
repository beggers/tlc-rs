
// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;


#[test]
fn basic_unicode_case() {
  read_and_parse::test("testdata/unicode/case-unicode.tla", "Basic Unicode CASE");
}


#[test]
fn conjlist_with_case() {
  read_and_parse::test("testdata/unicode/case-unicode.tla", "Conjlist with CASE");
}


#[test]
fn conjlist_inside_case() {
  read_and_parse::test("testdata/unicode/case-unicode.tla", "Conjlist inside CASE");
}


#[test]
fn disjlist_with_case() {
  read_and_parse::test("testdata/unicode/case-unicode.tla", "Disjlist with CASE");
}


#[test]
fn disjlist_inside_case() {
  read_and_parse::test("testdata/unicode/case-unicode.tla", "Disjlist inside CASE");
}

