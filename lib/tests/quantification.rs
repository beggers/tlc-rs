// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn bounded_quantification() {
    read_and_parse::test("testdata/quantification.tla", "Bounded Quantification");
}

#[test]
fn unbounded_quantification() {
    read_and_parse::test("testdata/quantification.tla", "Unbounded Quantification");
}

#[test]
fn bounded_choose() {
    read_and_parse::test("testdata/quantification.tla", "Bounded CHOOSE");
}

#[test]
fn unbounded_choose() {
    read_and_parse::test("testdata/quantification.tla", "Unbounded CHOOSE");
}
