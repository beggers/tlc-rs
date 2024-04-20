mod ast;
mod parser;

use crate::parser::parse_file;

fn main() {
  let filename = "testdata/string_test_module.tla";
  let _ = parse_file(filename);
}
