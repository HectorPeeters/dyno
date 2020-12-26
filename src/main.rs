mod ast;
mod elf;
mod error;
mod generator;
mod lexer;
mod parser;
mod types;

fn main() -> error::DynoResult<()> {
    let mut writer = std::fs::File::create("test.out").unwrap();
    elf::write_elf_file(&mut writer, &vec![], &vec![elf::NULL_SECTION], &[]);

    Ok(())
}
