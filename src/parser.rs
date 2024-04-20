use crate::ast::*;

use pest::{
  iterators::{
    Pair,
  },
  Parser
};
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TLAParser;

// Force recompile when the grammar changes.
const _GRAMMAR : &str = include_str!("../grammar.pest");

pub fn parse_file(filename: &str) -> Result<(), pest::error::Error<Rule>> {
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  parse_string(&contents)
}

// Public for testing.
pub fn parse_string(input: &str) -> Result<(), pest::error::Error<Rule>> {
  let parsed = TLAParser::parse(Rule::source_file, input)?.next().unwrap();
  let ast = parse_source_file(parsed);
  println!("{:?}", ast);
  Ok(())
}

fn parse_source_file(pair: Pair<Rule>) -> SourceFile {
  let mut mods = Vec::new();
  for inner_pair in pair.into_inner() {
    mods.push(parse_mod(inner_pair));
  }
  SourceFile{
    mods: mods
  }
}

fn parse_mod(pair: Pair<Rule>) -> Mod {
  let mut inner_pairs = pair.into_inner();
  let ident = parse_ident(inner_pairs.next().unwrap());
  let op_defn = parse_op_defn(inner_pairs.next().unwrap());
  Mod{
    ident: ident,
    op_defn: op_defn
  }
}

fn parse_ident(pair: Pair<Rule>) -> Ident {
  Ident{
    value: pair.as_str().to_string()
  }
}

fn parse_op_defn(pair: Pair<Rule>) -> OpDefn {
  let mut inner_pairs = pair.into_inner();
  let ident = parse_ident(inner_pairs.next().unwrap());
  let expr = parse_expr(inner_pairs.next().unwrap());
  OpDefn{
    ident: ident,
    expr: expr
  }
}

fn parse_string_lit(pair: Pair<Rule>) -> StringLit {
  StringLit{
    value: pair.as_str().to_string()
  }
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
  let string_lit = parse_string_lit(pair);
  Expr{
    string_lit: string_lit
  }
}