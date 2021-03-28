#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DynoType {
    UInt8(),
    UInt16(),
    UInt32(),
    UInt64(),
    Bool(),
    Void(),
}

impl DynoType {
    pub fn is_int(&self) -> bool {
        matches!(
            *self,
            DynoType::UInt8() | DynoType::UInt16() | DynoType::UInt32() | DynoType::UInt64()
        )
    }

    pub fn get_bits(&self) -> u8 {
        match *self {
            DynoType::UInt8() => 8,
            DynoType::UInt16() => 16,
            DynoType::UInt32() => 32,
            DynoType::UInt64() => 64,
            DynoType::Bool() => 8,
            DynoType::Void() => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DynoValue {
    UInt(u64),
    Bool(),
}
