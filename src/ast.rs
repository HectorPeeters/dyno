use crate::error::*;
use crate::lexer::TokenType;
use crate::types::DynoType;

#[derive(Debug, PartialEq)]
pub enum BinaryOperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    BinaryOperation(BinaryOperationType, Box<AstNode>, Box<AstNode>),
    IntegerLiteral(u128, u8),
    Block(Vec<AstNode>),
}

impl BinaryOperationType {
    pub fn from_token_type(token_type: TokenType) -> DynoResult<Self> {
        let operation = match token_type {
            TokenType::Plus => Self::Add,
            TokenType::Minus => Self::Subtract,
            TokenType::Asterix => Self::Multiply,
            TokenType::Slash => Self::Divide,
            _ => {
                return Err(DynoError::UnexpectedTokenError(
                    token_type,
                    vec![
                        TokenType::Plus,
                        TokenType::Minus,
                        TokenType::Asterix,
                        TokenType::Slash,
                    ],
                ))
            }
        };

        Ok(operation)
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            Self::Add => 1,
            Self::Subtract => 1,
            Self::Multiply => 2,
            Self::Divide => 2,
        }
    }
}

impl AstNode {
    fn get_type(&self) -> DynoResult<DynoType> {
        match self {
            AstNode::BinaryOperation(_, left, right) => {
                let left_type = left.get_type()?;
                let right_type = right.get_type()?;
                match (left_type, right_type) {
                    (DynoType::UnsignedInt(l_size), DynoType::UnsignedInt(r_size)) => {
                        Ok(DynoType::UnsignedInt(std::cmp::max(l_size, r_size)))
                    }
                    (_, _) => Err(DynoError::IncompatibleTypeError(left_type, right_type)),
                }
            }
            AstNode::IntegerLiteral(_, size) => Ok(DynoType::UnsignedInt(*size)),
            _ => Ok(DynoType::Void()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOperationType::*;

    #[test]
    fn test_bin_op_size() {
        let ast = AstNode::BinaryOperation(
            Add,
            Box::new(AstNode::IntegerLiteral(12, 4)),
            Box::new(AstNode::BinaryOperation(
                Multiply,
                Box::new(AstNode::IntegerLiteral(4, 3)),
                Box::new(AstNode::IntegerLiteral(7, 3)),
            )),
        );

        assert_eq!(ast.get_type(), Ok(DynoType::UnsignedInt(4)));
    }
}
