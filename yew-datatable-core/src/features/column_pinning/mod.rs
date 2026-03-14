/// Column pinning position enumeration.
///
/// Determines whether a column is pinned to the left or right
/// side of the table viewport.
pub mod column_pinning_position;

/// Complete column pinning state for the table.
///
/// Manages columns pinned to the left or right of the table viewport,
/// with support for pin/unpin operations and column reordering.
pub mod column_pinning_state;

/// Re-exports for convenient access to column pinning types.
///
/// Provides a centralized location for importing commonly used
/// column pinning-related types and enums.
pub mod prelude;
