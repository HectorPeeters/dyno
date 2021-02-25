use crate::ast::{AstNode, BinaryOperationType};
use crate::error::*;
use crate::lexer::{Token, TokenType};
use crate::types::DynoType;

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

        if token != expected {
            return Err(DynoError::UnexpectedTokenError(
                token.token_type,
                vec![expected],
            ));
        }

        Ok(token)
    }

    fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
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

    fn parse_primary_expression(&mut self) -> DynoResult<AstNode> {
        use TokenType::*;

        let next = self.peek()?;

        match next.token_type {
            IntegerLiteral => self.parse_integer_literal(),
            LeftParen => {
                self.consume_expect(LeftParen)?;
                let expression = self.parse_expression(0)?;
                self.consume_expect(RightParen)?;
                Ok(expression)
            }
            _ => Err(DynoError::UnexpectedTokenError(
                next.token_type,
                vec![IntegerLiteral, LeftParen],
            )),
        }
    }

    fn parse_unary_expression(&mut self) -> DynoResult<AstNode> {
        self.parse_primary_expression()
    }

    fn parse_expression(&mut self, precendence: u8) -> DynoResult<AstNode> {
        const DELIMETERS: [TokenType; 2] = [TokenType::SemiColon, TokenType::RightParen];

        let mut left = self.parse_unary_expression()?;

        let mut operator = self.peek()?;

        if DELIMETERS.contains(&operator.token_type) {
            return Ok(left);
        }

        let mut operator_type = BinaryOperationType::from_token_type(operator.token_type)?;
        let mut current_precendence = operator_type.get_precedence();

        while current_precendence > precendence {
            let token_type = operator.token_type;
            self.consume_expect(token_type)?;

            let right = self.parse_expression(current_precendence)?;

            left = AstNode::BinaryOperation(operator_type, Box::new(left), Box::new(right));

            operator = self.peek()?;

            if DELIMETERS.contains(&operator.token_type) {
                return Ok(left);
            }

            operator_type = BinaryOperationType::from_token_type(operator.token_type)?;
            current_precendence = operator_type.get_precedence();
        }

        Ok(left)
    }

    fn parse_type(&mut self) -> DynoResult<DynoType> {
        use TokenType::*;

        let token = self.consume()?;

        match token.token_type {
            UnsignedIntType(size) => Ok(DynoType::UnsignedInt(size)),
            SignedIntType(size) => Ok(DynoType::SignedInt(size)),
            Bool => Ok(DynoType::Bool()),
            _ => Err(DynoError::UnexpectedTokenError(
                token.token_type,
                vec![UnsignedIntType(0), SignedIntType(0), Bool],
            )),
        }
    }

    fn parse_declaration_or_assignment(&mut self) -> DynoResult<AstNode> {
        self.consume_expect(TokenType::Let)?;

        let variable_name = self.consume_expect(TokenType::Identifier)?.value.clone();
        self.consume_expect(TokenType::Colon)?;

        let variable_type = self.parse_type()?;

        if self.peek()?.token_type == TokenType::SemiColon {
            self.consume_expect(TokenType::SemiColon);
            // We just have a declaration here
            return Ok(AstNode::Declaration(variable_name, variable_type));
        }

        self.consume_expect(TokenType::Equals)?;

        let expression = self.parse_expression(0)?;
        self.consume_expect(TokenType::SemiColon)?;

        Ok(AstNode::Block(vec![
            AstNode::Declaration(variable_name.clone(), variable_type),
            AstNode::Assignment(variable_name, Box::new(expression)),
        ]))
    }

    fn parse_assignment(&mut self) -> DynoResult<AstNode> {
        let variable_name = self.consume_expect(TokenType::Identifier)?.value.clone();
        self.consume_expect(TokenType::Equals)?;

        let expression = self.parse_expression(0)?;
        self.consume_expect(TokenType::SemiColon)?;

        Ok(AstNode::Assignment(variable_name, Box::new(expression)))
    }

    fn parse_return_statement(&mut self) -> DynoResult<AstNode> {
        self.consume_expect(TokenType::Return)?;
        let expression = self.parse_expression(0)?;
        self.consume_expect(TokenType::SemiColon)?;

        Ok(AstNode::Return(Box::new(expression)))
    }
}

pub fn parse(input: Vec<Token>) -> DynoResult<AstNode> {
    let mut parser = Parser::new(input);

    let mut nodes: Vec<AstNode> = vec![];

    while !parser.is_eof() {
        let node = match parser.peek()?.token_type {
            TokenType::Let => parser.parse_declaration_or_assignment(),
            TokenType::Return => parser.parse_return_statement(),
            TokenType::Identifier => parser.parse_assignment(),
            _ => {
                let node = parser.parse_expression(0);
                parser.consume_expect(TokenType::SemiColon)?;
                node
            }
        }?;

        nodes.push(node);
    }

    Ok(match nodes.len() {
        1 => nodes.remove(0),
        _ => AstNode::Block(nodes),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOperationType::*;
    use crate::lexer::lex;
    use crate::lexer::TokenType::*;

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

        assert_eq!(parser.peek().unwrap(), Plus);
        assert_eq!(parser.peek_next(0).unwrap(), Plus);
        assert_eq!(parser.peek_next(1).unwrap(), Whitespace);
        assert_eq!(parser.peek_next(2).unwrap(), Minus);

        assert!(parser.peek_next(3).is_err());
    }

    #[test]
    fn parser_consume() {
        let mut parser = Parser::new(vec![
            Token::with_type(Plus),
            Token::with_type(Whitespace),
            Token::with_type(Minus),
        ]);

        assert_eq!(parser.consume().unwrap(), Plus);
        assert_eq!(parser.consume().unwrap(), Whitespace);
        assert_eq!(parser.consume().unwrap(), Minus);

        assert!(parser.consume().is_err());
    }

    fn get_ast(text: &str) -> DynoResult<AstNode> {
        parse(lex(text)?)
    }

    #[test]
    fn parser_basic_binary_op() -> DynoResult<()> {
        assert_eq!(
            get_ast("12 + 4;")?,
            AstNode::BinaryOperation(
                Add,
                Box::new(AstNode::IntegerLiteral(12, 4)),
                Box::new(AstNode::IntegerLiteral(4, 3)),
            )
        );
        Ok(())
    }

    #[test]
    fn parser_precendence_a() -> DynoResult<()> {
        assert_eq!(
            get_ast("12 + 4 * 7;")?,
            AstNode::BinaryOperation(
                Add,
                Box::new(AstNode::IntegerLiteral(12, 4)),
                Box::new(AstNode::BinaryOperation(
                    Multiply,
                    Box::new(AstNode::IntegerLiteral(4, 3)),
                    Box::new(AstNode::IntegerLiteral(7, 3)),
                )),
            )
        );
        Ok(())
    }

    #[test]
    fn parser_precendence_b() -> DynoResult<()> {
        assert_eq!(
            get_ast("12 * 4 + 7;")?,
            AstNode::BinaryOperation(
                Add,
                Box::new(AstNode::BinaryOperation(
                    Multiply,
                    Box::new(AstNode::IntegerLiteral(12, 4)),
                    Box::new(AstNode::IntegerLiteral(4, 3)),
                )),
                Box::new(AstNode::IntegerLiteral(7, 3)),
            )
        );
        Ok(())
    }

    #[test]
    fn parse_equals_operator() -> DynoResult<()> {
        assert_eq!(
            get_ast("1 == 2;")?,
            AstNode::BinaryOperation(
                Equal,
                Box::new(AstNode::IntegerLiteral(1, 1)),
                Box::new(AstNode::IntegerLiteral(2, 2))
            )
        );
        Ok(())
    }

    #[test]
    fn parse_simple_declaration() -> DynoResult<()> {
        let ast = get_ast("let a: u32;")?;
        assert_eq!(
            ast,
            AstNode::Declaration("a".to_string(), DynoType::UnsignedInt(32))
        );
        Ok(())
    }

    #[test]
    fn parse_simple_boolean() -> DynoResult<()> {
        let ast = get_ast("let a: bool;")?;
        assert_eq!(ast, AstNode::Declaration("a".to_string(), DynoType::Bool()));
        Ok(())
    }

    #[test]
    fn parser_simple_assignment() -> DynoResult<()> {
        let ast = get_ast("let a: u32 = 12;")?;

        assert_eq!(
            ast,
            AstNode::Block(vec![
                AstNode::Declaration("a".to_string(), DynoType::UnsignedInt(32)),
                AstNode::Assignment("a".to_string(), Box::new(AstNode::IntegerLiteral(12, 4)))
            ])
        );
        Ok(())
    }

    #[test]
    fn parser_complex_assignment() -> DynoResult<()> {
        let ast = get_ast("let a: u32 = 12 - 2 * 4;")?;

        assert_eq!(
            ast,
            AstNode::Block(vec![
                AstNode::Declaration("a".to_string(), DynoType::UnsignedInt(32)),
                AstNode::Assignment(
                    "a".to_string(),
                    Box::new(AstNode::BinaryOperation(
                        BinaryOperationType::Subtract,
                        Box::new(AstNode::IntegerLiteral(12, 4)),
                        Box::new(AstNode::BinaryOperation(
                            BinaryOperationType::Multiply,
                            Box::new(AstNode::IntegerLiteral(2, 2)),
                            Box::new(AstNode::IntegerLiteral(4, 3))
                        ))
                    ))
                )
            ])
        );
        Ok(())
    }

    #[test]
    fn parse_simple_parentheses() -> DynoResult<()> {
        let ast = get_ast("(12);")?;
        assert_eq!(ast, AstNode::IntegerLiteral(12, 4));
        Ok(())
    }

    #[test]
    fn parse_parentheses_expression() -> DynoResult<()> {
        use AstNode::*;
        use BinaryOperationType::*;
        let ast = get_ast("(4 + 2) * 3;")?;
        assert_eq!(
            ast,
            BinaryOperation(
                Multiply,
                Box::new(BinaryOperation(
                    Add,
                    Box::new(IntegerLiteral(4, 3)),
                    Box::new(IntegerLiteral(2, 2))
                )),
                Box::new(IntegerLiteral(3, 2))
            )
        );
        Ok(())
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
        let parser = Parser::new(vec![Token::with_type(SemiColon)]);
        let token = parser.peek_next(1);

        assert_eq!(token, Err(DynoError::TokenStreamOutOfBounds()));
    }

    #[test]
    fn parser_consume_expect_error() {
        let mut parser = Parser::new(vec![Token::with_type(SemiColon)]);
        let token = parser.consume_expect(IntegerLiteral);

        assert_eq!(
            token,
            Err(DynoError::UnexpectedTokenError(
                SemiColon,
                vec![IntegerLiteral]
            ))
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

        assert!(node.is_err());
    }

    #[test]
    fn parser_expression_two_ints_error() {
        let mut parser = Parser::new(lex("5 + 12 8").unwrap());
        let node = parser.parse_expression(0);

        assert!(node.is_err());
    }
}
