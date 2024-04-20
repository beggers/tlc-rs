use crate::parser::parse_file;

mod ast;
mod parser;

pub fn parse_test_file() {
  let filename = "string_test_module.tla";
  let _ = parse_file(filename);
}