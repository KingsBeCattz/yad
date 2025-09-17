/// Header bytes used in the YAD binary format.
pub const VERSION_HEADER: u8 = 0xF0; // Marks the start of the version segment.

pub const ROW_START_HEADER: u8 = 0xF1; // Marks the start of a row.
pub const ROW_NAME_HEADER: u8 = 0x60;  // Marks the beginning of a row's name.
pub const ROW_END_HEADER: u8 = 0xF2;   // Marks the end of a row.

pub const KEY_START_HEADER: u8 = 0xF3; // Marks the start of a key.
pub const KEY_NAME_HEADER: u8 = 0x70;  // Marks the beginning of a key's name.
pub const KEY_END_HEADER: u8 = 0xF4;   // Marks the end of a key.
