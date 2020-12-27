mod ast;
mod elf;
mod error;
mod generator;
mod jit;
mod lexer;
mod parser;
mod types;

fn main() -> error::DynoResult<()> {
    Ok(())
}
