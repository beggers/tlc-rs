// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn operators_with_colon_prefixes__jlists_gh87() {
    read_and_parse::test(
        "testdata/regression.tla",
        "Operators with Colon Prefixes  Jlists GH87",
    );
}
