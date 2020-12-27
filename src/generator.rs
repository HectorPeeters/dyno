use crate::ast::AstNode;
use std::io::BufWriter;

struct X86Generator {
    writer: BufWriter<Vec<u8>>,
    ast: AstNode,
}

impl X86Generator {
    fn new(ast: AstNode) -> Self {
        Self {
            writer: BufWriter::new(vec![]),
            ast,
        }
    }

    fn gen(&self) -> Vec<u8> {
        vec![]
    }
}

pub fn gen_assembly(ast: AstNode) -> Vec<u8> {
    let generator = X86Generator::new(ast);
    generator.gen()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parse;

    fn get_generator(input: &str) -> X86Generator {
        X86Generator::new(parse(lex(input).unwrap()).unwrap())
    }

    #[test]
    fn generator_new() {
        let _ = get_generator("1 + 12");
    }
}
