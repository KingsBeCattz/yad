use yad_core::Value;

fn main() {
    // println!("String: {}", Value::try_from("Hello!").unwrap());
    // println!("Unsigned Integer: {}", Value::from(324u16));
    // println!("Signed Integer: {}", Value::from(-28i8));
    // println!("Float: {}", Value::from(123.729304f32));
    // println!("Bool: {}", Value::from(false));
    println!("{:?}", Value::try_from(vec![Value::from(256u64), Value::from(255u8)]).unwrap());
    // let vec: Vec<Value> = value.try_into().unwrap();
    // println!("Vector: {:?}", vec);
}