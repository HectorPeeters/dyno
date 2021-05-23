pub mod x86_backend;

use crate::ast::{Expression, Statement};
use crate::error::DynoResult;

pub trait Backend {
    type Register;

    fn generate_statement(&mut self, statement: &Statement) -> DynoResult<()>;

    fn generate_expression(&mut self, expression: &Expression) -> DynoResult<Self::Register>;
}
