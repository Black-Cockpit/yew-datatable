//! Definition of a table column.
//!
//! `ColumnDef` is generic over the row data type `T` and provides
//! all configuration needed to display and interact with a column.

use std::fmt;

use crate::column::column_id::ColumnId;
use crate::column::column_meta::ColumnMeta;
use crate::column::data_table_dyn_accessor::DataTableDynAccessor;
use crate::column::data_table_dyn_value::DataTableDynValue;
use crate::features::aggregation::aggregation_fn::AggregationFn;
use crate::features::filtering::filter_fn::FilterFn;
use crate::features::sorting::sorting_fn::SortingFn;

/// Definition of a table column.
///
/// `ColumnDef` is generic over the row data type `T` and provides
/// all configuration needed to display and interact with a column.
pub struct ColumnDef<T> {
    /// Column metadata (ID, header, footer, etc.).
    pub meta: ColumnMeta,

    /// Accessor function to get the cell value from row data.
    pub accessor: Option<DataTableDynAccessor<T>>,

    /// Custom sorting function.
    pub sorting_fn: Option<SortingFn<T>>,

    /// Custom filter function.
    pub filter_fn: Option<FilterFn<T>>,

    /// Aggregation function for grouped rows.
    pub aggregation_fn: Option<AggregationFn<T>>,

    /// Child columns for grouped columns.
    pub columns: Vec<ColumnDef<T>>,

    /// Enable multi-sorting for this column.
    pub enable_multi_sort: bool,

    /// Invert the sort direction.
    pub invert_sorting: bool,

    /// Sort undefined values to the end.
    pub sort_undefined_last: bool,
}

impl<T> ColumnDef<T> {
    /// Creates a new column definition with the given ID and header.
    ///
    /// # Parameters
    ///
    /// - `id`: The column identifier.
    /// - `header`: The display header text.
    ///
    /// # Returns
    ///
    /// - `ColumnDef<T>`: A new column definition with default settings.
    pub fn new(id: impl Into<ColumnId>, header: impl Into<String>) -> Self {
        Self {
            meta: ColumnMeta::new(id, header),
            accessor: None,
            sorting_fn: None,
            filter_fn: None,
            aggregation_fn: None,
            columns: Vec::new(),
            enable_multi_sort: true,
            invert_sorting: false,
            sort_undefined_last: true,
        }
    }

    /// Creates a group column with child columns.
    ///
    /// # Parameters
    ///
    /// - `id`: The column identifier.
    /// - `header`: The display header text.
    /// - `columns`: The child column definitions.
    ///
    /// # Returns
    ///
    /// - `ColumnDef<T>`: A new group column definition.
    pub fn group(id: impl Into<ColumnId>, header: impl Into<String>, columns: Vec<ColumnDef<T>>) -> Self {
        Self {
            meta: ColumnMeta::new(id, header).as_group(),
            accessor: None,
            sorting_fn: None,
            filter_fn: None,
            aggregation_fn: None,
            columns,
            enable_multi_sort: false,
            invert_sorting: false,
            sort_undefined_last: true,
        }
    }

    /// Returns the column ID.
    ///
    /// # Returns
    ///
    /// - `&ColumnId`: A reference to the column identifier.
    pub fn id(&self) -> &ColumnId {
        &self.meta.id
    }

    /// Returns the column header text.
    ///
    /// # Returns
    ///
    /// - `&str`: The column header text.
    pub fn header(&self) -> &str {
        &self.meta.header
    }

    /// Returns whether this column is sortable.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether sorting is enabled for this column.
    pub fn is_sortable(&self) -> bool {
        self.meta.sortable
    }

    /// Returns whether this column is filterable.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether filtering is enabled for this column.
    pub fn is_filterable(&self) -> bool {
        self.meta.filterable
    }

    /// Returns whether this column is a group column.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether this is a group column containing sub-columns.
    pub fn is_group(&self) -> bool {
        self.meta.is_group
    }

    /// Returns the child columns for group columns.
    ///
    /// # Returns
    ///
    /// - `&[ColumnDef<T>]`: A slice of child column definitions.
    pub fn children(&self) -> &[ColumnDef<T>] {
        &self.columns
    }

    /// Gets the value from the row using the accessor.
    ///
    /// # Parameters
    ///
    /// - `row`: The row data to extract the value from.
    ///
    /// # Returns
    ///
    /// - `Option<Box<dyn DataTableDynValue>>`: The extracted value, or None if no accessor is set.
    pub fn get_value(&self, row: &T) -> Option<Box<dyn DataTableDynValue>> {
        // Invoke the accessor if present.
        self.accessor.as_ref().map(|a: &DataTableDynAccessor<T>| a.get(row))
    }

    /// Flattens the column hierarchy into a list of leaf columns.
    ///
    /// # Returns
    ///
    /// - `Vec<&ColumnDef<T>>`: A vector of references to leaf columns.
    pub fn flatten(&self) -> Vec<&ColumnDef<T>> {
        // Check if this column has children.
        if self.columns.is_empty() {
            vec![self]
        } else {
            // Recursively flatten all child columns.
            self.columns.iter().flat_map(|c| c.flatten()).collect()
        }
    }

    /// Returns all column IDs including nested columns.
    ///
    /// # Returns
    ///
    /// - `Vec<ColumnId>`: A vector of all column IDs in the hierarchy.
    pub fn all_ids(&self) -> Vec<ColumnId> {
        // Start with this column's ID.
        let mut ids = vec![self.meta.id.clone()];

        // Collect IDs from all child columns.
        for child in &self.columns {
            ids.extend(child.all_ids());
        }

        ids
    }
}

/// Formats the column definition for debug output.
impl<T> fmt::Debug for ColumnDef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ColumnDef")
            .field("meta", &self.meta)
            .field("columns", &self.columns.len())
            .finish_non_exhaustive()
    }
}
