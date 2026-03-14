//! Main table instance that coordinates all table functionality.
//!
//! `DataTable` is the primary entry point for interacting with the data table.
//! It manages columns, data, and state, and provides methods for all table operations.

use crate::column::column_def::ColumnDef;
use crate::column::column_id::ColumnId;
use crate::row::data_table_row::DataTableRow;
use crate::row::data_table_row_id::DataTableRowId;
use crate::row::data_table_row_model::DataTableRowModel;
use crate::state::data_table_state::DataTableState;
use crate::table::data_table_options::DataTableOptions;
use crate::table::data_table_options_builder::DataTableOptionsBuilder;

/// Main table instance that coordinates all table functionality.
///
/// Manages columns, data, and state, providing methods for sorting,
/// filtering, pagination, selection, and all other table operations.
pub struct DataTable<T: Clone> {
    /// Column definitions.
    columns: Vec<ColumnDef<T>>,

    /// Row model containing processed rows.
    row_model: DataTableRowModel<T>,

    /// Combined table state.
    state: DataTableState,

    /// Table options.
    options: DataTableOptions,
}

impl<T: Clone> DataTable<T> {
    /// Creates a new table with the given options.
    ///
    /// # Parameters
    ///
    /// - `options`: The table configuration options.
    ///
    /// # Returns
    ///
    /// - `DataTable<T>`: A new empty table instance.
    pub fn new(options: DataTableOptions) -> Self {
        Self {
            columns: Vec::new(),
            row_model: DataTableRowModel::default(),
            state: DataTableState::new(),
            options,
        }
    }

    /// Creates a table with columns and data.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column definitions.
    /// - `data`: The raw data rows.
    /// - `id_fn`: A function that generates a row ID from row data and index.
    ///
    /// # Returns
    ///
    /// - `DataTable<T>`: A new table with the provided columns and data.
    pub fn with_data<F>(columns: Vec<ColumnDef<T>>, data: Vec<T>, id_fn: F) -> Self
    where
        F: Fn(&T, usize) -> DataTableRowId,
    {
        Self {
            columns,
            row_model: DataTableRowModel::new(data, id_fn),
            state: DataTableState::new(),
            options: DataTableOptions::default(),
        }
    }

    /// Creates a table builder.
    ///
    /// # Returns
    ///
    /// - `DataTableOptionsBuilder`: A new options builder instance.
    pub fn builder() -> DataTableOptionsBuilder {
        DataTableOptionsBuilder::new()
    }

    /// Sets the column definitions.
    ///
    /// # Parameters
    ///
    /// - `columns`: The new column definitions.
    pub fn set_columns(&mut self, columns: Vec<ColumnDef<T>>) {
        // Replace the column definitions.
        self.columns = columns;

        // Mark the row model as needing reprocessing.
        self.row_model.invalidate();
    }

    /// Sets the table data.
    ///
    /// # Parameters
    ///
    /// - `data`: The new raw data rows.
    /// - `id_fn`: A function that generates a row ID from row data and index.
    pub fn set_data<F>(&mut self, data: Vec<T>, id_fn: F)
    where
        F: Fn(&T, usize) -> DataTableRowId,
    {
        // Update the row model with new data.
        self.row_model.set_data(data, id_fn);
    }

    /// Sets the data using index as row ID.
    ///
    /// # Parameters
    ///
    /// - `data`: The new raw data rows.
    pub fn set_data_indexed(&mut self, data: Vec<T>) {
        // Update the row model using index-based IDs.
        self.row_model.set_data(data, |_, idx| DataTableRowId::from_index(idx));
    }

    /// Returns the column definitions.
    ///
    /// # Returns
    ///
    /// - `&[ColumnDef<T>]`: A slice of column definitions.
    pub fn columns(&self) -> &[ColumnDef<T>] {
        &self.columns
    }

    /// Returns a mutable reference to the state.
    ///
    /// # Returns
    ///
    /// - `&mut DataTableState`: A mutable reference to the table state.
    pub fn state_mut(&mut self) -> &mut DataTableState {
        &mut self.state
    }

    /// Returns the table state.
    ///
    /// # Returns
    ///
    /// - `&DataTableState`: A reference to the table state.
    pub fn state(&self) -> &DataTableState {
        &self.state
    }

    /// Returns the table options.
    ///
    /// # Returns
    ///
    /// - `&DataTableOptions`: A reference to the table options.
    pub fn options(&self) -> &DataTableOptions {
        &self.options
    }

    /// Returns the row model.
    ///
    /// # Returns
    ///
    /// - `&DataTableRowModel<T>`: A reference to the row model.
    pub fn row_model(&self) -> &DataTableRowModel<T> {
        &self.row_model
    }

    /// Processes the row model with current state.
    pub fn process(&mut self) {
        // Run the full processing pipeline.
        self.row_model.process(
            &self.columns,
            &self.state.filtering,
            &self.state.sorting,
            &self.state.expanding,
            &self.state.grouping,
            &self.state.pagination,
            &self.state.row_selection,
        );
    }

    /// Returns visible rows after processing.
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item = &DataTableRow<T>>`: An iterator over visible rows.
    pub fn visible_rows(&self) -> impl Iterator<Item = &DataTableRow<T>> {
        self.row_model.visible_rows()
    }

    /// Returns the total number of rows.
    ///
    /// # Returns
    ///
    /// - `usize`: The total row count.
    pub fn total_row_count(&self) -> usize {
        self.row_model.total_row_count()
    }

    /// Returns the number of filtered rows.
    ///
    /// # Returns
    ///
    /// - `usize`: The filtered row count.
    pub fn filtered_row_count(&self) -> usize {
        self.row_model.filtered_row_count()
    }

    /// Returns the number of rows on the current page.
    ///
    /// # Returns
    ///
    /// - `usize`: The page row count.
    pub fn page_row_count(&self) -> usize {
        self.row_model.page_row_count()
    }

    /// Gets a row by ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The row identifier to look up.
    ///
    /// # Returns
    ///
    /// - `Option<&DataTableRow<T>>`: The row if found.
    pub fn get_row(&self, id: &DataTableRowId) -> Option<&DataTableRow<T>> {
        self.row_model.get_row(id)
    }

    /// Returns visible column IDs in order.
    ///
    /// # Returns
    ///
    /// - `Vec<ColumnId>`: The ordered list of visible column IDs.
    pub fn visible_column_ids(&self) -> Vec<ColumnId> {
        // Collect all column IDs.
        let all_ids: Vec<ColumnId> = self.columns.iter().map(|c| c.id().clone()).collect();

        // Apply custom ordering.
        let ordered = self.state.column_ordering.apply_order(&all_ids);

        // Apply pinning to reorder left/center/right.
        let pinned = self.state.column_pinning.apply_pinning(&ordered);

        // Filter by visibility.
        pinned
            .into_iter()
            .filter(|id| self.state.column_visibility.is_visible(id))
            .collect()
    }

    /// Returns visible columns in order.
    ///
    /// # Returns
    ///
    /// - `Vec<&ColumnDef<T>>`: References to visible column definitions.
    pub fn visible_columns(&self) -> Vec<&ColumnDef<T>> {
        // Get visible column IDs and look up their definitions.
        let visible_ids = self.visible_column_ids();
        visible_ids
            .iter()
            .filter_map(|id| self.columns.iter().find(|c| c.id() == id))
            .collect()
    }

    /// Gets a column by ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `Option<&ColumnDef<T>>`: The column definition if found.
    pub fn get_column(&self, id: &ColumnId) -> Option<&ColumnDef<T>> {
        // Search for the column by ID.
        self.columns.iter().find(|c| c.id() == id)
    }

    /// Toggles sorting for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to toggle sorting for.
    /// - `multi`: Whether to add to multi-sort list.
    pub fn toggle_sort(&mut self, column_id: impl Into<ColumnId>, multi: bool) {
        // Toggle the sort and invalidate the row model.
        self.state.sorting.toggle_sort(column_id, multi);
        self.row_model.invalidate();
    }

    /// Sets a column filter.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `value`: The filter value text.
    pub fn set_column_filter(&mut self, column_id: impl Into<ColumnId>, value: impl Into<String>) {
        // Set the filter, reset pagination, and invalidate.
        self.state.filtering.set_text_filter(column_id, value);
        self.state.pagination.go_to_first();
        self.row_model.invalidate();
    }

    /// Sets the global filter.
    ///
    /// # Parameters
    ///
    /// - `value`: The global search text.
    pub fn set_global_filter(&mut self, value: impl Into<String>) {
        // Set the global filter, reset pagination, and invalidate.
        self.state.filtering.set_global_filter(value);
        self.state.pagination.go_to_first();
        self.row_model.invalidate();
    }

    /// Toggles row selection.
    ///
    /// # Parameters
    ///
    /// - `row_id`: The row identifier to toggle selection for.
    pub fn toggle_row_selection(&mut self, row_id: DataTableRowId) {
        // Toggle the row selection state.
        self.state.row_selection.toggle(row_id);
    }

    /// Selects all filtered rows.
    pub fn select_all_rows(&mut self) {
        // Get filtered row IDs and select them all.
        let ids = self.row_model.filtered_row_ids();
        self.state.row_selection.select_all(ids);
    }

    /// Clears row selection.
    pub fn clear_selection(&mut self) {
        // Clear all selections.
        self.state.row_selection.clear();
    }

    /// Toggles row expansion.
    ///
    /// # Parameters
    ///
    /// - `row_id`: The row identifier to toggle expansion for.
    pub fn toggle_row_expansion(&mut self, row_id: DataTableRowId) {
        // Toggle the expansion state and invalidate.
        self.state.expanding.toggle(row_id);
        self.row_model.invalidate();
    }

    /// Toggles column visibility.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to toggle visibility for.
    pub fn toggle_column_visibility(&mut self, column_id: ColumnId) {
        // Toggle the column visibility state.
        self.state.column_visibility.toggle(column_id);
    }

    /// Goes to a specific page.
    ///
    /// # Parameters
    ///
    /// - `page`: The zero-based page index.
    pub fn go_to_page(&mut self, page: usize) {
        // Navigate to the specified page.
        let row_count = self.filtered_row_count();
        self.state.pagination.go_to_page(page, row_count);
    }

    /// Goes to the next page.
    pub fn next_page(&mut self) {
        // Navigate to the next page.
        let row_count = self.filtered_row_count();
        self.state.pagination.go_to_next(row_count);
    }

    /// Goes to the previous page.
    pub fn previous_page(&mut self) {
        // Navigate to the previous page.
        self.state.pagination.go_to_previous();
    }

    /// Sets the page size.
    ///
    /// # Parameters
    ///
    /// - `size`: The new page size.
    pub fn set_page_size(&mut self, size: usize) {
        // Update the page size.
        let row_count = self.filtered_row_count();
        self.state.pagination.set_page_size(size, row_count);
    }

    /// Resets all table state.
    pub fn reset(&mut self) {
        // Reset all state and invalidate.
        self.state.reset_all();
        self.row_model.invalidate();
    }
}
