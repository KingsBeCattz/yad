use crate::constants::error::{ErrorMessage, MALFORMED_KEY_VECTOR, MALFORMED_UTF8, NOT_ENOUGH_BYTES, STRING_MAX_LENGTH_EXCEEDED, STRING_OF_LENGTH_ZERO, VEC_MAX_LENGTH_EXCEEDED};
use crate::constants::headers::{KEY_END, KEY_NAME, KEY_START};
use crate::constants::length::ByteLength;
use crate::core::value::Value;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Key {
    pub name: String,
    pub value: Value
}

impl Key {
    pub fn new(name: String, value: Value) -> Self {
        Self {
            name, value
        }
    }

    pub fn decode(mut vec: Vec<u8>) -> Result<Self, ErrorMessage> {
        if vec.len() < 4 || vec.remove(0) != KEY_START || vec.pop().unwrap_or(0x00) != KEY_END {
            Err(ErrorMessage::from(MALFORMED_KEY_VECTOR))?
        }

        let key_name_byte_header = vec.remove(0);
        let byte_header_length_header = ByteLength::try_from(key_name_byte_header)?;

        let key_name_length = match byte_header_length_header {
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

        let key_name_bytes: Vec<u8> = vec.drain(0..=key_name_length - 1).collect();

        let key_name = String::from_utf8(key_name_bytes).map_err(|_| ErrorMessage(MALFORMED_UTF8))?;

        Ok(Self {
            name: key_name,
            value: Value::decode(vec)?
        })
    }

    pub fn encode(&self) -> Result<Vec<u8>, ErrorMessage> {
        let key_name_byte = KEY_NAME;
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

        let mut bytes = vec![KEY_START, key_name_byte | u8::from(byte_length)];

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
        bytes.extend_from_slice(&self.value.bytes);
        bytes.push(KEY_END);

        Ok(bytes)
    }
}