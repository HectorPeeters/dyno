use crate::lexer::Token;
use std::ops::Range;

#[derive(Debug)]
pub enum DynoError {
    LexerError(String, Range<usize>),
}

pub type DynoResult<T> = Result<T, DynoError>;
