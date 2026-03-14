//! Custom aggregation function type for column-specific aggregation logic.
//!
//! Allows users to provide custom aggregation functions for columns
//! that require specialized aggregation behavior in grouped rows.

use std::fmt;

/// Type alias for the boxed aggregation closure.
type AggregateFn<T> = Box<dyn Fn(&[&T]) -> String + Send + Sync>;

/// Custom aggregation function type.
///
/// Wraps a function that aggregates a slice of row references
/// into a string representation.
pub struct AggregationFn<T> {
    /// The aggregation function.
    aggregate: AggregateFn<T>,
}

impl<T> AggregationFn<T> {
    /// Creates a new aggregation function.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that aggregates row references into a string.
    ///
    /// # Returns
    ///
    /// - `AggregationFn<T>`: A new aggregation function instance.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&[&T]) -> String + Send + Sync + 'static,
    {
        Self { aggregate: Box::new(f) }
    }

    /// Aggregates a list of rows.
    ///
    /// # Parameters
    ///
    /// - `rows`: A slice of row references to aggregate.
    ///
    /// # Returns
    ///
    /// - `String`: The aggregated result as a string.
    pub fn aggregate(&self, rows: &[&T]) -> String {
        // Invoke the aggregation function.
        (self.aggregate)(rows)
    }
}

/// Formats the aggregation function for debug output.
impl<T> fmt::Debug for AggregationFn<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AggregationFn").finish_non_exhaustive()
    }
}
