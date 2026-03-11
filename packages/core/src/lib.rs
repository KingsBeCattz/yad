use std::fmt;
use crate::constants::error::{
    ErrorMessage,
    MALFORMED_UTF8,
    NOT_AN_ARRAY,
    NOT_A_BOOL,
    NOT_A_FLOAT16,
    NOT_A_FLOAT32,
    NOT_A_FLOAT32_VALUE,
    NOT_A_FLOAT64,
    NOT_A_FLOAT64_VALUE,
    NOT_A_FLOAT8,
    NOT_A_INT16,
    NOT_A_INT16_VALUE,
    NOT_A_INT32,
    NOT_A_INT32_VALUE,
    NOT_A_INT64,
    NOT_A_INT64_VALUE,
    NOT_A_INT8,
    NOT_A_INT8_VALUE,
    NOT_A_NUMBER,
    NOT_A_STRING,
    NOT_A_UINT16,
    NOT_A_UINT16_VALUE,
    NOT_A_UINT32,
    NOT_A_UINT32_VALUE,
    NOT_A_UINT64,
    NOT_A_UINT64_VALUE,
    NOT_A_UINT8,
    NOT_A_UINT8_VALUE,
    NOT_ENOUGH_BYTES,
    NESTING_TOO_DEEP,
    STRING_MAX_LENGTH_EXCEEDED,
    STRING_OF_LENGTH_ZERO,
    UNKNOWN,
    VEC_MAX_LENGTH_EXCEEDED,
    VEC_OF_LENGTH_ZERO,
};
use crate::constants::length::ByteLength;
use crate::constants::types::{Type, FLOATING_POINT_TYPE};
use float8::F8E4M3;
use float16::f16;

pub mod constants;
pub mod ffi;

// [FIX #2] Maximum nesting depth for arrays to prevent stack overflow via
// deeply nested malicious inputs. Adjust if legitimate use cases require deeper nesting.
const MAX_NESTING_DEPTH: usize = 64;

// [FIX #1] Maximum number of elements to pre-allocate in a Vec when decoding
// arrays. Prevents OOM attacks where the count field is large but the actual
// payload is small. The Vec will still grow beyond this if needed.
const MAX_PREALLOC_ELEMENTS: usize = 4096;

/// Choose the smallest `ByteLength` that can represent `len`.
///
/// Validates that `len` is non-zero and maps it to the smallest `ByteLength`
/// variant able to contain it. Returns an `ErrorMessage` when `len == 0` or
/// when `len` exceeds `u64::MAX`.
fn match_len_min_bytes(
    len: usize,
    len_zero_error: &'static str,
    exceded_max_len_error: &'static str,
) -> Result<ByteLength, ErrorMessage> {
    Ok(match len {
        l if l == 0 => {
            Err(ErrorMessage(len_zero_error))?
        }
        l if l <= u8::MAX as usize => ByteLength::One,
        l if l <= u16::MAX as usize => ByteLength::Two,
        l if l <= u32::MAX as usize => ByteLength::Four,
        l if l <= u64::MAX as usize => ByteLength::Eight,
        _ => Err(ErrorMessage(exceded_max_len_error))?,
    })
}

/// Append the big-endian length descriptor for `len` into `bytes`.
///
/// Uses `match_len_min_bytes` to choose the descriptor width, then appends
/// `len` encoded in big-endian using that width.
fn extend_bytes_with_len_bytes(
    len: usize,
    bytes: &mut Vec<u8>,
    len_zero_error: &'static str,
    exceded_max_len_error: &'static str,
) -> Result<(), ErrorMessage> {
    match match_len_min_bytes(len, len_zero_error, exceded_max_len_error)? {
        ByteLength::One => bytes.extend_from_slice(&(len as u8).to_be_bytes()),
        ByteLength::Two => bytes.extend_from_slice(&(len as u16).to_be_bytes()),
        ByteLength::Four => bytes.extend_from_slice(&(len as u32).to_be_bytes()),
        ByteLength::Eight => bytes.extend_from_slice(&(len as u64).to_be_bytes()),
        _ => Err(ErrorMessage(len_zero_error))?,
    }

    Ok(())
}

/// Represents a single value encoded in YAD's binary format.
///
/// A `Value` is the in-memory representation of one encoded item. It stores:
/// - `r#type`: a `Type` discriminant indicating how to interpret `bytes`.
/// - `length`: a `ByteLength` value used for numbers/collections where applicable.
/// - `bytes`: the raw encoded bytes for the whole encoded value (header + length + payload).
///
/// # Invariants
/// - `bytes[0]` is always the header byte (type | byte-length metadata).
/// - `isolate_value_bytes()` returns only the payload bytes (not the header or length descriptor).
/// - Conversions (`TryInto` / `From`) rely on `r#type` and `length` matching expected values.
/// - For nested `Array` values decoded via `TryInto<Vec<Value>>`, `bytes` always includes the
///   full encoding (header + length descriptor + payload) to preserve the invariant.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Value {
    /// Encoded type tag (header's type section). Use `Type::try_from(u8)` to obtain.
    pub r#type: Type,
    /// Encoded length descriptor width and semantics.
    pub length: ByteLength,
    /// The full encoded bytes for this value, always starting with the header byte.
    /// For arrays/strings: header + length descriptor + payload.
    /// For numbers: header + numeric bytes.
    /// For booleans: header only (1 byte).
    pub bytes: Vec<u8>,
}

impl Value {
    /// Decode a single top-level `Value` from `vec`.
    ///
    /// The provided `vec` must contain at least one whole encoded value starting
    /// at index 0. The function validates lengths, parses nested values for arrays,
    /// and returns a `Value` whose `bytes` field contains exactly the encoded
    /// chunk consumed from the input (header + length field + payload).
    ///
    /// # Errors
    /// Returns `ErrorMessage` constants defined in `constants::error`.
    ///
    /// # Nesting limit
    /// Array decoding is bounded by `MAX_NESTING_DEPTH` to prevent stack overflows.
    pub fn decode(vec: Vec<u8>) -> Result<Self, ErrorMessage> {
        if vec.len() < 1 {
            Err(ErrorMessage(NOT_ENOUGH_BYTES))?
        }

        // [FIX #2] Added `depth` parameter to consumed_for_value to enforce
        // MAX_NESTING_DEPTH and prevent stack overflows from deeply nested arrays.
        fn consumed_for_value(bytes: &[u8], depth: usize) -> Result<usize, ErrorMessage> {
            if bytes.is_empty() {
                return Err(ErrorMessage(NOT_ENOUGH_BYTES));
            }

            // [FIX #2] Reject inputs that exceed the maximum allowed nesting depth.
            if depth > MAX_NESTING_DEPTH {
                return Err(ErrorMessage(NESTING_TOO_DEEP));
            }

            let first = bytes[0];
            let r#type = Type::try_from(first)?;
            let bl = ByteLength::try_from(first)?;
            let len_field_size = usize::from(bl);

            if bytes.len() < 1 + len_field_size {
                return Err(ErrorMessage(NOT_ENOUGH_BYTES));
            }

            match r#type {
                Type::Uint | Type::Int | Type::Float => {
                    let total = 1 + len_field_size;
                    if bytes.len() < total {
                        return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                    }
                    Ok(total)
                }
                Type::Bool | Type::True | Type::False => Ok(1),
                Type::String => {
                    let str_len = match bl {
                        ByteLength::Zero => Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?,
                        ByteLength::One => *bytes.get(1).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))? as usize,
                        ByteLength::Two => {
                            let s = bytes.get(1..=2).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                            u16::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
                        }
                        ByteLength::Four => {
                            let s = bytes.get(1..=4).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                            u32::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
                        }
                        ByteLength::Eight => {
                            let s = bytes.get(1..=8).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                            let v = u64::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?);
                            if v as usize > usize::MAX { Err(ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))? }
                            v as usize
                        }
                    };
                    let total = 1 + len_field_size + str_len;
                    if bytes.len() < total {
                        return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                    }
                    Ok(total)
                }
                Type::Array => {
                    let count = match bl {
                        ByteLength::Zero => Err(ErrorMessage(VEC_OF_LENGTH_ZERO))?,
                        ByteLength::One => *bytes.get(1).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))? as usize,
                        ByteLength::Two => {
                            let s = bytes.get(1..=2).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                            u16::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
                        }
                        ByteLength::Four => {
                            let s = bytes.get(1..=4).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                            u32::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
                        }
                        ByteLength::Eight => {
                            let s = bytes.get(1..=8).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                            let v = u64::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?);
                            if v as usize > usize::MAX { Err(ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))? }
                            v as usize
                        }
                    };

                    let mut pos = 1 + len_field_size;
                    for _ in 0..count {
                        if pos >= bytes.len() {
                            return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                        }
                        // [FIX #2] Pass depth + 1 to enforce nesting limit recursively.
                        let consumed = consumed_for_value(&bytes[pos..], depth + 1)?;
                        pos = pos.checked_add(consumed).ok_or_else(|| ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))?;
                    }
                    if bytes.len() < pos {
                        return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                    }
                    Ok(pos)
                }
            }
        }

        let first = *vec.get(0).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
        let r#type = Type::try_from(first)?;
        let bl = ByteLength::try_from(first)?;
        let len_field_size = usize::from(bl);

        match r#type {
            Type::Uint | Type::Int | Type::Float => {
                let total = 1 + len_field_size;
                if vec.len() < total { Err(ErrorMessage(NOT_ENOUGH_BYTES))? }
                let chunk = vec[..total].to_vec();
                Self::from_number(chunk)
            }

            Type::String => {
                let str_len = match bl {
                    ByteLength::Zero => Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?,
                    ByteLength::One => *vec.get(1).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))? as usize,
                    ByteLength::Two => {
                        let s = vec.get(1..=2).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                        u16::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
                    }
                    ByteLength::Four => {
                        let s = vec.get(1..=4).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                        u32::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
                    }
                    ByteLength::Eight => {
                        let s = vec.get(1..=8).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                        let v = u64::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?);
                        if v as usize > usize::MAX { Err(ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))? }
                        v as usize
                    }
                };

                let total = 1 + len_field_size + str_len;
                if vec.len() < total { Err(ErrorMessage(NOT_ENOUGH_BYTES))? }

                let payload = &vec[(1 + len_field_size)..total];
                let s = String::from_bytes(payload)?;
                Self::try_from(s).map_err(|_e| ErrorMessage(UNKNOWN))
            }

            Type::Array => {
                let count = match bl {
                    ByteLength::Zero => Err(ErrorMessage(VEC_OF_LENGTH_ZERO))?,
                    ByteLength::One => *vec.get(1).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))? as usize,
                    ByteLength::Two => {
                        let s = vec.get(1..=2).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                        u16::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
                    }
                    ByteLength::Four => {
                        let s = vec.get(1..=4).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                        u32::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
                    }
                    ByteLength::Eight => {
                        let s = vec.get(1..=8).ok_or(ErrorMessage(NOT_ENOUGH_BYTES))?;
                        let v = u64::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?);
                        if v as usize > usize::MAX { Err(ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))? }
                        v as usize
                    }
                };

                // [FIX #1] Cap pre-allocation to MAX_PREALLOC_ELEMENTS to prevent OOM
                // when a malicious input declares a huge count but has few actual elements.
                let mut elements: Vec<Self> = Vec::with_capacity(count.min(MAX_PREALLOC_ELEMENTS));
                let mut pos = 1 + len_field_size;
                for _ in 0..count {
                    if pos >= vec.len() { Err(ErrorMessage(NOT_ENOUGH_BYTES))? }
                    // [FIX #2] Start at depth 1 since we are already inside one array.
                    let consumed = consumed_for_value(&vec[pos..], 1)?;
                    let chunk = vec[pos..pos + consumed].to_vec();
                    let element = Self::decode(chunk)?;
                    elements.push(element);
                    pos += consumed;
                }

                Self::try_from(elements).map_err(|_e| ErrorMessage(UNKNOWN))
            }

            Type::Bool | Type::False | Type::True => {
                Self::try_from(r#type != Type::False).map_err(|_e| ErrorMessage(UNKNOWN))
            }
        }
    }

    /// Build a `Value` representing a numeric encoded chunk.
    ///
    /// Accepts a `Vec<u8>` where `vec[0]` is the header byte and the following
    /// bytes are the big-endian numeric payload. Validates the header type nibble
    /// and that enough bytes are present for the declared size.
    ///
    /// # Errors
    /// Returns `NOT_ENOUGH_BYTES` if the slice is too short, or `NOT_A_NUMBER` if
    /// the header type nibble does not correspond to a numeric type.
    pub fn from_number(vec: Vec<u8>) -> Result<Self, ErrorMessage> {
        if vec.len() < 1 {
            Err(ErrorMessage(NOT_ENOUGH_BYTES))?
        }

        // [FIX #5] Replaced vec.remove(0) (O(n), shifts all elements) with direct
        // index access. The original Vec is no longer mutated unnecessarily.
        let chunk_a = vec[0];

        if chunk_a & 0xF0 > FLOATING_POINT_TYPE {
            Err(ErrorMessage(NOT_A_NUMBER))?;
        }

        let format = Type::try_from(chunk_a)?;
        let byte_length = ByteLength::try_from(chunk_a)?;

        // [FIX #5] Use a slice starting at index 1 instead of draining the original Vec.
        let payload = &vec[1..];

        if payload.len() < u8::from(byte_length) as usize {
            Err(ErrorMessage(NOT_ENOUGH_BYTES))?
        }

        let mut bytes = Vec::with_capacity(1 + byte_length as usize);
        bytes.push(chunk_a);
        bytes.extend_from_slice(&payload[..byte_length as usize]);

        Ok(Self {
            r#type: format,
            length: byte_length,
            bytes,
        })
    }

    /// Return only the payload bytes for this `Value` (excludes header and length descriptor).
    ///
    /// For numbers: skips the single header byte.
    /// For strings and arrays: skips header + length descriptor bytes.
    pub fn isolate_value_bytes(&self) -> &[u8] {
        let start = if self.r#type <= Type::Float {
            1
        } else {
            (self.length.as_byte_count() as u8 + 1) as usize
        };

        &self.bytes[start..]
    }
}

/// Trait used to decode primitive types from a byte slice according to YAD semantics.
pub trait FromYADNotation: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ErrorMessage>;
}

impl FromYADNotation for String {
    /// Decode a UTF-8 byte slice into a `String`, returning `MALFORMED_UTF8` on failure.
    fn from_bytes(bytes: &[u8]) -> Result<Self, ErrorMessage> {
        if let Some(string) = String::from_utf8(Vec::from(bytes)).ok() {
            Ok(string)
        } else {
            Err(ErrorMessage(MALFORMED_UTF8))
        }
    }
}

use std::convert::TryFrom;

/// Macro implementing `From<$t> for Value` and `TryFrom<&Value> for $t` for numeric types.
///
/// - `From<$t>`: encodes the value into YAD binary format (header + big-endian bytes).
/// - `TryFrom<&Value>`: validates that type and length match, then decodes the payload.
///
/// Parameters: numeric type, Type variant, ByteLength variant, error for type mismatch,
/// error for size mismatch.
macro_rules! impl_from_num {
    ($t:ty, $type_variant:expr, $len_variant:expr, $invalid_value:expr, $doesnt_fit:expr) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                let r#type = $type_variant;
                let length = $len_variant;

                let num_as_be = value.to_be_bytes();

                let mut bytes = vec![u8::from(r#type) | u8::from(length)];
                bytes.extend_from_slice(&num_as_be);

                Self { r#type, length, bytes }
            }
        }

        impl TryFrom<&Value> for $t {
            type Error = ErrorMessage;

            fn try_from(value: &Value) -> Result<$t, Self::Error> {
                if value.r#type != $type_variant || value.length != $len_variant {
                    return Err(ErrorMessage($invalid_value));
                }

                let data = &value.bytes[1..]; // skip header
                if data.len() != std::mem::size_of::<$t>() {
                    return Err(ErrorMessage($doesnt_fit));
                }

                let mut arr = [0u8; std::mem::size_of::<$t>()];
                arr.copy_from_slice(data);
                Ok(<$t>::from_be_bytes(arr))
            }
        }
    };
}

impl From<F8E4M3> for Value {
    fn from(value: F8E4M3) -> Self {
        let r#type = Type::Float;
        let length = ByteLength::One;

        let num_as_be = value.to_bits();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.push(num_as_be);

        Self { r#type, length, bytes }
    }
}

impl From<f16> for Value {
    fn from(value: f16) -> Self {
        let r#type = Type::Float;
        let length = ByteLength::Two;

        let num_as_be = value.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self { r#type, length, bytes }
    }
}

impl TryFrom<String> for Value {
    type Error = ErrorMessage;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let r#type = Type::String;
        let byte_length = match_len_min_bytes(value.len(), STRING_OF_LENGTH_ZERO, STRING_MAX_LENGTH_EXCEEDED)?;

        let mut bytes = vec![u8::from(r#type) | u8::from(byte_length)];
        extend_bytes_with_len_bytes(value.len(), &mut bytes, STRING_OF_LENGTH_ZERO, STRING_MAX_LENGTH_EXCEEDED)?;
        bytes.extend_from_slice(&value.as_bytes());

        Ok(Self { r#type, length: byte_length, bytes })
    }
}

impl TryFrom<&str> for Value {
    type Error = ErrorMessage;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let r#type = Type::String;
        let byte_length = match_len_min_bytes(value.len(), STRING_OF_LENGTH_ZERO, STRING_MAX_LENGTH_EXCEEDED)?;

        let mut bytes = vec![u8::from(r#type) | u8::from(byte_length)];
        extend_bytes_with_len_bytes(value.len(), &mut bytes, STRING_OF_LENGTH_ZERO, STRING_MAX_LENGTH_EXCEEDED)?;
        bytes.extend_from_slice(&value.as_bytes());

        Ok(Self { r#type, length: byte_length, bytes })
    }
}

impl TryFrom<Vec<Value>> for Value {
    type Error = ErrorMessage;
    fn try_from(value: Vec<Value>) -> Result<Self, Self::Error> {
        let r#type = Type::Array;
        let byte_length = match_len_min_bytes(value.len(), VEC_OF_LENGTH_ZERO, VEC_MAX_LENGTH_EXCEEDED)?;

        let mut bytes = vec![u8::from(r#type) | u8::from(byte_length)];
        extend_bytes_with_len_bytes(value.len(), &mut bytes, VEC_OF_LENGTH_ZERO, VEC_MAX_LENGTH_EXCEEDED)?;

        for i in value {
            bytes.extend_from_slice(i.bytes.as_slice());
        }

        Ok(Self { r#type, length: byte_length, bytes })
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        let r#type = if value { Type::True } else { Type::False };
        Self { r#type, length: ByteLength::Zero, bytes: vec![u8::from(r#type)] }
    }
}

/// Macro implementing `TryInto<T>` for `Value` for numeric types.
///
/// Validates `r#type` and `length`, extracts the isolated payload bytes,
/// and reconstructs the value using big-endian decoding. Returns the
/// provided error constant on any mismatch.
macro_rules! impl_try_into_num {
    ($t:ty, $type_variant:expr, $len_variant:expr, $not_a:expr) => {
        impl TryInto<$t> for Value {
            type Error = ErrorMessage;

            fn try_into(self) -> Result<$t, Self::Error> {
                if self.r#type != $type_variant || self.length != $len_variant {
                    return Err(ErrorMessage($not_a));
                }

                let bytes = self.isolate_value_bytes();

                let bytes: [u8; std::mem::size_of::<$t>()] = bytes
                    .try_into()
                    .map_err(|_| ErrorMessage($not_a))?;

                Ok(<$t>::from_be_bytes(bytes))
            }
        }
    };
}

// =========================
// Macro Invocations
// =========================

// Unsigned integers
impl_try_into_num!(u8,  Type::Uint,  ByteLength::One,   NOT_A_UINT8);
impl_from_num!(u8,      Type::Uint,  ByteLength::One,   NOT_A_UINT8,  NOT_A_UINT8_VALUE);
impl_try_into_num!(u16, Type::Uint,  ByteLength::Two,   NOT_A_UINT16);
impl_from_num!(u16,     Type::Uint,  ByteLength::Two,   NOT_A_UINT16, NOT_A_UINT16_VALUE);
impl_try_into_num!(u32, Type::Uint,  ByteLength::Four,  NOT_A_UINT32);
impl_from_num!(u32,     Type::Uint,  ByteLength::Four,  NOT_A_UINT32, NOT_A_UINT32_VALUE);
#[cfg(target_pointer_width = "32")]
impl_try_into_num!(usize, Type::Uint, ByteLength::Four, NOT_A_UINT32);
#[cfg(target_pointer_width = "32")]
impl_from_num!(usize,   Type::Uint,  ByteLength::Four,  NOT_A_UINT32, NOT_A_UINT32_VALUE);
impl_try_into_num!(u64, Type::Uint,  ByteLength::Eight, NOT_A_UINT64);
impl_from_num!(u64,     Type::Uint,  ByteLength::Eight, NOT_A_UINT64, NOT_A_UINT64_VALUE);
#[cfg(target_pointer_width = "64")]
impl_try_into_num!(usize, Type::Uint, ByteLength::Eight, NOT_A_UINT64);
#[cfg(target_pointer_width = "64")]
impl_from_num!(usize,   Type::Uint,  ByteLength::Eight, NOT_A_UINT64, NOT_A_UINT64_VALUE);

// Signed integers
impl_try_into_num!(i8,  Type::Int,   ByteLength::One,   NOT_A_INT8);
impl_from_num!(i8,      Type::Int,   ByteLength::One,   NOT_A_INT8,   NOT_A_INT8_VALUE);
impl_try_into_num!(i16, Type::Int,   ByteLength::Two,   NOT_A_INT16);
impl_from_num!(i16,     Type::Int,   ByteLength::Two,   NOT_A_INT16,  NOT_A_INT16_VALUE);
impl_try_into_num!(i32, Type::Int,   ByteLength::Four,  NOT_A_INT32);
impl_from_num!(i32,     Type::Int,   ByteLength::Four,  NOT_A_INT32,  NOT_A_INT32_VALUE);
#[cfg(target_pointer_width = "32")]
impl_try_into_num!(isize, Type::Int, ByteLength::Four,  NOT_A_INT32);
#[cfg(target_pointer_width = "32")]
impl_from_num!(isize,   Type::Int,   ByteLength::Four,  NOT_A_INT32,  NOT_A_INT32_VALUE);
impl_try_into_num!(i64, Type::Int,   ByteLength::Eight, NOT_A_INT64);
impl_from_num!(i64,     Type::Int,   ByteLength::Eight, NOT_A_INT64,  NOT_A_INT64_VALUE);
#[cfg(target_pointer_width = "64")]
impl_try_into_num!(isize, Type::Int, ByteLength::Eight, NOT_A_INT64);
#[cfg(target_pointer_width = "64")]
impl_from_num!(isize,   Type::Int,   ByteLength::Eight, NOT_A_INT64,  NOT_A_INT64_VALUE);

// Floating-point numbers
impl_try_into_num!(f32, Type::Float, ByteLength::Four,  NOT_A_FLOAT32);
impl_from_num!(f32,     Type::Float, ByteLength::Four,  NOT_A_FLOAT32, NOT_A_FLOAT32_VALUE);
impl_try_into_num!(f64, Type::Float, ByteLength::Eight, NOT_A_FLOAT64);
impl_from_num!(f64,     Type::Float, ByteLength::Eight, NOT_A_FLOAT64, NOT_A_FLOAT64_VALUE);

impl TryInto<F8E4M3> for Value {
    type Error = ErrorMessage;

    fn try_into(self) -> Result<F8E4M3, Self::Error> {
        if self.r#type != Type::Float || self.length != ByteLength::One {
            Err(ErrorMessage(NOT_A_FLOAT8))?;
        }

        let bytes = self.isolate_value_bytes()[0];
        Ok(F8E4M3::from_bits(bytes))
    }
}

impl TryInto<f16> for Value {
    type Error = ErrorMessage;

    fn try_into(self) -> Result<f16, Self::Error> {
        if self.r#type != Type::Float || self.length != ByteLength::Two {
            Err(ErrorMessage(NOT_A_FLOAT16))?;
        }

        let bytes = self.isolate_value_bytes();

        let bytes: [u8; 2] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_FLOAT16))?;

        Ok(f16::from_be_bytes(bytes))
    }
}

impl TryInto<String> for Value {
    type Error = ErrorMessage;

    fn try_into(self) -> Result<String, Self::Error> {
        if self.r#type != Type::String {
            Err(ErrorMessage(NOT_A_STRING))?;
        }

        let bytes = self.isolate_value_bytes();
        Ok(String::from_bytes(bytes).map_err(|_e| ErrorMessage(NOT_A_STRING))?)
    }
}

impl TryInto<Vec<Value>> for Value {
    type Error = ErrorMessage;

    /// Convert a `Value` encoded as `Array` into a `Vec<Value>`.
    ///
    /// Iterates over the payload bytes and decodes each element in sequence.
    /// Nested arrays are decoded recursively and stored as complete `Value`
    /// instances with their full encoding (header + length + payload) intact,
    /// preserving the `Value::bytes` invariant.
    ///
    /// # Errors
    /// Returns `NOT_AN_ARRAY` if the value's type is not `Array`.
    /// Returns `NESTING_TOO_DEEP` if arrays are nested beyond `MAX_NESTING_DEPTH`.
    fn try_into(self) -> Result<Vec<Value>, Self::Error> {
        if self.r#type != Type::Array {
            return Err(ErrorMessage(NOT_AN_ARRAY));
        }

        #[inline]
        fn parse_length(bytes: &[u8], len_type: ByteLength) -> Result<usize, ErrorMessage> {
            let size = len_type.as_byte_count() as usize;
            if bytes.len() < 1 + size {
                return Err(ErrorMessage(NOT_ENOUGH_BYTES));
            }
            match len_type {
                ByteLength::Zero => Ok(0),
                ByteLength::One => Ok(bytes[1] as usize),
                ByteLength::Two => {
                    let arr: [u8; 2] = bytes[1..=2].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?;
                    Ok(u16::from_be_bytes(arr) as usize)
                }
                ByteLength::Four => {
                    let arr: [u8; 4] = bytes[1..=4].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?;
                    Ok(u32::from_be_bytes(arr) as usize)
                }
                ByteLength::Eight => {
                    let arr: [u8; 8] = bytes[1..=8].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?;
                    Ok(u64::from_be_bytes(arr).try_into().map_err(|_| ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))?)
                }
            }
        }

        // [FIX #2] Added `depth` parameter to prevent stack overflow from deeply
        // nested malicious inputs.
        fn consumed_for_value(bytes: &[u8], depth: usize) -> Result<usize, ErrorMessage> {
            if bytes.is_empty() {
                return Err(ErrorMessage(NOT_ENOUGH_BYTES));
            }

            // [FIX #2] Reject inputs exceeding the maximum nesting depth.
            if depth > MAX_NESTING_DEPTH {
                return Err(ErrorMessage(NESTING_TOO_DEEP));
            }

            let header = bytes[0];
            let val_type = Type::try_from(header)?;
            let len_type = ByteLength::try_from(header)?;
            let len_size = len_type.as_byte_count() as usize;

            match val_type {
                Type::Uint | Type::Int | Type::Float => Ok(1 + len_size),
                Type::Bool | Type::True | Type::False => Ok(1),
                Type::String => {
                    if matches!(len_type, ByteLength::Zero) {
                        return Err(ErrorMessage(STRING_OF_LENGTH_ZERO));
                    }
                    let str_len = parse_length(bytes, len_type)?;
                    let total = 1 + len_size + str_len;
                    if bytes.len() < total {
                        return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                    }
                    Ok(total)
                }
                Type::Array => {
                    if matches!(len_type, ByteLength::Zero) {
                        return Err(ErrorMessage(VEC_OF_LENGTH_ZERO));
                    }
                    let count = parse_length(bytes, len_type)?;
                    let mut pos = 1 + len_size;
                    for _ in 0..count {
                        if pos >= bytes.len() {
                            return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                        }
                        // [FIX #2] Pass depth + 1 on each recursive call.
                        let used = consumed_for_value(&bytes[pos..], depth + 1)?;
                        pos = pos.checked_add(used).ok_or_else(|| ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))?;
                    }
                    Ok(pos)
                }
            }
        }

        let mut result = Vec::new();
        let mut bytes = self.isolate_value_bytes();

        while !bytes.is_empty() {
            let header = bytes[0];
            let val_type = Type::try_from(header)?;
            let len_type = ByteLength::try_from(header)?;
            let len_size = len_type.as_byte_count() as usize;

            match val_type {
                Type::Uint | Type::Int | Type::Float => {
                    let size = 1 + len_size;
                    if bytes.len() < size {
                        return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                    }
                    let (chunk, rest) = bytes.split_at(size);
                    result.push(Value::from_number(chunk.to_vec())?);
                    bytes = rest;
                }
                Type::String => {
                    if matches!(len_type, ByteLength::Zero) {
                        return Err(ErrorMessage(STRING_OF_LENGTH_ZERO));
                    }
                    let str_len = parse_length(bytes, len_type)?;
                    let start = 1 + len_size;
                    let end = start + str_len;
                    if bytes.len() < end {
                        return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                    }
                    let raw = &bytes[start..end];
                    let s = String::from_bytes(raw)?;
                    result.push(Value::try_from(s)?);
                    bytes = &bytes[end..];
                }
                Type::Bool | Type::True | Type::False => {
                    result.push(Value::from(val_type != Type::False));
                    bytes = &bytes[1..];
                }
                Type::Array => {
                    // [FIX #2] Pass depth = 1 since we are one level deep already.
                    let used = consumed_for_value(bytes, 1)?;
                    if bytes.len() < used {
                        return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                    }
                    let (chunk, rest) = bytes.split_at(used);

                    // [FIX #4] Store the full chunk (including the header byte) to preserve
                    // the Value::bytes invariant: bytes[0] must always be the header.
                    // Previously this stored chunk[1..] which stripped the header and broke
                    // re-encoding and Display for nested arrays.
                    result.push(Value {
                        r#type: Type::Array,
                        length: len_type,
                        bytes: chunk.to_vec(),
                    });
                    bytes = rest;
                }
            }
        }

        Ok(result)
    }
}

impl TryInto<bool> for Value {
    type Error = ErrorMessage;

    /// Convert a `Value` to `bool`.
    ///
    /// Returns `true` for `Type::True` and `false` for `Type::False`.
    /// Returns `NOT_A_BOOL` if the value's type is not a boolean variant.
    fn try_into(self) -> Result<bool, Self::Error> {
        // [FIX #6] The original used `|` (bitwise OR) with 0x0F, which forced the
        // lower nibble to all-ones and made the check nonsensical for most type values.
        // The correct check masks the upper nibble of both sides and compares them,
        // so any Bool/True/False variant (which share the same high nibble) passes.
        if u8::from(self.r#type) & 0xF0 != u8::from(Type::Bool) & 0xF0 {
            Err(ErrorMessage(NOT_A_BOOL))?;
        }

        Ok(self.r#type != Type::False)
    }
}

impl fmt::Display for Value {
    /// Produce a human-readable representation of a `Value`.
    ///
    /// - Numbers are decoded and formatted with their native Rust `Display`.
    /// - Strings are printed as plain UTF-8 text.
    /// - Arrays are displayed as `[a, b, c]` using recursive formatting.
    /// - Booleans are printed as `true` or `false`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.r#type {
            Type::Uint => match self.length {
                ByteLength::One => {
                    let v: u8 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                ByteLength::Two => {
                    let v: u16 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                ByteLength::Four => {
                    let v: u32 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                ByteLength::Eight => {
                    let v: u64 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                _ => write!(f, "{:?}", self.bytes),
            },
            Type::Int => match self.length {
                ByteLength::One => {
                    let v: i8 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                ByteLength::Two => {
                    let v: i16 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                ByteLength::Four => {
                    let v: i32 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                ByteLength::Eight => {
                    let v: i64 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                _ => write!(f, "{:?}", self.bytes),
            },
            Type::Float => match self.length {
                ByteLength::One => {
                    let v: F8E4M3 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                ByteLength::Two => {
                    let v: f16 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                ByteLength::Four => {
                    let v: f32 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                ByteLength::Eight => {
                    let v: f64 = self.clone().try_into().map_err(|_| fmt::Error)?;
                    write!(f, "{}", v)
                }
                _ => write!(f, "{:?}", self.bytes),
            },
            Type::String => {
                let s: String = self.clone().try_into().map_err(|_| fmt::Error)?;
                write!(f, "{}", s)
            }
            Type::Array => {
                let arr: Vec<Value> = self.clone().try_into().map_err(|_| fmt::Error)?;
                let mut string = String::from("[");
                for (i, item) in arr.iter().enumerate() {
                    string.push_str(&format!("{}", item));
                    if i < arr.len() - 1 {
                        string.push_str(", ");
                    }
                }
                string.push(']');
                write!(f, "{}", string)
            }
            Type::Bool | Type::True | Type::False => {
                let b: bool = self.clone().try_into().map_err(|_| fmt::Error)?;
                write!(f, "{}", b)
            }
        }
    }
}