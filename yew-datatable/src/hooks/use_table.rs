//! The `use_table` hook for managing table state in Yew components.

use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew_datatable_core::prelude::{
    ColumnDef, ColumnId, DataTable, DataTableOptions, DataTableRow, DataTableRowId, DataTableState, SortDirection,
};

/// Handle returned by `use_table` hook.
#[derive(Clone)]
pub struct UseTableHandle<T: Clone + 'static> {
    table: Rc<RefCell<DataTable<T>>>,
    state: UseStateHandle<DataTableState>,
    trigger: UseStateHandle<u32>,
    data_version: Rc<RefCell<u32>>,
}

impl<T: Clone + 'static> PartialEq for UseTableHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        // Compare by trigger state and data version for reactivity
        *self.trigger == *other.trigger
            && *self.data_version.borrow() == *other.data_version.borrow()
            && Rc::ptr_eq(&self.table, &other.table)
    }
}

impl<T: Clone + 'static> UseTableHandle<T> {
    /// Returns the visible rows after processing.
    pub fn visible_rows(&self) -> Vec<DataTableRow<T>> {
        self.table.borrow().visible_rows().cloned().collect()
    }

    /// Returns the visible column IDs in order.
    pub fn visible_column_ids(&self) -> Vec<ColumnId> {
        self.table.borrow().visible_column_ids()
    }

    /// Returns visible columns.
    pub fn visible_columns(&self) -> Vec<ColumnId> {
        self.table.borrow().visible_column_ids()
    }

    /// Gets column header by ID.
    pub fn get_column_header(&self, id: &ColumnId) -> Option<String> {
        self.table.borrow().get_column(id).map(|c| c.header().to_string())
    }

    /// Gets whether a column is sortable.
    pub fn is_column_sortable(&self, id: &ColumnId) -> bool {
        self.table
            .borrow()
            .get_column(id)
            .map(|c| c.is_sortable())
            .unwrap_or(false)
    }

    /// Gets value from a row for a column.
    pub fn get_cell_value(&self, row: &T, column_id: &ColumnId) -> Option<String> {
        self.table
            .borrow()
            .get_column(column_id)
            .and_then(|col| col.get_value(row))
            .map(|v| v.as_string())
    }

    /// Returns the total number of rows.
    pub fn total_row_count(&self) -> usize {
        self.table.borrow().total_row_count()
    }

    /// Returns the number of filtered rows.
    pub fn filtered_row_count(&self) -> usize {
        self.table.borrow().filtered_row_count()
    }

    /// Returns the number of rows on the current page.
    pub fn page_row_count(&self) -> usize {
        self.table.borrow().page_row_count()
    }

    /// Returns the current table state.
    pub fn state(&self) -> DataTableState {
        (*self.state).clone()
    }

    /// Triggers a re-render.
    fn update(&self) {
        self.state.set(self.table.borrow().state().clone());
        self.trigger.set(*self.trigger + 1);
    }

    /// Toggles sorting for a column.
    pub fn toggle_sort(&self, column_id: impl Into<ColumnId>, multi: bool) {
        {
            let mut table = self.table.borrow_mut();
            table.toggle_sort(column_id, multi);
            table.process();
        }
        self.update();
    }

    /// Sets a column filter.
    pub fn set_column_filter(&self, column_id: impl Into<ColumnId>, value: impl Into<String>) {
        {
            let mut table = self.table.borrow_mut();
            table.set_column_filter(column_id, value);
            table.process();
        }
        self.update();
    }

    /// Sets the global filter.
    pub fn set_global_filter(&self, value: impl Into<String>) {
        {
            let mut table = self.table.borrow_mut();
            table.set_global_filter(value);
            table.process();
        }
        self.update();
    }

    /// Toggles row selection.
    pub fn toggle_row_selection(&self, row_id: DataTableRowId) {
        self.table.borrow_mut().toggle_row_selection(row_id);
        self.update();
    }

    /// Selects all rows.
    pub fn select_all_rows(&self) {
        self.table.borrow_mut().select_all_rows();
        self.update();
    }

    /// Clears row selection.
    pub fn clear_selection(&self) {
        self.table.borrow_mut().clear_selection();
        self.update();
    }

    /// Toggles row expansion.
    pub fn toggle_row_expansion(&self, row_id: DataTableRowId) {
        {
            let mut table = self.table.borrow_mut();
            table.toggle_row_expansion(row_id);
            table.process();
        }
        self.update();
    }

    /// Toggles column visibility.
    pub fn toggle_column_visibility(&self, column_id: ColumnId) {
        self.table.borrow_mut().toggle_column_visibility(column_id);
        self.update();
    }

    /// Goes to a specific page (0-indexed).
    pub fn go_to_page(&self, page: usize) {
        {
            let mut table = self.table.borrow_mut();
            table.go_to_page(page);
            table.process();
        }
        self.update();
    }

    /// Goes to the next page.
    pub fn next_page(&self) {
        {
            let mut table = self.table.borrow_mut();
            table.next_page();
            table.process();
        }
        self.update();
    }

    /// Goes to the previous page.
    pub fn previous_page(&self) {
        {
            let mut table = self.table.borrow_mut();
            table.previous_page();
            table.process();
        }
        self.update();
    }

    /// Sets the page size.
    pub fn set_page_size(&self, size: usize) {
        {
            let mut table = self.table.borrow_mut();
            table.set_page_size(size);
            table.process();
        }
        self.update();
    }

    /// Resets all table state.
    pub fn reset(&self) {
        {
            let mut table = self.table.borrow_mut();
            table.reset();
            table.process();
        }
        self.update();
    }

    /// Returns whether a row is selected.
    pub fn is_row_selected(&self, row_id: &DataTableRowId) -> bool {
        self.table.borrow().state().row_selection.is_selected(row_id)
    }

    /// Returns whether a row is expanded.
    pub fn is_row_expanded(&self, row_id: &DataTableRowId) -> bool {
        self.table.borrow().state().expanding.is_expanded(row_id)
    }

    /// Returns whether a column is visible.
    pub fn is_column_visible(&self, column_id: &ColumnId) -> bool {
        self.table.borrow().state().column_visibility.is_visible(column_id)
    }

    /// Returns the current page index (0-indexed).
    pub fn current_page(&self) -> usize {
        self.table.borrow().state().pagination.page_index()
    }

    /// Returns the page size.
    pub fn page_size(&self) -> usize {
        self.table.borrow().state().pagination.page_size()
    }

    /// Returns the total number of pages.
    pub fn page_count(&self) -> usize {
        let table = self.table.borrow();
        table.state().pagination.page_count(table.filtered_row_count())
    }

    /// Returns whether there is a previous page.
    pub fn can_previous_page(&self) -> bool {
        self.table.borrow().state().pagination.can_go_previous()
    }

    /// Returns whether there is a next page.
    pub fn can_next_page(&self) -> bool {
        let table = self.table.borrow();
        table.state().pagination.can_go_next(table.filtered_row_count())
    }

    /// Gets the sort direction for a column.
    pub fn get_sort_direction(&self, column_id: &ColumnId) -> Option<SortDirection> {
        self.table.borrow().state().sorting.get_direction(column_id)
    }

    /// Gets the sort index for a column (for multi-sort).
    pub fn get_sort_index(&self, column_id: &ColumnId) -> Option<usize> {
        self.table.borrow().state().sorting.get_sort_index(column_id)
    }

    /// Updates the data in the table.
    pub fn set_data(&self, data: Vec<T>) {
        // Update the table data and reprocess.
        {
            let mut table = self.table.borrow_mut();
            table.set_data_indexed(data);
            table.process();
        }
        // Increment the data version counter.
        *self.data_version.borrow_mut() += 1;
        // Trigger a re-render.
        self.update();
    }

    /// Returns the current data version (incremented on each data change).
    pub fn data_version(&self) -> u32 {
        *self.data_version.borrow()
    }
}

/// Hook for creating and managing a table instance.
///
/// # Arguments
///
/// * `columns` - Column definitions for the table
/// * `data` - The data to display in the table
/// * `options` - Optional table configuration
///
/// # Returns
///
/// A `UseTableHandle` that provides access to table state and methods.
#[hook]
pub fn use_table<T: Clone + PartialEq + 'static>(
    columns: Vec<ColumnDef<T>>,
    data: Vec<T>,
    _options: Option<DataTableOptions>,
) -> UseTableHandle<T> {
    // Store the table persistently across renders
    let table_ref: Rc<RefCell<DataTable<T>>> = use_mut_ref(|| {
        let mut t = DataTable::with_data(columns, data, |_, idx| DataTableRowId::from_index(idx));
        t.process();
        t
    });

    let data_version: Rc<RefCell<u32>> = use_mut_ref(|| 0);
    let state = use_state(|| table_ref.borrow().state().clone());
    let trigger = use_state(|| 0u32);

    UseTableHandle {
        table: table_ref,
        state,
        trigger,
        data_version,
    }
}
