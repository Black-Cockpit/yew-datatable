//! Global filter that applies across all columns.
//!
//! Provides cross-column text search with optional column
//! inclusion filtering.

use crate::column::column_id::ColumnId;

/// Global filter that applies across all columns.
///
/// When active, rows must match the filter value in at least
/// one of the included columns.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlobalFilter {
    /// The filter value.
    pub value: String,

    /// Column IDs to include in global filtering (empty = all columns).
    pub column_ids: Vec<ColumnId>,
}

impl GlobalFilter {
    /// Creates a new global filter.
    ///
    /// # Parameters
    ///
    /// - `value`: The search text to filter by.
    ///
    /// # Returns
    ///
    /// - `GlobalFilter`: A new global filter instance.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            column_ids: Vec::new(),
        }
    }

    /// Sets the columns to include in global filtering.
    ///
    /// # Parameters
    ///
    /// - `column_ids`: The column identifiers to include.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified global filter.
    pub fn with_columns(mut self, column_ids: Vec<ColumnId>) -> Self {
        // Set the column inclusion list.
        self.column_ids = column_ids;
        self
    }

    /// Returns true if the filter is empty.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the filter value is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Returns true if the given column should be included in filtering.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to check.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the column is included in the global filter.
    pub fn includes_column(&self, column_id: &ColumnId) -> bool {
        // Include all columns if the list is empty.
        self.column_ids.is_empty() || self.column_ids.contains(column_id)
    }
}
