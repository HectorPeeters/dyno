use crate::error::*;
use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\n\f]+")]
    Whitespace,

    #[regex(r"if")]
    If,

    #[regex(r"[a-zA-Z]+", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r"[0-9]+", |lex| lex.slice().to_string())]
    IntegerLiteral(String),

    #[regex(r"\+")]
    Plus,
    #[regex(r"-")]
    Minus,
    #[regex(r"\*")]
    Asterix,
    #[regex(r"/")]
    Slash,

    #[error]
    Error,
}

pub fn lex(input: &str) -> DynoResult<Vec<Token>> {
    let mut lex = Token::lexer(input);

    let mut tokens: Vec<Token> = vec![];

    let mut token;
    loop {
        token = lex.next();

        if token.is_none() {
            break;
        }

        let token = token.unwrap();

        match token {
            Token::Error => {
                return Err(DynoError::LexerError(lex.slice().to_string(), lex.span()));
            }
            _ => tokens.push(token),
        }
    }

    return Ok(tokens);
}
