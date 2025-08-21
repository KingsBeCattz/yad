use std::collections::HashMap;
use crate::constants::length::{ByteLength, ONE_BYTE_LENGTH};
use crate::constants::types::Type;
use crate::core::key::Key;
use crate::core::row::Row;
use crate::core::value::Value;
use crate::core::version::Version;
use crate::core::yad::YAD;
use crate::deserializer::deserialize;
use crate::serializer::serialize;

pub mod constants;
pub mod core;
pub mod serializer;
pub mod deserializer;

fn main() {
    // let original_u8 = 128u8;
    // let yad_u8 = Value::from_u8(original_u8.clone());
    // println!("u8 : {} -> {:?} -> {}", original_u8, yad_u8, yad_u8.as_u8().unwrap());
    //
    // let original_i8 = -120i8;
    // let yad_i8 = Value::from_i8(original_i8.clone());
    // println!("i8 : {} -> {:?} -> {}", original_i8, yad_i8, yad_i8.as_i8().unwrap());
    //
    // let original_f8 = float8::F8E4M3::from(-1.1);
    // let yad_f8 = Value::from_f8(original_f8.clone());
    // println!("f8 : {} -> {:?} -> {}", original_f8, yad_f8, yad_f8.as_f8().unwrap());
    //
    // let original_u16 = 23236;
    // let yad_u16 = Value::from_u16(original_u16.clone());
    // println!("u16 : {} -> {:?} -> {}", original_u16, yad_u16, yad_u16.as_u16().unwrap());
    //
    // let original_i16 = -120i16;
    // let yad_i16 = Value::from_i16(original_i16.clone());
    // println!("i16 : {} -> {:?} -> {}", original_i16, yad_i16, yad_i16.as_i16().unwrap());
    //
    // let original_f16 = float16::f16::from_f32(-1.1);
    // let yad_f16 = Value::from_f16(original_f16.clone());
    // println!("f16 : {} -> {:?} -> {}", original_f16, yad_f16, yad_f16.as_f16().unwrap());
    //
    // let original_u32 = 23236;
    // let yad_u32 = Value::from_u32(original_u32.clone());
    // println!("u32 : {} -> {:?} -> {}", original_u32, yad_u32, yad_u32.as_u32().unwrap());
    //
    // let original_i32 = -120i32;
    // let yad_i32 = Value::from_i32(original_i32.clone());
    // println!("i32 : {} -> {:?} -> {}", original_i32, yad_i32, yad_i32.as_i32().unwrap());
    //
    // let original_f32 = -1.1f32;
    // let yad_f32 = Value::from_f32(original_f32.clone());
    // println!("f32 : {} -> {:?} -> {}", original_f32, yad_f32, yad_f32.as_f32().unwrap());
    //
    // let original_u64 = 23236;
    // let yad_u64 = Value::from_u64(original_u64.clone());
    // println!("u64 : {} -> {:?} -> {}", original_u64, yad_u64, yad_u64.as_u64().unwrap());
    //
    // let original_i64 = -120i64;
    // let yad_i64 = Value::from_i64(original_i64.clone());
    // println!("i64 : {} -> {:?} -> {}", original_i64, yad_i64, yad_i64.as_i64().unwrap());
    //
    // let original_f64 = -1.1f64;
    // let yad_f64 = Value::from_f64(original_f64.clone());
    // println!("f64 : {} -> {:?} -> {}", original_f64, yad_f64, yad_f64.as_f64().unwrap());
    //
    // let original_true = true;
    // let yad_bool_true = Value::from_bool(original_true.clone()).unwrap();
    // println!("bool (true) : {} -> {:?} -> {}", original_true, yad_bool_true, yad_bool_true.as_bool().unwrap());
    //
    // let original_false = false;
    // let yad_bool_false = Value::from_bool(original_false.clone()).unwrap();
    // println!("bool (false) : {} -> {:?} -> {}", original_false, yad_bool_false, yad_bool_false.as_bool().unwrap());
    //
    // let original_string = String::from("Hola");
    // let yad_string = Value::from_string(original_string.clone()).unwrap();
    // println!("string : {} -> {:?} -> {}", original_string, yad_string, yad_string.as_string().unwrap());

    // let vec = vec![
    //     Value::from_u8(20),
    //     Value::from_u8(50),
    //     Value::from_i8(100),
    //     Value::from_string(String::from("Ola")).unwrap(),
    //     Value::from_bool(true).unwrap(),
    //     Value::from_bool(false).unwrap(),
    //     Value::from_vec(vec![Value::from_i16(127)]).unwrap(),
    //     Value::from_vec(vec![Value::from_i8(100)]).unwrap()
    // ];
    // let vec_as_value = Value::from_vec(vec.clone()).unwrap();
    // println!("original vec : {:?}", vec);
    // println!("vec as yad : {:?}", vec_as_value);
    // println!("vec as yad to vec : {:?}", vec_as_value.as_array().unwrap());

    // let mut johan_keys = HashMap::new();
    //
    // let johan_key_age = Key::new(String::from("age"), Value::from_u8(17));
    // let johan_key_name = Key::new(String::from("name"), Value::from_string(String::from("Johan")).unwrap());
    // johan_keys.insert(johan_key_name.name.clone(), johan_key_name);
    // johan_keys.insert(johan_key_age.name.clone(), johan_key_age);
    //
    // let mut silence_keys = HashMap::new();
    //
    // let silence_key_age = Key::new(String::from("age"), Value::from_u8(17));
    // let silence_key_name = Key::new(String::from("name"), Value::from_string(String::from("Silence")).unwrap());
    // silence_keys.insert(silence_key_name.name.clone(), silence_key_name);
    // silence_keys.insert(silence_key_age.name.clone(), silence_key_age);
    //
    // let silence_row = Row::new(String::from("silence"), silence_keys);
    // let johan_row = Row::new(String::from("johan"), johan_keys);
    //
    // let mut rows = HashMap::new();
    //
    // rows.insert(johan_row.name.clone(), johan_row);
    // rows.insert(silence_row.name.clone(), silence_row);
    //
    // let file = YAD { version: Version {
    //     major: 0,
    //     minor: 0,
    //     patch: 1,
    //     beta: 1
    // }, rows };
    //
    // let serialized = serialize(&file).unwrap();
    // let deserialized = deserialize(serialized.clone()).unwrap();
    //
    // for byte in serialized {
    //     print!("{:#X} ", byte);
    // }
    // println!();

    // assert_eq!(file, deserialized);
}
