# YAD Core

**YAD Core** is a Rust crate focused exclusively on the low-level **Value** type, supporting numbers, strings, booleans, and arrays.

> ⚠️ NOTE: THIS CRATE, WHICH PREVIOUSLY HANDLED FULL SERIALIZATION AND DESERIALIZATION, NOW ONLY MANAGES VALUES. The previous full functionality has been moved to `serde_yad`.

This crate provides:

* The **Value** type for primitive and array data
* A lightweight **FFI module** for cross-language interoperability

📖 Learn more about the YAD project [here](https://github.com/KingsBeCattz/yad).

---

## ✨ Features

* Fast and lightweight representation of numbers, strings, booleans, and arrays
* Strongly typed primitive values
* Cross-language support via **FFI**

---

## 🚀 Usage

Add the crate to your `Cargo.toml`:

```
cargo add yad_core
```

### Example: Creating various Values

```rust
use yad_core::Value;

fn main() {
    println!("String: {}", Value::try_from("Hello!").unwrap());
    println!("Unsigned Integer: {}", Value::from(324u16));
    println!("Signed Integer: {}", Value::from(-28i8));
    println!("Float: {}", Value::from(123.729304f32));
    println!("Bool: {}", Value::from(false));
    println!("Vector: {:?}", Value::try_from(vec![Value::from(256u64)]).unwrap());
}
```

### FFI Usage

The `ffi` module provides interoperability with other languages, exposing functions to create and manipulate `Value` types from external code.
