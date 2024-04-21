// These are one-to-one with the non-silenced rules in grammar.pest.
//
// Enums seem best for the AST: structs are wasteful, since we often only
// store one of many many options. Unions are tough because they don't give us
// Debug and PartialEq for free. Enums give us both and they're easy to
// pattern-match on.
// Rust uses a mix of structs and enums. So as time goes on we may add structs.
// https://github.com/rust-lang/rust/blob/dbce3b43b6cb34dd3ba12c3ec6f708fe68e9c3df/compiler/rustc_ast/src/ast.rs

// ===================
// Base values
// ===================
#[derive(Clone, Debug, PartialEq)]
pub enum Ident {
    Ident { value: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    StringLit { value: String },
    NumberLit { value: NumberLit },
    NumberSetLit { value: NumberSetLit },
    StringSetLit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberLit {
    IntLit { value: i64 },
    RealLit { value: f64 },
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberSetLit {
    NatSetLit,
    IntSetLit,
    RealSetLit,
}

// ===================
// Structured literals
// ===================

#[derive(Clone, Debug, PartialEq)]
pub enum SeqLit {
    SeqLit { exprs: Vec<Expr> },
}

// ===================
// Expressions
// ===================

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    LiteralValue { value: LiteralValue },
    Ident { value: Ident },
    SeqLit { value: SeqLit },
    IfThenElse { value: Box<IfThenElse> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum IfThenElse {
    IfThenElse {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpDefn {
    SingleExprOpDefn { ident: Ident, expr: Expr },
}

// ===================
// Files and Modules
// ===================

// Things which can go inside of a module. A module contains zero or more
// of these.
#[derive(Clone, Debug, PartialEq)]
pub enum SourceFile {
    SourceFile { tla_mods: Vec<TLAMod> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum TLAModItem {
    OpDefn { op_defn: OpDefn },
    TLAMod { tla_mod: TLAMod },
    ExtendsList { extends_list: ExtendsList },
    VariableList { variable_list: VariableList },
}

#[derive(Clone, Debug, PartialEq)]
pub struct TLAMod {
    pub ident: Ident,
    pub items: Vec<TLAModItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExtendsList {
    pub idents: Vec<Ident>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableList {
    pub idents: Vec<Ident>,
}
