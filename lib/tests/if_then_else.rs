
// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;


#[test]
fn ite_with_strings() {
  read_and_parse::test("testdata/if_then_else.tla", "ITE with Strings");
}


#[test]
fn conjlist_with_ite() {
  read_and_parse::test("testdata/if_then_else.tla", "Conjlist with ITE");
}


#[test]
fn conjlist_inside_ite() {
  read_and_parse::test("testdata/if_then_else.tla", "Conjlist inside ITE");
}


#[test]
fn disjlist_with_ite() {
  read_and_parse::test("testdata/if_then_else.tla", "Disjlist with ITE");
}


#[test]
fn disjlist_inside_ite() {
  read_and_parse::test("testdata/if_then_else.tla", "Disjlist inside ITE");
}
