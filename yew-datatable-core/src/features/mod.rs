/// Aggregation functions for grouped row data.
///
/// Provides built-in and custom aggregation functions for computing
/// summary values from grouped rows.
pub mod aggregation;

/// Column ordering feature.
///
/// Supports reordering columns dynamically with move, swap,
/// and relative positioning operations.
pub mod column_ordering;

/// Column pinning feature.
///
/// Supports pinning columns to the left or right of the table
/// viewport for persistent visibility during horizontal scrolling.
pub mod column_pinning;

/// Column sizing feature.
///
/// Supports column resizing with minimum and maximum constraints,
/// supporting fixed, fit-content, and flex sizing modes.
pub mod column_sizing;

/// Column visibility feature.
///
/// Supports showing and hiding columns dynamically with
/// per-column overrides and default visibility settings.
pub mod column_visibility;

/// Row expansion feature.
///
/// Supports expandable rows for tree data and nested sub-rows,
/// with expand-all and auto-expand capabilities.
pub mod expanding;

/// Filtering feature.
///
/// Supports column-specific filters and global cross-column filtering
/// with built-in and custom filter functions.
pub mod filtering;

/// Row grouping feature.
///
/// Supports grouping rows by column values with multi-level grouping,
/// aggregation display, and default expansion behavior.
pub mod grouping;

/// Pagination feature.
///
/// Supports client-side and server-side pagination with configurable
/// page sizes and navigation methods.
pub mod pagination;

/// Row selection feature.
///
/// Supports single and multi-row selection with toggle, select-all,
/// and mode-aware selection behavior.
pub mod row_selection;

/// Sorting feature.
///
/// Supports multi-column sorting with stable sort algorithm,
/// custom sort functions, and built-in sorting strategies.
pub mod sorting;
