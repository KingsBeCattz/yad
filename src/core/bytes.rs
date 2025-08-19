// HEADERS
pub const YAD_VERSION_HEADER: u8 = 0xF0; // 240
pub const YAD_ROW_START: u8      = 0xF1; // 241
pub const YAD_ROW_END: u8        = 0xF2; // 242

// Types
pub const YAD_UNSIGNED_INT: u8 = 0x10; // 16
pub const YAD_SIGNED_INT: u8   = 0x20; // 32
pub const YAD_FLOAT: u8        = 0x30; // 48
pub const YAD_STRING: u8       = 0x40; // 64
pub const YAD_ARRAY: u8        = 0x50; // 80
pub const YAD_ROW_NAME: u8     = 0x60; // 96
pub const YAD_KEY_NAME: u8     = 0x70; // 112
pub const YAD_BOOL: u8         = 0x80; // 128

// SUB TYPES
pub const YAD_8_BITS: u8     = 0x01;
pub const YAD_16_BITS: u8    = 0x02;
pub const YAD_32_BITS: u8    = 0x03;
pub const YAD_64_BITS: u8    = 0x04;
pub const YAD_SIZE_BITS: u8  = 0x0F;

pub const YAD_BOOL_FALSE: u8 = 0x00;
pub const YAD_BOOL_TRUE: u8  = 0x01;

pub const YAD_NULL: u8 = 0x00;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YadLength {
    _8  = YAD_8_BITS,
    _16 = YAD_16_BITS,
    _32 = YAD_32_BITS,
    _64 = YAD_64_BITS,
}

impl YadLength {
    pub fn value(&self) -> u8 {
        *self as u8
    }
    pub fn max(&self) -> usize {
        match self {
            YadLength::_8 => u8::MAX as usize,
            YadLength::_16 => u16::MAX as usize,
            YadLength::_32 => u32::MAX as usize,
            YadLength::_64 => u64::MAX as usize,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YadBool {
    True = YAD_BOOL | YAD_BOOL_TRUE,
    False = YAD_BOOL | YAD_BOOL_FALSE
}

impl YadBool {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YadType {
    UnsignedInteger = YAD_UNSIGNED_INT,
    SignedInteger = YAD_SIGNED_INT,
    Float = YAD_FLOAT,
    String = YAD_STRING,
    Array = YAD_ARRAY,
    True = YAD_BOOL | YAD_BOOL_TRUE,
    False = YAD_BOOL | YAD_BOOL_FALSE
}

impl YadType {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}