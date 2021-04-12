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
    assert_run("let a: u32; a = 24; if 1 == 1 { a = 42; } return a;", 42)
}

#[test]
fn execute_simple_if_false() -> DynoResult<()> {
    assert_run("let a: u32; a = 24; if 1 == 0 { a = 42; } return a;", 24)
}
