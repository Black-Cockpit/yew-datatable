//! Complete column sizing state for the table.
//!
//! Manages column widths, resize operations, and sizing mode
//! with support for minimum and maximum constraints.

use std::collections::HashMap;

use crate::column::column_id::ColumnId;
use crate::features::column_sizing::column_size::ColumnSize;
use crate::features::column_sizing::column_sizing_mode::ColumnSizingMode;

/// Complete column sizing state for the table.
///
/// Tracks per-column sizes, the active sizing mode,
/// and any ongoing resize operations.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColumnSizingState {
    /// Map of column ID to size info.
    sizes: HashMap<ColumnId, ColumnSize>,

    /// Sizing mode.
    mode: ColumnSizingMode,

    /// Whether resizing is enabled.
    enabled: bool,

    /// Column currently being resized.
    resizing_column: Option<ColumnId>,
}

impl ColumnSizingState {
    /// Creates a new sizing state.
    ///
    /// # Returns
    ///
    /// - `ColumnSizingState`: A new sizing state with fixed mode and resizing enabled.
    pub fn new() -> Self {
        Self {
            sizes: HashMap::new(),
            mode: ColumnSizingMode::Fixed,
            enabled: true,
            resizing_column: None,
        }
    }

    /// Sets the sizing mode.
    ///
    /// # Parameters
    ///
    /// - `mode`: The column sizing mode.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified sizing state.
    pub fn with_mode(mut self, mode: ColumnSizingMode) -> Self {
        // Update the sizing mode.
        self.mode = mode;
        self
    }

    /// Sets whether resizing is enabled.
    ///
    /// # Parameters
    ///
    /// - `enabled`: Whether column resizing is allowed.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified sizing state.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        // Update the enabled flag.
        self.enabled = enabled;
        self
    }

    /// Returns the sizing mode.
    ///
    /// # Returns
    ///
    /// - `ColumnSizingMode`: The current sizing mode.
    pub fn mode(&self) -> ColumnSizingMode {
        self.mode
    }

    /// Returns whether resizing is enabled.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether column resizing is allowed.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the column currently being resized.
    ///
    /// # Returns
    ///
    /// - `Option<&ColumnId>`: The column being resized, if any.
    pub fn resizing_column(&self) -> Option<&ColumnId> {
        self.resizing_column.as_ref()
    }

    /// Returns whether any column is being resized.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether a resize operation is in progress.
    pub fn is_resizing(&self) -> bool {
        self.resizing_column.is_some()
    }

    /// Gets the size info for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `ColumnSize`: The column's size info, or defaults if not set.
    pub fn get_size(&self, column_id: &ColumnId) -> ColumnSize {
        // Return the stored size or the default.
        self.sizes.get(column_id).copied().unwrap_or_default()
    }

    /// Gets the width of a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `f64`: The column width in pixels.
    pub fn get_width(&self, column_id: &ColumnId) -> f64 {
        // Extract the width from the size info.
        self.get_size(column_id).width
    }

    /// Sets the size info for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `size`: The size info to set.
    pub fn set_size(&mut self, column_id: ColumnId, size: ColumnSize) {
        // Insert or update the column size.
        self.sizes.insert(column_id, size);
    }

    /// Sets the width of a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `width`: The desired width in pixels.
    pub fn set_width(&mut self, column_id: ColumnId, width: f64) {
        // Get the current size, update width, and store.
        let mut size = self.get_size(&column_id);
        size.set_width(width);
        self.sizes.insert(column_id, size);
    }

    /// Starts resizing a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to start resizing.
    pub fn start_resize(&mut self, column_id: ColumnId) {
        // Set the resizing column.
        self.resizing_column = Some(column_id);
    }

    /// Updates the resize delta.
    ///
    /// # Parameters
    ///
    /// - `delta`: The width change in pixels.
    pub fn update_resize(&mut self, delta: f64) {
        // Apply the delta to the resizing column's width.
        if let Some(column_id) = &self.resizing_column {
            let column_id = column_id.clone();
            let current_width = self.get_width(&column_id);
            self.set_width(column_id, current_width + delta);
        }
    }

    /// Ends resizing.
    pub fn end_resize(&mut self) {
        // Clear the resizing column.
        self.resizing_column = None;
    }

    /// Initializes sizes for columns.
    ///
    /// # Parameters
    ///
    /// - `column_ids`: The column identifiers to initialize.
    /// - `default_width`: The default width to use, or None for the built-in default.
    pub fn initialize(&mut self, column_ids: &[ColumnId], default_width: Option<f64>) {
        // Determine the width to use.
        let width = default_width.unwrap_or(ColumnSize::DEFAULT_WIDTH);

        // Initialize sizes for columns that don't already have one.
        for id in column_ids {
            if !self.sizes.contains_key(id) {
                self.sizes.insert(id.clone(), ColumnSize::with_width(width));
            }
        }
    }

    /// Resets all sizes to defaults.
    pub fn reset(&mut self) {
        // Clear all stored sizes and stop any resize.
        self.sizes.clear();
        self.resizing_column = None;
    }

    /// Resets a specific column's size.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to reset.
    pub fn reset_column(&mut self, column_id: &ColumnId) {
        // Remove the column's size entry.
        self.sizes.remove(column_id);
    }

    /// Calculates total width of given columns.
    ///
    /// # Parameters
    ///
    /// - `column_ids`: The column identifiers to sum.
    ///
    /// # Returns
    ///
    /// - `f64`: The total width in pixels.
    pub fn total_width(&self, column_ids: &[ColumnId]) -> f64 {
        // Sum the widths of all specified columns.
        column_ids.iter().map(|id| self.get_width(id)).sum()
    }
}
