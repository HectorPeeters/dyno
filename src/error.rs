#[derive(Debug)]
pub enum DynoError {}

pub type DynoResult<T> = Result<T, DynoError>;
