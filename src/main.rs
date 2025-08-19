use crate::core::bytes::{YAD_8_BITS, YAD_KEY_NAME};
use crate::core::parser_tools::{parse_key, parse_usize, parse_version};
use crate::core::serializer_tools::{serialize_row, serialize_type_header};

mod core;

fn main() {
    let example_file: Vec<u8> =
        vec![
            0xF0, 0x00, 0x00, 0x01,
            0xF1,
            0x64, 0x04, 0x75, 0x73, 0x65, 0x72,
            0x71, 0x04, 0x69, 0x64, 0x11, 0x2A,
            0x71, 0x04, 0x6E, 0x61, 0x6D, 0x65,
            0x41, 0x05, 0x4A, 0x6F, 0x68, 0x61, 0x6E,
            0xF2
        ];

    // let bytes = parse_key(vec![YAD_KEY_NAME | YAD_8_BITS]).unwrap();
    // for b in &bytes {
    //     print!("{:#02X} ", b);
    // }
    // println!();

    println!("{:?}", 1.1f16)

    // println!("{:?}", parse_key(vec![YAD_KEY_NAME]).unwrap());
}
