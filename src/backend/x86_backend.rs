use crate::ast::{Expression, Statement};
use crate::backend::Backend;
use crate::error::DynoResult;
use crate::types::DynoType;
use std::fs::File;
use std::io::BufWriter;

pub struct X86Backend {
    writer: BufWriter<File>,
}

impl Backend for X86Backend {
    fn generate_statement(&mut self, statement: &Statement) -> DynoResult<()> {
        match statement {
            Statement::If(condition, true_statement) => self.generate_if(condition, true_statement),
            Statement::While(condition, body) => self.generate_while(condition, body),
            Statement::Return(x) => self.generate_return(x),
            Statement::Block(children) => {
                for child in children {
                    self.generate_statement(&child)?;
                }
                Ok(())
            }
            Statement::Declaration(name, value_type) => self.generate_declaration(name, value_type),
            Statement::Assignment(name, expression) => self.generate_assignment(name, expression),
        }
    }
}

impl X86Backend {
    fn new() -> Self {
        Self {
            writer: BufWriter::new(File::create("output.s").unwrap()),
        }
    }

    fn generate_if(
        &mut self,
        condition: &Expression,
        true_statement: &Statement,
    ) -> DynoResult<()> {
        Ok(())
    }

    fn generate_while(&mut self, condition: &Expression, body: &Statement) -> DynoResult<()> {
        Ok(())
    }

    fn generate_return(&mut self, expression: &Expression) -> DynoResult<()> {
        Ok(())
    }

    fn generate_block(&mut self, children: &[Statement]) -> DynoResult<()> {
        Ok(())
    }

    fn generate_declaration(&mut self, name: &str, value_type: &DynoType) -> DynoResult<()> {
        Ok(())
    }

    fn generate_assignment(&mut self, name: &str, expression: &Expression) -> DynoResult<()> {
        Ok(())
    }
}

pub fn compile_and_run(ast: &Statement) -> DynoResult<u64> {
    Ok(0)
}
