/// Size info for a single column.
///
/// Stores the current width, minimum and maximum constraints,
/// and flex grow factor for a column.
pub mod column_size;

/// Column sizing mode enumeration.
///
/// Determines how columns are sized within the table layout,
/// supporting fixed, fit-content, and flex sizing strategies.
pub mod column_sizing_mode;

/// Complete column sizing state for the table.
///
/// Manages column widths, resize operations, and sizing mode
/// with support for minimum and maximum constraints.
pub mod column_sizing_state;

/// Re-exports for convenient access to column sizing types.
///
/// Provides a centralized location for importing commonly used
/// column sizing-related types, enums, and structs.
pub mod prelude;
