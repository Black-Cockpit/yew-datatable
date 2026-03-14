/// Main DataTable component that assembles all sub-components.
///
/// Provides a complete data table with global filter, sortable headers,
/// row body, and pagination support.
pub mod data_table;

/// Pagination component for navigating table pages.
///
/// Displays page navigation controls, page size selector,
/// and row count information.
pub mod pagination;

/// Table body component for rendering rows and cells.
///
/// Supports row selection, custom cell rendering via callbacks,
/// and selectable row highlighting.
pub mod table_body;

/// Table header component for rendering column headers.
///
/// Supports click-to-sort with multi-column sorting via shift-click,
/// and visual sort direction indicators.
pub mod table_header;
