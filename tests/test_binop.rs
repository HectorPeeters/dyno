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
fn execute_single_int() -> DynoResult<()> {
    assert_run("return 42;", 42)
}

#[test]
fn execute_add_expression() -> DynoResult<()> {
    assert_run("return 42 + 12;", 54)
}

#[test]
fn execute_subtract_expression() -> DynoResult<()> {
    assert_run("return 42 - 12;", 30)
}

#[test]
fn execute_add_subtract_expression() -> DynoResult<()> {
    assert_run("return 42 - 12 + 12 - 5 + 2284;", 2321)
}

#[test]
fn execute_multiply_expression() -> DynoResult<()> {
    assert_run("return 2 * 4 * 3;", 24)
}

#[test]
fn execute_divide_expression() -> DynoResult<()> {
    assert_run("return 16 / 4 / 2;", 2)
}

#[test]
fn execute_complete_expression() -> DynoResult<()> {
    assert_run("return 12 / 3 + 7 * 8 - 10 / 2 * 4;", 40)
}
