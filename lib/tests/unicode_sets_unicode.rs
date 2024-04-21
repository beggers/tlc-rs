// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn set_map() {
    read_and_parse::test("testdata/unicode/sets-unicode.tla", "Set Map");
}

#[test]
fn set_map_with_multiple_generators() {
    read_and_parse::test(
        "testdata/unicode/sets-unicode.tla",
        "Set Map with Multiple Generators",
    );
}

#[test]
fn set_filter() {
    read_and_parse::test("testdata/unicode/sets-unicode.tla", "Set Filter");
}

#[test]
fn set_filter_with_tuple() {
    read_and_parse::test("testdata/unicode/sets-unicode.tla", "Set Filter with Tuple");
}

#[test]
fn set_filter_precedence() {
    read_and_parse::test("testdata/unicode/sets-unicode.tla", "Set Filter Precedence");
}

#[test]
fn set_filter_with_jlist() {
    read_and_parse::test("testdata/unicode/sets-unicode.tla", "Set Filter with Jlist");
}