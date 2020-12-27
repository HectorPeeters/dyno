use crate::ast::AstNode;
use crate::error::*;
use std::io::BufWriter;
use std::io::Write;

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

    fn write_prologue(&mut self) -> DynoResult<()> {
        self.write(&[0x55, 0x48, 0x89, 0xE5])?;
        Ok(())
    }

    fn write_epilogue(&mut self) -> DynoResult<()> {
        self.write(&[0x48, 0x89, 0xE5, 0x5D, 0xC3])?;
        Ok(())
    }

    fn gen(&mut self) -> DynoResult<Vec<u8>> {
        self.write_prologue()?;
        self.write_u8(0xB8)?;
        self.write_u32(0x37)?;
        self.write_epilogue()?;

        println!("{:?}", self.writer.buffer());
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
