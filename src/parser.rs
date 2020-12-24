use crate::ast::AstNode;
use crate::error::*;
use crate::lexer::Token;

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn peek(&self) -> DynoResult<&Token> {
        if self.is_eof() {
            return Err(DynoError::TokenStreamOutOfBounds());
        }

        Ok(&self.tokens[self.index])
    }

    fn peek_next(&self, index: usize) -> DynoResult<&Token> {
        if self.index + index >= self.tokens.len() {
            return Err(DynoError::TokenStreamOutOfBounds());
        }

        Ok(&self.tokens[self.index + index])
    }

    fn consume(&mut self) -> DynoResult<&Token> {
        if self.is_eof() {
            return Err(DynoError::TokenStreamOutOfBounds());
        }

        let result = &self.tokens[self.index];
        self.index += 1;
        Ok(result)
    }

    fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
    }
}

pub fn parse(input: Vec<Token>) -> DynoResult<AstNode> {
    Ok(AstNode::Empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::AstNode::*;
    use crate::ast::BinaryOperation::*;
    use crate::lexer::Token::*;

    #[test]
    fn parser_new() {
        let parser = Parser::new(vec![]);

        assert!(parser.is_eof());
    }

    #[test]
    fn parser_peek() {
        let parser = Parser::new(vec![Plus, Whitespace, Minus]);

        let token = parser.peek();
        assert!(token.is_ok());
        assert_eq!(*token.unwrap(), Plus);

        let token = parser.peek_next(0);
        assert!(token.is_ok());
        assert_eq!(*token.unwrap(), Plus);

        let token = parser.peek_next(1);
        assert!(token.is_ok());
        assert_eq!(*token.unwrap(), Whitespace);

        let token = parser.peek_next(2);
        assert!(token.is_ok());
        assert_eq!(*token.unwrap(), Minus);

        assert!(parser.peek_next(3).is_err());
    }

    #[test]
    fn parser_consume() {
        let mut parser = Parser::new(vec![Plus, Whitespace, Minus]);

        let token = parser.consume();
        assert!(token.is_ok());
        assert_eq!(*token.unwrap(), Plus);

        let token = parser.consume();
        assert!(token.is_ok());
        assert_eq!(*token.unwrap(), Whitespace);

        let token = parser.consume();
        assert!(token.is_ok());
        assert_eq!(*token.unwrap(), Minus);

        assert!(parser.consume().is_err());
    }
}
