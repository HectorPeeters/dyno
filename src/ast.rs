use crate::error::*;
use crate::lexer::TokenType;

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
    IntegerLiteral(u128),
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
