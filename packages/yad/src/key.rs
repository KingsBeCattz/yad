use std::fmt::{Debug, Display, Formatter};
use yad_core::constants::error::ErrorMessage;
use yad_core::constants::length::ByteLength;
use yad_core::constants::types::Type;
use yad_core::Value;
use crate::constants::{KEY_END_HEADER, KEY_NAME_HEADER, KEY_START_HEADER};
use crate::{encode_name, usize_from_slice_bytes};
use crate::error::{MALFORMED_KEY_NAME_VECTOR, MALFORMED_KEY_VECTOR};

/// Represents a **key-value pair** inside a row structure.
///
/// A [`Key`] stores both:
/// - A unique string identifier (`name`) within its parent row.
/// - An associated [`Value`] representing the stored data.
///
/// Keys can be serialized and deserialized into/from a custom
/// **binary format** defined by the YAD protocol.
///
/// # Binary Layout
/// ```text
/// +---------------+----------------------+------------------+---------------+
/// | Start Header  | Encoded Key Name     | Encoded Value    | End Header    |
/// +---------------+----------------------+------------------+---------------+
/// ```
///
/// # Fields
/// - `name`: Unique identifier of the key within its parent row.
/// - `value`: Data associated with the key.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Key {
    /// The unique name of the key within its parent row.
    pub name: String,
    /// The value associated with this key.
    pub value: Value,
}

impl Key {
    /// Creates a new [`Key`] instance from a name and a [`Value`].
    ///
    /// # Type Parameters
    /// - `S`: Any type convertible into a `String`.
    ///
    /// # Arguments
    /// - `name`: The unique name of the key.
    /// - `value`: The associated value.
    ///
    /// # Returns
    /// A new [`Key`] instance with the provided name and value.
    pub fn new<S: ToString>(name: S, value: Value) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }

    /// Updates the value stored in the key.
    ///
    /// # Arguments
    /// - `new_value`: The new [`Value`] to assign.
    pub fn set_value(&mut self, new_value: Value) -> () {
        self.value = new_value;
    }

    /// Checks if a byte matches the **key start header**.
    ///
    /// # Arguments
    /// - `byte`: The byte to check.
    ///
    /// # Returns
    /// `true` if the byte equals [`KEY_START_HEADER`], otherwise `false`.
    fn byte_is_key_start_header(byte: u8) -> bool {
        KEY_START_HEADER == byte
    }

    /// Checks if a byte matches the **key end header**.
    ///
    /// # Arguments
    /// - `byte`: The byte to check.
    ///
    /// # Returns
    /// `true` if the byte equals [`KEY_END_HEADER`], otherwise `false`.
    fn byte_is_key_end_header(byte: u8) -> bool {
        KEY_END_HEADER == byte
    }

    /// Checks if a byte matches the **key name header** pattern.
    ///
    /// # Arguments
    /// - `byte`: The byte to check.
    ///
    /// # Returns
    /// `true` if the byte matches the name header mask, otherwise `false`.
    fn byte_is_key_name_header(byte: u8) -> bool {
        KEY_NAME_HEADER == (byte & 0xF0)
    }

    /// Validates that the first and last bytes of a byte vector
    /// correctly match the **key boundary headers**.
    ///
    /// # Arguments
    /// - `bytes`: Reference to the byte vector to validate.
    ///
    /// # Returns
    /// - `true`: If the vector has valid start and end headers.
    /// - `false`: Otherwise.
    fn check_boundary_bytes(bytes: &Vec<u8>) -> bool {
        let Some(first) = bytes.first() else {
            return false;
        };
        let Some(last) = bytes.last() else {
            return false;
        };

        Self::byte_is_key_start_header(*first) && Self::byte_is_key_end_header(*last)
    }

    /// Extracts and decodes a key's name from its binary representation.
    ///
    /// # Arguments
    /// - `bytes`: Byte vector containing the encoded key name.
    ///
    /// # Returns
    /// - `Some(String)`: Successfully decoded UTF-8 key name.
    /// - `None`: If validation or decoding fails.
    fn find_and_decode_name_from_bytes(bytes: Vec<u8>) -> Option<String> {
        if bytes.is_empty() {
            return None;
        }

        let first = *bytes.get(0)?;

        if !Self::byte_is_key_name_header(first) {
            return None;
        }

        let byte_length = ByteLength::try_from(first).ok()?;
        let be_length = usize_from_slice_bytes(&bytes[1..], byte_length)?;

        let metadata_length = 1 + byte_length.as_byte_count() as usize;

        if bytes.len() < metadata_length + be_length {
            return None;
        }

        let string_bytes = &bytes[metadata_length..metadata_length + be_length];

        // Attempt UTF-8 decoding
        String::from_utf8(string_bytes.to_vec()).ok()
    }

    /// Serializes the [`Key`] into its custom binary representation.
    ///
    /// The layout includes:
    /// - Start header
    /// - Encoded name (with metadata)
    /// - Encoded value
    /// - End header
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Binary representation of the key.
    /// - `Err(ErrorMessage)`: If name encoding or value serialization fails.
    pub fn serialize(&self) -> Result<Vec<u8>, ErrorMessage> {
        let mut bytes: Vec<u8> = vec![KEY_START_HEADER];

        bytes.extend_from_slice(encode_name(&self.name, KEY_NAME_HEADER)?.as_slice());
        bytes.extend_from_slice(self.value.bytes.as_slice());
        bytes.push(KEY_END_HEADER);

        Ok(bytes)
    }

    /// Deserializes a [`Key`] from its custom binary representation.
    ///
    /// # Arguments
    /// - `bytes`: Byte vector containing the serialized key.
    ///
    /// # Returns
    /// - `Ok(Key)`: Successfully decoded key.
    /// - `Err(ErrorMessage)`: If validation or decoding fails.
    pub fn deserialize(bytes: Vec<u8>) -> Result<Self, ErrorMessage> {
        // Validate headers
        if !Self::check_boundary_bytes(&bytes) {
            return Err(ErrorMessage(MALFORMED_KEY_VECTOR));
        }

        // Decode key name
        let name = Self::find_and_decode_name_from_bytes(bytes[1..].to_vec())
            .ok_or_else(|| ErrorMessage(MALFORMED_KEY_NAME_VECTOR))?;

        // Calculate name metadata length
        let name_metadata_length = 1 + ByteLength::One.as_byte_count() as usize + name.len();

        // Extract value bytes
        if bytes.len() < name_metadata_length + 2 {
            return Err(ErrorMessage(MALFORMED_KEY_VECTOR));
        }

        let value_bytes = &bytes[name_metadata_length + 1..bytes.len() - 1];
        let value = Value::decode(value_bytes.to_vec())?;

        Ok(Key { name, value })
    }
}

impl Display for Key {
    /// Formats the [`Key`] for human-readable display.
    ///
    /// Example:
    /// ```text
    /// myKey = 42
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}

impl Debug for Key {
    /// Provides a **debug-friendly representation** of the key,
    /// including its type and bit-length when applicable.
    ///
    /// # Formatting Rules
    /// - `String` / `Array`: Displays the plain value.
    /// - `Bool` / `True` / `False`: Displays the boolean value.
    /// - `Float`: Displays `<value>f<bit-length>`.
    /// - `Uint`: Displays `<value>u<bit-length>`.
    /// - `Int`: Displays `<value>i<bit-length>`.
    ///
    /// Example:
    /// ```text
    /// myKey = 123u32
    /// anotherKey = "hello"
    /// flag = true
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value_debug_format = match self.value.r#type {
            Type::String | Type::Array => format!("{}", self.value),
            Type::Bool | Type::True | Type::False => format!("{}", self.value),
            Type::Float => format!("{}f{}", self.value, self.value.length.as_byte_count() * 8),
            Type::Uint => format!("{}u{}", self.value, self.value.length.as_byte_count() * 8),
            Type::Int => format!("{}i{}", self.value, self.value.length.as_byte_count() * 8),
        };
        write!(f, "{} = {}", self.name, value_debug_format)
    }
}
