//! Complete expanding state for the table.
//!
//! Manages row expansion for tree data and nested sub-rows,
//! supporting expand-all, auto-expand, and per-row expansion.

use std::collections::HashSet;

use crate::row::data_table_row_id::DataTableRowId;

/// Type alias for expanded state map.
pub type ExpandedState = HashSet<DataTableRowId>;

/// Complete expanding state for the table.
///
/// Tracks which rows are expanded and provides configuration
/// for automatic expansion behavior.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpandingState {
    /// Set of expanded row IDs.
    expanded: ExpandedState,

    /// Whether all rows are expanded by default.
    expand_all: bool,

    /// Whether to auto-expand parent rows.
    auto_expand_parents: bool,

    /// Maximum depth for auto-expansion.
    auto_expand_depth: Option<usize>,
}

impl ExpandingState {
    /// Creates a new empty expanding state.
    ///
    /// # Returns
    ///
    /// - `ExpandingState`: A new expanding state with no rows expanded.
    pub fn new() -> Self {
        Self {
            expanded: HashSet::new(),
            expand_all: false,
            auto_expand_parents: false,
            auto_expand_depth: None,
        }
    }

    /// Creates an expanding state with all rows expanded.
    ///
    /// # Returns
    ///
    /// - `ExpandingState`: A new expanding state with expand-all enabled.
    pub fn all_expanded() -> Self {
        Self {
            expand_all: true,
            ..Self::new()
        }
    }

    /// Sets whether to auto-expand parent rows.
    ///
    /// # Parameters
    ///
    /// - `auto`: Whether parent rows should be auto-expanded.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified expanding state.
    pub fn with_auto_expand_parents(mut self, auto: bool) -> Self {
        // Update the auto-expand parents flag.
        self.auto_expand_parents = auto;
        self
    }

    /// Sets the maximum auto-expansion depth.
    ///
    /// # Parameters
    ///
    /// - `depth`: The maximum depth level for auto-expansion.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified expanding state.
    pub fn with_auto_expand_depth(mut self, depth: usize) -> Self {
        // Set the maximum auto-expand depth.
        self.auto_expand_depth = Some(depth);
        self
    }

    /// Returns whether the given row is expanded.
    ///
    /// # Parameters
    ///
    /// - `row_id`: The row identifier to check.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the row is expanded.
    pub fn is_expanded(&self, row_id: &DataTableRowId) -> bool {
        // Check expand-all or the individual expanded set.
        self.expand_all || self.expanded.contains(row_id)
    }

    /// Returns the number of expanded rows.
    ///
    /// # Returns
    ///
    /// - `usize`: The count of individually expanded rows.
    pub fn expanded_count(&self) -> usize {
        self.expanded.len()
    }

    /// Returns whether any rows are explicitly expanded.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether expand-all is set or individual rows are expanded.
    pub fn has_expanded(&self) -> bool {
        self.expand_all || !self.expanded.is_empty()
    }

    /// Returns the expanded row IDs.
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item = &DataTableRowId>`: An iterator over expanded row IDs.
    pub fn expanded_ids(&self) -> impl Iterator<Item = &DataTableRowId> {
        self.expanded.iter()
    }

    /// Returns whether all rows are expanded by default.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether expand-all mode is active.
    pub fn is_expand_all(&self) -> bool {
        self.expand_all
    }

    /// Expands a row.
    ///
    /// # Parameters
    ///
    /// - `row_id`: The row identifier to expand.
    pub fn expand(&mut self, row_id: DataTableRowId) {
        // Add the row to the expanded set.
        self.expanded.insert(row_id);
    }

    /// Collapses a row.
    ///
    /// # Parameters
    ///
    /// - `row_id`: The row identifier to collapse.
    pub fn collapse(&mut self, row_id: &DataTableRowId) {
        // Remove the row from the expanded set.
        self.expanded.remove(row_id);
    }

    /// Toggles the expansion state of a row.
    ///
    /// # Parameters
    ///
    /// - `row_id`: The row identifier to toggle.
    pub fn toggle(&mut self, row_id: DataTableRowId) {
        // Toggle between expanded and collapsed.
        if self.expanded.contains(&row_id) {
            self.expanded.remove(&row_id);
        } else {
            self.expanded.insert(row_id);
        }
    }

    /// Expands multiple rows.
    ///
    /// # Parameters
    ///
    /// - `row_ids`: The row identifiers to expand.
    pub fn expand_many(&mut self, row_ids: impl IntoIterator<Item = DataTableRowId>) {
        // Add all rows to the expanded set.
        self.expanded.extend(row_ids);
    }

    /// Collapses multiple rows.
    ///
    /// # Parameters
    ///
    /// - `row_ids`: The row identifiers to collapse.
    pub fn collapse_many(&mut self, row_ids: impl IntoIterator<Item = DataTableRowId>) {
        // Remove each row from the expanded set.
        for id in row_ids {
            self.expanded.remove(&id);
        }
    }

    /// Expands all rows.
    pub fn expand_all(&mut self) {
        // Enable expand-all mode.
        self.expand_all = true;
    }

    /// Collapses all rows.
    pub fn collapse_all(&mut self) {
        // Disable expand-all mode and clear individual expansions.
        self.expand_all = false;
        self.expanded.clear();
    }

    /// Toggles expand all state.
    pub fn toggle_all(&mut self) {
        // Toggle between expand-all and collapse-all.
        if self.expand_all {
            self.collapse_all();
        } else {
            self.expand_all();
        }
    }

    /// Resets to initial state.
    pub fn reset(&mut self) {
        // Clear all expansion state.
        self.expanded.clear();
        self.expand_all = false;
    }

    /// Checks if a row at the given depth should be auto-expanded.
    ///
    /// # Parameters
    ///
    /// - `depth`: The nesting depth level.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the row should be auto-expanded.
    pub fn should_auto_expand(&self, depth: usize) -> bool {
        // Check against the configured auto-expand depth.
        match self.auto_expand_depth {
            Some(max_depth) => depth < max_depth,
            None => false,
        }
    }
}
