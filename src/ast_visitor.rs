use crate::ast::AstNode;
use crate::error::*;

pub trait AstVisitor {
    fn visit_expression(&self, expression: &AstNode) -> DynoResult<()>;

    fn visit(&self, ast: &AstNode) -> DynoResult<()> {
        use AstNode::*;
        match ast {
            Assignment(_, expression) => self.visit_expression(expression),
            Return(expression) => self.visit_expression(expression),
            Block(nodes) => nodes.iter().try_for_each(|x| self.visit(x)),
            _ => Err(DynoError::VisitError(format!(
                "Unexpected ast node: {:#?}",
                ast
            ))),
        }
    }
}
