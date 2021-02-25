use crate::ast::AstNode;
use crate::error::*;

pub trait AstVisitor {
    fn visit_assignment(&self, symbol: &String, expression: &AstNode) -> DynoResult<()>;

    fn visit(&self, ast: &AstNode) -> DynoResult<()> {
        use AstNode::*;
        match ast {
            Assignment(symbol, expression) => self.visit_assignment(symbol, expression),
            Block(nodes) => nodes.iter().map(|x| self.visit(x)).collect(),
            _ => Err(DynoError::VisitError(format!(
                "Unexpected ast node: {:#?}",
                ast
            ))),
        }
    }
}
