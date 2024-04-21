# Put tests here as they pass. This helps ensure we don't regress.
# As we add more files, please keep them alphabetical so it's easy to see
# where the gaps are.
#
# When all test cases for a file pass, collapse them into one line!
FAILED_TESTS=()

function run_test {
    $1
    if [ $? -ne 0 ]; then
        FAILED_TESTS+=( "$1" )
    fi
}

function print_failed_tests {
    if [ ${#FAILED_TESTS[@]} -ne 0 ]; then
        echo "===================================="
        echo "===================================="
        echo "Failed tests:"
        for test in "${FAILED_TESTS[@]}"; do
            echo "-  $test"
        done
        echo "===================================="
        echo "===================================="
    fi
}

trap print_failed_tests EXIT

run_test "cargo test -p lib --test modules single_module"
run_test "cargo test -p lib --test modules nested_modules"
run_test "cargo test -p lib --test modules multiple_nested_modules"

run_test "cargo test -p lib --test number"

run_test "cargo test -p lib --test string basic_string"
run_test "cargo test -p lib --test string escape_quote_string"
run_test "cargo test -p lib --test string string_with_block_comment_start"

