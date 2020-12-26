use crate::lexer::TokenType;
use crate::types::DynoType;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum DynoError {
    LexerError(String, Range<usize>),
    TokenStreamOutOfBounds(),
    IntegerParseError(String),
    UnexpectedTokenError(TokenType, Vec<TokenType>),
    IncompatibleTypeError(DynoType, DynoType),
    ElfWriteError(),
}

pub type DynoResult<T> = Result<T, DynoError>;
