mod common;
use common::assert_run;

use dyno::error::DynoResult;

#[test]
fn execute_declare_and_assign() -> DynoResult<()> {
    assert_run(
        r"
        let x: u32;
        x = 13;
        return x; 
               ",
        13,
    )
}
