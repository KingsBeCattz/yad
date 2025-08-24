# YAD Core

**YAD** is a compact and efficient binary file format for structured data, inspired by JSON and BSON.

This crate provides the **Rust core implementation** of the YAD format, including:

- **Serialization & Deserialization** of YAD files
- APIs to **create and manipulate rows, keys, and values**
- **FFI bindings** for interoperability with other programming languages

📖 Learn more about the YAD project [here](https://github.com/KingsBeCattz/yad).

---

## ✨ Features

- Fast and lightweight binary format
- Human-readable structure (similar to JSON) with efficient storage (similar to BSON)
- Strongly typed rows, keys, and values
- Cross-language support via **FFI**

---

## 🚀 Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
yad-core = "0.1"
```

### Example: Creating and serializing a YAD file
```rust
use std::collections::HashMap;
use yad_core::core::{Key, Row, Value};
use yad_core::{deserialize, serialize, YAD};

fn main() {
    let mut yad = YAD::new();

    // Create a new user row
    let mut johan_user = Row::new("johan".to_string(), HashMap::new());

    // Add string key
    johan_user.keys.insert(
        "name".to_string(),
        Key::new("name".to_string(), Value::from_string("Johan".to_string()).unwrap())
    );

    // Add numeric key
    johan_user.keys.insert(
        "age".to_string(),
        Key::new("age".to_string(), Value::from_u8(17))
    );

    // Add a vector key
    let johan_projects = vec![Value::from_string("Yad".to_string()).unwrap()];
    johan_user.keys.insert(
        "projects".to_string(),
        Key::new("projects".to_string(), Value::from_vec(johan_projects).unwrap())
    );

    // Insert row into YAD
    yad.add_row(johan_user);

    // Serialize and save file
    let yad_bin = serialize(&yad).unwrap();
    std::fs::write("./examples/my_first_yad.yad", &yad_bin).unwrap();

    // Deserialize back and verify
    assert_eq!(yad, deserialize(yad_bin).unwrap());
}
```