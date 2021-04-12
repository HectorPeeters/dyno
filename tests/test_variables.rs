use dyno::error::DynoResult;
use dyno::generator::compile_and_run;
use dyno::lexer::lex;
use dyno::parser::parse;

fn assert_run(input: &str, value: u64) -> DynoResult<()> {
    let result = compile_and_run(&parse(lex(input)?)?)?;
    assert_eq!(result, value);
    Ok(())
}

#[test]
fn execute_declare_and_assign() -> DynoResult<()> {
    assert_run(
        r"
        let x: u32;
        x = 13;
        return 12;
               ",
        12,
    )
}
