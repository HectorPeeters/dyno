use crate::lexer::TokenType;
use std::ops::Range;

#[derive(Debug)]
pub enum DynoError {
    LexerError(String, Range<usize>),
    TokenStreamOutOfBounds(),
    IntegerParseError(String),
    UnexpectedTokenError(TokenType, Vec<TokenType>),
}

pub type DynoResult<T> = Result<T, DynoError>;
