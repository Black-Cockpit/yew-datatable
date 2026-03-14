//! Sort direction enumeration for column sorting.
//!
//! Represents the direction in which a column's values are sorted,
//! either ascending (A-Z, 0-9) or descending (Z-A, 9-0).

use std::cmp::Ordering;

/// Sort direction.
///
/// Determines the ordering of values when sorting a column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SortDirection {
    /// Ascending order (A-Z, 0-9).
    Asc,

    /// Descending order (Z-A, 9-0).
    Desc,
}

impl SortDirection {
    /// Toggles the sort direction.
    ///
    /// # Returns
    ///
    /// - `SortDirection`: The opposite direction.
    pub fn toggle(self) -> Self {
        // Return the opposite direction.
        match self {
            Self::Asc => Self::Desc,
            Self::Desc => Self::Asc,
        }
    }

    /// Returns the opposite direction.
    ///
    /// # Returns
    ///
    /// - `SortDirection`: The opposite direction.
    pub fn opposite(self) -> Self {
        // Delegate to toggle.
        self.toggle()
    }

    /// Applies this direction to an ordering.
    ///
    /// # Parameters
    ///
    /// - `ordering`: The base ordering to apply direction to.
    ///
    /// # Returns
    ///
    /// - `Ordering`: The directional ordering result.
    pub fn apply(self, ordering: Ordering) -> Ordering {
        // Reverse the ordering for descending direction.
        match self {
            Self::Asc => ordering,
            Self::Desc => ordering.reverse(),
        }
    }
}

/// Provides the default sort direction as ascending.
impl Default for SortDirection {
    fn default() -> Self {
        Self::Asc
    }
}
