//! Context provided when rendering a column header.
//!
//! Contains all information needed to render a column header,
//! including sort state, filter capability, and resize state.

use crate::column::column_id::ColumnId;
use crate::column::sort_direction::DataTableSortDirection;

/// Context provided when rendering a column header.
///
/// Contains sort state, filter capability, resize state,
/// pinning status, and column width information.
#[derive(Debug, Clone)]
pub struct DataTableHeaderContext {
    /// The column ID.
    pub column_id: ColumnId,

    /// The column index.
    pub column_index: usize,

    /// Whether the column is currently sorted.
    pub is_sorted: bool,

    /// The sort direction if sorted.
    pub sort_direction: Option<DataTableSortDirection>,

    /// The sort index for multi-column sorting.
    pub sort_index: Option<usize>,

    /// Whether the column can be sorted.
    pub can_sort: bool,

    /// Whether the column can be filtered.
    pub can_filter: bool,

    /// Whether the column can be resized.
    pub can_resize: bool,

    /// Whether the column is pinned.
    pub is_pinned: bool,

    /// The current column width.
    pub width: Option<f64>,
}

impl DataTableHeaderContext {
    /// Creates a new header context.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `column_index`: The column index in the current view.
    ///
    /// # Returns
    ///
    /// - `DataTableHeaderContext`: A new header context with default capabilities enabled.
    pub fn new(column_id: ColumnId, column_index: usize) -> Self {
        Self {
            column_id,
            column_index,
            is_sorted: false,
            sort_direction: None,
            sort_index: None,
            can_sort: true,
            can_filter: true,
            can_resize: true,
            is_pinned: false,
            width: None,
        }
    }

    /// Sets the sort state.
    ///
    /// # Parameters
    ///
    /// - `direction`: The sort direction.
    /// - `index`: The sort priority index for multi-column sorting.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified header context.
    pub fn with_sort(mut self, direction: DataTableSortDirection, index: usize) -> Self {
        // Mark the column as sorted and set direction and index.
        self.is_sorted = true;
        self.sort_direction = Some(direction);
        self.sort_index = Some(index);
        self
    }

    /// Sets whether the column can be sorted.
    ///
    /// # Parameters
    ///
    /// - `can_sort`: Whether sorting is enabled for this column.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified header context.
    pub fn with_can_sort(mut self, can_sort: bool) -> Self {
        // Update the sortable capability.
        self.can_sort = can_sort;
        self
    }

    /// Sets whether the column can be filtered.
    ///
    /// # Parameters
    ///
    /// - `can_filter`: Whether filtering is enabled for this column.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified header context.
    pub fn with_can_filter(mut self, can_filter: bool) -> Self {
        // Update the filterable capability.
        self.can_filter = can_filter;
        self
    }

    /// Sets whether the column can be resized.
    ///
    /// # Parameters
    ///
    /// - `can_resize`: Whether resizing is enabled for this column.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified header context.
    pub fn with_can_resize(mut self, can_resize: bool) -> Self {
        // Update the resizable capability.
        self.can_resize = can_resize;
        self
    }

    /// Sets the pinned state.
    ///
    /// # Parameters
    ///
    /// - `pinned`: Whether the column is pinned.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified header context.
    pub fn with_pinned(mut self, pinned: bool) -> Self {
        // Update the pinned state.
        self.is_pinned = pinned;
        self
    }

    /// Sets the column width.
    ///
    /// # Parameters
    ///
    /// - `width`: The column width in pixels.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified header context.
    pub fn with_width(mut self, width: f64) -> Self {
        // Set the column width.
        self.width = Some(width);
        self
    }
}
