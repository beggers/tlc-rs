use crate::ast::{
    Expr, ExtendsList, Ident, LiteralValue, NumberLit, NumberSetLit, OpDefn, SeqLit, SourceFile,
    TLAMod, TLAModItem,
};

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
    let parsed = TLAParser::parse(Rule::source_file, input)
        .unwrap()
        .next()
        .unwrap();
    // TODO this correctly grabs the first source file, but it should fail
    // if there's extra crap afterwards.
    let ast = parse_source_file(parsed);
    Ok(ast)
}

// These functions follow the same ordering as the rules in grammar.pest.
// TODO these should be Result<...>.

// ===================
// Base values
// ===================

fn parse_ident(pair: Pair<Rule>) -> Ident {
    Ident::Ident {
        value: pair.as_str().to_string(),
    }
}

fn parse_literal_value(pair: Pair<Rule>) -> LiteralValue {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::string_lit => {
            let string_lit = parse_string_lit(inner_pair);
            LiteralValue::StringLit { value: string_lit }
        }
        Rule::number_lit => {
            let number_lit = parse_number_lit(inner_pair);
            LiteralValue::NumberLit { value: number_lit }
        }
        Rule::number_set_lit => {
            let number_set_lit = parse_number_set_lit(inner_pair);
            LiteralValue::NumberSetLit {
                value: number_set_lit,
            }
        }
        Rule::string_set_lit => LiteralValue::StringSetLit,
        _ => panic!(
            "Unexpected rule in parse_literal_value: {:?}",
            inner_pair.as_rule()
        ),
    }
}

fn parse_string_lit(pair: Pair<Rule>) -> String {
    pair.as_str().to_string()
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
        _ => panic!(
            "Unexpected rule in parse_number_lit: {:?}",
            inner_pair.as_rule()
        ),
    }
}

fn parse_number_set_lit(pair: Pair<Rule>) -> NumberSetLit {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::nat_numbers_lit => NumberSetLit::NatSetLit,
        Rule::int_numbers_lit => NumberSetLit::IntSetLit,
        Rule::real_numbers_lit => NumberSetLit::RealSetLit,
        _ => panic!(
            "Unexpected rule in parse_number_set_lit: {:?}",
            inner_pair.as_rule()
        ),
    }
}

// ===================
// Structured literals
// ===================

fn parse_seq_lit(pair: Pair<Rule>) -> SeqLit {
    let mut exprs = Vec::new();
    for inner_pair in pair.into_inner() {
        exprs.push(parse_expr(inner_pair));
    }
    SeqLit::SeqLit { exprs: exprs }
}

// ===================
// Files and Modules
// ===================

fn parse_source_file(pair: Pair<Rule>) -> SourceFile {
    let mut tla_mods = Vec::new();
    for inner_pair in pair.into_inner() {
        tla_mods.push(parse_tla_mod(inner_pair));
    }
    SourceFile::SourceFile { tla_mods: tla_mods }
}

fn parse_tla_mod(pair: Pair<Rule>) -> TLAMod {
    let mut inner_pairs = pair.into_inner();
    let ident = parse_ident(inner_pairs.next().unwrap());
    let mut items = Vec::new();
    let inner_pair = inner_pairs.next();
    match inner_pair {
        None => {}
        Some(inner_pair) => match inner_pair.as_rule() {
            Rule::op_defn => {
                items.push(TLAModItem::OpDefn {
                    op_defn: parse_op_defn(inner_pair),
                });
            }
            Rule::tla_mod => {
                items.push(TLAModItem::TLAMod {
                    tla_mod: parse_tla_mod(inner_pair),
                });
            }
            Rule::extends_list => {
                items.push(TLAModItem::ExtendsList {
                    extends_list: parse_extends_list(inner_pair),
                });
            }
            _ => panic!(
                "Unexpected rule in parse_tla_mod: {:?}",
                inner_pair.as_rule()
            ),
        },
    }
    TLAMod {
        ident: ident,
        items: items,
    }
}

fn parse_extends_list(pair: Pair<Rule>) -> ExtendsList {
    let mut idents = Vec::new();
    for inner_pair in pair.into_inner() {
        idents.push(parse_ident(inner_pair));
    }
    ExtendsList { idents: idents }
}

// ===================
// Other things (TODO sort)
// ===================

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
        Rule::literal_value => {
            let literal_value = parse_literal_value(inner_pair);
            Expr::LiteralValue {
                value: literal_value,
            }
        }
        Rule::ident => {
            let ident = parse_ident(inner_pair);
            Expr::Ident {
                value: ident
            }
        }
        Rule::seq_lit => {
            let seq_lit = parse_seq_lit(inner_pair);
            Expr::SeqLit { value: seq_lit }
        }
        _ => panic!("Unexpected rule in parse_expr: {:?}", inner_pair.as_rule()),
    }
}
