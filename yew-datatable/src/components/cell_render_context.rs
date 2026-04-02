//! Context passed to custom cell renderers.

use yew_datatable_core::prelude::DataTableRowId;

/// Context passed to custom cell renderers.
#[derive(Clone)]
pub struct CellRenderContext<T> {
    /// The row data.
    pub row: T,

    /// The row ID.
    pub row_id: DataTableRowId,

    /// The row index.
    pub row_index: usize,

    /// The column ID.
    pub column_id: String,

    /// The column index.
    pub column_index: usize,

    /// The cell value as a string.
    pub value: String,
}
