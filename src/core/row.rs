use std::collections::HashMap;
use crate::constants::error::{ErrorMessage, MALFORMED_ROW_VECTOR, MALFORMED_UTF8, NOT_ENOUGH_BYTES, STRING_MAX_LENGTH_EXCEEDED, STRING_OF_LENGTH_ZERO, VEC_MAX_LENGTH_EXCEEDED};
use crate::constants::headers::{ROW_END, ROW_NAME, ROW_START};
use crate::constants::length::ByteLength;
use crate::core::key::Key;
use crate::core::segment_keys;
use crate::core::value::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Row {
    pub name: String,
    pub keys: HashMap<String, Key>
}

impl Row {
    pub fn new(name: String, keys: HashMap<String, Key>) -> Self {
        Self {
            name, keys
        }
    }

    pub fn decode(mut vec: Vec<u8>) -> Result<Self, ErrorMessage> {
        if vec.len() < 4 || vec.remove(0) != ROW_START || vec.pop().unwrap_or(0x00) != ROW_END {
            Err(ErrorMessage::from(MALFORMED_ROW_VECTOR))?
        }

        let row_name_byte_header = vec.remove(0);
        let byte_header_length_header = ByteLength::try_from(row_name_byte_header)?;

        let row_name_length = match byte_header_length_header {
            ByteLength::Zero => Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?,
            ByteLength::One => vec.remove(0) as usize,
            ByteLength::Two => {
                let s = vec.drain(0..=1).collect::<Vec<u8>>();
                u16::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
            }
            ByteLength::Four => {
                let s = vec.drain(0..=3).collect::<Vec<u8>>();
                u32::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?) as usize
            }
            ByteLength::Eight => {
                let s = vec.drain(0..=7).collect::<Vec<u8>>();
                let v = u64::from_be_bytes(s.try_into().map_err(|_| ErrorMessage(NOT_ENOUGH_BYTES))?);
                if v as usize > usize::MAX { Err(ErrorMessage(VEC_MAX_LENGTH_EXCEEDED))? }
                v as usize
            }
        };

        let row_name_bytes: Vec<u8> = vec.drain(0..=row_name_length - 1).collect();

        let row_name = String::from_utf8(row_name_bytes).map_err(|_| ErrorMessage(MALFORMED_UTF8))?;

        let mut row_keys: HashMap<String, Key> = HashMap::new();

        for raw_key in segment_keys(vec) {
            let decoded_key = Key::decode(raw_key)?;
            row_keys.insert(decoded_key.name.clone(), decoded_key);
        }

        Ok(Self {
            name: row_name,
            keys: row_keys
        })
    }
    pub fn encode(&self) -> Result<Vec<u8>, ErrorMessage> {
        let row_name_byte = ROW_NAME;
        let byte_length = match self.name.len() {
            l if l == 0 => {
                Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?
            }
            l if l <= u8::MAX as usize => {
                ByteLength::One
            }
            l if l <= u16::MAX as usize => {
                ByteLength::Two
            }
            l if l <= u32::MAX as usize => {
                ByteLength::Four
            }
            l if l <= u64::MAX as usize => {
                ByteLength::Eight
            }
            _ => {
                Err(ErrorMessage(STRING_MAX_LENGTH_EXCEEDED))?
            }
        };

        let mut bytes = vec![ROW_START, row_name_byte | u8::from(byte_length)];

        match byte_length {
            ByteLength::One => {
                bytes.extend_from_slice(&(self.name.len() as u8).to_be_bytes());
            }
            ByteLength::Two => {
                bytes.extend_from_slice(&(self.name.len() as u16).to_be_bytes());
            }
            ByteLength::Four => {
                bytes.extend_from_slice(&(self.name.len() as u32).to_be_bytes());
            }
            ByteLength::Eight => {
                bytes.extend_from_slice(&(self.name.len() as u64).to_be_bytes());
            }
            _ => {
                Err(ErrorMessage(STRING_OF_LENGTH_ZERO))?
            }
        }

        bytes.extend_from_slice(&self.name.as_bytes());

        for (_, key) in &self.keys {
            bytes.extend_from_slice(key.encode()?.as_slice());
        }

        bytes.push(ROW_END);

        Ok(bytes)
    }
}