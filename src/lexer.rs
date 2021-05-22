use crate::error::*;
use crate::token::{Token, TokenType};
use regex::Regex;

pub struct Lexer<'a> {
    rules: Vec<(Regex, TokenType)>,
    input: &'a str,
    pointer: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        use TokenType::*;

        let rules = vec![
            (r"[ \t\n\f]+", Whitespace),
            (r"let", Let),
            (r"while", While),
            (r"return", Return),
            (r"if", If),
            (r"u8", UInt8),
            (r"u16", UInt16),
            (r"u32", UInt32),
            (r"u64", UInt64),
            (r"bool", Bool),
            (r"[a-zA-Z][_a-zA-Z]*", Identifier),
            (r"[0-9]+", IntegerLiteral),
            (r"\+", Plus),
            (r"-", Minus),
            (r"\*", Asterix),
            (r"/", Slash),
            (r"==", DoubleEqual),
            (r"!=", NotEqual),
            (r"<=", LessThanEqual),
            (r"<", LessThan),
            (r">=", GreaterThanEqual),
            (r">", GreaterThan),
            (r"=", Equals),
            (r":", Colon),
            (r";", SemiColon),
            (r"\(", LeftParen),
            (r"\)", RightParen),
            (r"\{", LeftBrace),
            (r"\}", RightBrace),
        ];

        let rules = rules
            .into_iter()
            .map(|x| (Regex::new(x.0).unwrap(), x.1))
            .collect();

        Self {
            rules,
            input,
            pointer: 0,
        }
    }

    pub fn get_tokens(&mut self) -> DynoResult<Vec<Token>> {
        let mut result = vec![];

        loop {
            if self.pointer >= self.input.len() {
                break;
            }

            let mut matches = vec![];
            for rule in &self.rules {
                if let Some(x) = rule.0.find(&self.input[self.pointer..]) {
                    if x.start() == 0 {
                        matches.push(Token::new_with_span(rule.1, x.as_str(), x.range()));
                    }
                }
            }

            if matches.is_empty() {
                return Err(DynoError::LexerError("Unable to lex".to_string()));
            }

            matches.sort_unstable_by(|a, b| {
                (b.span.end - b.span.start).cmp(&(a.span.end - a.span.start))
            });
            let best_match = matches.remove(0);
            self.pointer += best_match.span.end;
            result.push(best_match);
        }

        Ok(result
            .into_iter()
            .filter(|x| x.token_type != TokenType::Whitespace)
            .collect())
    }
}

pub fn lex(input: &str) -> DynoResult<Vec<Token>> {
    Lexer::new(input).get_tokens()
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
    }

    #[test]
    fn lexer_test_error() {
        let tokens = lex("return &;");

        assert!(tokens.is_err());
    }
}
