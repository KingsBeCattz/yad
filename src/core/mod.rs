use crate::constants::headers::{KEY_END, KEY_START, ROW_END, ROW_START};

pub mod version;
pub mod value;
pub mod key;
pub mod row;
pub mod yad;

fn segment_keys<B: AsRef<Vec<u8>>>(bytes: B) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut inside = false;

    for b in bytes.as_ref() {
        if b == &KEY_START {
            current.clear();
            current.push(*b);
            inside = true;
        } else if b == &KEY_END && inside {
            current.push(*b);
            result.push(current.clone());
            current.clear();
            inside = false;
        } else if inside {
            current.push(*b);
        }
    }

    result
}

pub fn segment_rows<B: AsRef<Vec<u8>>>(bytes: B) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut inside = false;

    for b in bytes.as_ref() {
        if b == &ROW_START {
            current.clear();
            current.push(*b);
            inside = true;
        } else if b == &ROW_END && inside {
            current.push(*b);
            result.push(current.clone());
            current.clear();
            inside = false;
        } else if inside {
            current.push(*b);
        }
    }

    result
}