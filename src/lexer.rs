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

    loop {
        let token = lex.next();

        if token.is_none() {
            break;
        }

        let token = token.unwrap();

        match token {
            Token::Error => {
                return Err(DynoError::LexerError(lex.slice().to_string(), lex.span()));
            }
            Token::Whitespace => {}
            _ => tokens.push(token),
        }
    }

    return Ok(tokens);
}

#[cfg(test)]
mod tests {
    use super::Token::*;
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
    fn lexer_integer_literal() {
        let tokens = get_tokens("12 0 439394474 123");

        assert_eq!(tokens[0], IntegerLiteral("12".to_string()));
        assert_eq!(tokens[1], IntegerLiteral("0".to_string()));
        assert_eq!(tokens[2], IntegerLiteral("439394474".to_string()));
        assert_eq!(tokens[3], IntegerLiteral("123".to_string()));
    }

    #[test]
    fn lexer_binary_operands() {
        let tokens = get_tokens("+-*/");

        assert_eq!(tokens[0], Plus);
        assert_eq!(tokens[1], Minus);
        assert_eq!(tokens[2], Asterix);
        assert_eq!(tokens[3], Slash);
    }
}
