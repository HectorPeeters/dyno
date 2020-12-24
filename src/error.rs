use std::ops::Range;

#[derive(Debug)]
pub enum DynoError {
    LexerError(String, Range<usize>),
    TokenStreamOutOfBounds(),
}

pub type DynoResult<T> = Result<T, DynoError>;
