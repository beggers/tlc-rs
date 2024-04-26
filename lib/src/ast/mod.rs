// These are one-to-one with the non-silenced rules in grammar.pest.
//
// Rust AST for reference:
// https://github.com/rust-lang/rust/blob/dbce3b43b6cb34dd3ba12c3ec6f708fe68e9c3df/compiler/rustc_ast/src/ast.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstError {
    #[error("Unimplemented")]
    Unimplemented,
}

#[derive(Debug)]
pub struct Ast {
    pub source_file: SourceFile,
}

impl Ast {
    pub fn new(source_file: SourceFile) -> Ast {
        Ast { source_file }
    }
}

// ===================
// Identifiers and literals
// ===================

#[derive(Debug)]
pub struct Ident {
    pub value: String,
}

impl Ident {
    pub fn new(value: String) -> Ident {
        Ident { value }
    }
}

#[derive(Debug)]
pub struct IntRangeLiteral {
    pub start: i64,
    pub end: i64,
}

#[derive(Debug)]
pub enum NumberLit {
    IntLit { value: i64 },
    RealLit { value: f64 },
}

#[derive(Debug)]
pub enum NumberSetLit {
    NatSetLit,
    IntSetLit,
    RealSetLit,
}

#[derive(Debug)]
pub enum Literal {
    NumberLiteral(NumberLit),
    NumberSetLiteral(NumberSetLit),
    IntRangeLiteral(IntRangeLiteral),
    StringLiteral(String),
}

// ===================
// Structured literals
// ===================

#[derive(Debug)]
pub struct SeqLit {
    pub exprs: Vec<Box<Expr>>,
}

// ===================
// Expressions
// ===================

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    SeqLit(SeqLit),
    IfThenElse(IfThenElse),
    SetMembership(SetMembership),
    // TODO refactor things to be sensible (i.e. have an InfixOp type etc).
    Eq(Equals),
    Neq(NotEquals),
    Plus(Plus),
    InfixConjunct(InfixConjunct),
    // TODO refactor things to be sensible (i.e. have an UnaryOp type etc).
    Always(Always),
    Stutter(Stutter),
    Implication(Implication),
}

#[derive(Debug)]
pub struct IfThenElse {
    pub cond: Box<Expr>,
    pub then_expr: Box<Expr>,
    pub else_expr: Box<Expr>,
}

#[derive(Debug)]
pub struct OpDefn {
    pub ident: Ident,
    pub expr: Box<Expr>,
}

// TODO unify these into a single InfixOperator enum.
#[derive(Debug)]
pub struct SetMembership {
    pub ident: Ident,
    pub set_expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Equals {
    pub left: Ident,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct NotEquals {
    pub left: Ident,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Plus {
    pub left: Ident,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct InfixConjunct {
    pub left: Ident,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Always {
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Stutter {
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Implication {
    pub left: Ident,
    pub right: Box<Expr>,
}

// ===================
// Files and Modules
// ===================

#[derive(Debug)]
pub struct SourceFile {
    pub tla_mods: Vec<TLAMod>,
}

#[derive(Debug)]
pub enum TLAModItem {
    OpDefn(OpDefn),
    TLAMod(Box<TLAMod>),
    ConstantList(ConstantList),
    ExtendsList(ExtendsList),
    VariableList(VariableList),
}

#[derive(Debug)]
pub struct TLAMod {
    pub ident: Ident,
    pub items: Vec<TLAModItem>,
}

#[derive(Debug)]
pub struct ConstantList {
    pub idents: Vec<Ident>,
}

#[derive(Debug)]
pub struct ExtendsList {
    pub idents: Vec<Ident>,
}

#[derive(Debug)]
pub struct VariableList {
    pub idents: Vec<Ident>,
}
