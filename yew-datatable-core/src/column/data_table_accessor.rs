//! Type-safe accessor for extracting values from row data.
//!
//! Accessors provide compile-time guarantees that columns access
//! the correct fields from row data without string-based property access.

use std::fmt;
use std::marker::PhantomData;

/// Type-safe accessor for extracting values from row data.
///
/// Accessors provide compile-time guarantees that columns access
/// the correct fields from row data without string-based property access.
pub struct DataTableAccessor<T, V> {
    /// The accessor function that extracts a value from row data.
    accessor_fn: Box<dyn Fn(&T) -> V + Send + Sync>,

    /// Phantom data marker for type parameters.
    _marker: PhantomData<(T, V)>,
}

impl<T, V> DataTableAccessor<T, V> {
    /// Creates a new accessor from a function.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that extracts a value of type `V` from row data of type `T`.
    ///
    /// # Returns
    ///
    /// - `DataTableAccessor<T, V>`: A new accessor instance.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&T) -> V + Send + Sync + 'static,
    {
        Self {
            accessor_fn: Box::new(f),
            _marker: PhantomData,
        }
    }

    /// Gets the value from the row data.
    ///
    /// # Parameters
    ///
    /// - `row`: The row data to extract the value from.
    ///
    /// # Returns
    ///
    /// - `V`: The extracted value.
    pub fn get(&self, row: &T) -> V {
        // Invoke the accessor function on the row data.
        (self.accessor_fn)(row)
    }
}

/// Formats the accessor for debug output.
impl<T, V> fmt::Debug for DataTableAccessor<T, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataTableAccessor").finish_non_exhaustive()
    }
}
