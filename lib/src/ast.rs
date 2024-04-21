// These are one-to-one with the non-silenced rules in grammar.pest.
//
// Enums seem best for the AST: structs are wasteful, since we often only
// store one of many many options. Unions are tough because they don't give us
// Debug and PartialEq for free. Enums give us both and they're easy to
// pattern-match on.
// Rust uses a mix of structs and enums. So as time goes on we may add structs.
// https://github.com/rust-lang/rust/blob/dbce3b43b6cb34dd3ba12c3ec6f708fe68e9c3df/compiler/rustc_ast/src/ast.rs
//
// TODO should we use Boxes? Feels like we should use Boxes.

// ===================
// Literals
// ===================
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

#[derive(Clone, Debug, PartialEq)]
pub enum StringLit {
    StringLit { value: String },
}

// ===================
// Other things (TODO sort)
// ===================

#[derive(Clone, Debug, PartialEq)]
pub enum Ident {
    Ident { value: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    StringLit { string_lit: StringLit },
    NumberLit { number_lit: NumberLit },
    NumberSetLit { number_set_lit: NumberSetLit },
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpDefn {
    SingleExprOpDefn { ident: Ident, expr: Expr },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mod {
    EmptyMod,
    SingleOpDefnMod { ident: Ident, op_defn: OpDefn },
}

#[derive(Clone, Debug, PartialEq)]
pub enum SourceFile {
    SourceFile { mods: Vec<Mod> },
}
