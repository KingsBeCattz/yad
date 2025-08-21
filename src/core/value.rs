use crate::constants::error::{ErrorMessage, MALFORMED_UTF8, NOT_AN_ARRAY, NOT_A_BOOL, NOT_A_FLOAT16, NOT_A_FLOAT32, NOT_A_FLOAT64, NOT_A_FLOAT8, NOT_A_INT16, NOT_A_INT32, NOT_A_INT64, NOT_A_INT8, NOT_A_NUMBER, NOT_A_STRING, NOT_A_UINT16, NOT_A_UINT32, NOT_A_UINT64, NOT_A_UINT8, NOT_ENOUGH_BYTES, STRING_MAX_LENGTH_EXCEEDED, STRING_OF_LENGTH_ZERO, VEC_MAX_LENGTH_EXCEEDED, VEC_OF_LENGTH_ZERO};
use crate::constants::length::ByteLength;
use crate::constants::types::{Type, ARRAY_TYPE, BOOLEAN_TYPE, FLOATING_POINT_TYPE, SIGNED_INTEGER_TYPE, STRING_TYPE, UNSIGNED_INTEGER_TYPE};
use float8::F8E4M3;
use float16::f16;

/// Represents a single value within the YAD binary format.
///
/// A [`Value`] encapsulates a piece of data stored in the YAD system,
/// including its type, length, and raw binary representation.
/// It acts as the fundamental unit of information, supporting multiple data kinds
/// such as numbers, strings, arrays, or floating-point values.
///
/// # Structure Fields
/// - [`type`](Value::r#type): Defines the semantic meaning of the value (e.g., unsigned integer, string).
/// - [`length`](Value::length): Encodes the size of the value or the size descriptor length.
/// - [`bytes`](Value::bytes): The raw binary content corresponding to the value.
///
/// # Invariants
/// - The `length` field determines how the `bytes` field must be interpreted.
/// - The `type` field enforces the semantic meaning of the `bytes`.
/// - A mismatch between `type`, `length`, and `bytes` may result in invalid deserialization.
///
/// # Examples
/// ## Numeric Value
/// ```rust
/// use yad::constants::*;
///
/// yad::core::Value {
///     r#type: types::Type::Uint,       // Unsigned integer
///     length: length::ByteLength::One, // Stored in a single byte
///     bytes: vec![11, 42],             // 11 = Uint8 , 42 = Actual number
/// }
/// ```
///
/// ## UTF-8 String
/// ```rust
/// use yad::constants::*;
///
/// yad::core::Value {
///     r#type: types::Type::String,
///     length: length::ByteLength::Two,  // Uses 1 byte to describe string length
///     bytes: vec![65, 5, b'H', b'e', b'l', b'l', b'o'],
///     // 65 = Type and length indicator , 5 = Length of the string
/// }
/// ```
///
/// # Notes
/// - A [`Value`] does not interpret its own contents; it only stores type,
///   length, and raw bytes. Interpretation happens during deserialization or decoding.
/// - The combination of `type` + `length` + `bytes` must always respect
///   the **YAD binary specification** for validity.
///
/// # See Also
/// - [`Type`] for all supported value types.
/// - [`ByteLength`] for encoding how many bytes are used to store lengths.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Value {
    /// The type of the value, represented as a byte (e.g., `0x11`).
    ///
    /// - Indicates how the value should be interpreted.
    /// - Example: `0x11` represents an unsigned 8-bit integer (`u8`).
    pub r#type: Type,
    /// Represents the length of the value.
    ///
    /// - For numeric types, this indicates the number of bytes used to store the number.
    /// - For arrays and strings, the following byte(s) specify the length of the collection:
    ///   - For example, if `length` is 2 bytes, the next two bytes should be interpreted as a `u16`
    ///     representing the number of elements in the array or the number of UTF-8 bytes in the string.
    pub length: ByteLength,
    /// The raw byte representation of the value.
    ///
    /// - Contains the actual data of the value as a sequence of bytes (`Vec<u8>`).
    /// - Interpretation depends on the `type` and `length` fields.
    ///   For example, it may represent an integer, a floating-point number, a UTF-8 string, or an array.
    pub bytes: Vec<u8>,
}

impl Value {
    /// Decode a single `Value` from the provided byte vector (starting at index 0).
    ///
    /// This constructs a `Value` using your factory helpers when possible:
    /// - `from_number` for numeric types,
    /// - `from_string` for strings (uses `String::from_bytes`),
    /// - `from_bool` for booleans,
    /// - `from_vec` for arrays (parses nested elements recursively).
    ///
    /// The returned `Value` contains the entire encoded chunk (header + length-field + payload)
    /// in its `bytes` field. All length/bounds validations map to your `ErrorMessage` constants.
    pub fn decode(mut vec: Vec<u8>) -> Result<Self, ErrorMessage> {
        if vec.len() < 1 {
            Err(ErrorMessage(NOT_ENOUGH_BYTES))?
        }

        // helper: how many bytes does a Value starting at bytes[0] consume (header + len-field + payload)
        fn consumed_for_value(bytes: &[u8]) -> Result<usize, ErrorMessage> {
            if bytes.is_empty() {
                return Err(ErrorMessage(NOT_ENOUGH_BYTES));
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
                        let consumed = consumed_for_value(&bytes[pos..])?;
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
                // use your numeric factory
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

                // payload bytes (string) — use your String::from_bytes helper as in other places
                let payload = &vec[(1 + len_field_size)..total];
                let s = String::from_bytes(payload)?;
                Self::from_string(s)
            }

            Type::Array => {
                // parse elements individually (validate each element) and then use from_vec
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

                let mut elements: Vec<Self> = Vec::with_capacity(count);
                let mut pos = 1 + len_field_size;
                for _ in 0..count {
                    if pos >= vec.len() { Err(ErrorMessage(NOT_ENOUGH_BYTES))? }
                    let consumed = consumed_for_value(&vec[pos..])?;
                    let chunk = vec[pos..pos + consumed].to_vec();
                    // recursively decode each element (chunk contains a whole value)
                    let element = Self::decode(chunk)?;
                    elements.push(element);
                    pos += consumed;
                }

                // build the array Value using your factory
                Self::from_vec(elements)
            }

            Type::Bool | Type::False | Type::True => {
                Self::from_bool(r#type != Type::False)
            }
        }
    }

    /// Takes a `Vec<u8>` and decodes it like a number.
    ///
    /// You can create an unsigned integer, a signed integer, or a floating point number from 8 bits to 64 bits.
    pub fn from_number(mut vec: Vec<u8>) -> Result<Self, ErrorMessage> {
        if vec.len() < 1 {
            Err(ErrorMessage(NOT_ENOUGH_BYTES))?
        }

        let chunk_a = vec.remove(0);

        if chunk_a & 0xF0 > FLOATING_POINT_TYPE {
            Err(ErrorMessage(NOT_A_NUMBER))?;
        }

        let format = Type::try_from(chunk_a)?;
        let byte_length = ByteLength::try_from(chunk_a)?;

        if vec.len() < u8::from(byte_length) as usize {
            Err(ErrorMessage(NOT_ENOUGH_BYTES))?
        }

        let mut bytes = Vec::with_capacity(1 + byte_length as usize);
        bytes.push(chunk_a);
        bytes.extend(vec.drain(0..byte_length as usize));

        Ok(Self {
            r#type: format,
            length: byte_length,
            bytes
        })
    }

    /// Returns the "isolated" bytes of the value, ignoring metadata bytes.
    ///
    /// - For numeric types, ignores the type byte.
    /// - For arrays and strings, ignores the type byte and the length descriptor bytes.
    pub fn isolate_value_bytes(&self) -> &[u8] {
        let start = if self.r#type <= Type::Float {
            1
        } else {
            (self.length as u8 + 1) as usize
        };

        &self.bytes[start..]
    }

    /// Converts your value to u8.
    pub fn as_u8(&self) -> Result<u8, ErrorMessage> {
        if self.r#type != Type::Uint || self.length != ByteLength::One {
            Err(ErrorMessage(NOT_A_UINT8))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 1 byte
        let bytes: [u8; 1] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_UINT8))?;

        Ok(u8::from_be_bytes(bytes))
    }

    /// Convert your u8 to a YAD Value Structure
    pub fn from_u8(num: u8) -> Self {
        let r#type = Type::Uint;
        let length = ByteLength::One;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to i8.
    pub fn as_i8(&self) -> Result<i8, ErrorMessage> {
        if self.r#type != Type::Int || self.length != ByteLength::One {
            Err(ErrorMessage(NOT_A_INT8))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 1 byte
        let bytes: [u8; 1] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_INT8))?;

        Ok(i8::from_be_bytes(bytes))
    }

    /// Convert your i8 to a YAD Value Structure
    pub fn from_i8(num: i8) -> Self {
        let r#type = Type::Int;
        let length = ByteLength::One;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to f8.
    pub fn as_f8(&self) -> Result<F8E4M3, ErrorMessage> {
        if self.r#type != Type::Float || self.length != ByteLength::One {
            Err(ErrorMessage(NOT_A_FLOAT8))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 1 byte
        let bytes: [u8; 1] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_FLOAT8))?;

        Ok(F8E4M3::from_bits(bytes[0]))
    }

    /// Convert your f8 to a YAD Value Structure
    pub fn from_f8(num: F8E4M3) -> Self {
        let r#type = Type::Float;
        let length = ByteLength::One;
        let num_as_be = num.to_bits();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.push(num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to u16.
    pub fn as_u16(&self) -> Result<u16, ErrorMessage> {
        if self.r#type != Type::Uint || self.length != ByteLength::Two {
            Err(ErrorMessage(NOT_A_UINT16))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 2 bytes
        let bytes: [u8; 2] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_UINT16))?;

        Ok(u16::from_be_bytes(bytes))
    }

    /// Convert your u16 to a YAD Value Structure
    pub fn from_u16(num: u16) -> Self {
        let r#type = Type::Uint;
        let length = ByteLength::Two;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to i16.
    pub fn as_i16(&self) -> Result<i16, ErrorMessage> {
        if self.r#type != Type::Int || self.length != ByteLength::Two {
            Err(ErrorMessage(NOT_A_INT16))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 2 bytes
        let bytes: [u8; 2] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_INT16))?;

        Ok(i16::from_be_bytes(bytes))
    }

    /// Convert your i16 to a YAD Value Structure
    pub fn from_i16(num: i16) -> Self {
        let r#type = Type::Int;
        let length = ByteLength::Two;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to f16.
    pub fn as_f16(&self) -> Result<f16, ErrorMessage> {
        if self.r#type != Type::Float || self.length != ByteLength::Two {
            Err(ErrorMessage(NOT_A_FLOAT16))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 2 bytes
        let bytes: [u8; 2] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_FLOAT16))?;

        Ok(f16::from_be_bytes(bytes))
    }

    /// Convert your f16 to a YAD Value Structure
    pub fn from_f16(num: f16) -> Self {
        let r#type = Type::Float;
        let length = ByteLength::Two;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to u32.
    pub fn as_u32(&self) -> Result<u32, ErrorMessage> {
        if self.r#type != Type::Uint || self.length != ByteLength::Four {
            Err(ErrorMessage(NOT_A_UINT32))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 2 bytes
        let bytes: [u8; 4] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_UINT32))?;

        Ok(u32::from_be_bytes(bytes))
    }

    /// Convert your u32 to a YAD Value Structure
    pub fn from_u32(num: u32) -> Self {
        let r#type = Type::Uint;
        let length = ByteLength::Four;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to i32.
    pub fn as_i32(&self) -> Result<i32, ErrorMessage> {
        if self.r#type != Type::Int || self.length != ByteLength::Four {
            Err(ErrorMessage(NOT_A_INT32))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 2 bytes
        let bytes: [u8; 4] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_INT32))?;

        Ok(i32::from_be_bytes(bytes))
    }

    /// Convert your i32 to a YAD Value Structure
    pub fn from_i32(num: i32) -> Self {
        let r#type = Type::Int;
        let length = ByteLength::Four;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to f32.
    pub fn as_f32(&self) -> Result<f32, ErrorMessage> {
        if self.r#type != Type::Float || self.length != ByteLength::Four {
            Err(ErrorMessage(NOT_A_FLOAT32))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 2 bytes
        let bytes: [u8; 4] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_FLOAT32))?;

        Ok(f32::from_be_bytes(bytes))
    }

    /// Convert your f32 to a YAD Value Structure
    pub fn from_f32(num: f32) -> Self {
        let r#type = Type::Float;
        let length = ByteLength::Four;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to u64.
    pub fn as_u64(&self) -> Result<u64, ErrorMessage> {
        if self.r#type != Type::Uint || self.length != ByteLength::Eight {
            Err(ErrorMessage(NOT_A_UINT64))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 2 bytes
        let bytes: [u8; 8] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_UINT64))?;

        Ok(u64::from_be_bytes(bytes))
    }

    /// Convert your u64 to a YAD Value Structure
    pub fn from_u64(num: u64) -> Self {
        let r#type = Type::Uint;
        let length = ByteLength::Eight;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to i64.
    pub fn as_i64(&self) -> Result<i64, ErrorMessage> {
        if self.r#type != Type::Int || self.length != ByteLength::Eight {
            Err(ErrorMessage(NOT_A_INT64))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 2 bytes
        let bytes: [u8; 8] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_INT64))?;

        Ok(i64::from_be_bytes(bytes))
    }

    /// Convert your i64 to a YAD Value Structure
    pub fn from_i64(num: i64) -> Self {
        let r#type = Type::Int;
        let length = ByteLength::Eight;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to f64.
    pub fn as_f64(&self) -> Result<f64, ErrorMessage> {
        if self.r#type != Type::Float || self.length != ByteLength::Eight {
            Err(ErrorMessage(NOT_A_FLOAT64))?;
        }

        let bytes = self.isolate_value_bytes();

        // Ensure we have exactly 2 bytes
        let bytes: [u8; 8] = bytes
            .try_into()
            .map_err(|_| ErrorMessage(NOT_A_FLOAT64))?;

        Ok(f64::from_be_bytes(bytes))
    }

    /// Convert your f64 to a YAD Value Structure
    pub fn from_f64(num: f64) -> Self {
        let r#type = Type::Float;
        let length = ByteLength::Eight;
        let num_as_be = num.to_be_bytes();

        let mut bytes = vec![u8::from(r#type) | u8::from(length)];
        bytes.extend_from_slice(&num_as_be);

        Self {
            r#type,
            length,
            bytes
        }
    }

    /// Converts your value to string.
    pub fn as_string(&self) -> Result<String, ErrorMessage> {
        if self.r#type != Type::String {
            Err(ErrorMessage(NOT_A_STRING))?;
        }

        let bytes = self.isolate_value_bytes();

        Ok(String::from_bytes(bytes)?)
    }

    /// Convert your string to a YAD Value Structure
    pub fn from_string(str: String) -> Result<Self, ErrorMessage> {
        let r#type = Type::String;
        let byte_length = match str.len() {
            l if l == 0 => {
                Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?
            }
            l if l <= u8::MAX as usize => {
                ByteLength::One
            }
            l if l <= u16::MAX as usize => {
                ByteLength::Two
            }
            l if l <= u32::MAX as usize => {
                ByteLength::Four
            }
            l if l <= u64::MAX as usize => {
                ByteLength::Eight
            }
            _ => {
                Err(ErrorMessage(STRING_MAX_LENGTH_EXCEEDED))?
            }
        };

        let mut bytes = vec![u8::from(r#type) | u8::from(byte_length)];

        match byte_length {
            ByteLength::One => {
                bytes.extend_from_slice(&(str.len() as u8).to_be_bytes());
            }
            ByteLength::Two => {
                bytes.extend_from_slice(&(str.len() as u16).to_be_bytes());
            }
            ByteLength::Four => {
                bytes.extend_from_slice(&(str.len() as u32).to_be_bytes());
            }
            ByteLength::Eight => {
                bytes.extend_from_slice(&(str.len() as u64).to_be_bytes());
            }
            _ => {
                Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?
            }
        }

        bytes.extend_from_slice(&str.as_bytes());

        Ok(Self {
            r#type,
            length: byte_length,
            bytes
        })
    }

    /// Converts your value to bool.
    pub fn as_bool(&self) -> Result<bool, ErrorMessage> {
        if u8::from(self.r#type) | 0x0F != u8::from(Type::Bool) {
            Err(ErrorMessage(NOT_A_BOOL))?;
        }

        Ok(self.r#type != Type::False)
    }

    /// Convert your bool to a YAD Value Structure
    pub fn from_bool(bool: bool) -> Result<Self, ErrorMessage> {
        let r#type = if bool { Type::True } else { Type::False };

        Ok(Self {
            r#type,
            length: ByteLength::Zero,
            bytes: vec![u8::from(r#type)]
        })
    }

    /// Converts your value to array.
    pub fn as_array(&self) -> Result<Vec<Self>, ErrorMessage> {
        if self.r#type != Type::Array {
            return Err(ErrorMessage(NOT_AN_ARRAY));
        }

        fn consumed_for_value(bytes: &[u8]) -> Result<usize, ErrorMessage> {
            if bytes.is_empty() {
                return Err(ErrorMessage(NOT_ENOUGH_BYTES));
            }

            let first = bytes[0];
            let r#type = Type::try_from(first)?;
            let len_field_size = usize::from(ByteLength::try_from(first)?);

            if bytes.len() < 1 + len_field_size {
                return Err(ErrorMessage(NOT_ENOUGH_BYTES));
            }

            match r#type {
                Type::Uint | Type::Int | Type::Float => {
                    Ok(1 + len_field_size)
                }
                Type::Bool | Type::True | Type::False => {
                    Ok(1)
                }
                Type::String => {
                    let str_len = match ByteLength::try_from(len_field_size as u8)? {
                        ByteLength::Zero => Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?,
                        ByteLength::One => bytes[1] as u64,
                        ByteLength::Two => u16::from_be_bytes(bytes[1..=2].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as u64,
                        ByteLength::Four => u32::from_be_bytes(bytes[1..=4].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as u64,
                        ByteLength::Eight => u64::from_be_bytes(bytes[1..=8].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?),
                    } as usize;

                    let total = 1 + len_field_size + str_len;
                    if bytes.len() < total {
                        return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                    }
                    Ok(total)
                }
                Type::Array => {
                    let count = match ByteLength::try_from(len_field_size as u8)? {
                        ByteLength::Zero => Err(ErrorMessage(VEC_OF_LENGTH_ZERO))?,
                        ByteLength::One => bytes[1] as u64,
                        ByteLength::Two => u16::from_be_bytes(bytes[1..=2].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as u64,
                        ByteLength::Four => u32::from_be_bytes(bytes[1..=4].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as u64,
                        ByteLength::Eight => u64::from_be_bytes(bytes[1..=8].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?),
                    } as usize;

                    let mut pos = 1 + len_field_size;
                    for _ in 0..count {
                        if pos >= bytes.len() {
                            return Err(ErrorMessage(NOT_ENOUGH_BYTES));
                        }
                        let consumed = consumed_for_value(&bytes[pos..])?;
                        pos = pos.checked_add(consumed).ok_or_else(|| ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))?;
                    }
                    Ok(pos)
                }
            }
        }

        let mut vec: Vec<Self> = Vec::new();
        let mut bytes = self.isolate_value_bytes().to_vec();

        while !bytes.is_empty() {
            let first = bytes[0];
            let r#type = Type::try_from(first)?;
            let byte_length = usize::from(ByteLength::try_from(first)?);

            match r#type {
                Type::Uint | Type::Int | Type::Float => {
                    let chunk = bytes[..=byte_length].to_vec();
                    bytes = bytes[(1 + byte_length)..].to_vec();

                    vec.push(Value::from_number(chunk)?);
                }
                Type::String => {
                    let length = match ByteLength::try_from(byte_length as u8)? {
                        ByteLength::Zero => Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?,
                        ByteLength::One => bytes[1] as u64,
                        ByteLength::Two => u16::from_be_bytes(bytes[1..=2].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as u64,
                        ByteLength::Four => u32::from_be_bytes(bytes[1..=4].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as u64,
                        ByteLength::Eight => u64::from_be_bytes(bytes[1..=8].try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?),
                    };

                    if (bytes.len() as u64) < length {
                        Err(ErrorMessage(NOT_ENOUGH_BYTES))?;
                    }

                    let string_bytes = &bytes.to_owned()[(1 + byte_length)..=1 + length as usize];

                    bytes = bytes[2 + length as usize..].to_vec();

                    vec.push(Value::from_string(String::from_bytes(string_bytes)?)?);
                }

                Type::Bool | Type::True | Type::False => {
                    vec.push(Value::from_bool(r#type != Type::False)?);
                    bytes = bytes[1..].to_vec();
                }
                Type::Array => {
                    let consumed = consumed_for_value(&bytes)?;
                    let chunk = bytes[..consumed].to_vec();
                    bytes = bytes[consumed..].to_vec();

                    vec.push(Self {
                        r#type: Type::Array,
                        length: ByteLength::try_from(first)?,
                        bytes: chunk,
                    });
                }
            }
        }

        Ok(vec)
    }

    /// Convert your Vec of YAD Value Structure to a YAD Value Structure
    pub fn from_vec(vec: Vec<Self>) -> Result<Self, ErrorMessage> {
        let r#type = Type::Array;
        let byte_length = match vec.len() {
            l if l == 0 => {
                Err(ErrorMessage(VEC_OF_LENGTH_ZERO))?
            }
            l if l <= u8::MAX as usize => {
                ByteLength::One
            }
            l if l <= u16::MAX as usize => {
                ByteLength::Two
            }
            l if l <= u32::MAX as usize => {
                ByteLength::Four
            }
            l if l <= u64::MAX as usize => {
                ByteLength::Eight
            }
            _ => {
                Err(ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))?
            }
        };

        let mut bytes = vec![u8::from(r#type) | u8::from(byte_length)];

        match byte_length {
            ByteLength::One => {
                bytes.extend_from_slice(&(vec.len() as u8).to_be_bytes());
            }
            ByteLength::Two => {
                bytes.extend_from_slice(&(vec.len() as u16).to_be_bytes());
            }
            ByteLength::Four => {
                bytes.extend_from_slice(&(vec.len() as u32).to_be_bytes());
            }
            ByteLength::Eight => {
                bytes.extend_from_slice(&(vec.len() as u64).to_be_bytes());
            }
            _ => {
                Err(ErrorMessage(VEC_OF_LENGTH_ZERO))?
            }
        }

        for i in vec {
            bytes.extend_from_slice(i.bytes.as_slice());
        }

        Ok(Self {
            r#type,
            length: byte_length,
            bytes
        })
    }
}

pub trait FromYADNotation: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ErrorMessage>;
}

impl FromYADNotation for String {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ErrorMessage> {
        if let Some(string) = String::from_utf8(Vec::from(bytes)).ok() {
            Ok(string)
        } else {
            Err(ErrorMessage(MALFORMED_UTF8))
        }
    }
}
