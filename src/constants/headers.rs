/// Indicates the version; the following 4 bytes are the version in X.X.X-beta.X format.
pub const VERSION_HEADER: u8 = 0xF0;
/// This marks the beginning of a row.
pub const ROW_START: u8 = 0xF1;
/// This indicates the name of the current row. Works as a String.
///
/// NEEDS A LENGTH BYTE
pub const ROW_NAME: u8 = 0x60;
/// This marks the end of a row.
pub const ROW_END: u8 = 0xF2;
/// This marks the beginning of a key and his value.
pub const KEY_START: u8 = 0xF3;
/// This indicates the name of the current key. Works as a String.
///
/// NEEDS A LENGTH BYTE
pub const KEY_NAME: u8 = 0x70;
/// This marks the end of a key and his value.
pub const KEY_END: u8 = 0xF4;