// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn nonfix_minus_gh884() {
    read_and_parse::test("testdata/exclude/sany/operators.tla", "Nonfix Minus GH884");
}

#[test]
fn nonfix_submodule_excl_gh884() {
    read_and_parse::test(
        "testdata/exclude/sany/operators.tla",
        "Nonfix Submodule Excl GH884",
    );
}