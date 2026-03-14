//! Complete filtering state for the table.
//!
//! Manages column-specific filters and the global filter,
//! along with case sensitivity configuration.

use std::collections::HashMap;

use crate::column::column_id::ColumnId;
use crate::features::filtering::column_filter::ColumnFilter;
use crate::features::filtering::global_filter::GlobalFilter;

/// Complete filtering state for the table.
///
/// Aggregates column-specific filters and the global filter
/// into a single manageable state structure.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FilterState {
    /// Column-specific filters.
    column_filters: HashMap<ColumnId, ColumnFilter>,

    /// Global filter.
    global_filter: GlobalFilter,

    /// Whether filtering is case-sensitive.
    case_sensitive: bool,
}

impl FilterState {
    /// Creates a new empty filter state.
    ///
    /// # Returns
    ///
    /// - `FilterState`: A new empty filter state with case-insensitive defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets case sensitivity for filtering.
    ///
    /// # Parameters
    ///
    /// - `case_sensitive`: Whether filtering should be case-sensitive.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified filter state.
    pub fn with_case_sensitive(mut self, case_sensitive: bool) -> Self {
        // Update the case sensitivity flag.
        self.case_sensitive = case_sensitive;
        self
    }

    /// Returns the column filters.
    ///
    /// # Returns
    ///
    /// - `&HashMap<ColumnId, ColumnFilter>`: A reference to the column filter map.
    pub fn column_filters(&self) -> &HashMap<ColumnId, ColumnFilter> {
        &self.column_filters
    }

    /// Returns the global filter.
    ///
    /// # Returns
    ///
    /// - `&GlobalFilter`: A reference to the global filter.
    pub fn global_filter(&self) -> &GlobalFilter {
        &self.global_filter
    }

    /// Returns whether filtering is case-sensitive.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether case-sensitive filtering is enabled.
    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }

    /// Returns whether any filtering is active.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether any column or global filter is active.
    pub fn is_filtered(&self) -> bool {
        // Check both column filters and global filter.
        !self.column_filters.is_empty() || !self.global_filter.is_empty()
    }

    /// Gets the filter for a specific column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `Option<&ColumnFilter>`: The column filter if active.
    pub fn get_column_filter(&self, column_id: &ColumnId) -> Option<&ColumnFilter> {
        // Look up the column filter in the map.
        self.column_filters.get(column_id)
    }

    /// Sets a column filter.
    ///
    /// # Parameters
    ///
    /// - `filter`: The column filter to set.
    pub fn set_column_filter(&mut self, filter: ColumnFilter) {
        // Remove the filter if the value is empty, otherwise insert it.
        if filter.value.is_empty() {
            self.column_filters.remove(&filter.column_id);
        } else {
            self.column_filters.insert(filter.column_id.clone(), filter);
        }
    }

    /// Sets a text filter for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `value`: The text value to filter by.
    pub fn set_text_filter(&mut self, column_id: impl Into<ColumnId>, value: impl Into<String>) {
        // Create and set a text column filter.
        self.set_column_filter(ColumnFilter::text(column_id, value));
    }

    /// Clears the filter for a specific column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to clear.
    pub fn clear_column_filter(&mut self, column_id: &ColumnId) {
        // Remove the column filter from the map.
        self.column_filters.remove(column_id);
    }

    /// Clears all column filters.
    pub fn clear_all_column_filters(&mut self) {
        // Clear the entire column filter map.
        self.column_filters.clear();
    }

    /// Sets the global filter value.
    ///
    /// # Parameters
    ///
    /// - `value`: The global search text.
    pub fn set_global_filter(&mut self, value: impl Into<String>) {
        // Update the global filter value.
        self.global_filter.value = value.into();
    }

    /// Clears the global filter.
    pub fn clear_global_filter(&mut self) {
        // Clear the global filter value.
        self.global_filter.value.clear();
    }

    /// Clears all filters.
    pub fn clear_all(&mut self) {
        // Clear both column filters and global filter.
        self.column_filters.clear();
        self.global_filter.value.clear();
    }

    /// Resets to initial state.
    pub fn reset(&mut self) {
        // Delegate to clear_all.
        self.clear_all();
    }
}
