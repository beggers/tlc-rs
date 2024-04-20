mod read_and_parse;

#[test]
fn test_string() {
  let filename = "testdata/string.tla";
  let testname = "Basic String";
  read_and_parse::test(filename, testname);
}
