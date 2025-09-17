use yad_core::Value;
use serde_yad::key::Key;
use serde_yad::{Version, YAD};

/// Creates a new YAD file and writes it to disk.
///
/// This function demonstrates how to create an empty YAD structure,
/// insert a single row with a key/value pair, and serialize it to a file.
fn write_a_new_yad() {
    // Create a new empty YAD structure with version 1.0.0
    let mut yad = YAD::new_empty(Version {
        major: 1,
        minor: 0,
        patch: 0,
        beta: 0,
    });

    // Insert a row named "johan" with a single key/value pair
    // Key "name" has the value "Johan" converted to a YAD Value
    yad.insert_row("johan", vec![
        Key::new("name", Value::try_from("Johan").unwrap())
    ]);

    // Path where the serialized YAD file will be saved
    let yad_path = "./examples/my_first_yad.yad";

    // Serialize the YAD structure to bytes and write to file
    std::fs::write(yad_path, yad.serialize().unwrap()).unwrap();
}

/// Reads a YAD file from disk and prints its content.
///
/// This function demonstrates deserialization of a YAD file
/// and displaying its content in a human-readable format.
fn read_a_yad() {
    // Path to the YAD file to read
    let yad_path = "./examples/example.yad";

    // Read the file into a byte vector and deserialize it into a YAD object
    let yad = YAD::deserialize(std::fs::read(yad_path).unwrap()).unwrap();

    // Print the YAD structure
    println!("{}", yad);
}

fn main() {
    // Write a new YAD file to disk
    write_a_new_yad();

    // Read and print a YAD file from disk
    read_a_yad();
}
