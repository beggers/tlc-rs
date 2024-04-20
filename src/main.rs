mod ast;
mod parser;

use crate::parser::parse;

fn main() {
  let filename = "testdata/string_test_module.tla";
  let _ = parse(filename);
}
