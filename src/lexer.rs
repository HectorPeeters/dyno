use crate::error::*;
use logos::Logos;
use std::ops::Range;

#[derive(Logos, Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    #[regex(r"[ \t\n\f]+")]
    Whitespace,

    #[regex(r"let")]
    Let,

    #[regex(r"[a-zA-Z]+")]
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

    #[regex(r"=")]
    Equals,

    #[regex(r";")]
    SemiColon,

    #[regex(r"\(")]
    LeftParen,
    #[regex(r"\)")]
    RightParen,

    #[error]
    Error,

    Eof,
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
    let mut lex = TokenType::lexer(input);

    let mut tokens: Vec<Token> = vec![];

    loop {
        let token_type = lex.next();

        if token_type.is_none() {
            break;
        }

        let token_type = token_type.unwrap();

        match token_type {
            TokenType::Error => {
                return Err(DynoError::LexerError(lex.slice().to_string(), lex.span()));
            }
            TokenType::Whitespace => {}
            _ => {
                let token = Token::new_with_span(token_type, lex.slice(), lex.span());
                tokens.push(token);
            }
        }
    }

    tokens.push(Token::with_type(TokenType::Eof));

    Ok(tokens)
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
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, Eof);
    }

    #[test]
    fn lexer_integer_literal() {
        let tokens = get_tokens("12 0 439394474 123");

        assert_eq!(tokens[0], Token::new(IntegerLiteral, "12"));
        assert_eq!(tokens[1], Token::new(IntegerLiteral, "0"));
        assert_eq!(tokens[2], Token::new(IntegerLiteral, "439394474"));
        assert_eq!(tokens[3], Token::new(IntegerLiteral, "123"));
        assert_eq!(tokens[4].token_type, Eof);
    }

    #[test]
    fn lexer_binary_operands() {
        let tokens = get_tokens("+-*/");

        assert_eq!(tokens[0].token_type, Plus);
        assert_eq!(tokens[1].token_type, Minus);
        assert_eq!(tokens[2].token_type, Asterix);
        assert_eq!(tokens[3].token_type, Slash);
        assert_eq!(tokens[4].token_type, Eof);
    }
    #[test]
    fn lexer_test_error() {
        let tokens = lex("&");

        assert!(tokens.is_err());
        assert_eq!(
            tokens.err().unwrap(),
            DynoError::LexerError("&".to_string(), 0..1)
        );
    }
}
