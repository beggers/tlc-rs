// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn decimal_no_leading_zero_gh596() {
    read_and_parse::test(
        "testdata/exclude/sany/number.tla",
        "Decimal No Leading Zero GH596",
    );
}
