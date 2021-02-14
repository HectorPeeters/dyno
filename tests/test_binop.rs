use dyno::error::DynoResult;
use dyno::jit::Jit;
use dyno::lexer::lex;
use dyno::parser::parse;
use dyno::generator::gen_assembly;

fn get_asm(input: &str) -> DynoResult<Vec<u8>> {
    gen_assembly(parse(lex(input)?)?)
}

#[test]
fn jit_execute_single_int() -> DynoResult<()> {
    let jit = Jit::new(&get_asm("return 42;")?);
    assert_eq!(jit.run(), 42);
    Ok(())
}

#[test]
fn jit_execute_add_expression() -> DynoResult<()> {
    let jit = Jit::new(&get_asm("return 42 + 12;")?);
    assert_eq!(jit.run(), 54);
    Ok(())
}

#[test]
fn jit_execute_subtract_expression() -> DynoResult<()> {
    let jit = Jit::new(&get_asm("return 42 - 12;")?);
    assert_eq!(jit.run(), 30);
    Ok(())
}

#[test]
fn jit_execute_add_subtract_expression() -> DynoResult<()> {
    let jit = Jit::new(&get_asm("return 42 - 12 + 12 - 5 + 2284;")?);
    assert_eq!(jit.run(), 2321);
    Ok(())
}

#[test]
fn jit_execute_multiply_expression() -> DynoResult<()> {
    let jit = Jit::new(&get_asm("return 2 * 4 * 3;")?);
    assert_eq!(jit.run(), 24);
    Ok(())
}

#[test]
fn jit_execute_divide_expression() -> DynoResult<()> {
    let jit = Jit::new(&get_asm("return 16 / 4 / 2;")?);
    assert_eq!(jit.run(), 2);
    Ok(())
}

#[test]
fn jit_execute_complete_expression() -> DynoResult<()> {
    let jit = Jit::new(&get_asm("return 12 / 3 + 7 * 8 - 10 / 2 * 4;")?);
    assert_eq!(jit.run(), 40);
    Ok(())
}
