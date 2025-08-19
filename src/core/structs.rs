use crate::core::bytes::{YadLength, YadType};

pub struct YadKey {
    name: String,
    value: YadValue

}

pub struct YadValue {
    value_type: YadType,
    value_length: YadLength,
    value_content: Vec<u8>
}

impl YadValue {
    pub fn build(&self) -> Vec<u8> {
        let mut builded: Vec<u8> = vec![];

        builded.push(self.value_type.value() | self.value_length.value());


        builded
    }
}