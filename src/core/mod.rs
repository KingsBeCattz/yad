use crate::constants::headers::{KEY_END, KEY_START, ROW_END, ROW_START};

mod version;
pub use version::Version;
mod value;
pub use value::Value;
mod key;
pub use key::Key;
mod row;
pub use row::Row;
mod yad;
pub use yad::*;

/// Splits a byte buffer into segments bounded by start and end markers.
///
/// This function iterates over a byte buffer and extracts all contiguous slices
/// that begin with the `start` byte and end with the `end` byte. Each segment
/// includes both the start and end markers.
///
/// # Type Parameters
/// - `B`: Any type implementing [`AsRef<Vec<u8>>`], allowing use of `Vec<u8>`
///   or references like `&Vec<u8>`.
///
/// # Parameters
/// - `bytes`: The raw byte buffer to segment.
/// - `start`: The byte that marks the beginning of a segment.
/// - `end`: The byte that marks the end of a segment.
///
/// # Returns
/// - A `Vec<Vec<u8>>`, where each inner vector represents a segment including
///   its start and end markers.
///
/// # Notes
/// - Segments that do not start with `start` or do not end with `end` are ignored.
/// - Nested segments are **not supported**; only top-level segments are captured.
/// - If a start marker is found without a matching end marker, it will be ignored.
///
/// # Examples
/// ```rust
/// use yad::core::segment;
///
/// let data = vec![0x01, 0x02, 0x03, 0x04, 0x02];
/// let segments = segment(data, &0x01, &0x04);
/// // segments = vec![vec![0x01, 0x02, 0x03, 0x04]]
/// ```
pub fn segment<B: AsRef<Vec<u8>>>(bytes: B, start: &u8, end: &u8) -> Vec<Vec<u8>> {
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

/// Splits a byte buffer into individual key segments using the YAD key markers.
///
/// # Type Parameters
/// - `B`: Any type implementing [`AsRef<Vec<u8>>`].
///
/// # Parameters
/// - `bytes`: The raw byte buffer containing serialized keys.
///
/// # Returns
/// - A vector of byte vectors, each representing a complete key including
///   [`KEY_START`] and [`KEY_END`] markers.
///
/// # Examples
/// ```rust
/// use yad::constants::headers::*;
/// use yad::core::segment_keys;
///
/// let bytes = vec![KEY_START, 1, KEY_END, KEY_START, 2, KEY_END];
/// let keys = segment_keys(bytes);
/// assert_eq!(keys.len(), 2);
/// ```
pub fn segment_keys<B: AsRef<Vec<u8>>>(bytes: B) -> Vec<Vec<u8>> {
    segment(bytes, &KEY_START, &KEY_END)
}

/// Splits a byte buffer into individual row segments using the YAD row markers.
///
/// # Type Parameters
/// - `B`: Any type implementing [`AsRef<Vec<u8>>`].
///
/// # Parameters
/// - `bytes`: The raw byte buffer containing serialized rows.
///
/// # Returns
/// - A vector of byte vectors, each representing a complete row including
///   [`ROW_START`] and [`ROW_END`] markers.
///
/// # Examples
/// ```rust
/// use yad::constants::headers::*;
/// use yad::core::segment_rows;
///
/// let bytes = vec![ROW_START, 1, ROW_END, ROW_START, 2, ROW_END];
/// let rows = segment_rows(bytes);
/// assert_eq!(rows.len(), 2);
/// ```
pub fn segment_rows<B: AsRef<Vec<u8>>>(bytes: B) -> Vec<Vec<u8>> {
    segment(bytes, &ROW_START, &ROW_END)
}