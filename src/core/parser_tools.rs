use crate::core::bytes::{YadLength, YAD_16_BITS, YAD_32_BITS, YAD_64_BITS, YAD_8_BITS, YAD_KEY_NAME, YAD_NULL, YAD_SIZE_BITS, YAD_VERSION_HEADER};
use crate::core::error_messages::{MALFORMED_FILE, UNEXPECTED, USIZE_OVERFLOW};
use crate::core::YanFile;

pub fn parse_usize(size: usize) -> Result<YadLength, String> {
    match size {
        s if s <= u8::MAX as usize => {
            Ok(YadLength::_8)
        }
        s if s <= u16::MAX as usize => {
            Ok(YadLength::_16)
        }
        s if s <= u32::MAX as usize => {
            Ok(YadLength::_32)
        }
        s if s <= u64::MAX as usize => {
            Ok(YadLength::_64)
        }
        _ => Err(USIZE_OVERFLOW)?,
    }
}

pub fn parse_version(file: YanFile) -> Result<[u8;3], String> {
    if file.get(0).is_none_or(|b| (b & YAD_VERSION_HEADER) != YAD_VERSION_HEADER) {
        Err(MALFORMED_FILE)?
    };

    let version_slice = &file[1..4];

    let version: [u8; 3] = version_slice.try_into().expect(UNEXPECTED);

    Ok(version)
}

pub fn parse_usize_to_bytes(mut value: usize) -> Vec<u8> {
    let mut bytes = Vec::new();

    while value > 0 {
        bytes.push((value & 0xFF) as u8);
        value >>= 8;
    }

    if bytes.is_empty() {
        bytes.push(0);
    }

    bytes
}

pub fn parse_row(file: YanFile) {}

pub fn parse_key(buff: Vec<u8>) -> Result<Vec<u8>, String> {
    if let Some(b) = buff.get(0) {
        if ((b & YAD_KEY_NAME) != YAD_KEY_NAME) || ((b & YAD_SIZE_BITS) == YAD_NULL) {
            Err(MALFORMED_FILE)?
        }
    } else {
        Err(MALFORMED_FILE)?
    };

    if let Some(b) = buff.get(1) {

    } else {
        Err(MALFORMED_FILE)?
    };

    Ok(buff)
}