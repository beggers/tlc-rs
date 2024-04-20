
// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;


#[test]
fn conjlist_followed_by_disjunct() {
  read_and_parse::test("testdata/jlist.tla", "Conjlist Followed by Disjunct");
}


#[test]
fn disjlist_followed_by_conjunct() {
  read_and_parse::test("testdata/jlist.tla", "Disjlist Followed by Conjunct");
}


#[test]
fn two_mismatched_jlists() {
  read_and_parse::test("testdata/jlist.tla", "Two Mismatched Jlists");
}


#[test]
fn nested_alternating_jlists() {
  read_and_parse::test("testdata/jlist.tla", "Nested Alternating Jlists");
}

