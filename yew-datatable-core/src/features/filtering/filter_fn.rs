//! Custom filter function type for column-specific filter logic.
//!
//! Allows users to provide custom filter functions for columns
//! that require specialized filtering behavior.

use std::fmt;

use crate::features::filtering::filter_value::FilterValue;

/// Type alias for the boxed filter closure.
type FilterClosureFn<T> = Box<dyn Fn(&T, &FilterValue) -> bool + Send + Sync>;

/// Custom filter function type.
///
/// Wraps a filter function that tests whether a row passes
/// a filter for a specific column.
pub struct FilterFn<T> {
    /// The filter function.
    filter: FilterClosureFn<T>,
}

impl<T> FilterFn<T> {
    /// Creates a new filter function.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that tests whether a row passes the filter.
    ///
    /// # Returns
    ///
    /// - `FilterFn<T>`: A new filter function instance.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&T, &FilterValue) -> bool + Send + Sync + 'static,
    {
        Self { filter: Box::new(f) }
    }

    /// Tests if a row passes the filter.
    ///
    /// # Parameters
    ///
    /// - `row`: The row data to test.
    /// - `value`: The filter value to test against.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the row passes the filter.
    pub fn test(&self, row: &T, value: &FilterValue) -> bool {
        // Invoke the filter function.
        (self.filter)(row, value)
    }
}

/// Formats the filter function for debug output.
impl<T> fmt::Debug for FilterFn<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilterFn").finish_non_exhaustive()
    }
}
