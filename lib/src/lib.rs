use crate::parser::parse_file;

pub mod ast;
pub mod parser;

pub fn parse_test_file() {
  println!("Parsing test file");
  let filename = "string_test_module.tla";
  let ast = parse_file(filename);
  println!("{:?}", ast);
  println!("Done parsing test file");
}