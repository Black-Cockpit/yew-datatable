//! Complete sorting state for the table.
//!
//! Manages the list of active sorts, multi-column sorting configuration,
//! and sort removal behavior.

use crate::column::column_id::ColumnId;
use crate::features::sorting::sort_direction::SortDirection;
use crate::features::sorting::sort_state::SortState;

/// Complete sorting state for the table.
///
/// Tracks all active column sorts and their configuration,
/// supporting multi-column sorting with configurable limits.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SortingState {
    /// List of active sorts in order of priority.
    sorts: Vec<SortState>,

    /// Maximum number of columns that can be sorted simultaneously.
    max_multi_sort_columns: Option<usize>,

    /// Whether multi-sort is enabled.
    enable_multi_sort: bool,

    /// Whether to remove sorts when toggling past descending.
    enable_sort_removal: bool,
}

impl SortingState {
    /// Creates a new empty sorting state.
    ///
    /// # Returns
    ///
    /// - `SortingState`: A new empty sorting state with multi-sort and sort removal enabled.
    pub fn new() -> Self {
        Self {
            sorts: Vec::new(),
            max_multi_sort_columns: None,
            enable_multi_sort: true,
            enable_sort_removal: true,
        }
    }

    /// Creates sorting state with initial sorts.
    ///
    /// # Parameters
    ///
    /// - `sorts`: The initial list of sort states.
    ///
    /// # Returns
    ///
    /// - `SortingState`: A new sorting state with the provided sorts.
    pub fn with_sorts(sorts: Vec<SortState>) -> Self {
        Self { sorts, ..Self::new() }
    }

    /// Sets the maximum number of multi-sort columns.
    ///
    /// # Parameters
    ///
    /// - `max`: The maximum number of simultaneously sorted columns.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified sorting state.
    pub fn with_max_multi_sort_columns(mut self, max: usize) -> Self {
        // Set the maximum multi-sort column limit.
        self.max_multi_sort_columns = Some(max);
        self
    }

    /// Enables or disables multi-sort.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether multi-column sorting is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified sorting state.
    pub fn with_multi_sort(mut self, enable: bool) -> Self {
        // Update the multi-sort flag.
        self.enable_multi_sort = enable;
        self
    }

    /// Enables or disables sort removal on toggle.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether sorts are removed when toggling past descending.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified sorting state.
    pub fn with_sort_removal(mut self, enable: bool) -> Self {
        // Update the sort removal flag.
        self.enable_sort_removal = enable;
        self
    }

    /// Returns the current sorts.
    ///
    /// # Returns
    ///
    /// - `&[SortState]`: A slice of active sort states.
    pub fn sorts(&self) -> &[SortState] {
        &self.sorts
    }

    /// Returns whether any sorting is active.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether at least one column is sorted.
    pub fn is_sorted(&self) -> bool {
        !self.sorts.is_empty()
    }

    /// Returns the sort state for a specific column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `Option<&SortState>`: The sort state if the column is sorted.
    pub fn get_sort(&self, column_id: &ColumnId) -> Option<&SortState> {
        // Search for a sort matching the column ID.
        self.sorts.iter().find(|s| &s.column_id == column_id)
    }

    /// Returns the sort index for a column (for multi-sort display).
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `Option<usize>`: The zero-based sort priority index.
    pub fn get_sort_index(&self, column_id: &ColumnId) -> Option<usize> {
        // Find the position of the column in the sort list.
        self.sorts.iter().position(|s| &s.column_id == column_id)
    }

    /// Returns the sort direction for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `Option<SortDirection>`: The sort direction if the column is sorted.
    pub fn get_direction(&self, column_id: &ColumnId) -> Option<SortDirection> {
        // Extract the direction from the sort state.
        self.get_sort(column_id).map(|s| s.direction)
    }

    /// Toggles sorting for a column.
    ///
    /// If multi is true and multi-sort is enabled, adds to the sort list.
    /// Otherwise, replaces the current sort.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to toggle sorting for.
    /// - `multi`: Whether to add to multi-sort list (true) or replace (false).
    pub fn toggle_sort(&mut self, column_id: impl Into<ColumnId>, multi: bool) {
        // Convert the column ID.
        let column_id = column_id.into();

        // Check if the column is already sorted.
        if let Some(idx) = self.sorts.iter().position(|s| s.column_id == column_id) {
            let current = &mut self.sorts[idx];
            match current.direction {
                SortDirection::Asc => {
                    // Toggle from ascending to descending.
                    current.direction = SortDirection::Desc;
                }
                SortDirection::Desc => {
                    if self.enable_sort_removal {
                        // Remove the sort when toggling past descending.
                        self.sorts.remove(idx);
                    } else {
                        // Cycle back to ascending.
                        current.direction = SortDirection::Asc;
                    }
                }
            }
        } else {
            // Create a new ascending sort for this column.
            let new_sort = SortState::asc(column_id);

            if multi && self.enable_multi_sort {
                // Enforce the maximum multi-sort column limit.
                if let Some(max) = self.max_multi_sort_columns {
                    if self.sorts.len() >= max {
                        self.sorts.remove(0);
                    }
                }
                // Add to the multi-sort list.
                self.sorts.push(new_sort);
            } else {
                // Replace the current sort.
                self.sorts = vec![new_sort];
            }
        }
    }

    /// Sets the sort for a column directly.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `direction`: The sort direction to set.
    pub fn set_sort(&mut self, column_id: impl Into<ColumnId>, direction: SortDirection) {
        // Convert the column ID.
        let column_id = column_id.into();

        // Update existing sort or add a new one.
        if let Some(sort) = self.sorts.iter_mut().find(|s| s.column_id == column_id) {
            sort.direction = direction;
        } else {
            self.sorts.push(SortState::new(column_id, direction));
        }
    }

    /// Clears the sort for a specific column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to clear sorting for.
    pub fn clear_sort(&mut self, column_id: &ColumnId) {
        // Remove the sort matching the column ID.
        self.sorts.retain(|s| &s.column_id != column_id);
    }

    /// Clears all sorting.
    pub fn clear_all(&mut self) {
        // Remove all active sorts.
        self.sorts.clear();
    }

    /// Resets to initial state.
    pub fn reset(&mut self) {
        // Clear all sorts.
        self.sorts.clear();
    }
}
