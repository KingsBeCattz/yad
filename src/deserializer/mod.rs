use std::collections::HashMap;
use crate::constants::error::ErrorMessage;
use crate::core::Row;
use crate::core::segment_rows;
use crate::core::Version;
use crate::core::YAD;

/// Deserializes a binary buffer into a [`YAD`] structure.
///
/// # Type Parameters
/// - `V`: A generic type that implements [`AsMut<Vec<u8>>`].
///   This allows flexibility in passing a mutable vector container, such as
///   `Vec<u8>`, `&mut Vec<u8>`, or other types that can yield a mutable reference
///   to a `Vec<u8>`.
///
/// # Parameters
/// - `vec`: A mutable vector of raw bytes (or any wrapper around it) that will be
///   consumed and decoded into a [`YAD`] structure.
///   The function may **mutate** or partially consume this buffer during parsing,
///   depending on the internal deserialization process.
///
/// # Returns
/// - `Ok(YAD)`: A fully reconstructed [`YAD`] instance parsed from the given binary data.
/// - `Err(ErrorMessage)`: An error if the provided binary data is invalid, incomplete,
///   or does not conform to the expected YAD binary format.
///
/// # Errors
/// This function returns [`ErrorMessage`] in cases such as:
/// - The binary sequence is corrupted or truncated.
/// - Unsupported or unknown byte markers are encountered.
/// - Internal inconsistencies prevent constructing a valid [`YAD`] instance.
///
/// # Examples
/// ```rust
/// let bytes: Vec<u8> = vec![/* valid serialized YAD data */];
/// let yad = yad::deserialize(bytes)?;
/// // `yad` now contains the reconstructed structure
/// ```
///
/// # Notes
/// - This function assumes that the input follows the **YAD binary specification**.
///   Any deviation or corruption in the binary layout will result in an error.
/// - The provided buffer may be **mutated or emptied** after the operation,
///   depending on how the deserializer consumes bytes.
///
/// # See Also
/// - [`serialize`] for converting a [`YAD`] instance into its binary form.
pub fn deserialize<V: AsMut<Vec<u8>>>(mut vec: V) -> Result<YAD, ErrorMessage> {
    let borrowed_vec: &mut Vec<u8> = vec.as_mut();
    let version = Version::decode(&borrowed_vec)?;
    borrowed_vec.drain(..=4usize);

    let mut rows = HashMap::new();

    for raw_row in segment_rows(borrowed_vec) {
        let row = Row::decode(raw_row)?;
        rows.insert(row.name.clone(), row);
    }

    Ok(YAD {
        version,
        rows
    })
}