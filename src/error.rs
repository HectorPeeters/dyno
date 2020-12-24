use crate::lexer::TokenType;
use std::ops::Range;

#[derive(Debug)]
pub enum DynoError {
    LexerError(String, Range<usize>),
    TokenStreamOutOfBounds(),
    ExpectedTokenFailed(TokenType, TokenType),
}

pub type DynoResult<T> = Result<T, DynoError>;
