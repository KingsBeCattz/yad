use crate::constants::error::ErrorMessage;
use crate::core::yad::YAD;

/// Serializes a [`YAD`] structure into its binary representation.
///
/// # Type Parameters
/// - `Y`: A generic type that implements [`AsRef<YAD>`].
///   This allows passing either a reference to a [`YAD`] instance or any type that
///   can be converted into a reference to [`YAD`] (e.g., `&YAD`, `Box<YAD>`, etc.).
///
/// # Parameters
/// - `yad`: The input value implementing [`AsRef<YAD>`] that will be serialized
///   into a `Vec<u8>`.
///
/// # Returns
/// - `Ok(Vec<u8>)`: A vector of bytes containing the serialized binary data
///   representation of the [`YAD`] structure.
/// - `Err(ErrorMessage)`: An error indicating why serialization failed, such as
///   invalid data, corrupted state, or unsupported structure within the [`YAD`].
///
/// # Errors
/// Returns [`ErrorMessage`] if the serialization process encounters issues, such as:
/// - Encountering invalid or incomplete data inside the [`YAD`] structure.
/// - Inconsistent or unsupported field values.
/// - Any internal logic error preventing a correct binary encoding.
///
/// # Examples
/// ```rust
/// let yad = yad::YAD::new(); // Assume YAD is properly constructed
/// let bytes = yad::serialize(&yad)?;
/// assert!(!bytes.is_empty());
/// ```
///
/// # Notes
/// - This function performs a **lossless** serialization: all relevant
///   information stored in the [`YAD`] structure is preserved in the binary output.
/// - The output format follows the internal specification of YAD's binary layout,
///   ensuring compatibility across serialization and deserialization steps.
///
/// # See Also
/// - [`deserialize`] for converting a binary `Vec<u8>` back into a [`YAD`].
pub fn serialize<Y: AsRef<YAD>>(yad: Y) -> Result<Vec<u8>, ErrorMessage> {
    let mut file: Vec<u8> = vec![];

    file.extend_from_slice(&yad.as_ref().version.serialize());

    for (_, row) in &yad.as_ref().rows {
        let slice = row.encode()?;
        file.extend_from_slice(slice.as_slice())
    }

    Ok(file)
}