use crate::constants::error::{ErrorMessage, MALFORMED_FILE, MALFORMED_VERSION_HEADER};
use crate::constants::headers::VERSION_HEADER;

/// Represents the version metadata of a YAD file.
///
/// A [`Version`] encodes the compatibility information of a dataset by
/// following the **semantic versioning scheme** with an additional `beta` flag:
/// - `major`: Incompatible, breaking changes.
/// - `minor`: Backward-compatible feature additions.
/// - `patch`: Backward-compatible bug fixes or adjustments.
/// - `beta`: Pre-release or beta indicator (non-stable versions).
///
/// Each version is serialized into a fixed 5-byte array:
/// ```text
/// [ VERSION_HEADER, major, minor, patch, beta ]
/// ```
///
/// # Structure Fields
/// - [`major`](Version::major): The major version number.
///   Incremented for breaking or incompatible changes.
/// - [`minor`](Version::minor): The minor version number.
///   Incremented for new features that remain backward-compatible.
/// - [`patch`](Version::patch): The patch version number.
///   Incremented for bug fixes or small backward-compatible updates.
/// - [`beta`](Version::beta): Indicates pre-release state.
///   - `0` = stable release
///   - Non-zero values = beta/pre-release versions.
///
/// # Examples
/// ## Creating a Stable Version
/// ```rust
/// use yad_core::core::Version;
///
/// let v = Version::new(1, 0, 0, 0);
/// assert_eq!(v.major, 1);
/// assert_eq!(v.beta, 0); // stable
/// ```
///
/// ## Creating a Beta Version
/// ```rust
/// use yad_core::core::Version;
///
/// let v = Version::new(1, 1, 0, 2);
/// assert_eq!(v.minor, 1);
/// assert_eq!(v.beta, 2); // beta release
/// ```
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Version {
    /// The major version component (breaking changes).
    pub major: u8,
    /// The minor version component (new features).
    pub minor: u8,
    /// The patch version component (bug fixes).
    pub patch: u8,
    /// The beta/pre-release component.
    ///
    /// - `0` = stable release.
    /// - Non-zero values indicate beta/pre-release builds.
    pub beta: u8,
}

impl Version {
    /// Creates a new [`Version`] instance from its four components.
    ///
    /// # Parameters
    /// - `major`: Major version number (breaking changes).
    /// - `minor`: Minor version number (compatible features).
    /// - `patch`: Patch version number (compatible fixes).
    /// - `beta`: Beta version indicator (`0` = stable).
    ///
    /// # Returns
    /// A fully constructed [`Version`] instance.
    ///
    /// # Examples
    /// ```rust
    /// use yad_core::core::Version;
    ///
    /// let v = Version::new(1, 0, 0, 0);
    /// assert_eq!(v.major, 1);
    /// ```
    pub fn new(major: u8, minor: u8, patch: u8, beta: u8) -> Self {
        Version { major, minor, patch, beta }
    }

    /// Decodes a version header from a raw byte buffer.
    ///
    /// The expected format is:
    /// ```text
    /// [ VERSION_HEADER, major, minor, patch, beta ]
    /// ```
    ///
    /// # Type Parameters
    /// - `V`: Any type implementing [`AsRef<Vec<u8>>`], allowing direct use of
    ///   `Vec<u8>` or references like `&Vec<u8>`.
    ///
    /// # Parameters
    /// - `vec`: The byte buffer containing the version header.
    ///
    /// # Returns
    /// - `Ok(Version)`: A successfully decoded [`Version`] instance.
    /// - `Err(ErrorMessage)`: If the buffer is malformed or incomplete.
    ///
    /// # Errors
    /// Returns [`ErrorMessage`] if:
    /// - The buffer does not start with [`VERSION_HEADER`].
    /// - The buffer is shorter than 5 bytes.
    /// - The byte slice cannot be converted into the expected `[u8; 4]`.
    ///
    /// # Examples
    /// ```rust
    /// use yad_core::constants::headers::VERSION_HEADER;
    /// use yad_core::core::Version;
    ///
    /// let data = vec![VERSION_HEADER, 1, 0, 2, 0];
    /// let version = Version::decode(data).unwrap();
    /// assert_eq!(version.patch, 2);
    /// ```
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

        Ok(Version { major, minor, patch, beta })
    }

    /// Serializes the [`Version`] into a 5-byte array.
    ///
    /// Format:
    /// ```text
    /// [ VERSION_HEADER, major, minor, patch, beta ]
    /// ```
    ///
    /// # Returns
    /// A fixed `[u8; 5]` array representing the version header.
    ///
    /// # Examples
    /// ```rust
    /// use yad_core::constants::headers::VERSION_HEADER;
    /// use yad_core::core::Version;
    ///
    /// let v = Version::new(1, 0, 0, 0);
    /// let bytes = v.serialize();
    /// assert_eq!(bytes, [VERSION_HEADER, 1, 0, 0, 0]);
    /// ```
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
