/// Built-in sorting strategies for common comparison patterns.
///
/// Provides pre-built sorting functions for alphanumeric, numeric,
/// date/time, and basic lexicographic sorting.
pub mod built_in_sorting;

/// Natural ordering comparison for strings.
///
/// Compares strings with embedded numbers by their numeric value,
/// supporting both ASCII and Unicode numeric digits.
pub mod natord;

/// Sort direction enumeration.
///
/// Defines ascending and descending sort directions with
/// ordering transformation support.
pub mod sort_direction;

/// Single column sort state.
///
/// Represents the sort configuration for one column,
/// combining column identity with sort direction.
pub mod sort_state;

/// Custom sorting function wrapper.
///
/// Allows users to provide custom comparison functions
/// for columns requiring specialized sorting behavior.
pub mod sorting_fn;

/// Complete sorting state for the table.
///
/// Manages the list of active sorts, multi-column sorting
/// configuration, and sort removal behavior.
pub mod sorting_state;

/// Re-exports for convenient access to sorting types.
///
/// Provides a centralized location for importing commonly used
/// sorting-related types, enums, and structs.
pub mod prelude;
