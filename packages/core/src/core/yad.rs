use std::collections::HashMap;
use crate::core::row::Row;
use crate::core::version::Version;

pub const YAD_CURRENT_VERSION: Version = Version {
    major: 1,
    minor: 0,
    patch: 0,
    beta: 0
};

/// Represents the root container of the YAD data format.
///
/// A [`YAD`] instance stores the entire dataset, including a version header
/// and a collection of [`Row`] entries.
/// It acts as the **top-level structure** in the YAD hierarchy:
/// - [`YAD`] (collection of rows)
/// - [`Row`] (collection of keys)
/// - [`Key`] (collection of values)
/// - [`Value`] (individual data unit)
///
/// # Structure Fields
/// - [`version`](YAD::version): The version of the YAD format used for this dataset.
/// - [`rows`](YAD::rows): A mapping of row names to [`Row`] objects.
///
/// # Examples
/// ```rust
/// use std::collections::HashMap;
/// use yad_core::core::Row;
/// use yad_core::YAD;
/// 
/// let mut yad_core = YAD::new();
///
/// // Add a row
/// let row = Row::new("User".to_string(), HashMap::new());
/// yad_core.add_row(row);
///
/// // Retrieve the row
/// let retrieved = yad_core.get_row("User");
/// assert!(retrieved.is_some());
///
/// // Remove the row
/// let removed = yad_core.remove_row("User");
/// assert!(removed.is_some());
/// assert!(yad_core.get_row("User").is_none());
/// ```
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YAD {
    /// The version of the YAD dataset.
    ///
    /// - Encoded as a [`Version`] to ensure compatibility across serialized data.
    /// - Must match or be compatible with the internal specification when deserializing.
    pub version: Version,

    /// A mapping of row names to [`Row`] objects.
    ///
    /// - Each [`Row`] represents a logical group of keys.
    /// - The row name is unique within a single [`YAD`] dataset.
    pub rows: HashMap<String, Row>,
}

impl YAD {
    /// Creates a new empty [`YAD`] instance.
    ///
    /// # Returns
    /// - A [`YAD`] with its [`version`](Version) set to [`YAD_CURRENT_VERSION`].
    /// - Its [`rows`](YAD::rows) initialized as an empty `HashMap`.
    ///
    /// # Examples
    /// ```rust
    /// use yad_core::YAD;
    ///
    /// let yad_core = YAD::new();
    /// assert!(yad_core.rows.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            version: YAD_CURRENT_VERSION,
            rows: HashMap::new(),
        }
    }

    /// Adds a new [`Row`] into the dataset.
    ///
    /// # Parameters
    /// - `row`: The row to insert.
    ///   Its [`Row::name`] is used as the key in the internal `HashMap`.
    ///
    /// # Notes
    /// - If a row with the same name already exists, it will be **overwritten**.
    ///
    /// # Examples
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::Row;
    /// use yad_core::YAD;
    /// let mut yad_core = YAD::new();
    ///
    /// let row = Row { name: "Config".to_string(), keys: HashMap::new() };
    /// yad_core.add_row(row);
    /// assert!(yad_core.get_row("Config").is_some());
    /// ```
    pub fn add_row(&mut self, row: Row) {
        self.rows.insert(row.name.clone(), row);
    }

    /// Retrieves a reference to a [`Row`] by its name.
    ///
    /// # Parameters
    /// - `key`: The row name to search for.
    ///
    /// # Returns
    /// - `Some(&Row)` if a row with the given name exists.
    /// - `None` if no row matches the provided key.
    ///
    /// # Examples
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::Row;
    /// use yad_core::YAD;
    /// let mut yad_core = YAD::new();
    ///
    /// let row = Row { name: "Settings".to_string(), keys: HashMap::new() };
    /// yad_core.add_row(row);
    ///
    /// assert!(yad_core.get_row("Settings").is_some());
    /// assert!(yad_core.get_row("Missing").is_none());
    /// ```
    pub fn get_row(&self, key: &str) -> Option<&Row> {
        self.rows.get(key)
    }

    /// Retrieves a mutable reference to a [`Row`] by its name.
    ///
    /// This function is similar to [`get_row`](Self::get_row), but returns
    /// a mutable reference, allowing direct modification of the row's content.
    ///
    /// # Parameters
    /// - `key`: The row name to search for.
    ///
    /// # Returns
    /// - `Some(&mut Row)` if a row with the given name exists.
    /// - `None` if no row matches the provided key.
    ///
    /// # Examples
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::Row;
    /// use yad_core::YAD;
    ///
    /// let mut yad_core = YAD::new();
    ///
    /// let row = Row { name: "Config".to_string(), keys: HashMap::new() };
    /// yad_core.add_row(row);
    ///
    /// if let Some(row_mut) = yad_core.get_row_mut("Config") {
    ///     row_mut.name = "UpdatedConfig".to_string();
    /// }
    ///
    /// assert!(yad_core.get_row("UpdatedConfig").is_some());
    /// assert!(yad_core.get_row("Config").is_none());
    /// ```
    pub fn get_row_mut(&mut self, key: &str) -> Option<&mut Row> {
        self.rows.get_mut(key)
    }


    /// Removes a [`Row`] from the dataset by its name.
    ///
    /// # Parameters
    /// - `key`: The name of the row to remove.
    ///
    /// # Returns
    /// - `Some(Row)` containing the removed row if it existed.
    /// - `None` if no row with the given name was found.
    ///
    /// # Examples
    /// ```rust
    /// use std::collections::HashMap;
    /// use yad_core::core::Row;
    /// use yad_core::YAD;
    ///
    /// let mut yad_core = YAD::new();
    /// let row = Row { name: "Cache".to_string(), keys: HashMap::new() };
    /// yad_core.add_row(row);
    ///
    /// let removed = yad_core.remove_row("Cache");
    /// assert!(removed.is_some());
    /// assert!(yad_core.get_row("Cache").is_none());
    /// ```
    pub fn remove_row(&mut self, key: &str) -> Option<Row> {
        self.rows.remove(key)
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

impl std::fmt::Display for YAD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Version: {}\nRows: {{", self.version)?;

        for (i, row) in self.rows.values().enumerate() {
            write!(f, "\t{}", row)?;

            if i < self.rows.len() - 1 {
                writeln!(f, ",")?;
            } else {
                writeln!(f)?;
            }
        }

        write!(f, "}}")
    }
}