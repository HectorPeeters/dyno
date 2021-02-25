use crate::error::*;
use crate::lexer::TokenType;
use crate::types::DynoType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    BinaryOperation(BinaryOperationType, Box<AstNode>, Box<AstNode>),
    IntegerLiteral(u128, u8),
    Declaration(String, DynoType),
    Assignment(String, Box<AstNode>),
    Return(Box<AstNode>),
    Block(Vec<AstNode>),
}

impl BinaryOperationType {
    pub fn from_token_type(token_type: TokenType) -> DynoResult<Self> {
        let operation = match token_type {
            TokenType::Plus => Self::Add,
            TokenType::Minus => Self::Subtract,
            TokenType::Asterix => Self::Multiply,
            TokenType::Slash => Self::Divide,
            TokenType::DoubleEqual => Self::Equal,
            TokenType::NotEqual => Self::NotEqual,
            TokenType::LessThan => Self::LessThan,
            TokenType::LessThanEqual => Self::LessThanEqual,
            TokenType::GreaterThan => Self::GreaterThan,
            TokenType::GreaterThanEqual => Self::GreaterThanEqual,
            _ => {
                return Err(DynoError::UnexpectedTokenError(
                    token_type,
                    vec![
                        TokenType::Plus,
                        TokenType::Minus,
                        TokenType::Asterix,
                        TokenType::Slash,
                        TokenType::DoubleEqual,
                        TokenType::NotEqual,
                        TokenType::LessThan,
                        TokenType::LessThanEqual,
                        TokenType::GreaterThan,
                        TokenType::GreaterThanEqual,
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
            Self::Equal => 3,
            Self::NotEqual => 3,
            Self::LessThan => 3,
            Self::LessThanEqual => 3,
            Self::GreaterThan => 3,
            Self::GreaterThanEqual => 3,
        }
    }
}

impl AstNode {
    pub fn get_type(&self) -> DynoResult<DynoType> {
        match self {
            AstNode::BinaryOperation(op, left, right) => {
                use BinaryOperationType::*;

                let left_type = left.get_type()?;
                let right_type = right.get_type()?;
                match op {
                    Equal | NotEqual | LessThan | LessThanEqual | GreaterThan
                    | GreaterThanEqual => match (left_type, right_type) {
                        (DynoType::UnsignedInt(_), DynoType::UnsignedInt(_)) => {
                            Ok(DynoType::Bool())
                        }

                        (_, _) => Err(DynoError::IncompatibleTypeError(left_type, right_type)),
                    },
                    _ => match (left_type, right_type) {
                        (DynoType::UnsignedInt(l_size), DynoType::UnsignedInt(r_size)) => {
                            Ok(DynoType::UnsignedInt(std::cmp::max(l_size, r_size)))
                        }
                        (_, _) => Err(DynoError::IncompatibleTypeError(left_type, right_type)),
                    },
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
    fn test_precendence() {
        assert!(
            BinaryOperationType::Multiply.get_precedence()
                > BinaryOperationType::Add.get_precedence()
        );
        assert!(
            BinaryOperationType::Divide.get_precedence()
                > BinaryOperationType::Subtract.get_precedence()
        );
    }

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
