use crate::constants::error::ErrorMessage;
use crate::core::yad::YAD;

pub fn serialize<Y: AsRef<YAD>>(yad: Y) -> Result<Vec<u8>, ErrorMessage> {
    let mut file: Vec<u8> = vec![];

    file.extend_from_slice(&yad.as_ref().version.serialize());

    for (_, row) in &yad.as_ref().rows {
        let slice = row.encode()?;
        file.extend_from_slice(slice.as_slice())
    }

    Ok(file)
}