//! State for a single column's sort configuration.
//!
//! Represents the sort direction applied to a specific column,
//! used as an element in the multi-column sorting list.

use crate::column::column_id::ColumnId;
use crate::features::sorting::sort_direction::SortDirection;

/// State for a single column's sort.
///
/// Tracks which column is sorted and in which direction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SortState {
    /// The column being sorted.
    pub column_id: ColumnId,
    /// The sort direction.
    pub direction: SortDirection,
}

impl SortState {
    /// Creates a new sort state.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `direction`: The sort direction.
    ///
    /// # Returns
    ///
    /// - `SortState`: A new sort state instance.
    pub fn new(column_id: impl Into<ColumnId>, direction: SortDirection) -> Self {
        Self {
            column_id: column_id.into(),
            direction,
        }
    }

    /// Creates ascending sort state.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    ///
    /// # Returns
    ///
    /// - `SortState`: A new ascending sort state.
    pub fn asc(column_id: impl Into<ColumnId>) -> Self {
        // Create a sort state with ascending direction.
        Self::new(column_id, SortDirection::Asc)
    }

    /// Creates descending sort state.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    ///
    /// # Returns
    ///
    /// - `SortState`: A new descending sort state.
    pub fn desc(column_id: impl Into<ColumnId>) -> Self {
        // Create a sort state with descending direction.
        Self::new(column_id, SortDirection::Desc)
    }

    /// Toggles the sort direction.
    pub fn toggle(&mut self) {
        // Toggle the direction between ascending and descending.
        self.direction = self.direction.toggle();
    }
}
