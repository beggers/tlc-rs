// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn unicode_string() {
    read_and_parse::test("testdata/unicode/string-unicode.tla", "Unicode String");
}