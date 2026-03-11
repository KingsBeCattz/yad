use std::path::PathBuf;
use yad_core::Value;
use serde_yad::key::Key;
use serde_yad::{Version, YAD};

/// Creates a new [`YAD`] document in memory with version 1.0.0 and a sample row.
///
/// The document contains a single row named `"johan"` with two keys:
/// - `"name"`: a UTF-8 string value `"Johan"`
/// - `"age"`: an unsigned 8-bit integer value `17`
///
/// # Returns
/// A populated [`YAD`] document ready to be serialized or inspected.
fn create_a_new_yad() -> YAD {
    let mut yad = YAD::new_empty(Version {
        major: 1,
        minor: 0,
        patch: 0,
        beta: 0,
    });

    yad.insert_row("johan", vec![
        Key::new("name", Value::try_from("Johan").unwrap()),
        Key::new("age", Value::try_from(17u8).unwrap())
    ]);

    yad
}

/// Serializes a [`YAD`] document and writes it to the given path.
///
/// # Arguments
/// - `yad`: Reference to the document to serialize.
/// - `path_buf`: Destination path of the file. The file will be created or overwritten.
///
/// # Panics
/// If serialization fails or if the file cannot be written.
fn write_a_new_yad(yad: &YAD, path_buf: &PathBuf) {
    std::fs::write(path_buf, yad.serialize().unwrap()).unwrap();
}

/// Reads a `.yad` file from disk and deserializes it into a [`YAD`] document.
///
/// # Arguments
/// - `path_buf`: Path to the `.yad` file to read.
///
/// # Returns
/// The deserialized [`YAD`] document.
///
/// # Panics
/// Panics if the file cannot be read or if the bytes are not a valid YAD document.
fn read_a_yad(path_buf: &PathBuf) -> YAD {
    YAD::deserialize(std::fs::read(path_buf).unwrap()).unwrap()
}

fn main() {
    let yad = create_a_new_yad();
    println!("Created yad:\n{}", yad);

    let yad_path = PathBuf::from("./examples/my_first_yad.yad");

    write_a_new_yad(&yad, &yad_path);
    println!("Wrote yad to {}", yad_path.display());

    // Read the file back from the disk and verify round-trip consistency.
    let read_yad = read_a_yad(&yad_path);
    println!("Read yad:\n{}", &read_yad);

    assert_eq!(yad, read_yad)
}