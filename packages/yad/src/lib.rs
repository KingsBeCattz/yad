pub mod constants;
pub mod error;
pub mod key;
pub mod row;
pub mod ffi;

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use yad_core;
use yad_core::constants::error::ErrorMessage;
use yad_core::constants::length::ByteLength;
pub use yad_core::Value;

use crate::constants::{KEY_END_HEADER, KEY_START_HEADER, ROW_END_HEADER, ROW_START_HEADER, VERSION_HEADER};
use crate::error::{MALFORMED_FILE, MALFORMED_VERSION_HEADER};
use crate::key::Key;
use crate::row::Row;

/// Encodes a string name into a serialized binary representation using a header byte.
///
/// The header byte is combined with the length nibble to indicate the type of
/// entity being encoded (row or key).
///
/// # Type Parameters
/// - `S`: Any type implementing [`ToString`].
///
/// # Parameters
/// - `name`: Reference to the string to encode.
/// - `header`: The header byte used to mark the type (row or key).
///
/// # Returns
/// - `Ok(Vec<u8>)`: The encoded byte vector.
/// - `Err(ErrorMessage)`: If conversion fails.
pub(crate) fn encode_name<S: ToString>(name: &S, header: u8) -> Result<Vec<u8>, ErrorMessage> {
    let mut encoded_name = Value::try_from(name.to_string())?.bytes;

    if let Some(first_byte) = encoded_name.get_mut(0) {
        let length_nibble = *first_byte & 0x0F;
        *first_byte = header | length_nibble;
    }

    Ok(encoded_name)
}

/// Interprets a byte slice as a big-endian unsigned integer of a given byte length.
///
/// # Parameters
/// - `slice`: Slice of bytes to parse.
/// - `byte_length`: The expected byte length (0, 1, 2, 4, 8).
///
/// # Returns
/// - `Some(usize)`: Parsed value if successful.
/// - `None`: If the slice is too short or conversion fails.
pub(crate) fn usize_from_slice_bytes(slice: &[u8], byte_length: ByteLength) -> Option<usize> {
    match byte_length {
        ByteLength::Zero => Some(0),
        ByteLength::One => slice.get(0).map(|&b| b as usize),
        ByteLength::Two => slice.get(0..2)
            .and_then(|s| s.try_into().ok())
            .map(|arr: [u8; 2]| u16::from_be_bytes(arr) as usize),
        ByteLength::Four => slice.get(0..4)
            .and_then(|s| s.try_into().ok())
            .map(|arr: [u8; 4]| u32::from_be_bytes(arr) as usize),
        ByteLength::Eight => slice.get(0..8)
            .and_then(|s| s.try_into().ok())
            .map(|arr: [u8; 8]| u64::from_be_bytes(arr) as usize),
    }
}

/// Generic function to segment a byte buffer into sub-slices bounded by `start` and `end` bytes.
///
/// # Parameters
/// - `bytes`: Byte buffer to split.
/// - `start`: Start marker byte.
/// - `end`: End marker byte.
///
/// # Returns
/// - `Vec<Vec<u8>>`: Each element is a sub-slice including start and end markers.
///
/// # Notes
/// - Segments missing either marker are ignored.
/// - Nested segments are **not supported**.
pub(crate) fn segment<B: AsRef<Vec<u8>>>(bytes: B, start: &u8, end: &u8) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut inside = false;

    for b in bytes.as_ref() {
        if b == start {
            current.clear();
            current.push(*b);
            inside = true;
        } else if b == end && inside {
            current.push(*b);
            result.push(current.clone());
            current.clear();
            inside = false;
        } else if inside {
            current.push(*b);
        }
    }

    result
}

/// Segments a byte buffer into individual key byte sequences, including start and end markers.
pub(crate) fn segment_keys<B: AsRef<Vec<u8>>>(bytes: B) -> Vec<Vec<u8>> {
    segment(bytes, &KEY_START_HEADER, &KEY_END_HEADER)
}

/// Segments a byte buffer into individual row byte sequences, including start and end markers.
pub(crate) fn segment_rows<B: AsRef<Vec<u8>>>(bytes: B) -> Vec<Vec<u8>> {
    segment(bytes, &ROW_START_HEADER, &ROW_END_HEADER)
}

/// Represents a semantic version of the YAD file format.
///
/// Versioning uses: major, minor, patch, and beta (pre-release).
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Version {
    /// Major version (breaking changes)
    pub major: u8,
    /// Minor version (new features)
    pub minor: u8,
    /// Patch version (bug fixes)
    pub patch: u8,
    /// Beta or pre-release identifier (0 = stable)
    pub beta: u8,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, self.beta)
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}", self))
    }
}

impl Version {
    /// Serializes the version into 5 bytes: `[VERSION_HEADER, major, minor, patch, beta]`.
    pub fn serialize(&self) -> [u8; 5] {
        [VERSION_HEADER, self.major, self.minor, self.patch, self.beta]
    }

    /// Deserializes a version from a byte vector.
    ///
    /// # Errors
    /// Returns `ErrorMessage` if the header is missing or the byte slice is malformed.
    pub fn deserialize(bytes: Vec<u8>) -> Result<Self, ErrorMessage> {
        if bytes.is_empty() || bytes.first().map(|b| *b != VERSION_HEADER).unwrap_or(true) {
            Err(ErrorMessage(MALFORMED_FILE))?
        }

        if let Some(version) = bytes.get(1..=4) {
            if version.len() != 4 {
                Err(ErrorMessage(MALFORMED_VERSION_HEADER))?
            }

            Ok(Version {
                major: version[0],
                minor: version[1],
                patch: version[2],
                beta: version[3],
            })
        } else {
            Err(ErrorMessage(MALFORMED_VERSION_HEADER))
        }
    }
}

/// Represents a full YAD document containing a version and multiple rows.
#[derive(Eq, PartialEq)]
pub struct YAD {
    /// Document version
    pub version: Version,
    /// Rows in the document, keyed by row name
    pub rows: HashMap<String, Row>,
}

impl YAD {
    /// Constructs a new YAD document from a version and a list of rows.
    pub fn new(version: Version, rows: Vec<Row>) -> Self {
        Self {
            version,
            rows: rows.into_iter().map(|r| (r.name.clone(), r)).collect(),
        }
    }

    /// Constructs an empty YAD document for a given version.
    pub fn new_empty(version: Version) -> Self {
        Self {
            version, rows: HashMap::new()
        }
    }

    /// Returns an immutable reference to the rows.
    pub fn get_rows(&self) -> &HashMap<String, Row> {
        &self.rows
    }

    /// Returns a mutable reference to the rows.
    pub fn get_rows_mut(&mut self) -> &mut HashMap<String, Row> {
        &mut self.rows
    }

    /// Inserts a new row into the document.
    pub fn insert_row<S: ToString>(&mut self, name: S, keys: Vec<Key>) {
        let rows = self.get_rows_mut();
        rows.insert(name.to_string(), Row::new(name, keys));
    }

    /// Removes a row by name, returning it if it existed.
    pub fn remove_row<S: ToString>(&mut self, name: S) -> Option<Row> {
        let rows = self.get_rows_mut();
        rows.remove(&name.to_string())
    }

    /// Serializes the YAD document to bytes: version + rows.
    pub fn serialize(&self) -> Result<Vec<u8>, ErrorMessage> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend_from_slice(&self.version.serialize());

        for (_name, row) in &self.rows {
            bytes.extend_from_slice(row.serialize()?.as_slice())
        }

        Ok(bytes)
    }

    /// Deserializes a YAD document from bytes.
    pub fn deserialize(mut bytes: Vec<u8>) -> Result<Self, ErrorMessage> {
        let version = Version::deserialize(bytes.drain(..=4).collect())?;
        let mut rows: Vec<Row> = vec![];

        for row_bytes in segment_rows(bytes) {
            rows.push(Row::deserialize(row_bytes)?)
        }

        Ok(Self::new(version, rows))
    }
}

impl Display for YAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut rows: Vec<String> = vec![];

        for (_n, row) in &self.rows {
            rows.push(format!("{}", row))
        }

        write!(f, "YAD {{ version = {}", self.version)?;
        write!(f, " ; rows = {{ {} }} ", rows.join("; "))?;
        write!(f, "}}")
    }
}

impl Debug for YAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut rows: Vec<String> = vec![];

        for (_n, row) in &self.rows {
            rows.push(format!("{:?}", row))
        }

        write!(f, "YAD {{ version = {}", self.version)?;
        write!(f, " ; rows = {{ {} }} ", rows.join("; "))?;
        write!(f, "}}")
    }
}
