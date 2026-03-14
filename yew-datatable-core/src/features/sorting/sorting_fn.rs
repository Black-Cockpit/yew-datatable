//! Custom sorting function type for column-specific sort logic.
//!
//! Allows users to provide custom comparison functions for columns
//! that require specialized sorting behavior.

use std::cmp::Ordering;
use std::fmt;

/// Type alias for the boxed sorting comparison closure.
type CompareFn<T> = Box<dyn Fn(&T, &T) -> Ordering + Send + Sync>;

/// Custom sorting function type.
///
/// Wraps a comparison function that determines the ordering
/// of two rows for a specific column.
pub struct SortingFn<T> {
    /// The comparison function.
    compare: CompareFn<T>,
}

impl<T> SortingFn<T> {
    /// Creates a new sorting function.
    ///
    /// # Parameters
    ///
    /// - `f`: A comparison function that returns the ordering of two row values.
    ///
    /// # Returns
    ///
    /// - `SortingFn<T>`: A new sorting function instance.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&T, &T) -> Ordering + Send + Sync + 'static,
    {
        Self { compare: Box::new(f) }
    }

    /// Compares two rows.
    ///
    /// # Parameters
    ///
    /// - `a`: The first row to compare.
    /// - `b`: The second row to compare.
    ///
    /// # Returns
    ///
    /// - `Ordering`: The comparison result.
    pub fn compare(&self, a: &T, b: &T) -> Ordering {
        // Invoke the comparison function.
        (self.compare)(a, b)
    }
}

/// Formats the sorting function for debug output.
impl<T> fmt::Debug for SortingFn<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SortingFn").finish_non_exhaustive()
    }
}
