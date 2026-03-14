//! Complete column pinning state for the table.
//!
//! Manages columns pinned to the left or right of the table viewport,
//! with support for pin/unpin operations and column reordering.
use std::collections::HashSet;

use crate::column::column_id::ColumnId;
use crate::features::column_pinning::column_pinning_position::ColumnPinningPosition;

/// Complete column pinning state for the table.
///
/// Tracks which columns are pinned to the left or right side,
/// and provides methods for managing pinned columns.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColumnPinningState {
    /// Columns pinned to the left.
    left: Vec<ColumnId>,

    /// Columns pinned to the right.
    right: Vec<ColumnId>,
}

impl ColumnPinningState {
    /// Creates a new empty pinning state.
    ///
    /// # Returns
    ///
    /// - `ColumnPinningState`: A new state with no pinned columns.
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    /// Creates a pinning state with left-pinned columns.
    ///
    /// # Parameters
    ///
    /// - `left`: The column identifiers to pin to the left.
    ///
    /// # Returns
    ///
    /// - `ColumnPinningState`: A new state with the specified left-pinned columns.
    pub fn with_left(left: Vec<ColumnId>) -> Self {
        Self {
            left,
            right: Vec::new(),
        }
    }

    /// Creates a pinning state with right-pinned columns.
    ///
    /// # Parameters
    ///
    /// - `right`: The column identifiers to pin to the right.
    ///
    /// # Returns
    ///
    /// - `ColumnPinningState`: A new state with the specified right-pinned columns.
    pub fn with_right(right: Vec<ColumnId>) -> Self {
        Self {
            left: Vec::new(),
            right,
        }
    }

    /// Returns the left-pinned columns.
    ///
    /// # Returns
    ///
    /// - `&[ColumnId]`: A slice of left-pinned column identifiers.
    pub fn left(&self) -> &[ColumnId] {
        &self.left
    }

    /// Returns the right-pinned columns.
    ///
    /// # Returns
    ///
    /// - `&[ColumnId]`: A slice of right-pinned column identifiers.
    pub fn right(&self) -> &[ColumnId] {
        &self.right
    }

    /// Returns whether a column is pinned.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to check.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the column is pinned to either side.
    pub fn is_pinned(&self, column_id: &ColumnId) -> bool {
        // Check both left and right pinned lists.
        self.left.contains(column_id) || self.right.contains(column_id)
    }

    /// Returns the pinning position of a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `Option<ColumnPinningPosition>`: The pinning position if pinned.
    pub fn get_position(&self, column_id: &ColumnId) -> Option<ColumnPinningPosition> {
        // Check left first, then right.
        if self.left.contains(column_id) {
            Some(ColumnPinningPosition::Left)
        } else if self.right.contains(column_id) {
            Some(ColumnPinningPosition::Right)
        } else {
            None
        }
    }

    /// Returns whether any columns are pinned.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether at least one column is pinned.
    pub fn has_pinned(&self) -> bool {
        !self.left.is_empty() || !self.right.is_empty()
    }

    /// Returns all pinned column IDs.
    ///
    /// # Returns
    ///
    /// - `HashSet<ColumnId>`: A set of all pinned column identifiers.
    pub fn all_pinned(&self) -> HashSet<ColumnId> {
        // Collect both left and right pinned columns.
        let mut pinned = HashSet::new();
        pinned.extend(self.left.iter().cloned());
        pinned.extend(self.right.iter().cloned());
        pinned
    }

    /// Applies pinning to reorder columns: left-pinned, center, right-pinned.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column identifiers to reorder.
    ///
    /// # Returns
    ///
    /// - `Vec<ColumnId>`: The reordered column identifiers.
    pub fn apply_pinning(&self, columns: &[ColumnId]) -> Vec<ColumnId> {
        // Build the result with pinned columns on their respective sides.
        let mut result = Vec::with_capacity(columns.len());

        // Add left-pinned columns in order.
        for id in &self.left {
            if columns.contains(id) {
                result.push(id.clone());
            }
        }

        // Add center columns (not pinned).
        for id in columns {
            if !self.left.contains(id) && !self.right.contains(id) {
                result.push(id.clone());
            }
        }

        // Add right-pinned columns in order.
        for id in &self.right {
            if columns.contains(id) {
                result.push(id.clone());
            }
        }

        result
    }

    /// Pins a column to a position.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to pin.
    /// - `position`: The pinning position (left or right).
    pub fn pin(&mut self, column_id: ColumnId, position: ColumnPinningPosition) {
        // Remove from current position first.
        self.unpin(&column_id);

        // Add to the specified position.
        match position {
            ColumnPinningPosition::Left => self.left.push(column_id),
            ColumnPinningPosition::Right => self.right.push(column_id),
        }
    }

    /// Pins a column to the left.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to pin to the left.
    pub fn pin_left(&mut self, column_id: ColumnId) {
        // Delegate to pin with Left position.
        self.pin(column_id, ColumnPinningPosition::Left);
    }

    /// Pins a column to the right.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to pin to the right.
    pub fn pin_right(&mut self, column_id: ColumnId) {
        // Delegate to pin with Right position.
        self.pin(column_id, ColumnPinningPosition::Right);
    }

    /// Unpins a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to unpin.
    pub fn unpin(&mut self, column_id: &ColumnId) {
        // Remove from both left and right lists.
        self.left.retain(|id| id != column_id);
        self.right.retain(|id| id != column_id);
    }

    /// Toggles pinning for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to toggle.
    /// - `position`: The pinning position to toggle.
    pub fn toggle(&mut self, column_id: ColumnId, position: ColumnPinningPosition) {
        // Unpin if already at the target position, otherwise pin.
        if self.get_position(&column_id) == Some(position) {
            self.unpin(&column_id);
        } else {
            self.pin(column_id, position);
        }
    }

    /// Sets the left-pinned columns.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column identifiers to pin to the left.
    pub fn set_left(&mut self, columns: Vec<ColumnId>) {
        // Remove duplicates from right.
        for id in &columns {
            self.right.retain(|r| r != id);
        }

        // Set the left-pinned columns.
        self.left = columns;
    }

    /// Sets the right-pinned columns.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column identifiers to pin to the right.
    pub fn set_right(&mut self, columns: Vec<ColumnId>) {
        // Remove duplicates from left.
        for id in &columns {
            self.left.retain(|l| l != id);
        }

        // Set the right-pinned columns.
        self.right = columns;
    }

    /// Clears all pinning.
    pub fn clear(&mut self) {
        // Remove all pinned columns.
        self.left.clear();
        self.right.clear();
    }

    /// Resets to initial state.
    pub fn reset(&mut self) {
        // Delegate to clear.
        self.clear();
    }
}
