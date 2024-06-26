// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn basic_recursive_declaration() {
    read_and_parse::test("testdata/recursive.tla", "Basic Recursive Declaration");
}

#[test]
fn recursive_declaration_without_parameters() {
    read_and_parse::test(
        "testdata/recursive.tla",
        "Recursive Declaration Without Parameters",
    );
}

#[test]
fn multiple_recursive_declarations() {
    read_and_parse::test("testdata/recursive.tla", "Multiple Recursive Declarations");
}

#[test]
fn mixed_recursive_declarations() {
    read_and_parse::test("testdata/recursive.tla", "Mixed Recursive Declarations");
}
