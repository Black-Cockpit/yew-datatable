/// Built-in filter functions for common filtering strategies.
///
/// Provides pre-built filter functions for string matching,
/// numeric comparisons, and set membership operations.
pub mod built_in_filter;

/// Filter state for a single column.
///
/// Combines a column identifier with a filter value to represent
/// the active filter applied to that column.
pub mod column_filter;

/// Custom filter function wrapper.
///
/// Allows users to provide custom filter functions for columns
/// requiring specialized filtering behavior.
pub mod filter_fn;

/// Complete filtering state for the table.
///
/// Manages column-specific filters and the global filter,
/// along with case sensitivity configuration.
pub mod filter_state;

/// Filter value types for column filtering.
///
/// Represents different kinds of filter criteria including text,
/// numeric, boolean, multi-select, date range, and custom values.
pub mod filter_value;

/// Global filter that applies across all columns.
///
/// Provides cross-column text search with optional column
/// inclusion filtering.
pub mod global_filter;

/// Re-exports for convenient access to filtering types.
///
/// Provides a centralized location for importing commonly used
/// filtering-related types, enums, and structs.
pub mod prelude;
