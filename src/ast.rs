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
    fn get_type(&self) -> DynoType {
        match self {
            AstNode::BinaryOperation(t, left, right) => {
                //TODO: this needs to be changed to proper type handling
                left.get_type()
            }
            AstNode::IntegerLiteral(_, size) => DynoType::UnsignedInt(*size),
            _ => panic!("Trying to get type of unsupported AST node"),
        }
    }
}
