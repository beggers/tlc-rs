# Checks the percentage of test cases passing.

import re
import subprocess

TEST_RESULT_REGEX = r"^test result:.*(\d+) passed; (\d+) failed;.*"


def main():
    result = None
    try:
        result = subprocess.run(
            [
                "cargo test --all --no-fail-fast"
            ],
            shell=True,
            capture_output=True,
        )
    except subprocess.CalledProcessError:
        pass

    if not result:
        print("Error running tests -- no result")
        return

    output = result.stdout.decode("utf-8")

    # Parse the output to get the number of tests passed
    # and the number of tests failed
    passed = 0
    failed = 0
    for line in output.split("\n"):
        if not line.startswith("test result:"):
            continue

        # Use the regex
        match = re.match(TEST_RESULT_REGEX, line)
        if not match:
            print("Error parsing test results")
            return
        passed += int(match.group(1))
        failed += int(match.group(2))

    print(f"Tests passed: {passed}/{passed + failed}")


if __name__ == "__main__":
    main()
