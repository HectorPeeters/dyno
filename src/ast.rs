use crate::error::*;
use crate::lexer::TokenType;
use crate::types::{DynoType, DynoValue};

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
pub enum Expression {
    BinaryOperation(BinaryOperationType, Box<Expression>, Box<Expression>),
    Literal(DynoType, DynoValue),
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declaration(Expression, DynoType),
    Assignment(Expression, Expression),
    If(Expression, Box<Statement>),
    Return(Expression),
    Block(Vec<Statement>),
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

impl Expression {
    pub fn get_type(&self) -> DynoResult<DynoType> {
        match self {
            Expression::BinaryOperation(op, left, right) => {
                use BinaryOperationType::*;

                let left_type = left.get_type()?;
                let right_type = right.get_type()?;
                // TODO: this should probably get replaced by something better
                match op {
                    Equal | NotEqual | LessThan | LessThanEqual | GreaterThan
                    | GreaterThanEqual => {
                        if left_type == right_type {
                            Ok(DynoType::Bool())
                        } else {
                            Err(DynoError::IncompatibleTypeError(left_type, right_type))
                        }
                    }
                    _ => {
                        if left_type.is_int() && right_type.is_int() {
                            if left_type.get_bits() > right_type.get_bits() {
                                Ok(left_type)
                            } else {
                                Ok(right_type)
                            }
                        } else {
                            Err(DynoError::IncompatibleTypeError(left_type, right_type))
                        }
                    }
                }
            }
            Expression::Literal(value_type, _) => Ok(*value_type),
            _ => Ok(DynoType::Void()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOperationType::*;
    use crate::types::{DynoType, DynoValue};

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
            Box::new(AstNode::Literal(DynoType::UInt8(), DynoValue::UInt(4))),
            Box::new(AstNode::BinaryOperation(
                Multiply,
                Box::new(AstNode::Literal(DynoType::UInt8(), DynoValue::UInt(3))),
                Box::new(AstNode::Literal(DynoType::UInt8(), DynoValue::UInt(2))),
            )),
        );

        assert_eq!(ast.get_type(), Ok(DynoType::UInt8()));
    }
}
