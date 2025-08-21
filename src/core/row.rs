use std::collections::HashMap;
use crate::constants::error::{ErrorMessage, MALFORMED_ROW_VECTOR, MALFORMED_UTF8, NOT_ENOUGH_BYTES, STRING_MAX_LENGTH_EXCEEDED, STRING_OF_LENGTH_ZERO, VEC_MAX_LENGTH_EXCEEDED};
use crate::constants::headers::{ROW_END, ROW_NAME, ROW_START};
use crate::constants::length::ByteLength;
use crate::core::key::Key;
use crate::core::segment_keys;

/// Represents a single row within the YAD data structure.
///
/// A [`Row`] is a named collection of [`Key`] objects, each uniquely identified
/// by a string. Rows act as **logical containers** for keys, similar to documents
/// in NoSQL databases or records in relational databases.
///
/// # Fields
/// - [`name`](Row::name): The row's unique identifier.
///   Typically used as a human-readable label to reference the row.
/// - [`keys`](Row::keys): A map from string identifiers to [`Key`] objects,
///   representing the actual data stored in the row.
///
/// # Invariants
/// - Each key in [`keys`](Row::keys) must have a **unique name** within the row.
/// - The `name` field should not be empty for a valid row.
/// - A row may contain zero or more keys.
///
/// # Examples
/// ## Creating a Row with Keys
/// ```rust
/// use std::collections::HashMap;
/// use yad::core::{Row, Key, Value};
///
/// let mut keys = HashMap::new();
/// keys.insert("id".to_string(), Key::new("id".to_string(), Value::from_u8(1)));
/// keys.insert("username".to_string(), Key::new("username".to_string(), Value::from_string("Alice".to_string()).unwrap()));
///
/// let row = Row {
///     name: "User".to_string(),
///     keys,
/// };
///
/// assert_eq!(row.name, "User");
/// assert!(row.keys.contains_key("username"));
/// ```
///
/// ## Creating an Empty Row
/// ```rust
/// use std::collections::HashMap;
/// use yad::core::Row;
///
/// let row = Row {
///     name: "Empty".to_string(),
///     keys: HashMap::new(),
/// };
///
/// assert!(row.keys.is_empty());
/// ```
///
/// # Notes
/// - A [`Row`] is conceptually equivalent to a **document** or **record** in
///   other database systems.
/// - Keys in the row store strongly typed [`Value`] objects.
/// - Rows are the **second layer** of organization in YAD:
///   1. [`YAD`] – the top-level container holding multiple rows.
///   2. [`Row`] – holds multiple keys.
///   3. [`Key`] – holds one or more [`Value`] objects.
///
/// # See Also
/// - [`Key`] for details on individual data entries within a row.
/// - [`YAD`] for the top-level container that aggregates multiple rows.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Row {
    /// The unique name or identifier of the row.
    ///
    /// - Typically used to reference the row logically or semantically.
    /// - Should be non-empty for meaningful data organization.
    pub name: String,

    /// A mapping of string identifiers to [`Key`] objects.
    ///
    /// - Each key represents a stored field or property within the row.
    /// - The string identifier acts as the unique name of the field.
    pub keys: HashMap<String, Key>,
}


impl Row {
    pub fn new(name: String, keys: HashMap<String, Key>) -> Self {
        Self {
            name, keys
        }
    }

    pub fn decode(mut vec: Vec<u8>) -> Result<Self, ErrorMessage> {
        if vec.len() < 4 || vec.remove(0) != ROW_START || vec.pop().unwrap_or(0x00) != ROW_END {
            Err(ErrorMessage::from(MALFORMED_ROW_VECTOR))?
        }

        let row_name_byte_header = vec.remove(0);
        let byte_header_length_header = ByteLength::try_from(row_name_byte_header)?;

        let row_name_length = match byte_header_length_header {
            ByteLength::Zero => Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?,
            ByteLength::One => vec.remove(0) as usize,
            ByteLength::Two => {
                let s = vec.drain(0..=1).collect::<Vec<u8>>();
                u16::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
            }
            ByteLength::Four => {
                let s = vec.drain(0..=3).collect::<Vec<u8>>();
                u32::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
            }
            ByteLength::Eight => {
                let s = vec.drain(0..=7).collect::<Vec<u8>>();
                let v = u64::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?);
                if v as usize > usize::MAX { Err(ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))? }
                v as usize
            }
        };

        let row_name_bytes: Vec<u8> = vec.drain(0..=row_name_length - 1).collect();

        let row_name = String::from_utf8(row_name_bytes).map_err(|_| ErrorMessage(MALFORMED_UTF8))?;

        let mut row_keys: HashMap<String, Key> = HashMap::new();

        for raw_key in segment_keys(vec) {
            let decoded_key = Key::decode(raw_key)?;
            row_keys.insert(decoded_key.name.clone(), decoded_key);
        }

        Ok(Self {
            name: row_name,
            keys: row_keys
        })
    }
    pub fn encode(&self) -> Result<Vec<u8>, ErrorMessage> {
        let row_name_byte = ROW_NAME;
        let byte_length = match self.name.len() {
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

        let mut bytes = vec![ROW_START, row_name_byte | u8::from(byte_length)];

        match byte_length {
            ByteLength::One => {
                bytes.extend_from_slice(&(self.name.len() as u8).to_be_bytes());
            }
            ByteLength::Two => {
                bytes.extend_from_slice(&(self.name.len() as u16).to_be_bytes());
            }
            ByteLength::Four => {
                bytes.extend_from_slice(&(self.name.len() as u32).to_be_bytes());
            }
            ByteLength::Eight => {
                bytes.extend_from_slice(&(self.name.len() as u64).to_be_bytes());
            }
            _ => {
                Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?
            }
        }

        bytes.extend_from_slice(&self.name.as_bytes());

        for (_, key) in &self.keys {
            bytes.extend_from_slice(key.encode()?.as_slice());
        }

        bytes.push(ROW_END);

        Ok(bytes)
    }
}