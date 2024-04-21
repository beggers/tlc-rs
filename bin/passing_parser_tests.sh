# Put tests here as they pass. This helps ensure we don't regress.
# As we add more files, please keep them alphabetical so it's easy to see
# where the gaps are.
#
# When all test cases for a file pass, collapse them into one line!

cargo test -p lib --test number all_number_sets

cargo test -p lib --test string basic_string
cargo test -p lib --test string escape_quote_string
cargo test -p lib --test string string_with_block_comment_start
