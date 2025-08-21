use crate::constants::error::{ErrorMessage, MALFORMED_FILE, MALFORMED_VERSION_HEADER};
use crate::constants::headers::VERSION_HEADER;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
/// Represents the version of the yad file
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub beta: u8
}

impl Version {
    /// Create a new version struct
    pub fn new(major: u8, minor: u8, patch: u8, beta: u8) -> Self {
        Version {
            major, minor, patch, beta
        }
    }

    /// Decodes a vector to create a new instance of a version.
    pub fn decode<V: AsRef<Vec<u8>>>(vec: V) -> Result<Self, ErrorMessage> {
        if !vec.as_ref().starts_with(&[VERSION_HEADER]) {
            Err(ErrorMessage::from(MALFORMED_FILE))?
        }

        if vec.as_ref().len() < 5 {
            Err(ErrorMessage::from(MALFORMED_VERSION_HEADER))?
        }

        let [major, minor, patch, beta]: [u8; 4] = vec.as_ref()[1..5]
            .try_into()
            .map_err(|_| ErrorMessage::from(MALFORMED_VERSION_HEADER))?;

        Ok(Version {
            major,
            minor,
            patch,
            beta
        })
    }

    /// Generate a version slice by serializing the current struct
    pub fn serialize(&self) -> [u8; 5] {
        [VERSION_HEADER, self.major, self.minor, self.patch, self.beta]
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.beta > 0x00 {
            write!(f, "{}.{}.{}-beta.{}", self.major, self.minor, self.patch, self.beta)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}
