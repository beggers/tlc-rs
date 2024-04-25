use crate::ast::{
    Always, Ast, ConstantList, Equals, Expr, ExtendsList, Ident, IfThenElse, Implication,
    InfixConjunct, LiteralValue, NotEquals, NumberLit, NumberSetLit, OpDefn, Plus, SeqLit,
    SetMembership, SourceFile, Stutter, TLAMod, TLAModItem, VariableList,
};

mod base;

use crate::parser::base::{Rule, TLAParser};
use pest::{iterators::Pair, Parser};
use std::fs;

pub fn parse_file(filename: &str) -> Result<Ast, pest::error::Error<Rule>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    parse_string(&contents)
}

// Public for testing.
pub fn parse_string(input: &str) -> Result<Ast, pest::error::Error<Rule>> {
    let parsed = TLAParser::parse(Rule::source_file, input)
        .unwrap()
        .next()
        .unwrap();
    // TODO this correctly grabs the first source file, but it should fail
    // if there's extra crap afterwards.
    let sf = parse_source_file(parsed);
    Ok(Ast::new(&vec![sf]))
}

// These functions follow the same ordering as the rules in grammar.pest.
// TODO these should be Result<...>.

// ===================
// Base values
// ===================

fn parse_ident(pair: Pair<Rule>) -> Ident {
    Ident {
        value: pair.as_str().to_string(),
    }
}

fn parse_literal_value(pair: Pair<Rule>) -> LiteralValue {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
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
        Rule::int_range_lit => {
            let mut inner_pairs = inner_pair.into_inner();
            let start = parse_number_lit(inner_pairs.next().unwrap());
            let end = parse_number_lit(inner_pairs.next().unwrap());
            match (start, end) {
                (NumberLit::IntLit { value: start }, NumberLit::IntLit { value: end }) => {
                    LiteralValue::IntRangeLit {
                        start: start,
                        end: end,
                    }
                }
                _ => panic!("IntRangeLit must have two IntLit values"),
            }
        }
        Rule::string_set_lit => LiteralValue::StringSetLit,
        Rule::string_lit => {
            let string_lit = parse_string_lit(inner_pair);
            LiteralValue::StringLit { value: string_lit }
        }
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
// Expressions
// ===================

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
            Expr::Ident { value: ident }
        }
        Rule::if_then_else => {
            let if_then_else = parse_if_then_else(inner_pair);
            Expr::IfThenElse {
                value: Box::new(if_then_else),
            }
        }
        Rule::expr => {
            // Parens
            return parse_expr(inner_pair);
        }
        Rule::seq_lit => {
            let seq_lit = parse_seq_lit(inner_pair);
            Expr::SeqLit { value: seq_lit }
        }
        Rule::set_membership => {
            let set_membership = parse_set_membership(inner_pair);
            Expr::SetMembership {
                value: Box::new(set_membership),
            }
        }
        Rule::equals => {
            let equals = parse_equals(inner_pair);
            Expr::Equals {
                value: Box::new(equals),
            }
        }
        Rule::not_equals => {
            let not_equals = parse_not_equals(inner_pair);
            Expr::NotEquals {
                value: Box::new(not_equals),
            }
        }
        Rule::plus => {
            let plus = parse_plus(inner_pair);
            Expr::Plus {
                value: Box::new(plus),
            }
        }
        Rule::infix_conjunct => {
            let infix_conjunct = parse_infix_conjunct(inner_pair);
            Expr::InfixConjunct {
                value: Box::new(infix_conjunct),
            }
        }
        Rule::always => {
            let box_op = parse_always(inner_pair);
            Expr::Always {
                value: Box::new(box_op),
            }
        }
        Rule::stutter => {
            let stutter = parse_stutter(inner_pair);
            Expr::Stutter {
                value: Box::new(stutter),
            }
        }
        Rule::implication => {
            let implication = parse_implication(inner_pair);
            Expr::Implication {
                value: Box::new(implication),
            }
        }
        _ => panic!("Unexpected rule in parse_expr: {:?}", inner_pair.as_rule()),
    }
}

fn parse_if_then_else(pair: Pair<Rule>) -> IfThenElse {
    let mut inner_pairs = pair.into_inner();
    let cond = parse_expr(inner_pairs.next().unwrap());
    let then_expr = parse_expr(inner_pairs.next().unwrap());
    let else_expr = parse_expr(inner_pairs.next().unwrap());
    IfThenElse::IfThenElse {
        cond: Box::new(cond),
        then_expr: Box::new(then_expr),
        else_expr: Box::new(else_expr),
    }
}

fn parse_set_membership(pair: Pair<Rule>) -> SetMembership {
    let mut inner_pairs = pair.into_inner();
    let ident = parse_ident(inner_pairs.next().unwrap());
    let set_expr = parse_expr(inner_pairs.next().unwrap());
    SetMembership::SetMembership {
        ident: ident,
        set_expr: set_expr,
    }
}

fn parse_equals(pair: Pair<Rule>) -> Equals {
    let mut inner_pairs = pair.into_inner();
    let left = parse_ident(inner_pairs.next().unwrap());
    let right = parse_expr(inner_pairs.next().unwrap());
    Equals::Equals {
        left: left,
        right: right,
    }
}

fn parse_not_equals(pair: Pair<Rule>) -> NotEquals {
    let mut inner_pairs = pair.into_inner();
    let left = parse_ident(inner_pairs.next().unwrap());
    let right = parse_expr(inner_pairs.next().unwrap());
    NotEquals::NotEquals {
        left: left,
        right: right,
    }
}

fn parse_plus(pair: Pair<Rule>) -> Plus {
    let mut inner_pairs = pair.into_inner();
    let left = parse_ident(inner_pairs.next().unwrap());
    let right = parse_expr(inner_pairs.next().unwrap());
    Plus::Plus {
        left: left,
        right: right,
    }
}

fn parse_infix_conjunct(pair: Pair<Rule>) -> InfixConjunct {
    let mut inner_pairs = pair.into_inner();
    let left = parse_ident(inner_pairs.next().unwrap());
    let right = parse_expr(inner_pairs.next().unwrap());
    InfixConjunct::InfixConjunct {
        left: left,
        right: right,
    }
}

fn parse_always(pair: Pair<Rule>) -> Always {
    let expr = parse_expr(pair.into_inner().next().unwrap());
    Always::Always { expr: expr }
}

fn parse_stutter(pair: Pair<Rule>) -> Stutter {
    let expr = parse_expr(pair.into_inner().next().unwrap());
    Stutter::Stutter { expr: expr }
}

fn parse_implication(pair: Pair<Rule>) -> Implication {
    let mut inner_pairs = pair.into_inner();
    let left = parse_ident(inner_pairs.next().unwrap());
    let right = parse_expr(inner_pairs.next().unwrap());
    Implication::Implication {
        left: left,
        right: right,
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
    // Iterate over everything in the module
    for inner_pair in inner_pairs {
        match inner_pair.as_rule() {
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
            Rule::constant_list => {
                items.push(TLAModItem::ConstantList {
                    constant_list: parse_constant_list(inner_pair),
                });
            }
            Rule::extends_list => {
                items.push(TLAModItem::ExtendsList {
                    extends_list: parse_extends_list(inner_pair),
                });
            }
            Rule::variable_list => {
                items.push(TLAModItem::VariableList {
                    variable_list: parse_variable_list(inner_pair),
                });
            }
            Rule::theorem => {
                // Nothing
            }
            _ => panic!(
                "Unexpected rule in parse_tla_mod: {:?}. {:?}",
                inner_pair.as_rule(),
                inner_pair.as_str()
            ),
        }
    }
    TLAMod {
        ident: ident,
        items: items,
    }
}

fn parse_constant_list(pair: Pair<Rule>) -> ConstantList {
    let mut idents = Vec::new();
    for inner_pair in pair.into_inner() {
        idents.push(parse_ident(inner_pair));
    }
    ConstantList { idents: idents }
}

fn parse_extends_list(pair: Pair<Rule>) -> ExtendsList {
    let mut idents = Vec::new();
    for inner_pair in pair.into_inner() {
        idents.push(parse_ident(inner_pair));
    }
    ExtendsList { idents: idents }
}

fn parse_variable_list(pair: Pair<Rule>) -> VariableList {
    let mut idents = Vec::new();
    for inner_pair in pair.into_inner() {
        idents.push(parse_ident(inner_pair));
    }
    VariableList { idents: idents }
}
