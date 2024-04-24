use crate::ast;

#[derive(Debug)]
pub struct Analyzer {
    ast: ast::Ast,
}

impl Analyzer {
    pub fn new(ast: ast::Ast) -> Analyzer {
        Analyzer { ast }
    }

    pub fn analyze(&self) {
        println!("{:?}", self.ast);
    }
}
