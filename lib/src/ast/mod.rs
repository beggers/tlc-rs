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
pub struct Ast<'a> {
    source_files: &'a Vec<SourceFile<'a>>,
}

impl<'a> Ast<'a> {
    pub fn new(source_files: &'a Vec<SourceFile<'a>>) -> Ast<'a> {
        Ast { source_files }
    }
}

impl<'a> ASTNode for Ast<'a> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        for source_file in self.source_files {
            visitor.visit_source_file(source_file);
        }
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

pub enum LiteralValueType {
    NumberValue,
    NumberSetValue,
    IntRangeValue,
    StringValue,
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
pub union LiteralValue<'a> {
    pub number_value: &'a NumberLit,
    pub number_set_value: &'a NumberSetLit,
    pub int_range_value: &'a IntRangeLiteral,
    pub string_value: &'a String,
}

pub struct Literal<'a> {
    pub value: LiteralValue<'a>,
    pub value_type: LiteralValueType,
}

impl std::fmt::Debug for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value_type {
            LiteralValueType::NumberValue => {
                write!(f, "{:?}", self.value.number_value)
            }
            LiteralValueType::NumberSetValue => {
                write!(f, "{:?}", self.value.number_set_value)
            }
            LiteralValueType::IntRangeValue => {
                write!(f, "{:?}", self.value.int_range_value)
            }
            LiteralValueType::StringValue => {
                write!(f, "{:?}", self.value.string_value)
            }
        }
    }
}

impl ASTNode for Literal<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_literal(self);
    }
}

// ===================
// Structured literals
// ===================

#[derive(Debug)]
pub struct SeqLit<'a> {
    pub exprs: Vec<Expr<'a>>,
}

impl ASTNode for SeqLit<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_seq_lit(self);
    }
}

// ===================
// Expressions
// ===================

pub struct Expr<'a> {
    pub value: &'a ExprValue<'a>,
    pub value_type: ExprValueType,
}

// TODO refactor things to be sensible (i.e. have an InfixOp type etc).
pub enum ExprValueType {
    Literal,
    Ident,
    SeqLit,
    IfThenElse,
    SetMembership,
    Equals,
    NotEquals,
    Plus,
    InfixConjunct,
    Always,
    Stutter,
    Implication,
}

pub union ExprValue<'a> {
    pub literal_value: &'a Literal<'a>,
    pub ident_value: &'a Ident,
    pub seqlit_value: &'a SeqLit<'a>,
    pub ifthenelse_value: &'a IfThenElse<'a>,
    pub set_membership_value: &'a SetMembership<'a>,
    pub eq_value: &'a Equals<'a>,
    pub neq_value: &'a NotEquals<'a>,
    pub plus_value: &'a Plus<'a>,
    pub infixconjunct_value: &'a InfixConjunct<'a>,
    pub always_value: &'a Always<'a>,
    pub stutter_value: &'a Stutter<'a>,
    pub implication_value: &'a Implication<'a>,
}

impl std::fmt::Debug for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value_type {
            ExprValueType::Literal => {
                write!(f, "{:?}", self.value.literal_value)
            }
            ExprValueType::Ident => {
                write!(f, "{:?}", self.value.ident_value)
            }
            ExprValueType::SeqLit => {
                write!(f, "{:?}", self.value.seqlit_value)
            }
            ExprValueType::IfThenElse => {
                write!(f, "{:?}", self.value.ifthenelse_value)
            }
            ExprValueType::SetMembership => {
                write!(f, "{:?}", self.value.set_membership_value)
            }
            ExprValueType::Equals => {
                write!(f, "{:?}", self.value.eq_value)
            }
            ExprValueType::NotEquals => {
                write!(f, "{:?}", self.value.neq_value)
            }
            ExprValueType::Plus => {
                write!(f, "{:?}", self.value.plus_value)
            }
            ExprValueType::InfixConjunct => {
                write!(f, "{:?}", self.value.infixconjunct_value)
            }
            ExprValueType::Always => {
                write!(f, "{:?}", self.value.always_value)
            }
            ExprValueType::Stutter => {
                write!(f, "{:?}", self.value.stutter_value)
            }
            ExprValueType::Implication => {
                write!(f, "{:?}", self.value.implication_value)
            }
        }
    }
}

impl ASTNode for Expr<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_expr(self);
    }
}

#[derive(Debug)]
pub struct IfThenElse<'a> {
    pub cond: &'a Expr<'a>,
    pub then_expr: &'a Expr<'a>,
    pub else_expr: &'a Expr<'a>,
}

impl ASTNode for IfThenElse<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_ifthenelse(self);
    }
}

#[derive(Debug)]
pub struct OpDefn<'a> {
    pub ident: &'a Ident,
    pub expr: &'a Expr<'a>,
}

impl ASTNode for OpDefn<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_op_defn(self);
    }
}

// TODO unify these into a single InfixOperator enum.
#[derive(Debug)]
pub struct SetMembership<'a> {
    pub ident: &'a Ident,
    pub set_expr: &'a Expr<'a>,
}

impl ASTNode for SetMembership<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_set_membership(self);
    }
}

#[derive(Debug)]
pub struct Equals<'a> {
    pub left: &'a Ident,
    pub right: &'a Expr<'a>,
}

impl ASTNode for Equals<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_equals(self);
    }
}

#[derive(Debug)]
pub struct NotEquals<'a> {
    pub left: &'a Ident,
    pub right: &'a Expr<'a>,
}

impl ASTNode for NotEquals<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_not_equals(self);
    }
}

#[derive(Debug)]
pub struct Plus<'a> {
    pub left: &'a Ident,
    pub right: &'a Expr<'a>,
}

impl ASTNode for Plus<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_plus(self);
    }
}

#[derive(Debug)]
pub struct InfixConjunct<'a> {
    pub left: &'a Ident,
    pub right: &'a Expr<'a>,
}

impl ASTNode for InfixConjunct<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_infixconjunct(self);
    }
}

#[derive(Debug)]
pub struct Always<'a> {
    pub expr: &'a Expr<'a>,
}

impl ASTNode for Always<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_always(self);
    }
}

#[derive(Debug)]
pub struct Stutter<'a> {
    pub expr: &'a Expr<'a>,
}

impl ASTNode for Stutter<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_stutter(self);
    }
}

#[derive(Debug)]
pub struct Implication<'a> {
    pub left: &'a Ident,
    pub right: &'a Expr<'a>,
}

impl ASTNode for Implication<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_implication(self);
    }
}

// ===================
// Files and Modules
// ===================

#[derive(Debug)]
pub struct SourceFile<'a> {
    pub tla_mods: &'a Vec<TLAMod<'a>>,
}

pub union TLAModItemValue<'a> {
    pub op_defn: &'a OpDefn<'a>,
    pub tla_mod: &'a TLAMod<'a>,
    pub constant_list: &'a ConstantList<'a>,
    pub extends_list: &'a ExtendsList<'a>,
    pub variable_list: &'a VariableList<'a>,
}

pub enum TLAModItemValueType {
    OpDefn,
    TLAMod,
    ConstantList,
    ExtendsList,
    VariableList,
}

pub struct TLAModItem<'a> {
    pub value: TLAModItemValue<'a>,
    pub value_type: TLAModItemValueType,
}

pub struct TLAMod<'a> {
    pub ident: &'a Ident,
    pub items: &'a Vec<&'a TLAModItem<'a>>,
}

impl std::fmt::Debug for TLAMod<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ident);
        for item in self.items {
            match item.value_type {
                TLAModItemValueType::OpDefn => {
                    write!(f, "{:?}", item.value.op_defn)
                }
                TLAModItemValueType::TLAMod => {
                    write!(f, "{:?}", item.value.tla_mod)
                }
                TLAModItemValueType::ConstantList => {
                    write!(f, "{:?}", item.value.constant_list)
                }
                TLAModItemValueType::ExtendsList => {
                    write!(f, "{:?}", item.value.extends_list)
                }
                TLAModItemValueType::VariableList => {
                    write!(f, "{:?}", item.value.variable_list)
                }
            };
        }
        Ok(())
    }
}

impl ASTNode for TLAMod<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_tlamod(self);
    }
}

#[derive(Debug)]
pub struct ConstantList<'a> {
    pub idents: &'a Vec<Ident>,
}

impl ASTNode for ConstantList<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_constant_list(self);
    }
}

#[derive(Debug)]
pub struct ExtendsList<'a> {
    pub idents: &'a Vec<Ident>,
}

impl ASTNode for ExtendsList<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_extends_list(self);
    }
}

#[derive(Debug)]
pub struct VariableList<'a> {
    pub idents: &'a Vec<Ident>,
}

impl ASTNode for VariableList<'_> {
    fn accept_visitor(&self, visitor: &dyn ASTVisitor) {
        visitor.visit_variable_list(self);
    }
}
