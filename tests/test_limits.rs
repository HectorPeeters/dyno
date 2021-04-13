use dyno::error::DynoResult;
use dyno::generator::compile_and_run;
use dyno::lexer::lex;
use dyno::parser::parse;

fn assert_run(input: &str, value: u64) -> DynoResult<()> {
    println!("{:#?}", parse(lex(input)?)?);
    let result = compile_and_run(&parse(lex(input)?)?)?;
    assert_eq!(result, value);
    Ok(())
}

#[test]
fn limits_u8() -> DynoResult<()> {
    assert_run(
        r"
        let x: u8;
        x = 255 + 1;
        return x; 
               ",
        0,
    )?;

    assert_run(
        r"
        let x: u8;
        x = 0 - 1;
        return x; 
               ",
        255,
    )
}

#[test]
fn limits_u16() -> DynoResult<()> {
    assert_run(
        r"
        let x: u16;
        x = 65535 + 1;
        return x;",
        0,
    )

    //    assert_run(
    //        r"
    //        let x: u16;
    //        x = 0 - 1;
    //        return x;",
    //        65535,
    //    )
}
