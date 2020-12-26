mod ast;
mod elf;
mod error;
mod generator;
mod lexer;
mod parser;
mod types;

fn main() -> error::DynoResult<()> {
    let mut writer = std::fs::File::create("test.out").unwrap();

    let elf_file = elf::ElfFileInfo {
        program_header_table: vec![elf::ElfProgramHeaderEntry {
            segment_type: elf::ElfProgramHeaderEntryType::PtLoad,
            flags: elf::ELF_PROGRAM_FLAG_READ | elf::ELF_PROGRAM_FLAG_EXECUTE,
            offset: 0x00,
            virtual_address: 0x400000,
            physical_address: 0x400000,
            file_size: 0x8C,
            memory_size: 0x8C,
            align: 0x200000,
        }],
        section_header_table: vec![
            elf::NULL_SECTION,
            elf::ElfSectionHeaderEntry {
                name: ".text".to_string(),
                section_type: elf::ElfSectionType::ShtProgBits,
                flags: elf::ELF_SECTION_FLAG_ALLOC | elf::ELF_SECTION_FLAG_EXECINSTR,
                address: 0x400080,
                offset: 0x80,
                size: 0x0C,
                link: 0x00,
                info: 0x00,
                address_align: 0x10,
                entry_size: 0x00,
            },
            elf::ElfSectionHeaderEntry {
                name: ".shstrtab".to_string(),
                section_type: elf::ElfSectionType::ShtStrTab,
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

    elf::write_elf_file(&mut writer, &elf_file)?;

    Ok(())
}
