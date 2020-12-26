use crate::ast::{AstNode, BinaryOperationType};
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
        if self.index >= self.tokens.len() {
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
        if self.index >= self.tokens.len() {
            return Err(DynoError::TokenStreamOutOfBounds());
        }

        let result = &self.tokens[self.index];
        self.index += 1;
        Ok(result)
    }

    fn consume_expect(&mut self, expected: TokenType) -> DynoResult<&Token> {
        let token = self.consume()?;

        if token.token_type != expected {
            return Err(DynoError::UnexpectedTokenError(
                token.token_type,
                vec![expected],
            ));
        }

        Ok(token)
    }

    fn is_eof(&self) -> bool {
        self.index >= self.tokens.len() || self.tokens[self.index].token_type == TokenType::Eof
    }

    fn get_bit_count(value: u128) -> u8 {
        let floating_point = value as f64;
        let bits = floating_point.log2();

        bits as u8 + 1
    }

    fn parse_integer_literal(&mut self) -> DynoResult<AstNode> {
        let token = self.consume_expect(TokenType::IntegerLiteral)?;

        let value: Result<u128, _> = token.value.parse();
        match value {
            Ok(x) => {
                let bits = Parser::get_bit_count(x);
                Ok(AstNode::IntegerLiteral(x, bits))
            }
            Err(_) => Err(DynoError::IntegerParseError(token.value.clone())),
        }
    }

    fn parse_unary_expression(&mut self) -> DynoResult<AstNode> {
        let next = self.peek()?;

        match next.token_type {
            TokenType::IntegerLiteral => self.parse_integer_literal(),
            _ => Err(DynoError::UnexpectedTokenError(
                next.token_type,
                vec![TokenType::IntegerLiteral],
            )),
        }
    }

    fn parse_expression(&mut self, precendence: u8) -> DynoResult<AstNode> {
        let mut left = self.parse_unary_expression()?;

        let mut operator = self.peek()?;

        if operator.token_type == TokenType::Eof {
            return Ok(left);
        }

        let mut operator_type = BinaryOperationType::from_token_type(operator.token_type)?;
        let mut current_precendence = operator_type.get_precedence();

        while current_precendence > precendence {
            let token_type = operator.token_type;
            self.consume_expect(token_type)?;

            let right = self.parse_expression(current_precendence)?;

            //TODO: do type checking here

            left = AstNode::BinaryOperation(operator_type, Box::new(left), Box::new(right));

            operator = self.peek()?;

            if operator.token_type == TokenType::Eof {
                return Ok(left);
            }

            operator_type = BinaryOperationType::from_token_type(operator.token_type)?;
            current_precendence = operator_type.get_precedence();
        }

        Ok(left)
    }
}

pub fn parse(input: Vec<Token>) -> DynoResult<AstNode> {
    let mut parser = Parser::new(input);

    let mut nodes: Vec<AstNode> = vec![];

    loop {
        if parser.is_eof() {
            break;
        }

        let node = parser.parse_expression(0)?;

        nodes.push(node);
    }

    Ok(AstNode::Block(nodes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOperationType::*;
    use crate::lexer::TokenType::*;
    use crate::lexer::lex;

    #[test]
    fn parser_new() {
        let parser = Parser::new(vec![]);

        assert!(parser.is_eof());
    }

    #[test]
    fn parser_peek() {
        let parser = Parser::new(vec![
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

    #[test]
    fn parser_basic_binary_op() {
        let ast = parse(lex("12 + 4").unwrap()).unwrap();

        assert_eq!(
            ast,
            AstNode::Block(vec![AstNode::BinaryOperation(
                Add,
                Box::new(AstNode::IntegerLiteral(12, 4)),
                Box::new(AstNode::IntegerLiteral(4, 3)),
            )])
        );
    }

    #[test]
    fn parser_precendence_a() {
        let ast = parse(lex("12 + 4 * 7").unwrap()).unwrap();

        assert_eq!(
            ast,
            AstNode::Block(vec![AstNode::BinaryOperation(
                Add,
                Box::new(AstNode::IntegerLiteral(12, 4)),
                Box::new(AstNode::BinaryOperation(
                    Multiply,
                    Box::new(AstNode::IntegerLiteral(4, 3)),
                    Box::new(AstNode::IntegerLiteral(7, 3)),
                )),
            )])
        );
    }

    #[test]
    fn parser_precendence_b() {
        let ast = parse(lex("12 * 4 + 7").unwrap()).unwrap();

        assert_eq!(
            ast,
            AstNode::Block(vec![AstNode::BinaryOperation(
                Add,
                Box::new(AstNode::BinaryOperation(
                    Multiply,
                    Box::new(AstNode::IntegerLiteral(12, 4)),
                    Box::new(AstNode::IntegerLiteral(4, 3)),
                )),
                Box::new(AstNode::IntegerLiteral(7, 3)),
            )])
        );
    }

    #[test]
    fn parser_consume_out_of_bounds_error() {
        let mut parser = Parser::new(vec![]);
        let token = parser.consume();

        assert_eq!(token, Err(DynoError::TokenStreamOutOfBounds()));
    }

    #[test]
    fn parser_peek_out_of_bounds_error() {
        let parser = Parser::new(vec![]);
        let token = parser.peek();

        assert_eq!(token, Err(DynoError::TokenStreamOutOfBounds()));
    }

    #[test]
    fn parser_peek_next_out_of_bounds_error() {
        let parser = Parser::new(vec![Token::with_type(Eof)]);
        let token = parser.peek_next(1);

        assert_eq!(token, Err(DynoError::TokenStreamOutOfBounds()));
    }

    #[test]
    fn parser_consume_expect_error() {
        let mut parser = Parser::new(vec![Token::with_type(Eof)]);
        let token = parser.consume_expect(IntegerLiteral);

        assert_eq!(
            token,
            Err(DynoError::UnexpectedTokenError(Eof, vec![IntegerLiteral]))
        );
    }

    #[test]
    fn parser_integer_literal_error() {
        let mut parser = Parser::new(vec![Token::new(IntegerLiteral, "a")]);
        let node = parser.parse_integer_literal();

        assert_eq!(node, Err(DynoError::IntegerParseError("a".to_string())));
    }

    #[test]
    fn parser_unary_expression_error() {
        let mut parser = Parser::new(vec![Token::new(IntegerLiteral, "a")]);
        let node = parser.parse_integer_literal();

        assert_eq!(node, Err(DynoError::IntegerParseError("a".to_string())));
    }

    #[test]
    fn parser_expression_two_operands_error() {
        let mut parser = Parser::new(vec![
            Token::new(IntegerLiteral, "5"),
            Token::with_type(TokenType::Plus),
            Token::with_type(TokenType::Plus),
            Token::new(IntegerLiteral, "12"),
        ]);
        let node = parser.parse_expression(0);

        assert_eq!(
            node,
            Err(DynoError::UnexpectedTokenError(Plus, vec![IntegerLiteral]))
        );
    }

    #[test]
    fn parser_expression_two_ints_error() {
        let mut parser = Parser::new(lex("5 + 12 8").unwrap()
        );
        let node = parser.parse_expression(0);

        assert_eq!(
            node,
            Err(DynoError::UnexpectedTokenError(
                IntegerLiteral,
                vec![Plus, Minus, Asterix, Slash]
            ))
        );
    }
}
