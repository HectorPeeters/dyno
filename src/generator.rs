use crate::ast::AstNode;
use crate::error::*;
use std::io::BufWriter;
use std::io::Write;

struct X86Generator {
    writer: BufWriter<Vec<u8>>,
    ast: AstNode,
}

#[derive(Debug, Copy, Clone)]
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

impl X86Generator {
    fn new(ast: AstNode) -> Self {
        Self {
            writer: BufWriter::new(vec![]),
            ast,
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
        self.write(&[((data >> 0) & 0xFF) as u8, ((data >> 8) & 0xFF) as u8])
    }

    fn write_u32(&mut self, data: u32) -> DynoResult<()> {
        self.write(&[
            ((data >> 0) & 0xFF) as u8,
            ((data >> 8) & 0xFF) as u8,
            ((data >> 16) & 0xFF) as u8,
            ((data >> 24) & 0xFF) as u8,
        ])
    }

    fn write_u64(&mut self, data: u64) -> DynoResult<()> {
        self.write(&[
            ((data >> 0) & 0xFF) as u8,
            ((data >> 8) & 0xFF) as u8,
            ((data >> 16) & 0xFF) as u8,
            ((data >> 24) & 0xFF) as u8,
            ((data >> 32) & 0xFF) as u8,
            ((data >> 40) & 0xFF) as u8,
            ((data >> 48) & 0xFF) as u8,
            ((data >> 56) & 0xFF) as u8,
        ])
    }

    fn write_movq_imm(&mut self, value: u64, reg: Reg) -> DynoResult<()> {
        if (reg as u8) < (Reg::R8 as u8) {
            self.write(&[0x48, 0xb8 + reg as u8])?;
        } else {
            self.write(&[0x49, 0xb8 + reg as u8])?;
        }
        self.write_u64(value)
    }

    fn write_prologue(&mut self) -> DynoResult<()> {
        self.write(&[0x55, 0x48, 0x89, 0xE5])
    }

    fn write_epilogue(&mut self) -> DynoResult<()> {
        self.write(&[0x48, 0x89, 0xE5, 0x5D, 0xC3])
    }

    fn gen(&mut self) -> DynoResult<Vec<u8>> {
        self.write_prologue()?;
        self.write_movq_imm(0x1234, Reg::Rax);
        self.write_epilogue()?;

        Ok(self.writer.buffer().to_vec())
    }
}

pub fn gen_assembly(ast: AstNode) -> DynoResult<Vec<u8>> {
    let mut generator = X86Generator::new(ast);
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

    #[test]
    fn generator_write_u8() {
        let mut generator = get_generator("");

        generator.write_u8(12).unwrap();
        assert_eq!(generator.writer.buffer(), &[12]);
    }

    #[test]
    fn generator_write_u16() {
        let mut generator = get_generator("");

        generator.write_u16(0x1234).unwrap();
        assert_eq!(generator.writer.buffer(), &[0x34, 0x12]);
    }

    #[test]
    fn generator_write_u32() {
        let mut generator = get_generator("");

        generator.write_u32(0x12345678).unwrap();
        assert_eq!(generator.writer.buffer(), &[0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn generator_write_u64() {
        let mut generator = get_generator("");

        generator.write_u64(0x1234567812345678).unwrap();
        assert_eq!(
            generator.writer.buffer(),
            &[0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12]
        );
    }
}
