//! Options for table configuration.
//!
//! Controls which features are enabled or disabled in the table,
//! including sorting, filtering, pagination, selection, and more.

/// Options for table configuration.
///
/// Each field controls whether a specific feature is enabled
/// in the table instance.
#[derive(Debug, Clone)]
pub struct DataTableOptions {
    /// Whether to enable sorting.
    pub enable_sorting: bool,

    /// Whether to enable multi-column sorting.
    pub enable_multi_sort: bool,

    /// Whether to enable filtering.
    pub enable_filtering: bool,

    /// Whether to enable global filtering.
    pub enable_global_filter: bool,

    /// Whether to enable pagination.
    pub enable_pagination: bool,

    /// Whether to enable row selection.
    pub enable_row_selection: bool,

    /// Whether to enable row expansion.
    pub enable_expanding: bool,

    /// Whether to enable column visibility.
    pub enable_column_visibility: bool,

    /// Whether to enable column ordering.
    pub enable_column_ordering: bool,

    /// Whether to enable column pinning.
    pub enable_column_pinning: bool,

    /// Whether to enable column resizing.
    pub enable_column_resizing: bool,

    /// Whether to enable row grouping.
    pub enable_grouping: bool,

    /// Debug mode.
    pub debug: bool,
}

/// Provides default table options with all features enabled.
impl Default for DataTableOptions {
    fn default() -> Self {
        Self {
            enable_sorting: true,
            enable_multi_sort: true,
            enable_filtering: true,
            enable_global_filter: true,
            enable_pagination: true,
            enable_row_selection: true,
            enable_expanding: true,
            enable_column_visibility: true,
            enable_column_ordering: true,
            enable_column_pinning: true,
            enable_column_resizing: true,
            enable_grouping: true,
            debug: false,
        }
    }
}
