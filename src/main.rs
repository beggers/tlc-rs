use pest_derive::Parser;
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TLAParser;

fn main() {
    let contents = fs::read_to_string("testdata/string_test_module.tla").expect("Something went wrong reading the file");
    let parsed = TLAParser::parse(Rule::source_file, &contents).unwrap();
    println!("{:?}", parsed);
}
