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

#[derive(Debug)]
pub struct Analyzer<'a> {
    ast: &'a Ast,
}

impl<'a> Analyzer<'a> {
    pub fn new(ast: &'a Ast) -> Analyzer {
        Analyzer { ast }
    }

    pub fn analyze(&self) -> Result<Model, AnalyzerError> {
        println!("{:?}", self.ast);
        Ok(Model::new(self.ast))
    }
}
