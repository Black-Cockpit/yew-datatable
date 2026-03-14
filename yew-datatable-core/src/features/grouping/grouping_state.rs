//! Complete grouping state for the table.
//!
//! Manages row grouping by column values with support for
//! multi-level grouping, aggregation display, and default expansion.

use crate::column::column_id::ColumnId;

/// Complete grouping state for the table.
///
/// Tracks which columns are used for grouping and provides
/// configuration for aggregation and expansion behavior.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupingState {
    /// Columns to group by, in order.
    group_by: Vec<ColumnId>,

    /// Whether grouping is enabled.
    enabled: bool,

    /// Whether to show aggregated rows.
    show_aggregation: bool,

    /// Whether to expand grouped rows by default.
    expand_by_default: bool,
}

impl GroupingState {
    /// Creates a new empty grouping state.
    ///
    /// # Returns
    ///
    /// - `GroupingState`: A new grouping state with no groups configured.
    pub fn new() -> Self {
        Self {
            group_by: Vec::new(),
            enabled: true,
            show_aggregation: true,
            expand_by_default: false,
        }
    }

    /// Creates a grouping state with the given columns.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column identifiers to group by.
    ///
    /// # Returns
    ///
    /// - `GroupingState`: A new grouping state with the specified columns.
    pub fn with_columns(columns: Vec<ColumnId>) -> Self {
        Self {
            group_by: columns,
            ..Self::new()
        }
    }

    /// Sets whether grouping is enabled.
    ///
    /// # Parameters
    ///
    /// - `enabled`: Whether grouping is active.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified grouping state.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        // Update the enabled flag.
        self.enabled = enabled;
        self
    }

    /// Sets whether to show aggregation.
    ///
    /// # Parameters
    ///
    /// - `show`: Whether aggregated values should be displayed.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified grouping state.
    pub fn with_aggregation(mut self, show: bool) -> Self {
        // Update the aggregation display flag.
        self.show_aggregation = show;
        self
    }

    /// Sets whether to expand groups by default.
    ///
    /// # Parameters
    ///
    /// - `expand`: Whether groups start expanded.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified grouping state.
    pub fn with_expand_by_default(mut self, expand: bool) -> Self {
        // Update the default expansion flag.
        self.expand_by_default = expand;
        self
    }

    /// Returns whether grouping is enabled.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether grouping is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns whether any grouping is active.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether grouping is enabled and columns are configured.
    pub fn is_grouped(&self) -> bool {
        self.enabled && !self.group_by.is_empty()
    }

    /// Returns the group-by columns.
    ///
    /// # Returns
    ///
    /// - `&[ColumnId]`: A slice of column identifiers used for grouping.
    pub fn group_by(&self) -> &[ColumnId] {
        &self.group_by
    }

    /// Returns whether aggregation should be shown.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether aggregated values are displayed.
    pub fn show_aggregation(&self) -> bool {
        self.show_aggregation
    }

    /// Returns whether groups should expand by default.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether groups start in expanded state.
    pub fn expand_by_default(&self) -> bool {
        self.expand_by_default
    }

    /// Returns the grouping depth (number of group-by columns).
    ///
    /// # Returns
    ///
    /// - `usize`: The number of grouping levels.
    pub fn depth(&self) -> usize {
        self.group_by.len()
    }

    /// Checks if a column is being grouped by.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to check.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the column is used for grouping.
    pub fn is_grouped_by(&self, column_id: &ColumnId) -> bool {
        self.group_by.contains(column_id)
    }

    /// Gets the group index for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `Option<usize>`: The zero-based group index if the column is grouped.
    pub fn get_group_index(&self, column_id: &ColumnId) -> Option<usize> {
        // Find the position of the column in the group-by list.
        self.group_by.iter().position(|id| id == column_id)
    }

    /// Adds a column to group by.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to add.
    pub fn add_group(&mut self, column_id: ColumnId) {
        // Only add if not already in the group-by list.
        if !self.group_by.contains(&column_id) {
            self.group_by.push(column_id);
        }
    }

    /// Removes a column from grouping.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to remove.
    pub fn remove_group(&mut self, column_id: &ColumnId) {
        // Remove the column from the group-by list.
        self.group_by.retain(|id| id != column_id);
    }

    /// Toggles grouping for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to toggle.
    pub fn toggle_group(&mut self, column_id: ColumnId) {
        // Toggle between grouped and ungrouped.
        if self.group_by.contains(&column_id) {
            self.remove_group(&column_id);
        } else {
            self.add_group(column_id);
        }
    }

    /// Sets the group-by columns.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column identifiers to group by.
    pub fn set_group_by(&mut self, columns: Vec<ColumnId>) {
        // Replace the group-by list.
        self.group_by = columns;
    }

    /// Moves a group column to a new index.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to move.
    /// - `to_index`: The target index.
    pub fn move_group(&mut self, column_id: &ColumnId, to_index: usize) {
        // Find the current position and move to the target.
        if let Some(from_index) = self.get_group_index(column_id) {
            let id = self.group_by.remove(from_index);
            let to_index = to_index.min(self.group_by.len());
            self.group_by.insert(to_index, id);
        }
    }

    /// Clears all grouping.
    pub fn clear(&mut self) {
        // Remove all group-by columns.
        self.group_by.clear();
    }

    /// Resets to initial state.
    pub fn reset(&mut self) {
        // Clear all grouping.
        self.group_by.clear();
    }
}
