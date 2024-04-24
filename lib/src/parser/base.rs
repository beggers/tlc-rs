use pest_derive::Parser;

// Force recompile when the grammar changes.
const _GRAMMAR: &str = include_str!("../../grammar.pest");

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct TLAParser;
