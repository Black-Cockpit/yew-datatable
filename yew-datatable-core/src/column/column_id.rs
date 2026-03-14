//! Unique identifier for a table column.
//!
//! Column IDs are used to reference columns throughout the table API
//! for operations like sorting, filtering, visibility, and pinning.

use std::fmt;

/// Unique identifier for a column.
///
/// Column IDs are used to reference columns throughout the table API
/// for operations like sorting, filtering, visibility, and pinning.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ColumnId(String);

impl ColumnId {
    /// Creates a new column ID from a string.
    ///
    /// # Parameters
    ///
    /// - `id`: The column identifier string.
    ///
    /// # Returns
    ///
    /// - `ColumnId`: A new column identifier.
    pub fn new(id: impl Into<String>) -> Self {
        // Convert the input into a string and wrap it.
        Self(id.into())
    }

    /// Returns the column ID as a string slice.
    ///
    /// # Returns
    ///
    /// - `&str`: The column identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Formats the column identifier for debug output.
impl fmt::Debug for ColumnId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the column ID with a descriptive wrapper.
        write!(f, "ColumnId({})", self.0)
    }
}

/// Displays the column identifier as a plain string.
impl fmt::Display for ColumnId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display the raw column ID string.
        write!(f, "{}", self.0)
    }
}

/// Creates a `ColumnId` from a string slice.
impl From<&str> for ColumnId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// Creates a `ColumnId` from an owned `String`.
impl From<String> for ColumnId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// Provides a string slice reference to the inner identifier.
impl AsRef<str> for ColumnId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Serializes the column identifier as a plain string.
#[cfg(feature = "serde")]
impl serde::Serialize for ColumnId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize the inner string.
        self.0.serialize(serializer)
    }
}

/// Deserializes a column identifier from a string value.
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ColumnId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize a string and wrap it as a ColumnId.
        String::deserialize(deserializer).map(Self::new)
    }
}
