// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn pluscal_psyntax_procedures() {
    read_and_parse::test(
        "testdata/pluscal/procedures.tla",
        "PlusCal PSyntax Procedures",
    );
}

#[test]
fn pluscal_csyntax_procedures() {
    read_and_parse::test(
        "testdata/pluscal/procedures.tla",
        "PlusCal CSyntax Procedures",
    );
}
