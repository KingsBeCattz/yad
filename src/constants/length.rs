use crate::constants::error::{ErrorMessage, FAILED_TRANSFORMING_AN_U8_TO_VALID_LENGTH};

/// Indicates an 0-bit length.
pub const ZERO_BYTE_LENGTH: u8 = 0x00;

/// Indicates an 8-bit length.
pub const ONE_BYTE_LENGTH: u8 = 0x01;
/// Indicates an 16-bit length.
pub const TWO_BYTE_LENGTH: u8 = 0x02;
/// Indicates an 32-bit length.
pub const FOUR_BYTE_LENGTH: u8 = 0x03;
/// Indicates an 64-bit length.
pub const EIGHT_BYTE_LENGTH: u8 = 0x04;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum ByteLength {
    Zero = ZERO_BYTE_LENGTH,
    One = ONE_BYTE_LENGTH,
    Two = TWO_BYTE_LENGTH,
    Four = FOUR_BYTE_LENGTH,
    Eight = EIGHT_BYTE_LENGTH
}

impl TryFrom<u8> for ByteLength {
    type Error = ErrorMessage;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v & 0x0F == ZERO_BYTE_LENGTH => Ok(ByteLength::Zero),
            v if v & 0x0F == ONE_BYTE_LENGTH => Ok(ByteLength::One),
            v if v & 0x0F == TWO_BYTE_LENGTH => Ok(ByteLength::Two),
            v if v & 0x0F == FOUR_BYTE_LENGTH => Ok(ByteLength::Four),
            v if v & 0x0F == EIGHT_BYTE_LENGTH => Ok(ByteLength::Eight),
            _ => Err(ErrorMessage(FAILED_TRANSFORMING_AN_U8_TO_VALID_LENGTH)),
        }
    }
}

impl From<ByteLength> for u8 {
    fn from(t: ByteLength) -> u8 {
        (t as u8) & 0x0F
    }
}
impl From<ByteLength> for usize {
    fn from(t: ByteLength) -> usize {
        (t as usize) & 0x0F
    }
}

