// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn basic_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "Basic Conjlist");
}

#[test]
fn inline_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "Inline Conjlist");
}

#[test]
fn startofline_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "StartofLine Conjlist");
}

#[test]
fn leftshifted_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "LeftShifted Conjlist");
}

#[test]
fn rightshifted_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "RightShifted Conjlist");
}

#[test]
fn separated_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "Separated Conjlist");
}

#[test]
fn multiline_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "Multiline Conjlist");
}

#[test]
fn nested_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "Nested Conjlist");
}

#[test]
fn startofline_nested_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "StartofLine Nested Conjlist");
}

#[test]
fn infix_op_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "Infix Op Conjlist");
}

#[test]
fn division_infix_op_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "Division Infix Op Conjlist");
}

#[test]
fn infix_op_terminated_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "Infix Op Terminated Conjlist");
}

#[test]
fn division_infix_op_terminated_conjlist() {
    read_and_parse::test(
        "testdata/conjlist.tla",
        "Division Infix Op Terminated Conjlist",
    );
}

#[test]
fn not_a_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "Not a Conjlist");
}

#[test]
fn conjlist_with_parentheses() {
    read_and_parse::test("testdata/conjlist.tla", "Conjlist with Parentheses");
}

#[test]
fn conjlist_terminated_by_parentheses() {
    read_and_parse::test(
        "testdata/conjlist.tla",
        "Conjlist Terminated by Parentheses",
    );
}

#[test]
fn nested_conjlist_terminated_by_parentheses() {
    read_and_parse::test(
        "testdata/conjlist.tla",
        "Nested Conjlist Terminated by Parentheses",
    );
}

#[test]
fn doublenested_conjlist_terminated_by_parentheses() {
    read_and_parse::test(
        "testdata/conjlist.tla",
        "DoubleNested Conjlist Terminated by Parentheses",
    );
}

#[test]
fn moduleendterminated_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "ModuleEndTerminated Conjlist");
}

#[test]
fn keywordunitterminated_conjlist() {
    read_and_parse::test("testdata/conjlist.tla", "KeywordUnitTerminated Conjlist");
}

#[test]
fn conjlist_with_empty_tuple() {
    read_and_parse::test("testdata/conjlist.tla", "Conjlist with Empty Tuple");
}

#[test]
fn conjlist_with_empty_set() {
    read_and_parse::test("testdata/conjlist.tla", "Conjlist with Empty Set");
}
