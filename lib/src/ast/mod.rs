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

pub trait ASTVisitor {
    // TODO refactor the AST to suck less -- should just have
    // binaryOp, prefixOp, Expr etc types.
    fn visit_source_file(&self, source_file: &SourceFile);
    fn visit_ident(&self, ident: &Ident);
    fn visit_literal(&self, literal: &Literal);
    fn visit_seq_lit(&self, seq_lit: &SeqLit);
    fn visit_expr(&self, expr: &Expr);
    fn visit_ifthenelse(&self, ifthenelse: &IfThenElse);
    fn visit_op_defn(&self, op_defn: &OpDefn);
    fn visit_set_membership(&self, set_membership: &SetMembership);
    fn visit_equals(&self, equals: &Equals);
    fn visit_not_equals(&self, not_equals: &NotEquals);
    fn visit_plus(&self, plus: &Plus);
    fn visit_infixconjunct(&self, infixconjunct: &InfixConjunct);
    fn visit_always(&self, always: &Always);
    fn visit_stutter(&self, stutter: &Stutter);
    fn visit_implication(&self, implication: &Implication);
    fn visit_tlamod(&self, tlamod: &TLAMod);
    fn visit_constant_list(&self, constant_list: &ConstantList);
    fn visit_extends_list(&self, extends_list: &ExtendsList);
    fn visit_variable_list(&self, variable_list: &VariableList);
}

pub trait ASTNode {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor);
}

#[derive(Debug)]
pub struct Ast {
    source_file: SourceFile,
}

impl Ast {
    pub fn new(source_file: SourceFile) -> Ast {
        Ast { source_file }
    }
}

impl ASTNode for Ast {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_source_file(&self.source_file);
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

impl ASTNode for Ident {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_ident(self);
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

impl ASTNode for Literal {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_literal(self);
    }
}

// ===================
// Structured literals
// ===================

#[derive(Debug)]
pub struct SeqLit {
    pub exprs: Vec<Box<Expr>>,
}

impl ASTNode for SeqLit {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_seq_lit(self);
    }
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

impl ASTNode for Expr {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_expr(self);
    }
}

#[derive(Debug)]
pub struct IfThenElse {
    pub cond: Box<Expr>,
    pub then_expr: Box<Expr>,
    pub else_expr: Box<Expr>,
}

impl ASTNode for IfThenElse {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_ifthenelse(self);
    }
}

#[derive(Debug)]
pub struct OpDefn {
    pub ident: Ident,
    pub expr: Box<Expr>,
}

impl ASTNode for OpDefn {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_op_defn(self);
    }
}

// TODO unify these into a single InfixOperator enum.
#[derive(Debug)]
pub struct SetMembership {
    pub ident: Ident,
    pub set_expr: Box<Expr>,
}

impl ASTNode for SetMembership {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_set_membership(self);
    }
}

#[derive(Debug)]
pub struct Equals {
    pub left: Ident,
    pub right: Box<Expr>,
}

impl ASTNode for Equals {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_equals(self);
    }
}

#[derive(Debug)]
pub struct NotEquals {
    pub left: Ident,
    pub right: Box<Expr>,
}

impl ASTNode for NotEquals {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_not_equals(self);
    }
}

#[derive(Debug)]
pub struct Plus {
    pub left: Ident,
    pub right: Box<Expr>,
}

impl ASTNode for Plus {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_plus(self);
    }
}

#[derive(Debug)]
pub struct InfixConjunct {
    pub left: Ident,
    pub right: Box<Expr>,
}

impl ASTNode for InfixConjunct {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_infixconjunct(self);
    }
}

#[derive(Debug)]
pub struct Always {
    pub expr: Box<Expr>,
}

impl ASTNode for Always {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_always(self);
    }
}

#[derive(Debug)]
pub struct Stutter {
    pub expr: Box<Expr>,
}

impl ASTNode for Stutter {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_stutter(self);
    }
}

#[derive(Debug)]
pub struct Implication {
    pub left: Ident,
    pub right: Box<Expr>,
}

impl ASTNode for Implication {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_implication(self);
    }
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

impl ASTNode for TLAMod {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_tlamod(self);
    }
}

#[derive(Debug)]
pub struct ConstantList {
    pub idents: Vec<Ident>,
}

impl ASTNode for ConstantList {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_constant_list(self);
    }
}

#[derive(Debug)]
pub struct ExtendsList {
    pub idents: Vec<Ident>,
}

impl ASTNode for ExtendsList {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_extends_list(self);
    }
}

#[derive(Debug)]
pub struct VariableList {
    pub idents: Vec<Ident>,
}

impl ASTNode for VariableList {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_variable_list(self);
    }
}
