pub mod x86_backend;

use crate::ast::Statement;
use crate::error::DynoResult;

pub trait Backend {
    fn generate_statement(&mut self, statement: &Statement) -> DynoResult<()>;
}
