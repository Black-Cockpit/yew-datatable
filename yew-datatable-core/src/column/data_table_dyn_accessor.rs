//! Dynamic accessor that can return any comparable value for sorting and filtering.
//!
//! Unlike the typed `DataTableAccessor`, this accessor returns boxed trait objects
//! that support dynamic comparison and display through the `DataTableDynValue` trait.

use std::fmt;

use crate::column::data_table_dyn_value::DataTableDynValue;

/// Type alias for the boxed dynamic accessor closure.
type DynAccessorFn<T> = Box<dyn Fn(&T) -> Box<dyn DataTableDynValue> + Send + Sync>;

/// Dynamic accessor that returns boxed trait objects for sorting and filtering.
///
/// This accessor type erases the concrete value type, allowing columns
/// with different value types to be stored and processed uniformly.
pub struct DataTableDynAccessor<T> {
    /// The accessor function that extracts a dynamic value from row data.
    accessor_fn: DynAccessorFn<T>,
}

impl<T> DataTableDynAccessor<T> {
    /// Creates a new dynamic accessor.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that extracts a value implementing `DataTableDynValue` from row data.
    ///
    /// # Returns
    ///
    /// - `DataTableDynAccessor<T>`: A new dynamic accessor instance.
    pub fn new<V, F>(f: F) -> Self
    where
        V: DataTableDynValue + 'static,
        F: Fn(&T) -> V + Send + Sync + 'static,
    {
        Self {
            accessor_fn: Box::new(move |row| Box::new(f(row))),
        }
    }

    /// Gets the dynamic value from the row data.
    ///
    /// # Parameters
    ///
    /// - `row`: The row data to extract the value from.
    ///
    /// # Returns
    ///
    /// - `Box<dyn DataTableDynValue>`: The extracted dynamic value.
    pub fn get(&self, row: &T) -> Box<dyn DataTableDynValue> {
        // Invoke the accessor function on the row data.
        (self.accessor_fn)(row)
    }
}

/// Formats the dynamic accessor for debug output.
impl<T> fmt::Debug for DataTableDynAccessor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataTableDynAccessor").finish_non_exhaustive()
    }
}
