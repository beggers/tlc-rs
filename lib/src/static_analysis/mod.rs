use crate::ast;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyzerError {
    #[error("Unimplemented")]
    Unimplemented,
    #[error("Undefined symbol: {0}")]
    UndefinedSymbol(String),
}

#[derive(Debug)]
pub struct Analyzer {
    ast: ast::Ast,
}

impl Analyzer {
    pub fn new(ast: ast::Ast) -> Analyzer {
        Analyzer { ast }
    }

    pub fn analyze(&self) -> Result<(), AnalyzerError> {
        println!("{:?}", self.ast);
        Ok(())
    }
}
