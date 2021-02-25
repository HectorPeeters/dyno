use dyno::error::DynoResult;
use dyno::generator::gen_assembly;
use dyno::jit::Jit;
use dyno::lexer::lex;
use dyno::parser::parse;

fn assert_run(input: &str, value: u64) -> DynoResult<()> {
    let asm = gen_assembly(parse(lex(input)?)?)?;
    let jit = Jit::new(&asm);
    assert_eq!(jit.run(), value);
    Ok(())
}

#[test]
fn execute_assing_and_return() -> DynoResult<()> {
    assert_run(
        r"
        let x: u32 = 12;
        return x;
               ",
        12,
    )
}
