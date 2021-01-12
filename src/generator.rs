use crate::ast::{AstNode, BinaryOperationType};
use crate::error::*;
use std::io::BufWriter;
use std::io::Write;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Reg {
    Rax = 0,
    Rcx = 1,
    Rdx = 2,
    Rbx = 3,
    Rsp = 4,
    Rbp = 5,
    Rsi = 6,
    Rdi = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

impl Reg {
    fn is_r(self) -> bool {
        match self {
            Reg::R8 | Reg::R9 | Reg::R10 | Reg::R11 | Reg::R12 | Reg::R13 | Reg::R14 | Reg::R15 => {
                true
            }
            _ => false,
        }
    }
    fn get_r_adapted_value(self) -> u8 {
        if self.is_r() {
            self as u8 - 8
        } else {
            self as u8
        }
    }
}

impl From<usize> for Reg {
    fn from(x: usize) -> Reg {
        use Reg::*;

        match x {
            0 => Rax,
            1 => Rcx,
            2 => Rdx,
            3 => Rbx,
            4 => Rsp,
            5 => Rbp,
            6 => Rsi,
            7 => Rdi,
            8 => R8,
            9 => R9,
            10 => R10,
            11 => R11,
            12 => R12,
            13 => R13,
            14 => R14,
            15 => R15,
            _ => unreachable!(),
        }
    }
}

struct X86Generator {
    writer: BufWriter<Vec<u8>>,
    used_regs: Vec<bool>,
}

const EXPRESSION_REGISTER_OFFSET: usize = 6;

impl X86Generator {
    fn new() -> Self {
        Self {
            writer: BufWriter::new(vec![]),
            used_regs: vec![false; 3],
        }
    }

    fn new_reg(&mut self) -> DynoResult<Reg> {
        for i in 0..self.used_regs.len() {
            if !self.used_regs[i] {
                self.used_regs[i] = true;
                return Ok(Reg::from(i + EXPRESSION_REGISTER_OFFSET));
            }
        }
        Err(DynoError::GeneratorError(
            "Failed to allocate new register".to_string(),
        ))
    }

    fn free_reg(&mut self, reg: Reg) -> DynoResult<()> {
        match self.used_regs[reg as usize - EXPRESSION_REGISTER_OFFSET] {
            true => {
                self.used_regs[reg as usize - EXPRESSION_REGISTER_OFFSET] = false;
                Ok(())
            }
            false => Err(DynoError::GeneratorError(format!(
                "Trying to free reg which isn't used: {:?}",
                reg
            ))),
        }
    }

    fn write(&mut self, data: &[u8]) -> DynoResult<()> {
        match self.writer.write(data) {
            Ok(_) => Ok(()),
            Err(_) => Err(DynoError::X86WriteError()),
        }
    }

    fn write_u8(&mut self, data: u8) -> DynoResult<()> {
        self.write(&[data])
    }

    fn write_u16(&mut self, data: u16) -> DynoResult<()> {
        self.write(&[(data & 0xFF) as u8, ((data >> 8) & 0xFF) as u8])
    }

    fn write_u32(&mut self, data: u32) -> DynoResult<()> {
        self.write(&[
            (data & 0xFF) as u8,
            ((data >> 8) & 0xFF) as u8,
            ((data >> 16) & 0xFF) as u8,
            ((data >> 24) & 0xFF) as u8,
        ])
    }

    fn write_u64(&mut self, data: u64) -> DynoResult<()> {
        self.write(&[
            (data & 0xFF) as u8,
            ((data >> 8) & 0xFF) as u8,
            ((data >> 16) & 0xFF) as u8,
            ((data >> 24) & 0xFF) as u8,
            ((data >> 32) & 0xFF) as u8,
            ((data >> 40) & 0xFF) as u8,
            ((data >> 48) & 0xFF) as u8,
            ((data >> 56) & 0xFF) as u8,
        ])
    }

    fn write_movq_imm_reg(&mut self, value: u64, dst: Reg) -> DynoResult<()> {
        if dst.is_r() {
            self.write(&[0x49, 0xb8 + (dst as u8 - 8)])?;
        } else {
            self.write(&[0x48, 0xb8 + dst as u8])?;
        }
        self.write_u64(value)
    }

    fn write_movq_reg_reg(&mut self, src: Reg, dst: Reg) -> DynoResult<()> {
        let instr_byte_1 = match (src.is_r(), dst.is_r()) {
            (false, false) => 0x48,
            (false, true) => 0x49,
            (true, false) => 0x4c,
            (true, true) => 0x4d,
        };
        self.write(&[
            instr_byte_1,
            0x89,
            0xC0 + (src.get_r_adapted_value() * 8 + dst.get_r_adapted_value()),
        ])
    }

    fn write_addq_reg_reg(&mut self, src: Reg, dst: Reg) -> DynoResult<()> {
        let instr_byte_1 = match (src.is_r(), dst.is_r()) {
            (false, false) => 0x48,
            (false, true) => 0x49,
            (true, false) => 0x4c,
            (true, true) => 0x4d,
        };
        self.write(&[
            instr_byte_1,
            0x01,
            0xC0 + (src.get_r_adapted_value() * 8 + dst.get_r_adapted_value()),
        ])
    }

    fn write_subq_reg_reg(&mut self, src: Reg, dst: Reg) -> DynoResult<()> {
        let instr_byte_1 = match (src.is_r(), dst.is_r()) {
            (false, false) => 0x48,
            (false, true) => 0x49,
            (true, false) => 0x4c,
            (true, true) => 0x4d,
        };
        self.write(&[
            instr_byte_1,
            0x29,
            0xC0 + (src.get_r_adapted_value() * 8 + dst.get_r_adapted_value()),
        ])
    }

    fn write_mulq_reg(&mut self, src: Reg) -> DynoResult<()> {
        match src.is_r() {
            false => self.write(&[0x48, 0xF7, 0xE0 + src as u8]),
            true => self.write(&[0x49, 0xF7, 0xE0 + (src as u8 - 8)]),
        }
    }

    fn write_divq_reg(&mut self, src: Reg) -> DynoResult<()> {
        match src.is_r() {
            false => self.write(&[0x48, 0xF7, 0xf0 + src as u8]),
            true => self.write(&[0x49, 0xF7, 0xf0 + (src as u8 - 8)]),
        }
    }

    fn write_cqto(&mut self) -> DynoResult<()> {
        self.write(&[0x48, 0x99])
    }

    fn write_prologue(&mut self) -> DynoResult<()> {
        self.write(&[0x55, 0x48, 0x89, 0xE5])
    }

    fn write_epilogue(&mut self) -> DynoResult<()> {
        self.write(&[0x48, 0x89, 0xE5, 0x5D, 0xC3])
    }

    fn gen_expression(&mut self, ast: &AstNode) -> DynoResult<Reg> {
        match ast {
            AstNode::IntegerLiteral(value, _) => {
                let reg = self.new_reg()?;
                self.write_movq_imm_reg(*value as u64, reg)?;
                Ok(reg)
            }
            AstNode::BinaryOperation(op_type, left, right) => {
                let left_reg = self.gen_expression(left)?;
                let right_reg = self.gen_expression(right)?;

                match op_type {
                    BinaryOperationType::Add => self.write_addq_reg_reg(right_reg, left_reg)?,
                    BinaryOperationType::Subtract => {
                        self.write_subq_reg_reg(right_reg, left_reg)?
                    }
                    BinaryOperationType::Multiply => {
                        self.write_movq_reg_reg(right_reg, Reg::Rax)?;
                        self.write_mulq_reg(left_reg)?;
                        self.write_movq_reg_reg(Reg::Rax, left_reg)?;
                    }
                    BinaryOperationType::Divide => {
                        self.write_movq_reg_reg(left_reg, Reg::Rax)?;
                        self.write(&[0x99])?;
                        self.write_divq_reg(right_reg)?;
                        self.write_movq_reg_reg(Reg::Rax, left_reg)?;
                    }
                }

                self.free_reg(right_reg)?;

                Ok(left_reg)
            }
            _ => Err(DynoError::GeneratorError(format!(
                "Cannot gen expression for {:?}",
                ast
            ))),
        }
    }

    fn gen_single_node(&mut self, node: &AstNode) -> DynoResult<()> {
        match node {
            AstNode::Return(expression) => {
                let reg = self.gen_expression(expression)?;
                self.write_movq_reg_reg(reg, Reg::Rax)?;
                self.write_epilogue()?;
            }
            AstNode::Block(nodes) => {
                for node in nodes {
                    self.gen_single_node(node)?;
                }
            }
            _ => {
                return Err(DynoError::GeneratorError(format!(
                    "Cannot generate code for {:?}",
                    node,
                )))
            }
        }

        Ok(())
    }

    fn gen(&mut self, ast: &AstNode) -> DynoResult<Vec<u8>> {
        self.write_prologue()?;

        self.gen_single_node(ast)?;

        Ok(self.writer.buffer().to_vec())
    }
}

pub fn gen_assembly(ast: AstNode) -> DynoResult<Vec<u8>> {
    let mut generator = X86Generator::new();
    generator.gen(&ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_new() {
        let _ = X86Generator::new();
    }

    #[test]
    fn generator_write_u8() {
        let mut generator = X86Generator::new();

        generator.write_u8(12).unwrap();
        assert_eq!(generator.writer.buffer(), &[12]);
    }

    #[test]
    fn generator_write_u16() {
        let mut generator = X86Generator::new();

        generator.write_u16(0x1234).unwrap();
        assert_eq!(generator.writer.buffer(), &[0x34, 0x12]);
    }

    #[test]
    fn generator_write_u32() {
        let mut generator = X86Generator::new();

        generator.write_u32(0x12345678).unwrap();
        assert_eq!(generator.writer.buffer(), &[0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn generator_write_u64() {
        let mut generator = X86Generator::new();

        generator.write_u64(0x1234567812345678).unwrap();
        assert_eq!(
            generator.writer.buffer(),
            &[0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12]
        );
    }

    #[test]
    fn generator_write_movq_reg_reg() {
        let mut generator = X86Generator::new();

        generator.write_movq_reg_reg(Reg::R15, Reg::R12).unwrap();
        generator.write_movq_reg_reg(Reg::Rbx, Reg::R12).unwrap();
        generator.write_movq_reg_reg(Reg::R13, Reg::Rsi).unwrap();
        generator.write_movq_reg_reg(Reg::Rcx, Reg::Rdx).unwrap();

        assert_eq!(
            generator.writer.buffer(),
            &[0x4D, 0x89, 0xFC, 0x49, 0x89, 0xDC, 0x4C, 0x89, 0xEE, 0x48, 0x89, 0xCA]
        );
    }

    #[test]
    fn generator_write_single_int_literal() {
        gen_assembly(AstNode::Return(Box::new(AstNode::IntegerLiteral(1234, 8)))).unwrap();
    }
}
