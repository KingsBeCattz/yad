use crate::core::bytes::{YadType, YAD_KEY_NAME, YAD_ROW_END, YAD_ROW_NAME, YAD_ROW_START, YAD_VERSION_HEADER};
use crate::core::parser_tools::{parse_usize, parse_usize_to_bytes};

pub fn serialize_type_header(major: u8, minor: u8, fix: u8) -> [u8; 4] {
    [YAD_VERSION_HEADER, major, minor, fix]
}

pub fn serialize_row(name: String, content: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut row: Vec<u8> = vec![];

    row.push(YAD_ROW_START);
    row.push(YAD_ROW_NAME | parse_usize(name.len())?.value());
    for len_chunk in parse_usize_to_bytes(name.len()) {
        row.push(len_chunk);
    }
    for char in name.chars() {
        row.push(char as u8);
    }
    for byte in content.iter() {
        row.push(*byte);
    }

    row.push(YAD_ROW_END);

    Ok(row)
}

/* Author: Johan | Date: 8/17/2025 11:45 PM
This below lol
*/
pub fn serialize_key(name: String, key_type: YadType, content: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut row: Vec<u8> = vec![];

    row.push(YAD_KEY_NAME | parse_usize(name.len())?.value());
    for len_chunk in parse_usize_to_bytes(name.len()) {
        row.push(len_chunk);
    }
    for char in name.chars() {
        row.push(char as u8);
    }

    row.push(key_type.value() | parse_usize(name.len())?.value());

    Ok(row)
}