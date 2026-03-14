//! Complete column visibility state for the table.
//!
//! Manages which columns are visible or hidden, with support for
//! default visibility settings and bulk operations.

use std::collections::HashMap;

use crate::column::column_id::ColumnId;

/// Complete column visibility state for the table.
///
/// Tracks per-column visibility overrides with a configurable
/// default visibility for columns not explicitly set.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColumnVisibilityState {
    /// Map of column ID to visibility state (true = visible).
    visibility: HashMap<ColumnId, bool>,

    /// Default visibility for columns not in the map.
    default_visible: bool,
}

/// Provides default visibility state with all columns visible.
impl Default for ColumnVisibilityState {
    fn default() -> Self {
        Self {
            visibility: HashMap::new(),
            default_visible: true,
        }
    }
}

impl ColumnVisibilityState {
    /// Creates a new visibility state with all columns visible.
    ///
    /// # Returns
    ///
    /// - `ColumnVisibilityState`: A new state with default visible set to true.
    pub fn new() -> Self {
        Self {
            visibility: HashMap::new(),
            default_visible: true,
        }
    }

    /// Creates a visibility state with specific hidden columns.
    ///
    /// # Parameters
    ///
    /// - `hidden`: The column identifiers to hide.
    ///
    /// # Returns
    ///
    /// - `ColumnVisibilityState`: A new state with the specified columns hidden.
    pub fn with_hidden(hidden: impl IntoIterator<Item = ColumnId>) -> Self {
        // Create a new state and mark the specified columns as hidden.
        let mut state = Self::new();
        for id in hidden {
            state.visibility.insert(id, false);
        }
        state
    }

    /// Creates a visibility state with specific visible columns (all others hidden).
    ///
    /// # Parameters
    ///
    /// - `visible`: The column identifiers to show.
    ///
    /// # Returns
    ///
    /// - `ColumnVisibilityState`: A new state with only the specified columns visible.
    pub fn with_visible(visible: impl IntoIterator<Item = ColumnId>) -> Self {
        // Create a state with default hidden and mark specified columns as visible.
        let mut state = Self {
            visibility: HashMap::new(),
            default_visible: false,
        };
        for id in visible {
            state.visibility.insert(id, true);
        }
        state
    }

    /// Sets the default visibility for columns not explicitly set.
    ///
    /// # Parameters
    ///
    /// - `visible`: The default visibility state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified visibility state.
    pub fn with_default_visible(mut self, visible: bool) -> Self {
        // Update the default visibility.
        self.default_visible = visible;
        self
    }

    /// Returns whether a column is visible.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to check.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the column is visible.
    pub fn is_visible(&self, column_id: &ColumnId) -> bool {
        // Check the override map, falling back to the default.
        self.visibility.get(column_id).copied().unwrap_or(self.default_visible)
    }

    /// Returns the visibility map.
    ///
    /// # Returns
    ///
    /// - `&HashMap<ColumnId, bool>`: A reference to the visibility override map.
    pub fn visibility_map(&self) -> &HashMap<ColumnId, bool> {
        &self.visibility
    }

    /// Returns the list of visible column IDs from a given list.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column identifiers to filter.
    ///
    /// # Returns
    ///
    /// - `Vec<&ColumnId>`: References to visible column IDs.
    pub fn visible_columns<'a>(&self, columns: &'a [ColumnId]) -> Vec<&'a ColumnId> {
        // Filter columns by visibility.
        columns.iter().filter(|id| self.is_visible(id)).collect()
    }

    /// Returns the list of hidden column IDs from a given list.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column identifiers to filter.
    ///
    /// # Returns
    ///
    /// - `Vec<&ColumnId>`: References to hidden column IDs.
    pub fn hidden_columns<'a>(&self, columns: &'a [ColumnId]) -> Vec<&'a ColumnId> {
        // Filter columns by hidden state.
        columns.iter().filter(|id| !self.is_visible(id)).collect()
    }

    /// Sets the visibility of a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `visible`: Whether the column should be visible.
    pub fn set_visibility(&mut self, column_id: ColumnId, visible: bool) {
        // Insert the visibility override.
        self.visibility.insert(column_id, visible);
    }

    /// Shows a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to show.
    pub fn show(&mut self, column_id: ColumnId) {
        // Set the column as visible.
        self.set_visibility(column_id, true);
    }

    /// Hides a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to hide.
    pub fn hide(&mut self, column_id: ColumnId) {
        // Set the column as hidden.
        self.set_visibility(column_id, false);
    }

    /// Toggles the visibility of a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to toggle.
    pub fn toggle(&mut self, column_id: ColumnId) {
        // Get the current visibility and invert it.
        let current = self.is_visible(&column_id);
        self.set_visibility(column_id, !current);
    }

    /// Shows multiple columns.
    ///
    /// # Parameters
    ///
    /// - `column_ids`: The column identifiers to show.
    pub fn show_many(&mut self, column_ids: impl IntoIterator<Item = ColumnId>) {
        // Mark each column as visible.
        for id in column_ids {
            self.visibility.insert(id, true);
        }
    }

    /// Hides multiple columns.
    ///
    /// # Parameters
    ///
    /// - `column_ids`: The column identifiers to hide.
    pub fn hide_many(&mut self, column_ids: impl IntoIterator<Item = ColumnId>) {
        // Mark each column as hidden.
        for id in column_ids {
            self.visibility.insert(id, false);
        }
    }

    /// Shows all columns.
    pub fn show_all(&mut self) {
        // Clear overrides and set default to visible.
        self.visibility.clear();
        self.default_visible = true;
    }

    /// Hides all columns.
    pub fn hide_all(&mut self) {
        // Clear overrides and set default to hidden.
        self.visibility.clear();
        self.default_visible = false;
    }

    /// Resets the visibility for a specific column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to reset.
    pub fn reset_column(&mut self, column_id: &ColumnId) {
        // Remove the visibility override for this column.
        self.visibility.remove(column_id);
    }

    /// Resets to initial state.
    pub fn reset(&mut self) {
        // Clear all overrides and restore default visibility.
        self.visibility.clear();
        self.default_visible = true;
    }
}
