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

run_test "cargo test -p lib --test exclude_sany_number"

run_test "cargo test -p lib --test expressions -- --exact tuple_literal"
run_test "cargo test -p lib --test expressions nested"

run_test "cargo test -p lib --test if_then_else ite_with_string"

run_test "cargo test -p lib --test modules"
run_test "cargo test -p lib --test number"
run_test "cargo test -p lib --test string"

run_test "cargo test -p lib --test unit variable_declaration"
