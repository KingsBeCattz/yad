pub const UNKNOWN: &'static str = "Unknown error occurred";
pub const FAILED_TRANSFORMING_AN_U8_TO_VALID_TYPE: &'static str = "Failure to convert a u8 to a valid type in YAD";
pub const FAILED_TRANSFORMING_AN_U8_TO_VALID_LENGTH: &'static str = "Failure to convert a u8 to a valid length in YAD";
pub const FAILED_DECODING_AN_VECTOR_TO_VALID_VALUE: &'static str = "Failure to decode a vector to a valid value in YAD";
pub const NOT_ENOUGH_BYTES: &'static str = "Not enough bytes";
pub const MALFORMED_UTF8: &'static str = "The provided UTF-8 String from a YAD FILE is malformed.";
pub const NOT_A_NUMBER: &'static str = "You cannot decode something like a number that is not a number.";
pub const INVALID_YAD_VALUE: &'static str = "The provided YAD value is malformed.";
pub const NOT_A_UINT8: &'static str = "You cannot convert something that is not u8 to u8.";
pub const NOT_A_INT8: &'static str = "You cannot convert something that is not i8 to i8.";
pub const NOT_A_UINT16: &'static str = "You cannot convert something that is not u16 to u16.";
pub const NOT_A_INT16: &'static str = "You cannot convert something that is not i16 to i16.";
pub const NOT_A_UINT32: &'static str = "You cannot convert something that is not u32 to u32.";
pub const NOT_A_INT32: &'static str = "You cannot convert something that is not i32 to i32.";
pub const NOT_A_UINT64: &'static str = "You cannot convert something that is not u64 to u64.";
pub const NOT_A_INT64: &'static str = "You cannot convert something that is not i64 to i64.";
pub const NOT_A_FLOAT8: &'static str = "You cannot convert something that is not f8 to f8.";
pub const NOT_A_FLOAT16: &'static str = "You cannot convert something that is not f16 to f16.";
pub const NOT_A_FLOAT32: &'static str = "You cannot convert something that is not f32 to f32.";
pub const NOT_A_FLOAT64: &'static str = "You cannot convert something that is not f64 to f64.";
pub const NOT_A_STRING: &'static str = "You cannot convert something that is not string to string.";
pub const NOT_A_BOOL: &'static str = "You cannot convert something that is not boolean to boolean.";
pub const NOT_AN_ARRAY: &'static str = "You cannot convert something that is not array to array.";
pub const NOT_A_UINT8_VALUE: &'static str = "You cannot convert something that is not value of u8 to u8.";
pub const NOT_A_INT8_VALUE: &'static str = "You cannot convert something that is not value of i8 to i8.";
pub const NOT_A_UINT16_VALUE: &'static str = "You cannot convert something that is not value of u16 to u16.";
pub const NOT_A_INT16_VALUE: &'static str = "You cannot convert something that is not value of i16 to i16.";
pub const NOT_A_UINT32_VALUE: &'static str = "You cannot convert something that is not value of u32 to u32.";
pub const NOT_A_INT32_VALUE: &'static str = "You cannot convert something that is not value of i32 to i32.";
pub const NOT_A_UINT64_VALUE: &'static str = "You cannot convert something that is not value of u64 to u64.";
pub const NOT_A_INT64_VALUE: &'static str = "You cannot convert something that is not value of i64 to i64.";
pub const NOT_A_FLOAT8_VALUE: &'static str = "You cannot convert something that is not value of f8 to f8.";
pub const NOT_A_FLOAT16_VALUE: &'static str = "You cannot convert something that is not value of f16 to f16.";
pub const NOT_A_FLOAT32_VALUE: &'static str = "You cannot convert something that is not value of f32 to f32.";
pub const NOT_A_FLOAT64_VALUE: &'static str = "You cannot convert something that is not value of f64 to f64.";
pub const NOT_A_STRING_VALUE: &'static str = "You cannot convert something that is not value of string to string.";
pub const NOT_A_BOOL_VALUE: &'static str = "You cannot convert something that is not value of boolean to boolean.";
pub const NOT_AN_ARRAY_VALUE: &'static str = "You cannot convert something that is not value of array to array.";
pub const STRING_MAX_LENGTH_EXCEEDED: &'static str = "Your string exceeds the limit of 2^64 − 1 characters.";
pub const STRING_OF_LENGTH_ZERO: &'static str = "Your string must have at least one character.";
pub const VEC_MAX_LENGTH_EXCEEDED: &'static str = "Your vector exceeds the limit of 2^64 − 1 items.";
pub const VEC_OF_LENGTH_ZERO: &'static str = "Your vector must have at least one item.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorMessage(pub &'static str);

impl From<&'static str> for ErrorMessage {
    fn from(s: &'static str) -> Self {
        ErrorMessage(s)
    }
}