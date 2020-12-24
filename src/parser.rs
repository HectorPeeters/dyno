use crate::ast::AstNode;
use crate::error::*;
use crate::lexer::{Token, TokenType};

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

    fn consume_expect(&mut self, expected: TokenType) -> DynoResult<&Token> {
        let token = self.consume()?;

        if token.token_type != expected {
            return Err(DynoError::ExpectedTokenFailed(token.token_type, expected));
        }

        Ok(token)
    }

    fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn parse_integer_literal(&mut self) -> AstNode {
        AstNode::Empty()
    }
}

pub fn parse(input: Vec<Token>) -> DynoResult<AstNode> {
    let mut parser = Parser::new(input);

    loop {
        if parser.is_eof() {
            break;
        }

        let token = parser.consume();
    }

    Ok(AstNode::Empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOperation::*;
    use crate::lexer::TokenType::*;

    #[test]
    fn parser_new() {
        let parser = Parser::new(vec![]);

        assert!(parser.is_eof());
    }

    #[test]
    fn parser_peek() {
        let mut parser = Parser::new(vec![
            Token::with_type(Plus),
            Token::with_type(Whitespace),
            Token::with_type(Minus),
        ]);

        assert_eq!(parser.peek().unwrap().token_type, Plus);
        assert_eq!(parser.peek_next(0).unwrap().token_type, Plus);
        assert_eq!(parser.peek_next(1).unwrap().token_type, Whitespace);
        assert_eq!(parser.peek_next(2).unwrap().token_type, Minus);

        assert!(parser.peek_next(3).is_err());
    }

    #[test]
    fn parser_consume() {
        let mut parser = Parser::new(vec![
            Token::with_type(Plus),
            Token::with_type(Whitespace),
            Token::with_type(Minus),
        ]);

        assert_eq!(parser.consume().unwrap().token_type, Plus);
        assert_eq!(parser.consume().unwrap().token_type, Whitespace);
        assert_eq!(parser.consume().unwrap().token_type, Minus);

        assert!(parser.consume().is_err());
    }

    //    #[test]
    //    fn parser_basic_binary_op() {
    //        let ast = parse(vec![
    //            Token::new(IntegerLiteral, "12"),
    //            Token::new(Plus, "+"),
    //            Token::new(IntegerLiteral, "4"),
    //        ])
    //        .unwrap();
    //
    //        assert_eq!(
    //            ast,
    //            AstNode::BinaryOperation(
    //                Box::new(AstNode::IntegerLiteral(12)),
    //                Box::new(AstNode::IntegerLiteral(4)),
    //                Add
    //            )
    //        );
    //    }
}
