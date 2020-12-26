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

pub struct ElfFileInfo {
    pub program_header_table: Vec<ElfProgramHeaderEntry>,
    pub section_header_table: Vec<ElfSectionHeaderEntry>,
    pub code: Vec<u8>,
}

impl ElfFileInfo {
    pub fn get_names(&self) -> Vec<u8> {
        let mut writer = std::io::BufWriter::new(vec![]);

        for section in &self.section_header_table {
            write(&mut writer, section.name.as_bytes());
            write(&mut writer, &[0x00]);
        }

        writer.buffer().to_vec()
    }

    pub fn get_name_offset(&self, section_index: usize) -> u32 {
        if self.section_header_table[section_index].name.len() == 0 {
            return 0;
        }

        let mut result: u32 = 1;
        for i in 0..section_index {
            if self.section_header_table[i].name.len() != 0 {
                result += self.section_header_table[i].name.len() as u32 + 1;
            }
        }

        result
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ElfProgramHeaderEntryType {
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

pub struct ElfProgramHeaderEntry {
    pub segment_type: ElfProgramHeaderEntryType,
    pub flags: u32,
    pub offset: u64,
    pub virtual_address: u64,
    pub physical_address: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub align: u64,
}

#[derive(Debug, Copy, Clone)]
pub enum ElfSectionType {
    ShtNull = 0x00,
    ShtProgBits = 0x01,
    ShtSymTab = 0x02,
    ShtStrTab = 0x03,
    ShtRela = 0x04,
    ShtHash = 0x05,
    ShtDynamic = 0x06,
    ShtNote = 0x07,
    ShtNoBits = 0x08,
    ShtRel = 0x09,
    ShtShLib = 0x0A,
    ShtDynSym = 0x0B,
    ShtInitArray = 0x0E,
    ShtFiniArray = 0x0F,
    ShtPreInitArray = 0x10,
    ShtGroup = 0x11,
    ShtSymTabShndx = 0x12,
    ShtNum = 0x13,
    ShtLoos = 0x60000000,
}

pub struct ElfSectionHeaderEntry {
    pub name: String,
    pub section_type: ElfSectionType,
    pub flags: u64,
    pub address: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub address_align: u64,
    pub entry_size: u64,
}

fn write(writer: &mut dyn Write, data: &[u8]) -> DynoResult<()> {
    match writer.write(data) {
        Ok(_) => Ok(()),
        Err(_) => Err(DynoError::ElfWriteError()),
    }
}

fn write_elf_header_1<T>(writer: &mut T, file_info: &ElfFileInfo) -> DynoResult<()>
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

    const PROGRAM_TABLE_ENTRY_SIZE: u16 = 56;
    let program_header_size =
        PROGRAM_TABLE_ENTRY_SIZE * file_info.program_header_table.len() as u16;

    const SECTION_TABLE_ENTRY_SIZE: u16 = 64;
    let section_header_size =
        SECTION_TABLE_ENTRY_SIZE * file_info.section_header_table.len() as u16;

    // elf type
    let elf_type = ElfType::EtExec;
    write(writer, &(elf_type as u16).to_le_bytes())?;

    // machine
    write(writer, &(0x3e as u16).to_le_bytes())?;

    // version
    write(writer, &(0x01 as u32).to_le_bytes())?;

    // entry
    write(writer, &(0x400080 as u64).to_le_bytes())?;
    // program header offset
    write(writer, &(0x40 as u64).to_le_bytes())?;

    // section table offset
    write(
        writer,
        &(0x40
            + program_header_size as u64
            + file_info.code.len() as u64
            + file_info.get_names().len() as u64
            + 8)
        .to_le_bytes(),
    )?;

    // flags
    write(writer, &(0x0 as u32).to_le_bytes())?;

    // header size
    write(writer, &(0x40 as u16).to_le_bytes())?;

    // program header table size
    write(writer, &PROGRAM_TABLE_ENTRY_SIZE.to_le_bytes())?;

    // program header entry num
    write(
        writer,
        &(file_info.program_header_table.len() as u16).to_le_bytes(),
    )?;

    // section header entry size
    write(writer, &SECTION_TABLE_ENTRY_SIZE.to_le_bytes())?;

    // section header entry num
    write(
        writer,
        &(file_info.section_header_table.len() as u16).to_le_bytes(),
    )?;

    // section name header table entry
    write(writer, &(0x02 as u16).to_le_bytes())?;

    Ok(())
}

fn write_elf_program_header<T>(writer: &mut T, elf_file: &ElfFileInfo) -> DynoResult<()>
where
    T: Write,
{
    for program in &elf_file.program_header_table {
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

pub const NULL_SECTION: ElfSectionHeaderEntry = ElfSectionHeaderEntry {
    name: String::new(),
    section_type: ElfSectionType::ShtNull,
    flags: 0,
    address: 0,
    offset: 0,
    size: 0,
    link: 0,
    info: 0,
    address_align: 0,
    entry_size: 0,
};

fn write_elf_section_header<T>(writer: &mut T, elf_file: &ElfFileInfo) -> DynoResult<()>
where
    T: Write,
{
    for (index, section) in elf_file.section_header_table.iter().enumerate() {
        let name_index: u32 = elf_file.get_name_offset(index);
        println!("{}", name_index);
        write(writer, &name_index.to_le_bytes())?;

        write(writer, &(section.section_type as u32).to_le_bytes())?;

        write(writer, &section.flags.to_le_bytes())?;

        write(writer, &section.address.to_le_bytes())?;

        write(writer, &section.offset.to_le_bytes())?;

        write(writer, &section.size.to_le_bytes())?;

        write(writer, &section.link.to_le_bytes())?;

        write(writer, &section.info.to_le_bytes())?;

        write(writer, &section.address_align.to_le_bytes())?;

        write(writer, &section.entry_size.to_le_bytes())?;
    }

    Ok(())
}

pub fn write_elf_file<T>(writer: &mut T, elf_file: &ElfFileInfo) -> DynoResult<()>
where
    T: Write,
{
    write_elf_header_1(writer, elf_file)?;
    write_elf_program_header(writer, elf_file)?;

    write(writer, &[0; 8])?;

    write(writer, &elf_file.code)?;

    write(writer, &elf_file.get_names())?;

    write_elf_section_header(writer, elf_file)?;

    Ok(())
}
