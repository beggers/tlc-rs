use crate::ast::{Expr, Ident, Mod, NumberLit, OpDefn, SourceFile, StringLit};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TLAParser;

// Force recompile when the grammar changes.
const _GRAMMAR: &str = include_str!("../grammar.pest");

pub fn parse_file(filename: &str) -> Result<SourceFile, pest::error::Error<Rule>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    parse_string(&contents)
}

// Public for testing.
pub fn parse_string(input: &str) -> Result<SourceFile, pest::error::Error<Rule>> {
    let parsed = TLAParser::parse(Rule::source_file, input)?.next().unwrap();
    // TODO this correctly grabs the first source file, but it should fail
    // if there's extra crap afterwards.
    let ast = parse_source_file(parsed);
    Ok(ast)
}

// These functions follow the same ordering as the rules in grammar.pest.
// TODO these should be Result<...>.

// ===================
// Literals
// ===================

fn parse_string_lit(pair: Pair<Rule>) -> StringLit {
    StringLit::StringLit {
        value: pair.as_str().to_string(),
    }
}

fn parse_number_lit(pair: Pair<Rule>) -> NumberLit {
    let mut inner_pairs = pair.into_inner();
    let inner_pair = inner_pairs.next().unwrap();
    match inner_pair.as_rule() {
        Rule::nat_lit => {
            let value = inner_pair.as_str().parse::<i64>().unwrap();
            NumberLit::IntLit { value: value }
        }
        Rule::binary_lit => {
            let value = inner_pair.as_str().parse::<i64>().unwrap();
            NumberLit::IntLit { value: value }
        }
        Rule::octal_lit => {
            let value = inner_pair.as_str().parse::<i64>().unwrap();
            NumberLit::IntLit { value: value }
        }
        Rule::hex_lit => {
            let value = inner_pair.as_str().parse::<i64>().unwrap();
            NumberLit::IntLit { value: value }
        }
        Rule::real_lit => {
            let value = inner_pair.as_str().parse::<f64>().unwrap();
            NumberLit::RealLit { value: value }
        }
        _ => panic!("Unexpected rule"),
    }
}

// ===================
// Other things (TODO sort)
// ===================

fn parse_source_file(pair: Pair<Rule>) -> SourceFile {
    let mut mods = Vec::new();
    for inner_pair in pair.into_inner() {
        mods.push(parse_mod(inner_pair));
    }
    SourceFile::SourceFile { mods: mods }
}

fn parse_mod(pair: Pair<Rule>) -> Mod {
    let mut inner_pairs = pair.into_inner();
    let ident = parse_ident(inner_pairs.next().unwrap());
    let op_defn = parse_op_defn(inner_pairs.next().unwrap());
    Mod::SingleOpDefnMod {
        ident: ident,
        op_defn: op_defn,
    }
}

fn parse_ident(pair: Pair<Rule>) -> Ident {
    Ident::Ident {
        value: pair.as_str().to_string(),
    }
}

fn parse_op_defn(pair: Pair<Rule>) -> OpDefn {
    let mut inner_pairs = pair.into_inner();
    let ident = parse_ident(inner_pairs.next().unwrap());
    let expr = parse_expr(inner_pairs.next().unwrap());
    OpDefn::SingleExprOpDefn {
        ident: ident,
        expr: expr,
    }
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    let mut inner_pairs = pair.into_inner();
    let inner_pair = inner_pairs.next().unwrap();
    match inner_pair.as_rule() {
        Rule::string_lit => {
            let string_lit = parse_string_lit(inner_pair);
            Expr::StringLit {
                string_lit: string_lit,
            }
        }
        Rule::number_lit => {
            let number_lit = parse_number_lit(inner_pair);
            Expr::NumberLit {
                number_lit: number_lit,
            }
        }
        _ => panic!("Unexpected rule in parse_expr: {:?}", inner_pair.as_rule()),
    }
}
