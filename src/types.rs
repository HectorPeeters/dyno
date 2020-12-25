#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DynoType {
    UnsignedInt(u8),
    SignedInt(u8),
    Bool(),
    Void(),
}
