use crate::error::*;
use std::io::Write;

/// An enum representing all possible ELF file types.
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

/// Struct used to generate an ELF file.
///
/// The `program_header_table`, `section_header_table` and `code` field will written to the output
/// file.
pub struct ElfFileInfo {
    pub program_header_table: Vec<ElfProgramHeaderEntry>,
    pub section_header_table: Vec<ElfSectionHeaderEntry>,
    pub code: Vec<u8>,
}

impl ElfFileInfo {
    /// Returns a byte array containing the names of all sections
    pub fn get_names(&self) -> DynoResult<Vec<u8>> {
        let mut writer = std::io::BufWriter::new(vec![]);

        for section in &self.section_header_table {
            write(&mut writer, section.name.as_bytes())?;
            write(&mut writer, &[0x00])?;
        }

        Ok(writer.buffer().to_vec())
    }

    /// Calculates the offset of a section name in the name byte array
    pub fn get_name_offset(&self, section_index: usize) -> u32 {
        if self.section_header_table[section_index].name.is_empty() {
            return 0;
        }

        let mut result: u32 = 1;
        for i in 0..section_index {
            if !self.section_header_table[i].name.is_empty() {
                // Add the size of the name plus one for a null byte
                result += self.section_header_table[i].name.len() as u32 + 1;
            }
        }

        result
    }
}

/// An enum representing the type of an ELF program header entry.
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

pub const ELF_PROGRAM_FLAG_EXECUTE: u32 = 0x01;
pub const ELF_PROGRAM_FLAG_WRITE: u32 = 0x02;
pub const ELF_PROGRAM_FLAG_READ: u32 = 0x04;

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

pub const ELF_SECTION_FLAG_WRITE: u64 = 0x01;
pub const ELF_SECTION_FLAG_ALLOC: u64 = 0x02;
pub const ELF_SECTION_FLAG_EXECINSTR: u64 = 0x04;
pub const ELF_SECTION_FLAG_MERGE: u64 = 0x10;
pub const ELF_SECTION_FLAG_STRINGS: u64 = 0x20;
pub const ELF_SECTION_FLAG_INFO_LINK: u64 = 0x40;
pub const ELF_SECTION_FLAG_OS_NONCONFORMING: u64 = 0x100;
pub const ELF_SECTION_FLAG_GROUP: u64 = 0x200;
pub const ELF_SECTION_FLAG_TLS: u64 = 0x400;
pub const ELF_SECTION_FLAG_MASKOS: u64 = 0x0FF00000;
pub const ELF_SECTION_FLAG_MASKPROC: u64 = 0xF0000000;
pub const ELF_SECTION_FLAG_ORDERED: u64 = 0x40000000;
pub const ELF_SECTION_FLAG_EXCLUDE: u64 = 0x80000000;

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

/// The NULL section added to every ELF program.
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

/// Helper function to write a byte array.
///
/// This function tries to write a byte array using a Write trait. If the write fails it returns a
/// DynoError::ElfWriteError. If it succeeds, it returns an empty Ok value.
fn write(writer: &mut dyn Write, data: &[u8]) -> DynoResult<()> {
    match writer.write(data) {
        Ok(_) => Ok(()),
        Err(_) => Err(DynoError::ElfWriteError()),
    }
}

/// Writes the first part of the ELF header.
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
            + file_info.get_names()?.len() as u64
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

/// Writes the ELF program header.
fn write_elf_program_header<T>(writer: &mut T, elf_file: &ElfFileInfo) -> DynoResult<()>
where
    T: Write,
{
    for program in &elf_file.program_header_table {
        // the segment type
        write(writer, &(program.segment_type as u32).to_le_bytes())?;

        // the flags of this segment
        write(writer, &program.flags.to_le_bytes())?;

        // the segment offset
        write(writer, &program.offset.to_le_bytes())?;

        // the virtual address of the segment
        write(writer, &program.virtual_address.to_le_bytes())?;

        // the physical address of the segment, this is often the same as the virtual address
        write(writer, &program.physical_address.to_le_bytes())?;

        // the size of the segment in the file
        write(writer, &program.file_size.to_le_bytes())?;

        // the size of the segment in memory, this is often the same as the file_size
        write(writer, &program.memory_size.to_le_bytes())?;

        // the alignment of the segment
        write(writer, &program.align.to_le_bytes())?;
    }

    Ok(())
}

/// Writes the ELF section header.
fn write_elf_section_header<T>(writer: &mut T, elf_file: &ElfFileInfo) -> DynoResult<()>
where
    T: Write,
{
    for (index, section) in elf_file.section_header_table.iter().enumerate() {
        // calculate the starting index in the .shstrtab section
        let name_index: u32 = elf_file.get_name_offset(index);
        write(writer, &name_index.to_le_bytes())?;

        // the section type
        write(writer, &(section.section_type as u32).to_le_bytes())?;

        // the flags for this section
        write(writer, &section.flags.to_le_bytes())?;

        // the address of this section
        write(writer, &section.address.to_le_bytes())?;

        // the offset of this section
        write(writer, &section.offset.to_le_bytes())?;

        // the size of the section
        write(writer, &section.size.to_le_bytes())?;

        // an optional link to another section
        write(writer, &section.link.to_le_bytes())?;

        // the optional info of the section
        write(writer, &section.info.to_le_bytes())?;

        // the alignment of the addresses in the section
        write(writer, &section.address_align.to_le_bytes())?;

        // the size in bytes of each entry
        write(writer, &section.entry_size.to_le_bytes())?;
    }

    Ok(())
}

/// Writes a full ELF file to using a Write trait writer.
pub fn write_elf_file<T>(writer: &mut T, elf_file: &ElfFileInfo) -> DynoResult<()>
where
    T: Write,
{
    // write the first part of the header
    write_elf_header_1(writer, elf_file)?;

    // write the program table header
    write_elf_program_header(writer, elf_file)?;

    // write the padding
    write(writer, &[0; 8])?;

    // write the actual code of the executabe
    write(writer, &elf_file.code)?;

    // writes the names of all the sections
    write(writer, &elf_file.get_names()?)?;

    // writes the section table header
    write_elf_section_header(writer, elf_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elf_write_full_file() {
        let mut writer = std::io::BufWriter::new(vec![]);

        let elf_file = ElfFileInfo {
            program_header_table: vec![ElfProgramHeaderEntry {
                segment_type: ElfProgramHeaderEntryType::PtLoad,
                flags: ELF_PROGRAM_FLAG_READ | ELF_PROGRAM_FLAG_EXECUTE,
                offset: 0x00,
                virtual_address: 0x400000,
                physical_address: 0x400000,
                file_size: 0x8C,
                memory_size: 0x8C,
                align: 0x200000,
            }],
            section_header_table: vec![
                NULL_SECTION,
                ElfSectionHeaderEntry {
                    name: ".text".to_string(),
                    section_type: ElfSectionType::ShtProgBits,
                    flags: ELF_SECTION_FLAG_ALLOC | ELF_SECTION_FLAG_EXECINSTR,
                    address: 0x400080,
                    offset: 0x80,
                    size: 0x0C,
                    link: 0x00,
                    info: 0x00,
                    address_align: 0x10,
                    entry_size: 0x00,
                },
                ElfSectionHeaderEntry {
                    name: ".shstrtab".to_string(),
                    section_type: ElfSectionType::ShtStrTab,
                    flags: 0x00,
                    address: 0x00,
                    offset: 0x8C,
                    size: 0x11,
                    link: 0x00,
                    info: 0x00,
                    address_align: 0x01,
                    entry_size: 0x00,
                },
            ],
            code: vec![
                0xB8, 0x01, 0x00, 0x00, 0x00, 0xBB, 0x2A, 0x00, 0x00, 0x00, 0xCD, 0x80,
            ],
        };

        write_elf_file(&mut writer, &elf_file).unwrap();
    }
}
