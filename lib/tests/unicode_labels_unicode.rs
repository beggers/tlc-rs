// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn basic_label() {
    read_and_parse::test("testdata/unicode/labels-unicode.tla", "Basic Label");
}

#[test]
fn label_with_parameters() {
    read_and_parse::test(
        "testdata/unicode/labels-unicode.tla",
        "Label With Parameters",
    );
}
