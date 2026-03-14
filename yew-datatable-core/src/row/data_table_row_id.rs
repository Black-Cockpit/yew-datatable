//! Unique identifier for a table row.
//!
//! Row IDs are used to reference rows throughout the table API
//! for operations like selection and expansion.

use std::fmt;
use std::hash::Hash;

/// Unique identifier for a row.
///
/// Row IDs are used to reference rows throughout the table API
/// for operations like selection and expansion.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DataTableRowId(String);

impl DataTableRowId {
    /// Creates a new row ID from a string.
    ///
    /// # Parameters
    ///
    /// - `id`: The row identifier string.
    ///
    /// # Returns
    ///
    /// - `DataTableRowId`: A new row identifier.
    pub fn new(id: impl Into<String>) -> Self {
        // Convert the input into a string and wrap it.
        Self(id.into())
    }

    /// Creates a row ID from an index.
    ///
    /// # Parameters
    ///
    /// - `index`: The row index to use as the identifier.
    ///
    /// # Returns
    ///
    /// - `DataTableRowId`: A new row identifier from the index.
    pub fn from_index(index: usize) -> Self {
        // Convert the index to a string representation.
        Self(index.to_string())
    }

    /// Returns the row ID as a string slice.
    ///
    /// # Returns
    ///
    /// - `&str`: The row identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Formats the row identifier for debug output.
impl fmt::Debug for DataTableRowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the row ID with a descriptive wrapper.
        write!(f, "DataTableRowId({})", self.0)
    }
}

/// Displays the row identifier as a plain string.
impl fmt::Display for DataTableRowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display the raw row ID string.
        write!(f, "{}", self.0)
    }
}

/// Creates a `DataTableRowId` from a string slice.
impl From<&str> for DataTableRowId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// Creates a `DataTableRowId` from an owned `String`.
impl From<String> for DataTableRowId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// Creates a `DataTableRowId` from an index value.
impl From<usize> for DataTableRowId {
    fn from(index: usize) -> Self {
        Self::from_index(index)
    }
}

/// Provides a string slice reference to the inner identifier.
impl AsRef<str> for DataTableRowId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Serializes the row identifier as a plain string.
#[cfg(feature = "serde")]
impl serde::Serialize for DataTableRowId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize the inner string.
        self.0.serialize(serializer)
    }
}

/// Deserializes a row identifier from a string value.
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DataTableRowId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize a string and wrap it as a DataTableRowId.
        String::deserialize(deserializer).map(Self::new)
    }
}
