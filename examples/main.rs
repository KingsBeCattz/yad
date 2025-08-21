use std::collections::HashMap;
use yad_core::core::{Key, Row, Value};
use yad_core::{deserialize, serialize, YAD};

fn main() {
    let mut yad = YAD::new();

    let mut johan_user = Row::new("johan".to_string(), HashMap::new());
    johan_user.keys.insert(
        "name".to_string(),
        Key::new("name".to_string(), Value::from_string("Johan".to_string()).unwrap())
    );
    johan_user.keys.insert(
        "age".to_string(),
        Key::new("age".to_string(), Value::from_u8(17))
    );
    let johan_proyects = vec![Value::from_string("Yad".to_string()).unwrap()];

    johan_user.keys.insert(
        "projects".to_string(),
        Key::new("projects".to_string(), Value::from_vec(johan_proyects).unwrap())
    );

    yad.add_row(johan_user);

    let yad_bin = serialize(&yad).unwrap();
    std::fs::write("./examples/my_first_yad.yad", &yad_bin).unwrap();

    assert_eq!(yad, deserialize(yad_bin).unwrap());
}