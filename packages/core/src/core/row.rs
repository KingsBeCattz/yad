use std::collections::HashMap;
use crate::constants::error::{ErrorMessage, MALFORMED_ROW_VECTOR, MALFORMED_UTF8, NOT_ENOUGH_BYTES, STRING_MAX_LENGTH_EXCEEDED, STRING_OF_LENGTH_ZERO, VEC_MAX_LENGTH_EXCEEDED};
use crate::constants::headers::{ROW_END, ROW_NAME, ROW_START};
use crate::constants::length::ByteLength;
use crate::core::key::Key;
use crate::core::{segment_keys, Value};

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
/// use yad_core::core::{Row, Key, Value};
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
/// use yad_core::core::Row;
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
    /// Creates a new [`Row`] instance.
    ///
    /// # Parameters
    /// - `name`: The unique name of the row.
    /// - `keys`: A map of keys to initialize the row with.
    ///
    /// # Returns
    /// - A new [`Row`] with the provided name and keys.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::{Row, Key, Value};
    ///
    /// let mut keys = HashMap::new();
    /// keys.insert("id".to_string(), Key::new("id".to_string(), Value::from_u8(42)));
    ///
    /// let row = Row::new("TestRow".to_string(), keys);
    /// assert_eq!(row.name, "TestRow");
    /// ```
    pub fn new(name: String, keys: HashMap<String, Key>) -> Self {
        Self { name, keys }
    }


    /// Decodes a serialized [`Row`] from its raw byte representation.
    ///
    /// # Parameters
    /// - `vec`: A vector of bytes representing a serialized row.
    ///
    /// # Returns
    /// - `Ok(Row)` if decoding succeeds.
    /// - `Err(ErrorMessage)` if the byte vector is malformed or inconsistent.
    ///
    /// # Errors
    /// - If the vector is too short or lacks start/end markers.
    /// - If the row name length is invalid or exceeds system limits.
    /// - If contained keys fail to decode.
    ///
    /// # Example
    /// ```rust
    /// use yad_core::core::Row;
    ///
    /// let encoded: Vec<u8> = vec![/* valid encoded row */];
    /// let result = Row::decode(encoded);
    ///
    /// assert!(result.is_ok() || result.is_err()); // depends on data
    /// ```
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

    /// Encodes the current [`Row`] into a byte vector.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)` containing the serialized representation.
    /// - `Err(ErrorMessage)` if the row is invalid (e.g., empty name).
    ///
    /// # Errors
    /// - Row name length exceeds supported size.
    /// - Invalid or empty name.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::{Row, Key, Value};
    ///
    /// let mut keys = HashMap::new();
    /// keys.insert("a".to_string(), Key::new("a".to_string(), Value::from_u8(1)));
    ///
    /// let row = Row::new("MyRow".to_string(), keys);
    /// let encoded = row.encode();
    ///
    /// assert!(encoded.is_ok());
    /// ```
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

    /// Retrieves an immutable reference to all keys contained in the row.
    ///
    /// This provides direct access to the internal [`HashMap`] of [`Key`] objects
    /// without allowing modification.
    ///
    /// # Returns
    /// - `&HashMap<String, Key>` containing all keys of the row.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::{Row, Value};
    ///
    /// let mut row = Row::new("User".to_string(), HashMap::new());
    /// row.new_key("active", Value::from_bool(true).unwrap());
    ///
    /// let keys = row.get_keys();
    /// assert!(keys.contains_key("active"));
    /// ```
    pub fn get_keys(&self) -> &HashMap<String, Key> {
        &self.keys
    }

    /// Retrieves a mutable reference to all keys contained in the row.
    ///
    /// This allows direct modification of the internal [`HashMap`] of [`Key`] objects,
    /// enabling insertion, removal, or mutation of existing keys.
    ///
    /// # Returns
    /// - `&mut HashMap<String, Key>` containing all keys of the row.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::{Row, Key, Value};
    ///
    /// let mut row = Row::new("Config".to_string(), HashMap::new());
    /// row.new_key("mode", Value::from_string("debug".to_string()).unwrap());
    ///
    /// let keys_mut = row.get_keys_mut();
    /// keys_mut.remove("mode");
    ///
    /// assert!(row.get_key("mode").is_none());
    /// ```
    pub fn get_keys_mut(&mut self) -> &mut HashMap<String, Key> {
        &mut self.keys
    }

    /// Adds a new [`Key`] to the row using the given name and [`Value`].
    ///
    /// This is a convenience wrapper around [`add_key`](Self::add_key).
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::{Row, Value};
    ///
    /// let mut row = Row::new("Config".to_string(), HashMap::new());
    /// row.new_key("theme", Value::from_string("dark".to_string()).unwrap());
    ///
    /// assert!(row.keys.contains_key("theme"));
    /// ```
    pub fn new_key(&mut self, key: &str, value: Value) {
        self.add_key(Key::new(key.to_string(), value))
    }

    /// Inserts a [`Key`] into the row, replacing any existing key with the same name.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::{Row, Key, Value};
    ///
    /// let mut row = Row::new("Settings".to_string(), HashMap::new());
    /// let key = Key::new("language".to_string(), Value::from_string("EN".to_string()).unwrap());
    /// row.add_key(key);
    ///
    /// assert!(row.keys.contains_key("language"));
    /// ```
    pub fn add_key(&mut self, key: Key) {
        self.keys.insert(key.name.clone(), key);
    }

    /// Retrieves an immutable reference to a [`Key`] by its name.
    ///
    /// # Returns
    /// - `Some(&Key)` if the key exists.
    /// - `None` if no key with the provided name is found.
    ///
    /// # Example
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use yad_core::core::{Row, Key, Value};
    /// let mut row = Row::new("Data".to_string(), HashMap::new());
    /// row.new_key("enabled", Value::from_bool(true).unwrap());
    ///
    /// assert!(row.get_key("enabled").is_some());
    /// assert!(row.get_key("missing").is_none());
    /// ```
    pub fn get_key(&self, key: &str) -> Option<&Key> {
        self.keys.get(key)
    }

    /// Retrieves a mutable reference to a [`Key`] by its name.
    ///
    /// # Returns
    /// - `Some(&mut Key)` if the key exists.
    /// - `None` if no key with the provided name is found.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::{Row, Value};
    ///
    /// let mut row = Row::new("Editable".to_string(), HashMap::new());
    /// row.new_key("count", Value::from_u8(1));
    ///
    /// if let Some(k) = row.get_key_mut("count") {
    ///     k.value = Value::from_u8(2);
    /// }
    ///
    /// assert_eq!(row.get_key("count").unwrap().value.as_u8().unwrap(), 2);
    /// ```
    pub fn get_key_mut(&mut self, key: &str) -> Option<&mut Key> {
        self.keys.get_mut(key)
    }

    /// Removes a [`Key`] from the row by its name.
    ///
    /// If the key does not exist, this operation does nothing.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::{Row, Value};
    ///
    /// let mut row = Row::new("Logs".to_string(), HashMap::new());
    /// row.new_key("last_id", Value::from_u8(5));
    ///
    /// row.remove_key("last_id");
    /// assert!(row.get_key("last_id").is_none());
    /// ```
    pub fn remove_key(&mut self, key: &str) {
        self.keys.remove(key);
    }
}