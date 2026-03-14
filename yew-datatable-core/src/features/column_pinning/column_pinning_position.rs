//! Column pinning position enumeration.
//!
//! Determines whether a column is pinned to the left or right
//! side of the table viewport.

/// Column pinning position.
///
/// Specifies which side of the table a column is pinned to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ColumnPinningPosition {
    /// Pinned to the left.
    Left,

    /// Pinned to the right.
    Right,
}
