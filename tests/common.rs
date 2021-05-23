use dyno::backend::x86_backend::compile_and_run;
use dyno::error::DynoResult;
use dyno::lexer::lex;
use dyno::parser::parse;

pub fn assert_run(input: &str, value: u64) -> DynoResult<()> {
    let result = compile_and_run(&parse(lex(input)?)?)?;
    assert_eq!(result, value);
    Ok(())
}
