// This file was generated by scripts/gen_parser_tests.py
// Do not edit this file directly.

mod read_and_parse;

#[test]
fn basic_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Basic Unicode Conjlist",
    );
}

#[test]
fn inline_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Inline Unicode Conjlist",
    );
}

#[test]
fn startofline_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "StartofLine Unicode Conjlist",
    );
}

#[test]
fn leftshifted_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "LeftShifted Unicode Conjlist",
    );
}

#[test]
fn rightshifted_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "RightShifted Unicode Conjlist",
    );
}

#[test]
fn separated_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Separated Unicode Conjlist",
    );
}

#[test]
fn multiline_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Multiline Unicode Conjlist",
    );
}

#[test]
fn nested_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Nested Unicode Conjlist",
    );
}

#[test]
fn startofline_nested_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "StartofLine Nested Unicode Conjlist",
    );
}

#[test]
fn infix_op_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Infix Op Unicode Conjlist",
    );
}

#[test]
fn division_infix_op_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Division Infix Op Unicode Conjlist",
    );
}

#[test]
fn infix_op_terminated_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Infix Op Terminated Unicode Conjlist",
    );
}

#[test]
fn division_infix_op_terminated_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Division Infix Op Terminated Unicode Conjlist",
    );
}

#[test]
fn not_a_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Not a Unicode Conjlist",
    );
}

#[test]
fn unicode_conjlist_with_parentheses() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Unicode Conjlist with Parentheses",
    );
}

#[test]
fn unicode_conjlist_terminated_by_parentheses() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Unicode Conjlist Terminated by Parentheses",
    );
}

#[test]
fn nested_unicode_conjlist_terminated_by_parentheses() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Nested Unicode Conjlist Terminated by Parentheses",
    );
}

#[test]
fn doublenested_unicode_conjlist_terminated_by_parentheses() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "DoubleNested Unicode Conjlist Terminated by Parentheses",
    );
}

#[test]
fn moduleendterminated_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "ModuleEndTerminated Unicode Conjlist",
    );
}

#[test]
fn keywordunitterminated_unicode_conjlist() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "KeywordUnitTerminated Unicode Conjlist",
    );
}

#[test]
fn unicode_conjlist_with_empty_tuple() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Unicode Conjlist with Empty Tuple",
    );
}

#[test]
fn unicode_conjlist_with_empty_set() {
    read_and_parse::test(
        "testdata/unicode/conjlist-unicode.tla",
        "Unicode Conjlist with Empty Set",
    );
}
