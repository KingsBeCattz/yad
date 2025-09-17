# serde_yad

**serde_yad** is a Rust crate for binary serialization and deserialization of **YAD Core** structures.  
It takes the primitive `Value` types from **yad_core** (all supported types and their representations) and implements them in a binary format inspired by JSON/BSON.  
With **serde_yad**, you can serialize and deserialize entire YAD files, individual rows, keys, or single values.

---

## Features

- Serialize and deserialize `Value`, `Key`, and `Row`.
- Fully binary-oriented format inspired by JSON/BSON, supporting all primitive YAD types.
- Handles nested structures and arrays.
- Supports integers, floats (F8/F16/F32/F64), booleans, and strings.
- Can serialize/deserialize entire YAD files or individual elements.
- No external dependencies beyond Rust standard library.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde_yad = "1.0.0"
yad_core = "2.0.0"
```

---

## Usage

### Serialize a `Value` or `Key`

```rust
use serde_yad::{encode_value, encode_key, decode_value, decode_key};
use yad_core::{Value, Key};

let key = Key {
    name: "score".to_string(),
    value: Value::from(100),
};

// Encode key to binary
let bytes = encode_key(&key).unwrap();

// Decode back to Key
let decoded_key = decode_key(&bytes).unwrap();

assert_eq!(decoded_key.name, key.name);
assert_eq!(decoded_key.value, key.value);
```

### Serialize a `Row` with multiple `Key`s

```rust
use serde_yad::{encode_row, decode_row};
use yad_core::{Row, Key, Value};

let row = Row {
    name: "player1".to_string(),
    keys: vec![
        Key { name: "score".into(), value: Value::from(100) },
        Key { name: "level".into(), value: Value::from(5) },
    ],
};

// Encode row
let bytes = encode_row(&row).unwrap();

// Decode row
let decoded_row = decode_row(&bytes).unwrap();

assert_eq!(decoded_row.name, row.name);
assert_eq!(decoded_row.keys.len(), row.keys.len());
```

---

## Example: main.rs

This example demonstrates creating a YAD file, writing it to disk, and reading it back.

```rust
use yad_core::Value;
use serde_yad::key::Key;
use serde_yad::{Version, YAD};

fn write_a_new_yad() {
    let mut yad = YAD::new_empty(Version {
        major: 1,
        minor: 0,
        patch: 0,
        beta: 0,
    });

    yad.insert_row("johan", vec![
        Key::new("name", Value::try_from("Johan").unwrap())
    ]);

    let yad_path = "./examples/my_first_yad.yad";
    std::fs::write(yad_path, yad.serialize().unwrap()).unwrap();
}

fn read_a_yad() {
    let yad_path = "./examples/example.yad";
    let yad = YAD::deserialize(std::fs::read(yad_path).unwrap()).unwrap();
    println!("{}", yad);
}

fn main() {
    write_a_new_yad();
    read_a_yad();
}
```

This example demonstrates:

1. Creating a new empty YAD structure with `Version`.
2. Inserting a row with a `Key` containing a `Value`.
3. Serializing and saving the YAD to a file.
4. Reading a YAD file from disk and deserializing it.
5. Printing the YAD content.

---

## Binary Format Overview

- `ROW_START_HEADER (0xF1)` – marks the beginning of a row.
- `ROW_NAME_HEADER (0x60)` – row name follows.
- `ROW_END_HEADER (0xF2)` – marks the end of a row.
- `KEY_START_HEADER (0xF3)` – marks the beginning of a key.
- `KEY_NAME_HEADER (0x70)` – key name follows.

Each `Value` type has its own byte representation for efficient storage.

---

## Error Handling

All encoding/decoding functions return `Result<_, ErrorMessage>`.  
Common errors include:

- `MALFORMED_ROW_VECTOR` – invalid row binary.
- `MALFORMED_KEY_VECTOR` – invalid key binary.
- `MALFORMED_ROW_NAME_VECTOR` / `MALFORMED_KEY_NAME_VECTOR` – invalid names.
- `Value` type mismatches.

```rust
use serde_yad::{decode_key, ErrorMessage};

match decode_key(&invalid_bytes) {
    Ok(key) => println!("Decoded key: {:?}", key),
    Err(err) => eprintln!("Failed to decode: {:?}", err),
}
```

---

## Contributing

Contributions are welcome! Open an issue or submit a pull request.  
Ensure that new features preserve the binary format and add tests for all encoding/decoding operations.

---

## License

MIT License. See [LICENSE](LICENSE) for details.
