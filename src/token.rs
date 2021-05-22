use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Whitespace,

    Let,
    While,
    Return,
    If,

    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Bool,

    Identifier,

    IntegerLiteral,

    Plus,
    Minus,
    Asterix,
    Slash,
    DoubleEqual,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    Equals,

    Colon,
    SemiColon,

    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub span: Range<usize>,
}

impl Token {
    pub fn new(token_type: TokenType, value: &str) -> Self {
        Self {
            token_type,
            value: value.to_string(),
            span: 0..0,
        }
    }

    pub fn with_type(token_type: TokenType) -> Self {
        Self {
            token_type,
            value: String::default(),
            span: 0..0,
        }
    }

    pub fn new_with_span(token_type: TokenType, value: &str, span: Range<usize>) -> Self {
        Self {
            token_type,
            value: value.to_string(),
            span,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type && self.value == other.value
    }
}

impl PartialEq<TokenType> for &Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.token_type == *other
    }
}
