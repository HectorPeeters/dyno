use crate::ast::AstNode;
use crate::error::*;

pub trait AstVisitor {
    fn visit_expression(&self, expression: &AstNode) -> DynoResult<()>;

    fn visit(&self, ast: &AstNode) -> DynoResult<()> {
        use AstNode::*;
        match ast {
            Assignment(_, expression) => self.visit_expression(expression),
            Return(expression) => self.visit_expression(expression),
            Block(nodes) => nodes.iter().map(|x| self.visit(x)).collect(),
            _ => Err(DynoError::VisitError(format!(
                "Unexpected ast node: {:#?}",
                ast
            ))),
        }
    }
}
