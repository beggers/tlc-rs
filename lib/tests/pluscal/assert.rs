
// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;


#[test]
fn pluscal_assert() {
  read_and_parse::test("testdata/pluscal/assert.tla", "PlusCal Assert");
}
