use crate::error::*;
use logos::Logos;
use std::ops::Range;

#[derive(Logos, Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    #[regex(r"[ \t\n\f]+")]
    Whitespace,

    #[regex(r"let")]
    Let,
    #[regex(r"while")]
    While,
    #[regex(r"return")]
    Return,
    #[regex(r"if")]
    If,

    #[regex(r"u8")]
    UInt8,
    #[regex(r"u16")]
    UInt16,
    #[regex(r"u32")]
    UInt32,
    #[regex(r"u64")]
    UInt64,
    #[regex(r"bool")]
    Bool,

    #[regex(r"[a-zA-Z][_a-zA-Z]*")]
    Identifier,

    #[regex(r"[0-9]+")]
    IntegerLiteral,

    #[regex(r"\+")]
    Plus,
    #[regex(r"-")]
    Minus,
    #[regex(r"\*")]
    Asterix,
    #[regex(r"/")]
    Slash,

    #[regex(r"==")]
    DoubleEqual,
    #[regex(r"!=")]
    NotEqual,
    #[regex(r"<")]
    LessThan,
    #[regex(r"<=")]
    LessThanEqual,
    #[regex(r">")]
    GreaterThan,
    #[regex(r">=")]
    GreaterThanEqual,

    #[regex(r"=")]
    Equals,

    #[regex(r":")]
    Colon,
    #[regex(r";")]
    SemiColon,

    #[regex(r"\(")]
    LeftParen,
    #[regex(r"\)")]
    RightParen,

    #[regex(r"\{")]
    LeftBrace,
    #[regex(r"\}")]
    RightBrace,

    #[error]
    Error,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub span: Range<usize>,
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

pub fn lex(input: &str) -> DynoResult<Vec<Token>> {
    TokenType::lexer(input)
        .spanned()
        .filter(|t| t.0 != TokenType::Whitespace)
        .map(|t| match t.0 {
            TokenType::Error => Err(DynoError::LexerError(input[t.1.clone()].to_string(), t.1)),
            _ => Ok(Token::new_with_span(t.0, &input[t.1.clone()], t.1)),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::TokenType::*;
    use super::*;

    fn get_tokens(input: &str) -> Vec<Token> {
        let tokens = lex(input);
        assert!(tokens.is_ok());
        tokens.unwrap()
    }

    #[test]
    fn lexer_empty() {
        let tokens = get_tokens("");
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn lexer_types() {
        let tokens = get_tokens("u8 u16 u32 u64 bool");

        assert_eq!(tokens[0].token_type, UInt8);
        assert_eq!(tokens[1].token_type, UInt16);
        assert_eq!(tokens[2].token_type, UInt32);
        assert_eq!(tokens[3].token_type, UInt64);
        assert_eq!(tokens[4].token_type, Bool);
    }

    #[test]
    fn lexer_keywords() {
        let tokens = get_tokens("let return if");

        assert_eq!(tokens[0].token_type, Let);
        assert_eq!(tokens[1].token_type, Return);
        assert_eq!(tokens[2].token_type, If);
    }

    #[test]
    fn lexer_integer_literal() {
        let tokens = get_tokens("12 0 439394474 123");

        assert_eq!(tokens[0], Token::new(IntegerLiteral, "12"));
        assert_eq!(tokens[1], Token::new(IntegerLiteral, "0"));
        assert_eq!(tokens[2], Token::new(IntegerLiteral, "439394474"));
        assert_eq!(tokens[3], Token::new(IntegerLiteral, "123"));
    }

    #[test]
    fn lexer_binary_operators() {
        let tokens = get_tokens("+-*/");

        assert_eq!(tokens[0].token_type, Plus);
        assert_eq!(tokens[1].token_type, Minus);
        assert_eq!(tokens[2].token_type, Asterix);
        assert_eq!(tokens[3].token_type, Slash);
    }

    #[test]
    fn lexer_test_comparison_operators() -> DynoResult<()> {
        let tokens = lex("== != < <= > >=")?;

        assert_eq!(tokens[0].token_type, DoubleEqual);
        assert_eq!(tokens[1].token_type, NotEqual);
        assert_eq!(tokens[2].token_type, LessThan);
        assert_eq!(tokens[3].token_type, LessThanEqual);
        assert_eq!(tokens[4].token_type, GreaterThan);
        assert_eq!(tokens[5].token_type, GreaterThanEqual);

        Ok(())
    }

    #[test]
    fn lexer_identifier() {
        let tokens = get_tokens("test test_with_underscore");

        assert_eq!(tokens[0], Token::new(Identifier, "test"));
        assert_eq!(tokens[1], Token::new(Identifier, "test_with_underscore"));
    }

    #[test]
    fn lexer_identifier_error() {
        let tokens = lex("_identifier");

        assert!(tokens.is_err());
        assert_eq!(
            tokens.err().unwrap(),
            DynoError::LexerError("_".to_string(), 0..1)
        );
    }

    #[test]
    fn lexer_test_error() {
        let tokens = lex("return &;");

        assert!(tokens.is_err());
        assert_eq!(
            tokens.err().unwrap(),
            DynoError::LexerError("&".to_string(), 7..8)
        );
    }
}
