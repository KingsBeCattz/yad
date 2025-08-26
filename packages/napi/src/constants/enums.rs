use napi_derive::napi;

#[napi]
#[repr(u8)]
pub enum Type {
    Uint = 0x10,
    Int = 0x20,
    Float = 0x30,
    String = 0x40
}

#[napi]
#[repr(u8)]
pub enum ByteLength {
    One = 0x01,
    Two = 0x02,
    Four = 0x03,
    Eight = 0x04
}
