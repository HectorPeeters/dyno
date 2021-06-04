use crate::ast::{BinaryOperationType, Expression, Statement};
use crate::backend::Backend;
use crate::error::{DynoError, DynoResult};
use crate::types::{DynoType, DynoValue};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::process::Command;
use std::time::SystemTime;

const REG_NAMES: [&'static str; 4] = ["%r8", "%r9", "%r10", "%r11"];

pub struct X86Backend {
    writer: BufWriter<File>,
    regs: [bool; 4],
}

type Register = usize;

impl Backend for X86Backend {
    type Register = Register;

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

    fn generate_expression(&mut self, expression: &Expression) -> DynoResult<Self::Register> {
        match expression {
            Expression::BinaryOperation(op_type, left, right) => {
                self.generate_binop(op_type, left, right)
            }
            Expression::Literal(value_type, value) => self.generate_literal(value_type, value),
            Expression::Widen(expression, value_type) => {
                self.generate_widen(expression, value_type)
            }
            Expression::Identifier(name) => self.generate_identifier(name),
        }
    }
}

impl X86Backend {
    fn new(file_name: &str) -> Self {
        Self {
            writer: BufWriter::new(File::create(file_name).unwrap()),
            regs: [false; 4],
        }
    }

    fn allocate_reg(&mut self) -> DynoResult<Register> {
        for (i, reg) in self.regs.iter().enumerate() {
            if !reg {
                self.regs[i] = true;
                return Ok(i);
            }
        }

        Err(DynoError::GeneratorError(
            "All registers are allocated".to_string(),
        ))
    }

    fn deallocate_reg(&mut self, reg: Register) -> DynoResult<()> {
        if !self.regs[reg] {
            return Err(DynoError::GeneratorError(
                "Trying to free a register which is not used".to_string(),
            ));
        }

        self.regs[reg] = false;
        Ok(())
    }

    fn finish(&mut self) -> DynoResult<()> {
        self.writer.flush()?;
        Ok(())
    }

    fn generate_header(&mut self) -> DynoResult<()> {
        writeln!(self.writer, ".globl main")?;
        writeln!(self.writer, ".text")?;
        writeln!(self.writer, "main:")?;
        Ok(())
    }

    fn generate_binop(
        &mut self,
        op_type: &BinaryOperationType,
        left: &Expression,
        right: &Expression,
    ) -> DynoResult<Register> {
        use BinaryOperationType::*;

        let left = self.generate_expression(left)?;
        let right = self.generate_expression(right)?;

        match op_type {
            Add => writeln!(self.writer, "add {}, {}", REG_NAMES[left], REG_NAMES[right])?,
            _ => todo!(),
        }
        self.deallocate_reg(left)?;
        Ok(right)
    }

    fn generate_literal(
        &mut self,
        value_type: &DynoType,
        value: &DynoValue,
    ) -> DynoResult<Register> {
        use crate::types::DynoType::*;
        use crate::types::DynoValue::*;

        let reg = self.allocate_reg()?;

        match (value_type, value) {
            (UInt8(), UInt(x)) => writeln!(self.writer, "mov ${}, {}", x, REG_NAMES[reg])?,
            (UInt16(), UInt(x)) => writeln!(self.writer, "mov ${}, {}", x, REG_NAMES[reg])?,
            (UInt32(), UInt(x)) => writeln!(self.writer, "mov ${}, {}", x, REG_NAMES[reg])?,
            (UInt64(), UInt(x)) => writeln!(self.writer, "mov ${}, {}", x, REG_NAMES[reg])?,
            _ => {
                return Err(DynoError::GeneratorError(format!(
                    "Failed to generate literal for {:?}, {:?}",
                    value_type, value,
                )))
            }
        }

        Ok(0)
    }

    fn generate_widen(
        &mut self,
        expression: &Expression,
        value_type: &DynoType,
    ) -> DynoResult<Register> {
        Ok(0)
    }

    fn generate_identifier(&mut self, name: &str) -> DynoResult<Register> {
        Ok(0)
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
        let reg = self.generate_expression(expression)?;

        writeln!(self.writer, "mov {}, %rax", REG_NAMES[reg])?;
        writeln!(self.writer, "ret")?;

        self.deallocate_reg(reg)
    }

    fn generate_block(&mut self, children: &[Statement]) -> DynoResult<()> {
        for child in children {
            self.generate_statement(child)?;
        }
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
    std::fs::create_dir_all("target/x86")?;

    //TODO: replace this with a hash or something
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let assembly_file = format!("target/x86/{}.s", time);

    let mut backend = X86Backend::new(&assembly_file);
    backend.generate_header()?;
    backend.generate_statement(ast)?;
    backend.finish()?;

    let executable = format!("target/x86/{}.out", time);

    let compile_status = Command::new("cc")
        .arg(&assembly_file)
        .arg("-o")
        .arg(&executable)
        .status()?;

    if compile_status.code().unwrap() != 0 {
        return Err(DynoError::GeneratorError(
            "Failed to compile assembly".to_string(),
        ));
    }

    let status = Command::new(&executable).status()?;

    Ok(status.code().unwrap() as u64)
}
