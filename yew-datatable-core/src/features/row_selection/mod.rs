/// Row selection mode enumeration.
///
/// Determines whether row selection is disabled, limited to a single row,
/// or allows multiple rows to be selected simultaneously.
pub mod row_selection_mode;

/// Complete row selection state for the table.
///
/// Manages the set of selected rows with support for single
/// and multi-row selection modes.
pub mod row_selection_state;

/// Re-exports for convenient access to row selection types.
///
/// Provides a centralized location for importing commonly used
/// row selection-related types and enums.
pub mod prelude;
