use std::collections::HashMap;
use crate::constants::error::ErrorMessage;
use crate::core::row::Row;
use crate::core::segment_rows;
use crate::core::version::Version;
use crate::core::yad::YAD;

pub fn deserialize<V: AsMut<Vec<u8>>>(mut vec: V) -> Result<YAD, ErrorMessage> {
    let borrowed_vec: &mut Vec<u8> = vec.as_mut();
    let version = Version::decode(&borrowed_vec)?;
    borrowed_vec.drain(..=4usize);

    let mut rows = HashMap::new();

    for raw_row in segment_rows(borrowed_vec) {
        let row = Row::decode(raw_row)?;
        rows.insert(row.name.clone(), row);
    }

    Ok(YAD {
        version,
        rows
    })
}