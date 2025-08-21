use std::collections::HashMap;
use crate::core::row::Row;
use crate::core::version::Version;

pub const YAD_CURRENT_VERSION: Version = Version {
    major: 1,
    minor: 0,
    patch: 0,
    beta: 0
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YAD {
    pub version: Version,
    pub rows: HashMap<String, Row>
}

impl YAD {
    pub fn new() -> Self {
        Self {
            version: YAD_CURRENT_VERSION,
            rows: HashMap::new()
        }
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.insert(row.name.clone(), row);
    }

    pub fn get_row(&self, key: &str) -> Option<&Row> {
        self.rows.get(key)
    }
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