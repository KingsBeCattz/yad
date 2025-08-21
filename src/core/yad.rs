use std::collections::HashMap;
use crate::core::row::Row;
use crate::core::version::Version;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YAD {
    pub version: Version,
    pub rows: HashMap<String, Row>
}

impl AsRef<YAD> for YAD {
    fn as_ref(&self) -> &YAD {
        &self
    }
}

impl AsMut<YAD> for YAD {
    fn as_mut(&mut self) -> &mut YAD {
        self
    }
}