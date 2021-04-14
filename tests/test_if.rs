mod common;
use common::assert_run;

use dyno::error::DynoResult;

#[test]
fn execute_simple_if() -> DynoResult<()> {
    assert_run("let a: u32; a = 24; if 1 == 1 { a = 42; } return a;", 42)
}

#[test]
fn execute_simple_if_false() -> DynoResult<()> {
    assert_run("let a: u32; a = 24; if 1 == 0 { a = 42; } return a;", 24)
}
