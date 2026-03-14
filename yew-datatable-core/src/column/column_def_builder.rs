//! Builder pattern for creating column definitions with a fluent API.
//!
//! Provides a convenient way to construct `ColumnDef` instances
//! with various configuration options.

use crate::column::column_def::ColumnDef;
use crate::column::column_id::ColumnId;
use crate::column::column_meta::ColumnMeta;
use crate::column::data_table_dyn_accessor::DataTableDynAccessor;
use crate::column::data_table_dyn_value::DataTableDynValue;
use crate::features::aggregation::aggregation_fn::AggregationFn;
use crate::features::filtering::filter_fn::FilterFn;
use crate::features::sorting::sorting_fn::SortingFn;

/// Builder for creating column definitions with a fluent API.
///
/// Allows step-by-step configuration of column properties
/// before building the final `ColumnDef` instance.
pub struct ColumnDefBuilder<T> {
    /// Column metadata being constructed.
    meta: ColumnMeta,

    /// Optional dynamic accessor for cell values.
    accessor: Option<DataTableDynAccessor<T>>,

    /// Optional custom sorting function.
    sorting_fn: Option<SortingFn<T>>,

    /// Optional custom filter function.
    filter_fn: Option<FilterFn<T>>,

    /// Optional aggregation function for grouped rows.
    aggregation_fn: Option<AggregationFn<T>>,

    /// Child columns for group columns.
    columns: Vec<ColumnDef<T>>,

    /// Whether multi-column sorting is enabled.
    enable_multi_sort: bool,

    /// Whether to invert the sort direction.
    invert_sorting: bool,

    /// Whether undefined values should be sorted last.
    sort_undefined_last: bool,
}

impl<T> ColumnDefBuilder<T> {
    /// Creates a new column builder with the given ID and header.
    ///
    /// # Parameters
    ///
    /// - `id`: The column identifier.
    /// - `header`: The display header text.
    ///
    /// # Returns
    ///
    /// - `ColumnDefBuilder<T>`: A new builder instance with default settings.
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

    /// Sets the accessor function for this column.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that extracts a value from row data.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn accessor<V, F>(mut self, f: F) -> Self
    where
        V: DataTableDynValue + 'static,
        F: Fn(&T) -> V + Send + Sync + 'static,
    {
        // Create a dynamic accessor from the provided function.
        self.accessor = Some(DataTableDynAccessor::new(f));
        self
    }

    /// Sets the footer text.
    ///
    /// # Parameters
    ///
    /// - `footer`: The footer text to display.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        // Update the footer in the metadata.
        self.meta = self.meta.with_footer(footer);
        self
    }

    /// Sets whether the column is sortable.
    ///
    /// # Parameters
    ///
    /// - `sortable`: Whether sorting is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn sortable(mut self, sortable: bool) -> Self {
        // Update the sortable flag in the metadata.
        self.meta = self.meta.with_sortable(sortable);
        self
    }

    /// Sets whether the column is filterable.
    ///
    /// # Parameters
    ///
    /// - `filterable`: Whether filtering is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn filterable(mut self, filterable: bool) -> Self {
        // Update the filterable flag in the metadata.
        self.meta = self.meta.with_filterable(filterable);
        self
    }

    /// Sets whether the column is resizable.
    ///
    /// # Parameters
    ///
    /// - `resizable`: Whether resizing is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn resizable(mut self, resizable: bool) -> Self {
        // Update the resizable flag in the metadata.
        self.meta = self.meta.with_resizable(resizable);
        self
    }

    /// Sets whether the column is visible by default.
    ///
    /// # Parameters
    ///
    /// - `visible`: Whether the column is visible.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn visible(mut self, visible: bool) -> Self {
        // Update the visible flag in the metadata.
        self.meta = self.meta.with_visible(visible);
        self
    }

    /// Sets the minimum column width.
    ///
    /// # Parameters
    ///
    /// - `width`: The minimum width in pixels.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn min_width(mut self, width: f64) -> Self {
        // Set the minimum width constraint.
        self.meta = self.meta.with_min_width(width);
        self
    }

    /// Sets the maximum column width.
    ///
    /// # Parameters
    ///
    /// - `width`: The maximum width in pixels.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn max_width(mut self, width: f64) -> Self {
        // Set the maximum width constraint.
        self.meta = self.meta.with_max_width(width);
        self
    }

    /// Sets the default column width.
    ///
    /// # Parameters
    ///
    /// - `width`: The default width in pixels.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn default_width(mut self, width: f64) -> Self {
        // Set the default width.
        self.meta = self.meta.with_default_width(width);
        self
    }

    /// Sets the filter placeholder text.
    ///
    /// # Parameters
    ///
    /// - `placeholder`: The placeholder text for the filter input.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn filter_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        // Set the filter placeholder text.
        self.meta = self.meta.with_filter_placeholder(placeholder);
        self
    }

    /// Sets a custom sorting function.
    ///
    /// # Parameters
    ///
    /// - `f`: The custom sorting function.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn sorting_fn(mut self, f: SortingFn<T>) -> Self {
        // Set the custom sorting function.
        self.sorting_fn = Some(f);
        self
    }

    /// Sets a custom filter function.
    ///
    /// # Parameters
    ///
    /// - `f`: The custom filter function.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn filter_fn(mut self, f: FilterFn<T>) -> Self {
        // Set the custom filter function.
        self.filter_fn = Some(f);
        self
    }

    /// Sets an aggregation function for grouped rows.
    ///
    /// # Parameters
    ///
    /// - `f`: The aggregation function.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn aggregation_fn(mut self, f: AggregationFn<T>) -> Self {
        // Set the aggregation function.
        self.aggregation_fn = Some(f);
        self
    }

    /// Enables or disables multi-column sorting for this column.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether multi-sort is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn enable_multi_sort(mut self, enable: bool) -> Self {
        // Update the multi-sort flag.
        self.enable_multi_sort = enable;
        self
    }

    /// Inverts the sort direction for this column.
    ///
    /// # Parameters
    ///
    /// - `invert`: Whether to invert the sort direction.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn invert_sorting(mut self, invert: bool) -> Self {
        // Update the invert sorting flag.
        self.invert_sorting = invert;
        self
    }

    /// Sets whether undefined values should be sorted last.
    ///
    /// # Parameters
    ///
    /// - `last`: Whether undefined values sort last.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn sort_undefined_last(mut self, last: bool) -> Self {
        // Update the sort undefined last flag.
        self.sort_undefined_last = last;
        self
    }

    /// Adds child columns to make this a group column.
    ///
    /// # Parameters
    ///
    /// - `columns`: The child column definitions.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn columns(mut self, columns: Vec<ColumnDef<T>>) -> Self {
        // Mark as group column and set child columns.
        self.meta = self.meta.as_group();
        self.columns = columns;
        self
    }

    /// Builds the column definition.
    ///
    /// # Returns
    ///
    /// - `ColumnDef<T>`: The constructed column definition.
    pub fn build(self) -> ColumnDef<T> {
        // Construct the final column definition from the builder state.
        ColumnDef {
            meta: self.meta,
            accessor: self.accessor,
            sorting_fn: self.sorting_fn,
            filter_fn: self.filter_fn,
            aggregation_fn: self.aggregation_fn,
            columns: self.columns,
            enable_multi_sort: self.enable_multi_sort,
            invert_sorting: self.invert_sorting,
            sort_undefined_last: self.sort_undefined_last,
        }
    }
}

/// Provides a default column definition builder with empty ID and header.
impl<T> Default for ColumnDefBuilder<T> {
    fn default() -> Self {
        Self::new("", "")
    }
}
