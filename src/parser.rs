use crate::ast::AstNode;
use crate::error::DynoResult;
use crate::lexer::Token;

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn peek_next(&mut self, index: usize) -> &Token {
        &self.tokens[self.index + index]
    }

    pub fn consume(&mut self) -> &Token {
        let result = &self.tokens[self.index];
        self.index += 1;
        result
    }
}

pub fn parse(input: Vec<Token>) -> DynoResult<AstNode> {
    Ok(AstNode::Empty())
}
