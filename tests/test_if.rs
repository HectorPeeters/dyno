use dyno::error::DynoResult;
use dyno::generator::gen_assembly;
use dyno::jit::Jit;
use dyno::lexer::lex;
use dyno::parser::parse;

fn assert_run(input: &str, value: u64) -> DynoResult<()> {
    let asm = gen_assembly(parse(lex(input)?)?)?;
    println!("{:02x?}", asm);
    let jit = Jit::new(&asm);
    assert_eq!(jit.run(), value);
    Ok(())
}

#[test]
fn execute_simple_if() -> DynoResult<()> {
    assert_run("if 1 == 1 { return 42; } return 24;", 42)
}

#[test]
fn execute_simple_if_false() -> DynoResult<()> {
    assert_run("if 1 == 0 { return 42; } return 24;", 24)
}
