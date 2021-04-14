mod common;
use common::assert_run;

use dyno::error::DynoResult;

#[test]
fn execute_simple_while() -> DynoResult<()> {
    assert_run(
        "let a: u32; a = 10; while a > 1 { a = a - 1; } return a;",
        1,
    )
}

#[test]
fn execute_while() -> DynoResult<()> {
    assert_run(
        "let a: u32; a = 10; let b: u16; b = 5; while a > b { a = a - 1; } return a;",
        5,
    )
}
