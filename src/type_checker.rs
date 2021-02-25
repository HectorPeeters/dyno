use crate::ast::AstNode;
pub use crate::ast_visitor::AstVisitor;
use crate::error::*;

pub struct TypeChecker {}

impl TypeChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl AstVisitor for TypeChecker {
    fn visit_expression(&self, expression: &AstNode) -> DynoResult<()> {
        expression.get_type().map(|_| ())
    }
}
