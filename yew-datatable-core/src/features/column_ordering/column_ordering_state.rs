//! Complete column ordering state for the table.
//!
//! Manages the custom ordering of columns, allowing columns to be
//! reordered, swapped, and moved relative to each other.

use crate::column::column_id::ColumnId;

/// Complete column ordering state for the table.
///
/// Tracks a custom column order and provides methods for
/// reordering columns dynamically.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColumnOrderingState {
    /// Ordered list of column IDs.
    order: Vec<ColumnId>,
}

impl ColumnOrderingState {
    /// Creates a new empty ordering state.
    ///
    /// # Returns
    ///
    /// - `ColumnOrderingState`: A new ordering state with no custom order.
    pub fn new() -> Self {
        Self { order: Vec::new() }
    }

    /// Creates an ordering state with the given order.
    ///
    /// # Parameters
    ///
    /// - `order`: The initial column order.
    ///
    /// # Returns
    ///
    /// - `ColumnOrderingState`: A new ordering state with the specified order.
    pub fn with_order(order: Vec<ColumnId>) -> Self {
        Self { order }
    }

    /// Returns the column order.
    ///
    /// # Returns
    ///
    /// - `&[ColumnId]`: A slice of column identifiers in the custom order.
    pub fn order(&self) -> &[ColumnId] {
        &self.order
    }

    /// Returns whether a custom order is set.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether a custom column order is configured.
    pub fn has_custom_order(&self) -> bool {
        !self.order.is_empty()
    }

    /// Gets the index of a column in the order.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `Option<usize>`: The zero-based index in the custom order.
    pub fn get_index(&self, column_id: &ColumnId) -> Option<usize> {
        // Find the position of the column in the order list.
        self.order.iter().position(|id| id == column_id)
    }

    /// Applies the custom order to a list of column IDs.
    ///
    /// Columns not in the order are appended at the end.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column identifiers to reorder.
    ///
    /// # Returns
    ///
    /// - `Vec<ColumnId>`: The reordered column identifiers.
    pub fn apply_order(&self, columns: &[ColumnId]) -> Vec<ColumnId> {
        // Return the original order if no custom order is set.
        if self.order.is_empty() {
            return columns.to_vec();
        }

        // Build the result with specified order first.
        let mut result = Vec::with_capacity(columns.len());

        // Add columns in the specified order.
        for id in &self.order {
            if columns.contains(id) {
                result.push(id.clone());
            }
        }

        // Append any remaining columns not in the order.
        for id in columns {
            if !self.order.contains(id) {
                result.push(id.clone());
            }
        }

        result
    }

    /// Sets the column order.
    ///
    /// # Parameters
    ///
    /// - `order`: The new column order.
    pub fn set_order(&mut self, order: Vec<ColumnId>) {
        // Replace the current order.
        self.order = order;
    }

    /// Moves a column to a new index.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to move.
    /// - `to_index`: The target index.
    pub fn move_column(&mut self, column_id: &ColumnId, to_index: usize) {
        // Find the current position and move to the target.
        if let Some(from_index) = self.get_index(column_id) {
            let id = self.order.remove(from_index);
            let to_index = to_index.min(self.order.len());
            self.order.insert(to_index, id);
        }
    }

    /// Swaps two columns.
    ///
    /// # Parameters
    ///
    /// - `column_a`: The first column identifier.
    /// - `column_b`: The second column identifier.
    pub fn swap_columns(&mut self, column_a: &ColumnId, column_b: &ColumnId) {
        // Find both indices and swap them.
        if let (Some(idx_a), Some(idx_b)) = (self.get_index(column_a), self.get_index(column_b)) {
            self.order.swap(idx_a, idx_b);
        }
    }

    /// Moves a column before another column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to move.
    /// - `before_id`: The column identifier to move before.
    pub fn move_before(&mut self, column_id: &ColumnId, before_id: &ColumnId) {
        // Find the target position and move the column there.
        if let Some(target_index) = self.get_index(before_id) {
            self.move_column(column_id, target_index);
        }
    }

    /// Moves a column after another column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to move.
    /// - `after_id`: The column identifier to move after.
    pub fn move_after(&mut self, column_id: &ColumnId, after_id: &ColumnId) {
        // Find the target position and move the column after it.
        if let Some(target_index) = self.get_index(after_id) {
            self.move_column(column_id, target_index + 1);
        }
    }

    /// Initializes the order from a list of columns.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column identifiers to initialize from.
    pub fn initialize(&mut self, columns: &[ColumnId]) {
        // Copy the provided column order.
        self.order = columns.to_vec();
    }

    /// Resets to default order.
    pub fn reset(&mut self) {
        // Clear the custom order.
        self.order.clear();
    }
}
