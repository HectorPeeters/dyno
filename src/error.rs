use crate::token::TokenType;
use crate::types::DynoType;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DynoError {
    LexerError(String),
    TokenStreamOutOfBounds(),
    IntegerParseError(String),
    UnexpectedTokenError(TokenType, Vec<TokenType>),
    IncompatibleTypeError(DynoType, DynoType),
    IdentifierError(String),
    ElfWriteError(),
    X86WriteError(),
    GeneratorError(String),
    VisitError(String),
    NoneError(),
    IntoInnerError(),
}

impl<T> From<std::io::IntoInnerError<T>> for DynoError {
    fn from(_error: std::io::IntoInnerError<T>) -> Self {
        DynoError::IntoInnerError()
    }
}

pub type DynoResult<T> = Result<T, DynoError>;

impl fmt::Display for DynoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DynoError::*;

        match self {
            LexerError(message) => write!(f, "Lexer error on: {}", message),
            TokenStreamOutOfBounds() => write!(f, "Token stream out of bounds"),
            IntegerParseError(contents) => write!(f, "Integer parse error: {}", contents),
            UnexpectedTokenError(received, expected) => {
                write!(
                    f,
                    "Unexpected token {:?}, expected any of these: {:?}",
                    received, expected
                )
            }
            IncompatibleTypeError(left, right) => {
                write!(f, "Incompatible types {:?} and {:?}", left, right)
            }
            IdentifierError(message) => write!(f, "Identifier error: {}", message),
            ElfWriteError() => write!(f, "Error while writing ELF file"),
            X86WriteError() => write!(f, "Error while writing x86 assembly"),
            GeneratorError(message) => write!(f, "Code generator error: {}", message),
            VisitError(message) => write!(f, "Visit error: {}", message),
            NoneError() => write!(f, "None error"),
            IntoInnerError() => write!(f, "Into inner error"),
        }
    }
}
