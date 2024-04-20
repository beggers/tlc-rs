// These are one-to-one with the non-silenced rules in grammar.pest.
#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
  pub value: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringLit {
  pub value: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
  pub string_lit: StringLit
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpDefn {
  pub ident: Ident,
  pub expr: Expr
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mod {
  pub ident: Ident,
  pub op_defn: OpDefn
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceFile {
  pub mods: Vec<Mod>
}