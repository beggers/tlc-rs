
// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;


#[test]
fn unicode_number_sets() {
  read_and_parse::test("testdata/unicode/numbers-unicode.tla", "Unicode Number Sets");
}

