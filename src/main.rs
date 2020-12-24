mod ast;
mod error;
mod lexer;
mod parser;

fn main() -> error::DynoResult<()> {
    let tokens = lexer::lex("12 + 9 / 4")?;

    println!("Tokens:");
    for token in &tokens {
        println!("{:?}", token);
    }

    println!("\nAst:");
    println!("{:#?}", parser::parse(tokens)?);

    Ok(())
}
