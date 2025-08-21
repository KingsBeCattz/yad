use crate::constants::error::{ErrorMessage, FAILED_TRANSFORMING_AN_U8_TO_VALID_TYPE};

/// > **NEEDS A LENGTH BYTE**
///
/// Indicates an unsigned integer
pub const UNSIGNED_INTEGER_TYPE: u8 = 0x10;
/// > **NEEDS A LENGTH BYTE**
///
/// Indicates a signed integer
pub const SIGNED_INTEGER_TYPE: u8 = 0x20;
/// > **NEEDS A LENGTH BYTE**
///
/// Indicates a floating point number
pub const FLOATING_POINT_TYPE: u8 = 0x30;
/// > **NEEDS A LENGTH BYTE**
///
/// Indicates a floating point number
pub const STRING_TYPE: u8 = 0x40;
/// > **NEEDS A LENGTH BYTE**
///
/// Indicates a floating point number
pub const ARRAY_TYPE: u8 = 0x50;
/// This is a Boolean unifier.
///
/// Any value between `0x81` and `0x8F` is considered `true`, however each write will be truncated to `0x81`.
pub const BOOLEAN_TYPE: u8 = 0x8F;
/// This is a Boolean with a value of `false`.
pub const FALSE_BOOLEAN_TYPE: u8 = 0x80;
/// This is a Boolean with a value of `true`.
pub const TRUE_BOOLEAN_TYPE: u8 = 0x81;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Type {
    Uint = UNSIGNED_INTEGER_TYPE,
    Int = SIGNED_INTEGER_TYPE,
    Float = FLOATING_POINT_TYPE,
    String = STRING_TYPE,
    Array = ARRAY_TYPE,
    Bool = BOOLEAN_TYPE,
    False = FALSE_BOOLEAN_TYPE,
    True = TRUE_BOOLEAN_TYPE
}

impl TryFrom<u8> for Type {
    type Error = ErrorMessage;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v & 0xF0 == UNSIGNED_INTEGER_TYPE => Ok(Type::Uint),
            v if v & 0xF0 == SIGNED_INTEGER_TYPE => Ok(Type::Int),
            v if v & 0xF0 == FLOATING_POINT_TYPE => Ok(Type::Float),
            v if v & 0xF0 == STRING_TYPE => Ok(Type::String),
            v if v & 0xF0 == ARRAY_TYPE => Ok(Type::Array),
            v if v & 0xF0 == BOOLEAN_TYPE => Ok(Type::Bool),
            FALSE_BOOLEAN_TYPE => Ok(Type::False),
            TRUE_BOOLEAN_TYPE => Ok(Type::True),
            _ => Err(ErrorMessage(FAILED_TRANSFORMING_AN_U8_TO_VALID_TYPE)),
        }
    }
}

impl From<Type> for u8 {
    fn from(t: Type) -> u8 {
        t as u8
    }
}