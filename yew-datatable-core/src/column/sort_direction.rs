//! Sort direction indicator for header context rendering.
//!
//! Represents the visual sort direction state displayed in column
//! headers when sorting is active on a column.

/// Sort direction for header context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataTableSortDirection {
    /// Ascending order.
    Asc,

    /// Descending order.
    Desc,
}
