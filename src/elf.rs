use crate::error::*;
use std::io::Write;

#[derive(Debug, Copy, Clone)]
pub enum ElfType {
    EtNone = 0x00,
    EtRel = 0x01,
    EtExec = 0x02,
    EtDyn = 0x03,
    EtCore = 0x04,
    EtLoos = 0xFE00,
    EtHios = 0xFEFF,
    EtLoProc = 0xFF00,
    EtHiProc = 0xFFFF,
}

fn write(writer: &mut dyn Write, data: &[u8]) -> DynoResult<()> {
    match writer.write(data) {
        Ok(_) => Ok(()),
        Err(_) => Err(DynoError::ElfWriteError()),
    }
}

fn write_elf_header_ident<T>(writer: &mut T) -> DynoResult<()>
where
    T: Write,
{
    // ELF magic number
    write(writer, &[0x7F, 0x45, 0x4C, 0x46])?;

    // 32 (0x01)  or 64 (0x02) bit
    write(writer, &[0x02])?;

    // little or big endianness
    write(writer, &[0x01])?;

    // one for current ELF version
    write(writer, &[0x01])?;

    // target os
    write(writer, &[0x00])?;

    // abi and pad
    write(writer, &[0x00; 8])?;

    Ok(())
}

fn write_elf_header_other<T>(writer: &mut T, header_table: Vec<ElfHeaderEntry>) -> DynoResult<()>
where
    T: Write,
{
    // elf type
    let elf_type = ElfType::EtDyn;
    write(writer, &(elf_type as u16).to_le_bytes())?;

    // machine
    write(writer, &(0x3e as u16).to_le_bytes())?;

    // version
    write(writer, &(0x01 as u32).to_le_bytes())?;

    // entry
    write(writer, &(0x550 as u64).to_le_bytes())?;

    // header table offset
    write(writer, &(0x40 as u64).to_le_bytes())?;

    // section table offset
    write(writer, &(0x1978 as u64).to_le_bytes())?;

    // flags
    write(writer, &(0x0 as u32).to_le_bytes())?;

    // header size
    write(writer, &(0x40 as u16).to_le_bytes())?;

    // program header table size
    const HEADER_TABLE_ENTRY_SIZE: u16 = 56;
    write(
        writer,
        &(HEADER_TABLE_ENTRY_SIZE * header_table.len() as u16).to_le_bytes(),
    )?;

    // program header entry num
    write(writer, &(header_table.len() as u16).to_le_bytes())?;

    // section header table size
    write(writer, &(0x40 as u16).to_le_bytes())?;

    // section header entry num
    write(writer, &(0x1D as u16).to_le_bytes())?;

    // section name header table entry
    write(writer, &(0x1C as u16).to_le_bytes())?;

    Ok(())
}

enum ElfHeaderEntryType {
    PtNull = 0x00,
    PtLoad = 0x01,
    PtDynamic = 0x02,
    PtInterp = 0x03,
    PtNote = 0x04,
    PtShlib = 0x05,
    PtPhdr = 0x06,
    PtLts = 0x07,
    PtLoOs = 0x60000000,
    PtHiOs = 0x6FFFFFFF,
    PtLoProc = 0x70000000,
    PtHiProc = 0x7FFFFFFF,
}

struct ElfHeaderEntry {
    pub segment_type: ElfHeaderEntryType,
    pub flags: u32,
    pub offset: u64,
    pub virtual_address: u64,
    pub physical_address: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub align: u64,
}

fn write_elf_header_table<T>(writer: &mut T, programs: Vec<ElfHeaderEntry>) -> DynoResult<()>
where
    T: Write,
{
    for program in programs {
        write(writer, &(program.segment_type as u32).to_le_bytes())?;

        write(writer, &program.flags.to_le_bytes())?;

        write(writer, &program.offset.to_le_bytes())?;

        write(writer, &program.virtual_address.to_le_bytes())?;

        write(writer, &program.physical_address.to_le_bytes())?;

        write(writer, &program.file_size.to_le_bytes())?;

        write(writer, &program.memory_size.to_le_bytes())?;

        write(writer, &program.align.to_le_bytes())?;
    }

    Ok(())
}

pub fn write_elf_header<T>(writer: &mut T) -> DynoResult<()>
where
    T: Write,
{
    let header_table: Vec<ElfHeaderEntry> = vec![];
    write_elf_header_ident(writer)?;
    write_elf_header_other(writer, header_table)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_write_elf_header_ident() {
        let mut writer = BufWriter::new(Vec::<u8>::new());
        write_elf_header_ident(&mut writer).unwrap();

        assert_eq!(
            writer.buffer(),
            &[
                0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00
            ]
        );
        assert_eq!(writer.buffer().len(), 16);
    }

    #[test]
    fn test_write_elf_header_other() {
        let mut writer = BufWriter::new(Vec::<u8>::new());
        write_elf_header_other(&mut writer, vec![]).unwrap();

        assert_eq!(
            writer.buffer(),
            &[
                0x03, 0x00, 0x3e, 0x00, 0x01, 0x00, 0x00, 0x00, 0x50, 0x05, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x19, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x40, 0x00, 0x1D, 0x00, 0x1C, 0x00
            ]
        );
        assert_eq!(writer.buffer().len(), 48);
    }

    #[test]
    fn test_write_elf_header() {
        let mut writer = BufWriter::new(Vec::<u8>::new());
        write_elf_header(&mut writer).unwrap();

        assert_eq!(writer.buffer().len(), 64);
    }
}
