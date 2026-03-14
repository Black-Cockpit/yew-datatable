//! Builder for table options with a fluent API.
//!
//! Provides a convenient way to construct `DataTableOptions` instances
//! with selective feature enablement.

use crate::table::data_table_options::DataTableOptions;

/// Builder for table options.
///
/// Allows step-by-step configuration of feature flags
/// before building the final `DataTableOptions` instance.
#[derive(Debug, Default)]
pub struct DataTableOptionsBuilder {
    /// The options being constructed.
    options: DataTableOptions,
}

impl DataTableOptionsBuilder {
    /// Creates a new builder with default options.
    ///
    /// # Returns
    ///
    /// - `DataTableOptionsBuilder`: A new builder with all features enabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables or disables sorting.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether sorting is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn sorting(mut self, enable: bool) -> Self {
        // Update the sorting flag.
        self.options.enable_sorting = enable;
        self
    }

    /// Enables or disables multi-column sorting.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether multi-column sorting is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn multi_sort(mut self, enable: bool) -> Self {
        // Update the multi-sort flag.
        self.options.enable_multi_sort = enable;
        self
    }

    /// Enables or disables filtering.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether filtering is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn filtering(mut self, enable: bool) -> Self {
        // Update the filtering flag.
        self.options.enable_filtering = enable;
        self
    }

    /// Enables or disables global filtering.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether global filtering is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn global_filter(mut self, enable: bool) -> Self {
        // Update the global filter flag.
        self.options.enable_global_filter = enable;
        self
    }

    /// Enables or disables pagination.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether pagination is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn pagination(mut self, enable: bool) -> Self {
        // Update the pagination flag.
        self.options.enable_pagination = enable;
        self
    }

    /// Enables or disables row selection.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether row selection is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn row_selection(mut self, enable: bool) -> Self {
        // Update the row selection flag.
        self.options.enable_row_selection = enable;
        self
    }

    /// Enables or disables row expansion.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether row expansion is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn expanding(mut self, enable: bool) -> Self {
        // Update the expanding flag.
        self.options.enable_expanding = enable;
        self
    }

    /// Enables or disables column visibility.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether column visibility is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn column_visibility(mut self, enable: bool) -> Self {
        // Update the column visibility flag.
        self.options.enable_column_visibility = enable;
        self
    }

    /// Enables or disables column ordering.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether column ordering is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn column_ordering(mut self, enable: bool) -> Self {
        // Update the column ordering flag.
        self.options.enable_column_ordering = enable;
        self
    }

    /// Enables or disables column pinning.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether column pinning is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn column_pinning(mut self, enable: bool) -> Self {
        // Update the column pinning flag.
        self.options.enable_column_pinning = enable;
        self
    }

    /// Enables or disables column resizing.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether column resizing is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn column_resizing(mut self, enable: bool) -> Self {
        // Update the column resizing flag.
        self.options.enable_column_resizing = enable;
        self
    }

    /// Enables or disables row grouping.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether row grouping is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn grouping(mut self, enable: bool) -> Self {
        // Update the grouping flag.
        self.options.enable_grouping = enable;
        self
    }

    /// Enables or disables debug mode.
    ///
    /// # Parameters
    ///
    /// - `enable`: Whether debug mode is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn debug(mut self, enable: bool) -> Self {
        // Update the debug flag.
        self.options.debug = enable;
        self
    }

    /// Builds the table options.
    ///
    /// # Returns
    ///
    /// - `DataTableOptions`: The constructed table options.
    pub fn build(self) -> DataTableOptions {
        // Return the constructed options.
        self.options
    }
}
