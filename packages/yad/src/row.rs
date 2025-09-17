use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use yad_core::constants::error::ErrorMessage;
use yad_core::constants::length::ByteLength;
use yad_core::Value;
use crate::constants::{ROW_END_HEADER, ROW_NAME_HEADER, ROW_START_HEADER};
use crate::error::{MALFORMED_ROW_NAME_VECTOR, MALFORMED_ROW_VECTOR};
use crate::key::Key;
use crate::{encode_name, segment_keys, usize_from_slice_bytes};

/// Represents a **row structure** in the YAD binary format.
///
/// A [`Row`] acts as a container object that groups multiple [`Key`] instances
/// under a single unique name. Rows serve as high-level organizational units
/// when encoding or decoding structured YAD data.
///
/// # Binary Layout
/// ```text
/// +---------------+---------------------+-------------------+---------------+
/// | Start Header  | Encoded Row Name    | Encoded Keys...   | End Header    |
/// +---------------+---------------------+-------------------+---------------+
/// ```
///
/// # Fields
/// - `name`: A unique string identifier for the row.
/// - `keys`: A [`HashMap`] mapping key names to their associated [`Key`] objects.
#[derive(Clone, Eq, PartialEq)]
pub struct Row {
    /// The row’s unique identifier.
    pub name: String,
    /// The collection of keys belonging to this row.
    /// Keys are stored in a hashmap for fast lookup by name.
    pub keys: HashMap<String, Key>,
}

impl Row {
    /// Creates a new [`Row`] from a name and a vector of [`Key`] objects.
    ///
    /// # Type Parameters
    /// - `S`: Any type that can be converted into a [`String`].
    ///
    /// # Arguments
    /// - `name`: The unique name of the row.
    /// - `keys`: A vector of [`Key`] objects to attach to the row.
    ///
    /// # Returns
    /// A [`Row`] populated with the provided name and keys.
    pub fn new<S: ToString>(name: S, keys: Vec<Key>) -> Self {
        let keys = keys
            .into_iter()
            .map(|item| (item.name.to_owned(), item))
            .collect();
        Self {
            name: name.to_string(),
            keys,
        }
    }

    /// Creates a new [`Row`] with no keys.
    ///
    /// # Type Parameters
    /// - `S`: Any type that can be converted into a [`String`].
    ///
    /// # Arguments
    /// - `name`: The unique name of the row.
    ///
    /// # Returns
    /// A [`Row`] containing the given name but no keys.
    pub fn new_empty<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
            keys: HashMap::new(),
        }
    }

    /// Returns an immutable reference to the row’s key collection.
    pub fn get_keys(&self) -> &HashMap<String, Key> {
        &self.keys
    }

    /// Returns a mutable reference to the row’s key collection.
    pub fn get_keys_mut(&mut self) -> &mut HashMap<String, Key> {
        &mut self.keys
    }

    /// Inserts a new [`Key`] into the row.
    ///
    /// If a key with the same name already exists, it will be replaced.
    ///
    /// # Arguments
    /// - `name`: The unique name of the key.
    /// - `value`: The value associated with the key.
    pub fn insert_key<S: ToString>(&mut self, name: S, value: Value) {
        let rows = self.get_keys_mut();
        rows.insert(name.to_string(), Key::new(name, value));
    }

    /// Removes a [`Key`] from the row by its name.
    ///
    /// # Arguments
    /// - `name`: The name of the key to remove.
    ///
    /// # Returns
    /// - `Some(Key)`: The removed key if it existed.
    /// - `None`: If the key was not found.
    pub fn remove_key<S: ToString>(&mut self, name: S) -> Option<Key> {
        let rows = self.get_keys_mut();
        rows.remove(&name.to_string())
    }

    /// Checks if a byte matches the **row start header** marker.
    fn byte_is_row_start_header(byte: u8) -> bool {
        ROW_START_HEADER == byte
    }

    /// Checks if a byte matches the **row end header** marker.
    fn byte_is_row_end_header(byte: u8) -> bool {
        ROW_END_HEADER == byte
    }

    /// Checks if a byte matches the **row name header** marker.
    fn byte_is_row_name_header(byte: u8) -> bool {
        ROW_NAME_HEADER == (byte & 0xF0)
    }

    /// Validates that the first and last bytes of a vector
    /// correspond to valid **row boundary headers**.
    ///
    /// # Arguments
    /// - `bytes`: The byte vector to validate.
    ///
    /// # Returns
    /// - `true`: If both start and end headers are valid.
    /// - `false`: Otherwise.
    fn check_boundary_bytes(bytes: &Vec<u8>) -> bool {
        let Some(first) = bytes.first() else {
            return false;
        };
        let Some(last) = bytes.last() else {
            return false;
        };

        Self::byte_is_row_start_header(*first) && Self::byte_is_row_end_header(*last)
    }

    /// Extracts and decodes the row’s name from its binary representation.
    ///
    /// # Arguments
    /// - `bytes`: A byte vector containing the encoded row name and metadata.
    ///
    /// # Returns
    /// - `Some(String)`: The decoded row name if successful.
    /// - `None`: If validation fails or UTF-8 decoding fails.
    fn find_and_decode_name_from_bytes(bytes: Vec<u8>) -> Option<String> {
        if bytes.is_empty() {
            return None;
        }

        let first = *bytes.get(0)?;
        if !Self::byte_is_row_name_header(first) {
            return None;
        }

        let byte_length = ByteLength::try_from(first).ok()?;
        let be_length = usize_from_slice_bytes(&bytes[1..], byte_length)?;

        let metadata_length = 1 + byte_length.as_byte_count() as usize;

        if bytes.len() < metadata_length + be_length {
            return None;
        }

        let string_bytes = &bytes[metadata_length..metadata_length + be_length];

        String::from_utf8(string_bytes.to_vec()).ok()
    }

    /// Serializes the [`Row`] into its binary representation.
    ///
    /// # Layout
    /// - Start header
    /// - Encoded row name
    /// - Encoded keys
    /// - End header
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Binary representation of the row.
    /// - `Err(ErrorMessage)`: If name encoding or key serialization fails.
    pub fn serialize(&self) -> Result<Vec<u8>, ErrorMessage> {
        let mut bytes: Vec<u8> = vec![ROW_START_HEADER];

        bytes.extend_from_slice(encode_name(&self.name, ROW_NAME_HEADER)?.as_slice());

        for (_n, value) in &self.keys {
            bytes.extend_from_slice(value.serialize()?.as_slice());
        }

        bytes.push(ROW_END_HEADER);

        Ok(bytes)
    }

    /// Deserializes a [`Row`] from its binary representation.
    ///
    /// # Arguments
    /// - `bytes`: The serialized row data.
    ///
    /// # Returns
    /// - `Ok(Row)`: A decoded row if successful.
    /// - `Err(ErrorMessage)`: If boundary headers or name decoding fail.
    pub fn deserialize(bytes: Vec<u8>) -> Result<Self, ErrorMessage> {
        if !Self::check_boundary_bytes(&bytes) {
            return Err(ErrorMessage(MALFORMED_ROW_VECTOR));
        }

        let mut keys: Vec<Key> = vec![];

        for key_bytes in segment_keys(&bytes) {
            keys.push(Key::deserialize(key_bytes)?)
        }

        let name = Self::find_and_decode_name_from_bytes(bytes[1..].to_vec())
            .ok_or_else(|| ErrorMessage(MALFORMED_ROW_NAME_VECTOR))?;

        Ok(Self::new(name, keys))
    }
}

impl Display for Row {
    /// Formats the [`Row`] as a human-readable string.
    ///
    /// Example:
    /// ```text
    /// row_name = { key1 = value1; key2 = value2 }
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut keys: Vec<String> = vec![];

        for (_name, key) in &self.keys {
            keys.push(format!("{}", key))
        }

        write!(f, "{} = {{ {} }}", self.name, keys.join("; "))
    }
}

impl Debug for Row {
    /// Formats the [`Row`] with debug-friendly output.
    ///
    /// Example:
    /// ```text
    /// row_name = { Key { name: "key1", value: 123 }; Key { name: "key2", value: "abc" } }
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut keys: Vec<String> = vec![];

        for (_name, key) in &self.keys {
            keys.push(format!("{:?}", key))
        }

        write!(f, "{} = {{ {} }}", self.name, keys.join("; "))
    }
}
