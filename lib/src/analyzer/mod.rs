use crate::ast::Ast;
use crate::model::Model;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyzerError {
    #[error("Unimplemented")]
    Unimplemented,
    #[error("Undefined symbol: {0}")]
    UndefinedSymbol(String),
}

// Analyzes the AST (making sure that all symbols are defined, etc) and produces
// a Model which can be executed.
#[derive(Debug)]
pub struct Analyzer<'a> {
    ast: &'a Ast,
}

impl<'a> Analyzer<'a> {
    pub fn new(ast: &'a Ast) -> Analyzer<'a> {
        Analyzer { ast }
    }

    pub fn analyze(&self) -> Result<Model, AnalyzerError> {
        println!("{:?}", self.ast);
        Ok(Model::new(self.ast))
    }
}
