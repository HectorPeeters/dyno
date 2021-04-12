use crate::lexer::TokenType;
use crate::types::DynoType;
use std::fmt;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum DynoError {
    LexerError(String, Range<usize>),
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
    LlvmError(String),
}

impl<T> From<std::io::IntoInnerError<T>> for DynoError {
    fn from(_error: std::io::IntoInnerError<T>) -> Self {
        DynoError::IntoInnerError()
    }
}

impl From<inkwell::support::LLVMString> for DynoError {
    fn from(error: inkwell::support::LLVMString) -> Self {
        DynoError::LlvmError(error.to_string())
    }
}

pub type DynoResult<T> = Result<T, DynoError>;

impl fmt::Display for DynoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DynoError::*;

        match self {
            LexerError(message, location) => write!(
                f,
                "Lexer error on {}-{}: {}",
                location.start, location.end, message
            ),
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
            LlvmError(message) => write!(f, "LLVM error: {}", message),
        }
    }
}
