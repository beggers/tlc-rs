# I want each test case in testdata/ to be its own test case in the test suite.
# This provides several benefits:
# 1. Tests can be run in parallel, which is faster.
# 2. Tests can be run in isolation, which is more reliable.
# 3. We can track overall parser completeness by number of tests passed.
# 4. We can set up incremental CI by only enabling those tests we expect to pass.
#
# This script generates the tests.
