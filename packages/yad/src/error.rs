/// Error constants for YAD file parsing and validation.

/// The provided YAD file is malformed or corrupted.
pub const MALFORMED_FILE: &str = "The provided YAD file is malformed or corrupted.";

/// The version header of the YAD file is malformed or invalid.
pub const MALFORMED_VERSION_HEADER: &str = "The provided YAD file has a malformed version header.";

/// The given byte vector cannot be decoded as a valid key.
pub const MALFORMED_KEY_VECTOR: &str = "The provided vector cannot be decoded as a valid key.";

/// The given byte vector cannot be decoded as a valid key name.
pub const MALFORMED_KEY_NAME_VECTOR: &str = "The provided vector cannot be decoded as a valid key name.";

/// The key name must contain at least one character.
pub const KEY_NAME_OF_LENGTH_ZERO: &str = "Key names must contain at least one character.";

/// The given byte vector cannot be decoded as a valid row.
pub const MALFORMED_ROW_VECTOR: &str = "The provided vector cannot be decoded as a valid row.";

/// The given byte vector cannot be decoded as a valid row name.
pub const MALFORMED_ROW_NAME_VECTOR: &str = "The provided vector cannot be decoded as a valid row name.";

/// The row name must contain at least one character.
pub const ROW_NAME_OF_LENGTH_ZERO: &str = "Row names must contain at least one character.";
