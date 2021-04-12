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
fn execute_simple_if() -> DynoResult<()> {
    assert_run("if 1 == 1 { return 42; } return 24;", 42)
}

#[test]
fn execute_simple_if_false() -> DynoResult<()> {
    assert_run("if 1 == 0 { return 42; } return 24;", 24)
}
