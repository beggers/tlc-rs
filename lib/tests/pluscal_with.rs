// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn pluscal_psyntax_with() {
    read_and_parse::test("testdata/pluscal/with.tla", "PlusCal PSyntax With");
}

#[test]
fn pluscal_csyntax_with() {
    read_and_parse::test("testdata/pluscal/with.tla", "PlusCal CSyntax With");
}
