use crate::ast::AstNode;

pub trait Generator {
    fn gen_assembly(ast: AstNode) -> Vec<String>;
}

pub struct X86Generator;

impl Generator for X86Generator {
    fn gen_assembly(ast: AstNode) -> Vec<String> {
        vec![]
    }
}
