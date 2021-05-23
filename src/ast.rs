use crate::error::*;
use crate::scope::Scope;
use crate::token::TokenType;
use crate::types::{DynoType, DynoValue};
use std::cmp::Ordering;

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
    Widen(Box<Expression>, DynoType),
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declaration(String, DynoType),
    Assignment(String, Expression),
    If(Expression, Box<Statement>),
    While(Expression, Box<Statement>),
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
    pub fn make_binop_compatible(
        op_type: BinaryOperationType,
        left: Expression,
        right: Expression,
        scope: &Scope<DynoType>,
    ) -> DynoResult<Option<Expression>> {
        match op_type {
            BinaryOperationType::Add
            | BinaryOperationType::Subtract
            | BinaryOperationType::Multiply
            | BinaryOperationType::Divide
            | BinaryOperationType::Equal
            | BinaryOperationType::NotEqual
            | BinaryOperationType::LessThan
            | BinaryOperationType::LessThanEqual
            | BinaryOperationType::GreaterThan
            | BinaryOperationType::GreaterThanEqual => {
                let left_type = left.get_type(scope)?;
                let right_type = right.get_type(scope)?;
                let left_size = left_type.get_bits();
                let right_size = right_type.get_bits();

                Ok(Some(match left_size.cmp(&right_size) {
                    Ordering::Less => Expression::BinaryOperation(
                        op_type,
                        Box::new(Expression::Widen(Box::new(left), right_type)),
                        Box::new(right),
                    ),
                    Ordering::Greater => Expression::BinaryOperation(
                        op_type,
                        Box::new(left),
                        Box::new(Expression::Widen(Box::new(right), left_type)),
                    ),
                    Ordering::Equal => {
                        Expression::BinaryOperation(op_type, Box::new(left), Box::new(right))
                    }
                }))
            }
        }
    }

    pub fn make_assignment_compatible(
        left_type: DynoType,
        right: Expression,
        scope: &Scope<DynoType>,
    ) -> DynoResult<Expression> {
        let right_type = right.get_type(scope)?;
        let left_size = left_type.get_bits();
        let right_size = right_type.get_bits();

        match left_size.cmp(&right_size) {
            Ordering::Greater => match right {
                Expression::BinaryOperation(op_type, l, r) => Ok(Expression::BinaryOperation(
                    op_type,
                    Box::new(Expression::make_assignment_compatible(
                        left_type, *l, scope,
                    )?),
                    Box::new(Expression::make_assignment_compatible(
                        left_type, *r, scope,
                    )?),
                )),
                Expression::Literal(_, _) => Ok(Expression::Widen(Box::new(right), left_type)),
                Expression::Widen(e, _) => Ok(Expression::Widen(e, left_type)),
                Expression::Identifier(_) => Ok(Expression::Widen(Box::new(right), left_type)),
            },
            Ordering::Less => Err(DynoError::IncompatibleTypeError(left_type, right_type)),
            Ordering::Equal => Ok(right),
        }
    }

    pub fn get_type(&self, scope: &Scope<DynoType>) -> DynoResult<DynoType> {
        match self {
            Expression::BinaryOperation(op, left, right) => {
                use BinaryOperationType::*;

                let left_type = left.get_type(scope)?;
                let right_type = right.get_type(scope)?;
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
                        if left_type.is_int()
                            && right_type.is_int()
                            && (left_type.get_bits() == right_type.get_bits())
                        {
                            Ok(left_type)
                        } else {
                            Err(DynoError::IncompatibleTypeError(left_type, right_type))
                        }
                    }
                }
            }
            Expression::Literal(value_type, _) => Ok(*value_type),
            Expression::Widen(_, value_type) => Ok(*value_type),
            Expression::Identifier(x) => scope.find(x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOperationType::*;
    use crate::ast::Expression::{BinaryOperation, Literal};
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
        let ast = BinaryOperation(
            Add,
            Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(4))),
            Box::new(BinaryOperation(
                Multiply,
                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(3))),
                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(2))),
            )),
        );

        let ast_type = ast.get_type(&Scope::default());
        assert!(ast_type.is_ok());
        assert_eq!(ast_type.unwrap(), DynoType::UInt8());
    }
}
