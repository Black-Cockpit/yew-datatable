/// Aggregated value for a column in grouped rows.
///
/// Represents the result of an aggregation operation on a column,
/// storing the computed value and the function used to compute it.
pub mod aggregated_value;

/// Custom aggregation function wrapper.
///
/// Allows users to provide custom aggregation functions for columns
/// requiring specialized aggregation behavior in grouped rows.
pub mod aggregation_fn;

/// Complete aggregation state for the table.
///
/// Manages column-level aggregation function assignments
/// and provides configuration for aggregation behavior.
pub mod aggregation_state;

/// Built-in aggregation functions for grouped rows.
///
/// Provides pre-built aggregation strategies for common
/// statistical operations on grouped row data.
pub mod built_in_aggregation;

/// Re-exports for convenient access to aggregation types.
///
/// Provides a centralized location for importing commonly used
/// aggregation-related types, enums, and structs.
pub mod prelude;
