#[derive(Debug, PartialEq)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    BinaryOperation(Box<AstNode>, Box<AstNode>, BinaryOperation),
    IntegerLiteral(u128),
    Empty(),
}
