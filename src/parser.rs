use crate::ast::{BinaryOperationType, Expression, Statement};
use crate::error::*;
use crate::scope::Scope;
use crate::token::{Token, TokenType};
use crate::types::{DynoType, DynoValue};

struct Parser {
    tokens: Vec<Token>,
    index: usize,
    variable_scope: Scope<DynoType>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            variable_scope: Scope::new(),
        }
    }

    fn peek(&self) -> DynoResult<&Token> {
        if self.index >= self.tokens.len() {
            return Err(DynoError::TokenStreamOutOfBounds());
        }

        Ok(&self.tokens[self.index])
    }

    #[allow(dead_code)]
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

    fn parse_integer_literal(&mut self) -> DynoResult<Expression> {
        let token = self.consume_expect(TokenType::IntegerLiteral)?;

        let value = token.value.parse::<u64>();
        match value {
            Ok(value) => {
                let mut value_type = DynoType::UInt64();
                if value < 2_u64.pow(8) {
                    value_type = DynoType::UInt8();
                } else if value < 2_u64.pow(16) {
                    value_type = DynoType::UInt16();
                } else if value < 2_u64.pow(32) {
                    value_type = DynoType::UInt32();
                }

                Ok(Expression::Literal(value_type, DynoValue::UInt(value)))
            }
            Err(_) => Err(DynoError::IntegerParseError(token.value.clone())),
        }
    }

    fn parse_identifier(&mut self) -> DynoResult<String> {
        let token = self.consume_expect(TokenType::Identifier)?;
        Ok(token.value.clone())
    }

    fn parse_primary_expression(&mut self) -> DynoResult<Expression> {
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
            Identifier => Ok(Expression::Identifier(self.parse_identifier()?)),
            _ => Err(DynoError::UnexpectedTokenError(
                next.token_type,
                vec![IntegerLiteral, LeftParen, Identifier],
            )),
        }
    }

    fn parse_unary_expression(&mut self) -> DynoResult<Expression> {
        self.parse_primary_expression()
    }

    fn parse_expression(&mut self, precendence: u8) -> DynoResult<Expression> {
        const DELIMETERS: [TokenType; 3] = [
            TokenType::SemiColon,
            TokenType::RightParen,
            TokenType::LeftBrace,
        ];

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
            let left_type = left.get_type(&self.variable_scope)?;
            let right_type = right.get_type(&self.variable_scope)?;

            left = Expression::make_binop_compatible(
                operator_type,
                left,
                right,
                &self.variable_scope,
            )?
            .ok_or(DynoError::IncompatibleTypeError(left_type, right_type))?;

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
            UInt8 => Ok(DynoType::UInt8()),
            UInt16 => Ok(DynoType::UInt16()),
            UInt32 => Ok(DynoType::UInt32()),
            UInt64 => Ok(DynoType::UInt64()),
            Bool => Ok(DynoType::Bool()),
            _ => Err(DynoError::UnexpectedTokenError(
                token.token_type,
                vec![Bool, UInt8, UInt16, UInt32, UInt64],
            )),
        }
    }

    fn parse_declaration(&mut self) -> DynoResult<Statement> {
        self.consume_expect(TokenType::Let)?;

        let identifier = self.parse_identifier()?;
        self.consume_expect(TokenType::Colon)?;

        let variable_type = self.parse_type()?;
        self.consume_expect(TokenType::SemiColon)?;

        self.variable_scope.insert(&identifier, variable_type)?;

        Ok(Statement::Declaration(identifier, variable_type))
    }

    fn parse_assignment(&mut self) -> DynoResult<Statement> {
        let identifier = self.parse_identifier()?;
        self.consume_expect(TokenType::Equals)?;

        let expression = self.parse_expression(0)?;
        self.consume_expect(TokenType::SemiColon)?;

        let variable_type = self.variable_scope.find(&identifier)?;

        Ok(Statement::Assignment(
            identifier,
            Expression::make_assignment_compatible(
                variable_type,
                expression,
                &self.variable_scope,
            )?,
        ))
    }

    fn parse_return_statement(&mut self) -> DynoResult<Statement> {
        self.consume_expect(TokenType::Return)?;
        let expression = self.parse_expression(0)?;
        self.consume_expect(TokenType::SemiColon)?;

        Ok(Statement::Return(expression))
    }

    fn parse_block(&mut self) -> DynoResult<Statement> {
        self.consume_expect(TokenType::LeftBrace)?;

        self.variable_scope.push();

        let mut statements = vec![];
        while self.peek()?.token_type != TokenType::RightBrace {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.variable_scope.pop()?;

        self.consume_expect(TokenType::RightBrace)?;
        if statements.len() == 1 {
            Ok(statements.remove(0))
        } else {
            Ok(Statement::Block(statements))
        }
    }

    fn parse_if_statement(&mut self) -> DynoResult<Statement> {
        self.consume_expect(TokenType::If)?;
        let condition = self.parse_expression(0)?;
        let true_node = self.parse_block()?;
        Ok(Statement::If(condition, Box::new(true_node)))
    }

    fn parse_while_statement(&mut self) -> DynoResult<Statement> {
        self.consume_expect(TokenType::While)?;
        let condition = self.parse_expression(0)?;
        let body = self.parse_block()?;
        Ok(Statement::While(condition, Box::new(body)))
    }

    fn parse_statement(&mut self) -> DynoResult<Statement> {
        match self.peek()?.token_type {
            TokenType::Let => self.parse_declaration(),
            TokenType::While => self.parse_while_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::Identifier => self.parse_assignment(),
            TokenType::LeftBrace => self.parse_block(),
            _ => Err(DynoError::UnexpectedTokenError(
                self.peek()?.token_type,
                vec![
                    TokenType::Let,
                    TokenType::While,
                    TokenType::Return,
                    TokenType::If,
                    TokenType::Identifier,
                    TokenType::LeftBrace,
                ],
            )),
        }
    }
}

pub fn parse(input: Vec<Token>) -> DynoResult<Statement> {
    let mut parser = Parser::new(input);

    let mut nodes: Vec<Statement> = vec![];

    while !parser.is_eof() {
        let node = parser.parse_statement()?;
        nodes.push(node);
    }

    Ok(match nodes.len() {
        1 => nodes.remove(0),
        _ => Statement::Block(nodes),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOperationType::*;
    use crate::ast::Expression::{BinaryOperation, Literal, Widen};
    use crate::ast::Statement::{Assignment, Block, Declaration, If, Return};
    use crate::lexer::lex;
    use crate::token::TokenType::*;

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

    fn get_statement(text: &str) -> DynoResult<Statement> {
        parse(lex(text)?)
    }

    #[test]
    fn parser_basic_binary_op() -> DynoResult<()> {
        assert_eq!(
            get_statement("return 12 + 4;")?,
            Return(BinaryOperation(
                Add,
                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(12))),
                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(4))),
            ))
        );
        Ok(())
    }

    #[test]
    fn parser_precendence_a() -> DynoResult<()> {
        assert_eq!(
            get_statement("return 12 + 4 * 7;")?,
            Return(BinaryOperation(
                Add,
                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(12))),
                Box::new(BinaryOperation(
                    Multiply,
                    Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(4))),
                    Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(7))),
                )),
            ))
        );
        Ok(())
    }

    #[test]
    fn parser_precendence_b() -> DynoResult<()> {
        assert_eq!(
            get_statement("return 12 * 4 + 7;")?,
            Return(BinaryOperation(
                Add,
                Box::new(BinaryOperation(
                    Multiply,
                    Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(12))),
                    Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(4))),
                )),
                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(7))),
            ))
        );
        Ok(())
    }

    #[test]
    fn parse_equals_operator() -> DynoResult<()> {
        assert_eq!(
            get_statement("return 1 == 2;")?,
            Return(BinaryOperation(
                Equal,
                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(1))),
                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(2))),
            ))
        );
        Ok(())
    }

    #[test]
    fn parse_simple_declaration() -> DynoResult<()> {
        let ast = get_statement("let a: u32;")?;
        assert_eq!(ast, Declaration("a".to_string(), DynoType::UInt32()));
        Ok(())
    }

    #[test]
    fn parse_simple_boolean() -> DynoResult<()> {
        let ast = get_statement("let a: bool;")?;
        assert_eq!(ast, Declaration("a".to_string(), DynoType::Bool()));
        Ok(())
    }

    #[test]
    fn parser_simple_assignment() -> DynoResult<()> {
        let ast = get_statement("let a: u32; a = 12;")?;

        assert_eq!(
            ast,
            Block(vec![
                Declaration("a".to_string(), DynoType::UInt32()),
                Assignment(
                    "a".to_string(),
                    Widen(
                        Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(12))),
                        DynoType::UInt32()
                    )
                )
            ])
        );
        Ok(())
    }

    #[test]
    fn parser_complex_assignment() -> DynoResult<()> {
        let ast = get_statement("let a: u32; a = 12 - 2 * 4;")?;

        assert_eq!(
            ast,
            Block(vec![
                Declaration("a".to_string(), DynoType::UInt32()),
                Assignment(
                    "a".to_string(),
                    BinaryOperation(
                        BinaryOperationType::Subtract,
                        Box::new(Widen(
                            Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(12))),
                            DynoType::UInt32()
                        )),
                        Box::new(BinaryOperation(
                            BinaryOperationType::Multiply,
                            Box::new(Widen(
                                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(2))),
                                DynoType::UInt32()
                            )),
                            Box::new(Widen(
                                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(4))),
                                DynoType::UInt32()
                            )),
                        ))
                    ),
                )
            ])
        );
        Ok(())
    }

    #[test]
    fn parse_simple_parentheses() -> DynoResult<()> {
        let ast = get_statement("return (12);")?;
        assert_eq!(ast, Return(Literal(DynoType::UInt8(), DynoValue::UInt(12))));
        Ok(())
    }

    #[test]
    fn parse_parentheses_expression() -> DynoResult<()> {
        let ast = get_statement("return (4 + 2) * 3;")?;
        assert_eq!(
            ast,
            Return(BinaryOperation(
                Multiply,
                Box::new(BinaryOperation(
                    Add,
                    Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(4))),
                    Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(2))),
                )),
                Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(3))),
            ))
        );
        Ok(())
    }

    #[test]
    fn parse_simple_if() -> DynoResult<()> {
        let ast = get_statement("if 1 == 2 { return 3; }")?;

        assert_eq!(
            ast,
            If(
                BinaryOperation(
                    BinaryOperationType::Equal,
                    Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(1))),
                    Box::new(Literal(DynoType::UInt8(), DynoValue::UInt(2)))
                ),
                Box::new(Return(Literal(DynoType::UInt8(), DynoValue::UInt(3))))
            )
        );
        Ok(())
    }

    #[test]
    fn parser_reassign_variable_different_scope() -> DynoResult<()> {
        let result = parse(lex("{let a: u8; {let a: u32;}}")?)?;
        assert_eq!(
            result,
            Block(vec![
                Declaration("a".to_owned(), DynoType::UInt8()),
                Declaration("a".to_owned(), DynoType::UInt32())
            ])
        );

        Ok(())
    }

    #[test]
    fn parser_consume_out_of_bounds_error() {
        let mut parser = Parser::new(vec![]);
        let token = parser.consume();

        assert!(token.is_err());
    }

    #[test]
    fn parser_peek_out_of_bounds_error() {
        let parser = Parser::new(vec![]);
        let token = parser.peek();

        assert!(token.is_err());
    }

    #[test]
    fn parser_peek_next_out_of_bounds_error() {
        let parser = Parser::new(vec![Token::with_type(SemiColon)]);
        let token = parser.peek_next(1);

        assert!(token.is_err());
    }

    #[test]
    fn parser_consume_expect_error() {
        let mut parser = Parser::new(vec![Token::with_type(SemiColon)]);
        let token = parser.consume_expect(IntegerLiteral);

        assert!(token.is_err());
    }

    #[test]
    fn parser_integer_literal_error() {
        let mut parser = Parser::new(vec![Token::new(IntegerLiteral, "a")]);
        let node = parser.parse_integer_literal();

        assert!(node.is_err());
    }

    #[test]
    fn parser_unary_expression_error() {
        let mut parser = Parser::new(vec![Token::new(IntegerLiteral, "a")]);
        let node = parser.parse_integer_literal();

        assert!(node.is_err());
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

    #[test]
    fn parser_assign_variable_too_big_error() -> DynoResult<()> {
        let result = parse(lex("{let a: u8; a = 256;}")?);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn parser_reassign_variable() -> DynoResult<()> {
        let result = parse(lex("{let a: u8; let a: u32;}")?);
        assert!(result.is_err());
        Ok(())
    }
}
