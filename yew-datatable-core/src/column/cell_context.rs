//! Context provided when rendering a table cell.
//!
//! Contains all information needed to render a cell value,
//! including the row data, column information, and table state.

use crate::column::column_id::ColumnId;
use crate::row::data_table_row_id::DataTableRowId;

/// Context provided when rendering a cell.
///
/// Contains all information needed to render a cell value,
/// including the row data, column information, and table state.
#[derive(Debug, Clone)]
pub struct DataTableCellContext<'a, T> {
    /// The row data.
    pub row: &'a T,

    /// The row ID.
    pub row_id: DataTableRowId,

    /// The row index in the current view.
    pub row_index: usize,

    /// The column ID.
    pub column_id: ColumnId,

    /// The column index.
    pub column_index: usize,

    /// Whether the row is selected.
    pub is_selected: bool,

    /// Whether the row is expanded.
    pub is_expanded: bool,

    /// The depth level for nested rows.
    pub depth: usize,
}

impl<'a, T> DataTableCellContext<'a, T> {
    /// Creates a new cell context.
    ///
    /// # Parameters
    ///
    /// - `row`: The row data reference.
    /// - `row_id`: The unique identifier of the row.
    /// - `row_index`: The row index in the current view.
    /// - `column_id`: The column identifier.
    /// - `column_index`: The column index in the current view.
    ///
    /// # Returns
    ///
    /// - `DataTableCellContext<'a, T>`: A new cell context with default selection, expansion, and depth state.
    pub fn new(row: &'a T, row_id: DataTableRowId, row_index: usize, column_id: ColumnId, column_index: usize) -> Self {
        Self {
            row,
            row_id,
            row_index,
            column_id,
            column_index,
            is_selected: false,
            is_expanded: false,
            depth: 0,
        }
    }

    /// Sets the selection state.
    ///
    /// # Parameters
    ///
    /// - `selected`: Whether the row is selected.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified cell context.
    pub fn with_selected(mut self, selected: bool) -> Self {
        // Update the selection state.
        self.is_selected = selected;
        self
    }

    /// Sets the expanded state.
    ///
    /// # Parameters
    ///
    /// - `expanded`: Whether the row is expanded.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified cell context.
    pub fn with_expanded(mut self, expanded: bool) -> Self {
        // Update the expansion state.
        self.is_expanded = expanded;
        self
    }

    /// Sets the depth level.
    ///
    /// # Parameters
    ///
    /// - `depth`: The nesting depth level.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified cell context.
    pub fn with_depth(mut self, depth: usize) -> Self {
        // Update the depth level.
        self.depth = depth;
        self
    }
}
