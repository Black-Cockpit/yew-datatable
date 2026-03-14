//! Complete row selection state for the table.
//!
//! Manages the set of selected rows with support for single
//! and multi-row selection modes.

use std::collections::HashSet;

use crate::features::row_selection::row_selection_mode::RowSelectionMode;
use crate::row::data_table_row_id::DataTableRowId;

/// Complete row selection state for the table.
///
/// Tracks selected rows and provides methods for selection
/// management with mode-aware behavior.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RowSelectionState {
    /// Set of selected row IDs.
    selected: HashSet<DataTableRowId>,

    /// Selection mode.
    mode: RowSelectionMode,

    /// Whether to enable row click selection.
    enable_row_click: bool,

    /// Whether to enable sub-row selection.
    enable_sub_row_selection: bool,
}

impl RowSelectionState {
    /// Creates a new empty selection state.
    ///
    /// # Returns
    ///
    /// - `RowSelectionState`: A new multi-selection state with row click enabled.
    pub fn new() -> Self {
        Self {
            selected: HashSet::new(),
            mode: RowSelectionMode::Multi,
            enable_row_click: true,
            enable_sub_row_selection: true,
        }
    }

    /// Creates a selection state with the given mode.
    ///
    /// # Parameters
    ///
    /// - `mode`: The selection mode to use.
    ///
    /// # Returns
    ///
    /// - `RowSelectionState`: A new selection state with the specified mode.
    pub fn with_mode(mode: RowSelectionMode) -> Self {
        Self { mode, ..Self::new() }
    }

    /// Sets whether row click selection is enabled.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether clicking a row selects it.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified selection state.
    pub fn with_row_click(mut self, enable: bool) -> Self {
        // Update the row click flag.
        self.enable_row_click = enable;
        self
    }

    /// Sets whether sub-row selection is enabled.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether sub-rows can be selected.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified selection state.
    pub fn with_sub_row_selection(mut self, enable: bool) -> Self {
        // Update the sub-row selection flag.
        self.enable_sub_row_selection = enable;
        self
    }

    /// Returns the selection mode.
    ///
    /// # Returns
    ///
    /// - `RowSelectionMode`: The current selection mode.
    pub fn mode(&self) -> RowSelectionMode {
        self.mode
    }

    /// Returns whether row click selection is enabled.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether clicking a row selects it.
    pub fn is_row_click_enabled(&self) -> bool {
        self.enable_row_click
    }

    /// Returns whether selection is enabled.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether any selection mode is active.
    pub fn is_enabled(&self) -> bool {
        self.mode != RowSelectionMode::None
    }

    /// Returns whether the given row is selected.
    ///
    /// # Parameters
    ///
    /// - `row_id`: The row identifier to check.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the row is currently selected.
    pub fn is_selected(&self, row_id: &DataTableRowId) -> bool {
        self.selected.contains(row_id)
    }

    /// Returns the number of selected rows.
    ///
    /// # Returns
    ///
    /// - `usize`: The count of selected rows.
    pub fn selected_count(&self) -> usize {
        self.selected.len()
    }

    /// Returns whether any rows are selected.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether at least one row is selected.
    pub fn has_selection(&self) -> bool {
        !self.selected.is_empty()
    }

    /// Returns the selected row IDs.
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item = &DataTableRowId>`: An iterator over selected row IDs.
    pub fn selected_ids(&self) -> impl Iterator<Item = &DataTableRowId> {
        self.selected.iter()
    }

    /// Returns the selected row IDs as a vector.
    ///
    /// # Returns
    ///
    /// - `Vec<DataTableRowId>`: A vector of selected row IDs.
    pub fn selected_ids_vec(&self) -> Vec<DataTableRowId> {
        self.selected.iter().cloned().collect()
    }

    /// Checks if all provided rows are selected.
    ///
    /// # Parameters
    ///
    /// - `row_ids`: The row identifiers to check.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether all provided rows are selected.
    pub fn is_all_selected(&self, row_ids: &[DataTableRowId]) -> bool {
        // Return false for empty input.
        if row_ids.is_empty() {
            return false;
        }

        // Check that every row is in the selected set.
        row_ids.iter().all(|id| self.selected.contains(id))
    }

    /// Checks if some (but not all) provided rows are selected.
    ///
    /// # Parameters
    ///
    /// - `row_ids`: The row identifiers to check.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether some but not all rows are selected.
    pub fn is_some_selected(&self, row_ids: &[DataTableRowId]) -> bool {
        // Count how many of the provided rows are selected.
        let selected_count = row_ids.iter().filter(|id| self.selected.contains(id)).count();

        // Some selected means at least one but fewer than all.
        selected_count > 0 && selected_count < row_ids.len()
    }

    /// Selects a row.
    ///
    /// # Parameters
    ///
    /// - `row_id`: The row identifier to select.
    pub fn select(&mut self, row_id: DataTableRowId) {
        // Apply selection based on the current mode.
        match self.mode {
            RowSelectionMode::None => {}
            RowSelectionMode::Single => {
                // Clear existing selection and select the new row.
                self.selected.clear();
                self.selected.insert(row_id);
            }
            RowSelectionMode::Multi => {
                // Add to the selection set.
                self.selected.insert(row_id);
            }
        }
    }

    /// Deselects a row.
    ///
    /// # Parameters
    ///
    /// - `row_id`: The row identifier to deselect.
    pub fn deselect(&mut self, row_id: &DataTableRowId) {
        // Remove from the selection set.
        self.selected.remove(row_id);
    }

    /// Toggles the selection state of a row.
    ///
    /// # Parameters
    ///
    /// - `row_id`: The row identifier to toggle.
    pub fn toggle(&mut self, row_id: DataTableRowId) {
        // Check if the row is already selected.
        if self.selected.contains(&row_id) {
            self.selected.remove(&row_id);
        } else {
            self.select(row_id);
        }
    }

    /// Selects multiple rows.
    ///
    /// # Parameters
    ///
    /// - `row_ids`: The row identifiers to select.
    pub fn select_many(&mut self, row_ids: impl IntoIterator<Item = DataTableRowId>) {
        // Apply selection based on the current mode.
        match self.mode {
            RowSelectionMode::None => {}
            RowSelectionMode::Single => {
                // Only select the first row in single mode.
                if let Some(id) = row_ids.into_iter().next() {
                    self.selected.clear();
                    self.selected.insert(id);
                }
            }
            RowSelectionMode::Multi => {
                // Add all rows to the selection set.
                self.selected.extend(row_ids);
            }
        }
    }

    /// Deselects multiple rows.
    ///
    /// # Parameters
    ///
    /// - `row_ids`: The row identifiers to deselect.
    pub fn deselect_many(&mut self, row_ids: impl IntoIterator<Item = DataTableRowId>) {
        // Remove each row from the selection set.
        for id in row_ids {
            self.selected.remove(&id);
        }
    }

    /// Selects all provided rows.
    ///
    /// # Parameters
    ///
    /// - `row_ids`: The row identifiers to select.
    pub fn select_all(&mut self, row_ids: impl IntoIterator<Item = DataTableRowId>) {
        // Only allow in multi mode.
        if self.mode == RowSelectionMode::Multi {
            self.selected.extend(row_ids);
        }
    }

    /// Toggles selection of all provided rows.
    ///
    /// # Parameters
    ///
    /// - `row_ids`: The row identifiers to toggle.
    pub fn toggle_all(&mut self, row_ids: Vec<DataTableRowId>) {
        // Only allow in multi mode.
        if self.mode != RowSelectionMode::Multi {
            return;
        }

        // Deselect all if all are selected, otherwise select all.
        if self.is_all_selected(&row_ids) {
            self.deselect_many(row_ids);
        } else {
            self.select_all(row_ids);
        }
    }

    /// Clears all selections.
    pub fn clear(&mut self) {
        // Remove all entries from the selection set.
        self.selected.clear();
    }

    /// Sets the selection mode.
    ///
    /// # Parameters
    ///
    /// - `mode`: The new selection mode.
    pub fn set_mode(&mut self, mode: RowSelectionMode) {
        // Update the mode.
        self.mode = mode;

        // Enforce mode constraints.
        if mode == RowSelectionMode::Single && self.selected.len() > 1 {
            // Keep only the first selected row in single mode.
            if let Some(first) = self.selected.iter().next().cloned() {
                self.selected.clear();
                self.selected.insert(first);
            }
        } else if mode == RowSelectionMode::None {
            // Clear all selections in none mode.
            self.selected.clear();
        }
    }

    /// Resets to initial state.
    pub fn reset(&mut self) {
        // Clear all selections.
        self.selected.clear();
    }
}
