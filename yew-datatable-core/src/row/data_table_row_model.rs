//! Row model that manages data processing through the table pipeline.
//!
//! Handles filtering, sorting, grouping, expansion, and pagination
//! of row data, maintaining index mappings at each processing stage.

use std::collections::HashMap;

use crate::column::column_def::ColumnDef;
use crate::features::expanding::expanding_state::ExpandingState;
use crate::features::filtering::built_in_filter::BuiltInFilter;
use crate::features::filtering::filter_state::FilterState;
use crate::features::grouping::grouping_state::GroupingState;
use crate::features::pagination::pagination_state::PaginationState;
use crate::features::row_selection::row_selection_state::RowSelectionState;
use crate::features::sorting::sort_direction::SortDirection;
use crate::features::sorting::sorting_state::SortingState;
use crate::row::data_table_row::DataTableRow;
use crate::row::data_table_row_id::DataTableRowId;

/// Row model containing processed rows and index mappings.
///
/// Manages the complete row processing pipeline from raw data
/// through filtering, sorting, grouping, expansion, and pagination.
pub struct DataTableRowModel<T> {
    /// All rows in the model.
    rows: Vec<DataTableRow<T>>,

    /// Map from row ID to index for fast lookup.
    row_index_map: HashMap<DataTableRowId, usize>,

    /// Indices after filtering.
    filtered_indices: Vec<usize>,

    /// Indices after sorting.
    sorted_indices: Vec<usize>,

    /// Indices after expansion.
    expanded_indices: Vec<usize>,

    /// Indices after pagination (final visible set).
    paginated_indices: Vec<usize>,

    /// Whether the model needs reprocessing.
    dirty: bool,
}

/// Provides a default empty row model with no data loaded.
impl<T> Default for DataTableRowModel<T> {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            row_index_map: HashMap::new(),
            filtered_indices: Vec::new(),
            sorted_indices: Vec::new(),
            expanded_indices: Vec::new(),
            paginated_indices: Vec::new(),
            dirty: false,
        }
    }
}

impl<T> DataTableRowModel<T> {
    /// Creates a new row model with the given data.
    ///
    /// # Parameters
    ///
    /// - `data`: The raw data rows.
    /// - `id_fn`: A function that generates a row ID from row data and index.
    ///
    /// # Returns
    ///
    /// - `DataTableRowModel<T>`: A new row model with all indices initialized.
    pub fn new<F>(data: Vec<T>, id_fn: F) -> Self
    where
        F: Fn(&T, usize) -> DataTableRowId,
    {
        // Create rows with IDs from the provided function.
        let rows: Vec<DataTableRow<T>> = data
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let id = id_fn(&item, index);
                DataTableRow::new(id, item, index)
            })
            .collect();

        // Build the row ID to index lookup map.
        let row_index_map: HashMap<DataTableRowId, usize> = rows
            .iter()
            .enumerate()
            .map(|(idx, row)| (row.id.clone(), idx))
            .collect();

        // Initialize all index vectors with sequential indices.
        let indices: Vec<usize> = (0..rows.len()).collect();

        Self {
            rows,
            row_index_map,
            filtered_indices: indices.clone(),
            sorted_indices: indices.clone(),
            expanded_indices: indices.clone(),
            paginated_indices: indices,
            dirty: false,
        }
    }

    /// Creates a row model using index as ID.
    ///
    /// # Parameters
    ///
    /// - `data`: The raw data rows.
    ///
    /// # Returns
    ///
    /// - `DataTableRowModel<T>`: A new row model with index-based row IDs.
    pub fn from_data(data: Vec<T>) -> Self {
        // Use the index as the row ID.
        Self::new(data, |_, index| DataTableRowId::from_index(index))
    }

    /// Returns the total number of rows.
    ///
    /// # Returns
    ///
    /// - `usize`: The total row count.
    pub fn total_row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the number of filtered rows.
    ///
    /// # Returns
    ///
    /// - `usize`: The filtered row count.
    pub fn filtered_row_count(&self) -> usize {
        self.filtered_indices.len()
    }

    /// Returns the number of rows in the current page.
    ///
    /// # Returns
    ///
    /// - `usize`: The paginated row count.
    pub fn page_row_count(&self) -> usize {
        self.paginated_indices.len()
    }

    /// Returns all rows.
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item = &DataTableRow<T>>`: An iterator over all rows.
    pub fn all_rows(&self) -> impl Iterator<Item = &DataTableRow<T>> {
        self.rows.iter()
    }

    /// Returns filtered rows.
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item = &DataTableRow<T>>`: An iterator over filtered rows.
    pub fn filtered_rows(&self) -> impl Iterator<Item = &DataTableRow<T>> {
        self.filtered_indices.iter().map(|&idx| &self.rows[idx])
    }

    /// Returns sorted rows.
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item = &DataTableRow<T>>`: An iterator over sorted rows.
    pub fn sorted_rows(&self) -> impl Iterator<Item = &DataTableRow<T>> {
        self.sorted_indices.iter().map(|&idx| &self.rows[idx])
    }

    /// Returns the final visible rows (after all processing).
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item = &DataTableRow<T>>`: An iterator over visible rows.
    pub fn visible_rows(&self) -> impl Iterator<Item = &DataTableRow<T>> {
        self.paginated_indices.iter().map(|&idx| &self.rows[idx])
    }

    /// Returns visible rows as a vector.
    ///
    /// # Returns
    ///
    /// - `Vec<&DataTableRow<T>>`: A vector of references to visible rows.
    pub fn visible_rows_vec(&self) -> Vec<&DataTableRow<T>> {
        self.paginated_indices.iter().map(|&idx| &self.rows[idx]).collect()
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
        // Look up the index and return the row.
        self.row_index_map.get(id).map(|&idx| &self.rows[idx])
    }

    /// Gets a row by index.
    ///
    /// # Parameters
    ///
    /// - `index`: The row index to look up.
    ///
    /// # Returns
    ///
    /// - `Option<&DataTableRow<T>>`: The row if the index is valid.
    pub fn get_row_by_index(&self, index: usize) -> Option<&DataTableRow<T>> {
        self.rows.get(index)
    }

    /// Returns all row IDs.
    ///
    /// # Returns
    ///
    /// - `Vec<DataTableRowId>`: A vector of all row IDs.
    pub fn all_row_ids(&self) -> Vec<DataTableRowId> {
        self.rows.iter().map(|r| r.id.clone()).collect()
    }

    /// Returns filtered row IDs.
    ///
    /// # Returns
    ///
    /// - `Vec<DataTableRowId>`: A vector of filtered row IDs.
    pub fn filtered_row_ids(&self) -> Vec<DataTableRowId> {
        self.filtered_indices
            .iter()
            .map(|&idx| self.rows[idx].id.clone())
            .collect()
    }

    /// Returns visible row IDs.
    ///
    /// # Returns
    ///
    /// - `Vec<DataTableRowId>`: A vector of visible row IDs.
    pub fn visible_row_ids(&self) -> Vec<DataTableRowId> {
        self.paginated_indices
            .iter()
            .map(|&idx| self.rows[idx].id.clone())
            .collect()
    }

    /// Marks the model as needing recomputation.
    pub fn invalidate(&mut self) {
        // Set the dirty flag.
        self.dirty = true;
    }

    /// Checks if the model needs recomputation.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the model needs reprocessing.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl<T: Clone> DataTableRowModel<T> {
    /// Updates the data and rebuilds the model.
    ///
    /// # Parameters
    ///
    /// - `data`: The new raw data rows.
    /// - `id_fn`: A function that generates a row ID from row data and index.
    pub fn set_data<F>(&mut self, data: Vec<T>, id_fn: F)
    where
        F: Fn(&T, usize) -> DataTableRowId,
    {
        // Rebuild all rows from new data.
        self.rows = data
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let id = id_fn(&item, index);
                DataTableRow::new(id, item, index)
            })
            .collect();

        // Rebuild the row ID to index lookup map.
        self.row_index_map = self
            .rows
            .iter()
            .enumerate()
            .map(|(idx, row)| (row.id.clone(), idx))
            .collect();

        // Reinitialize all index vectors.
        let indices: Vec<usize> = (0..self.rows.len()).collect();
        self.filtered_indices = indices.clone();
        self.sorted_indices = indices.clone();
        self.expanded_indices = indices.clone();
        self.paginated_indices = indices;

        // Mark as needing reprocessing.
        self.dirty = true;
    }

    /// Applies the complete row model pipeline.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column definitions.
    /// - `filter_state`: The filtering state.
    /// - `sorting_state`: The sorting state.
    /// - `expanding_state`: The expanding state.
    /// - `grouping_state`: The grouping state.
    /// - `pagination_state`: The pagination state.
    /// - `_selection_state`: The row selection state (reserved for future use).
    #[allow(clippy::too_many_arguments)]
    pub fn process(
        &mut self,
        columns: &[ColumnDef<T>],
        filter_state: &FilterState,
        sorting_state: &SortingState,
        expanding_state: &ExpandingState,
        grouping_state: &GroupingState,
        pagination_state: &PaginationState,
        _selection_state: &RowSelectionState,
    ) {
        // Apply the pipeline stages in order.
        self.apply_filtering(columns, filter_state);
        self.apply_sorting(columns, sorting_state);
        self.apply_grouping(columns, grouping_state);
        self.apply_expansion(expanding_state);
        self.apply_pagination(pagination_state);

        // Mark as clean after processing.
        self.dirty = false;
    }

    /// Applies filtering to the rows.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column definitions.
    /// - `filter_state`: The filtering state.
    fn apply_filtering(&mut self, columns: &[ColumnDef<T>], filter_state: &FilterState) {
        // Skip filtering if no filters are active.
        if !filter_state.is_filtered() {
            self.filtered_indices = (0..self.rows.len()).collect();
            return;
        }

        // Filter rows based on global and column filters.
        self.filtered_indices = (0..self.rows.len())
            .filter(|&idx| {
                let row = &self.rows[idx];

                // Check global filter first.
                if !filter_state.global_filter().is_empty() {
                    let global_value = filter_state.global_filter().value.to_lowercase();
                    let mut matches_global = false;

                    // Test each column against the global filter.
                    for column in columns {
                        if !filter_state.global_filter().includes_column(column.id()) {
                            continue;
                        }
                        if let Some(value) = column.get_value(&row.original) {
                            if value.as_string().to_lowercase().contains(&global_value) {
                                matches_global = true;
                                break;
                            }
                        }
                    }

                    // Exclude row if it doesn't match the global filter.
                    if !matches_global {
                        return false;
                    }
                }

                // Check column-specific filters.
                for (column_id, column_filter) in filter_state.column_filters() {
                    if let Some(column) = columns.iter().find(|c| c.id() == column_id) {
                        if let Some(value) = column.get_value(&row.original) {
                            // Apply the includes string filter.
                            let passes =
                                BuiltInFilter::IncludesString.test_string(&value.as_string(), &column_filter.value);

                            // Exclude row if it doesn't pass the column filter.
                            if !passes {
                                return false;
                            }
                        }
                    }
                }

                true
            })
            .collect();
    }

    /// Applies sorting to the filtered rows.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column definitions.
    /// - `sorting_state`: The sorting state.
    fn apply_sorting(&mut self, columns: &[ColumnDef<T>], sorting_state: &SortingState) {
        // Start with the filtered indices.
        self.sorted_indices = self.filtered_indices.clone();

        // Skip sorting if no sorts are active.
        if !sorting_state.is_sorted() {
            return;
        }

        // Get the active sort definitions.
        let sorts = sorting_state.sorts();
        if sorts.is_empty() {
            return;
        }

        // Sort the indices using multi-column comparison.
        self.sorted_indices.sort_by(|&a, &b| {
            let row_a = &self.rows[a];
            let row_b = &self.rows[b];

            // Apply each sort in priority order.
            for sort in sorts {
                if let Some(column) = columns.iter().find(|c| c.id() == &sort.column_id) {
                    // Get values from both rows.
                    let value_a = column.get_value(&row_a.original);
                    let value_b = column.get_value(&row_b.original);

                    // Compare the values.
                    let ordering = match (value_a, value_b) {
                        (Some(a), Some(b)) => a.compare(&*b),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    };

                    // Apply the sort direction.
                    let ordering = match sort.direction {
                        SortDirection::Asc => ordering,
                        SortDirection::Desc => ordering.reverse(),
                    };

                    // Return if not equal, otherwise continue to next sort.
                    if ordering != std::cmp::Ordering::Equal {
                        return ordering;
                    }
                }
            }

            std::cmp::Ordering::Equal
        });
    }

    /// Applies grouping to create grouped rows.
    ///
    /// # Parameters
    ///
    /// - `columns`: The column definitions.
    /// - `grouping_state`: The grouping state.
    fn apply_grouping(&mut self, columns: &[ColumnDef<T>], grouping_state: &GroupingState) {
        // Skip grouping if not active.
        if !grouping_state.is_grouped() {
            return;
        }

        // Get the grouping columns.
        let group_columns = grouping_state.group_by();
        if group_columns.is_empty() {
            return;
        }

        // Get the first grouping column.
        let group_col_id = &group_columns[0];
        let group_column = match columns.iter().find(|c| c.id() == group_col_id) {
            Some(col) => col,
            None => return,
        };

        // Group rows by the column value.
        let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
        for &idx in &self.sorted_indices {
            let row = &self.rows[idx];
            let group_value = group_column
                .get_value(&row.original)
                .map(|v| v.as_string())
                .unwrap_or_else(|| "(empty)".to_string());
            groups.entry(group_value).or_default().push(idx);
        }

        // Sort group keys for consistent ordering.
        let mut group_keys: Vec<String> = groups.keys().cloned().collect();
        group_keys.sort();

        // Update rows with group information.
        for group_key in &group_keys {
            if let Some(row_indices) = groups.get(group_key) {
                for &idx in row_indices {
                    // Set the group value and depth.
                    self.rows[idx].group_value = Some(group_key.clone());
                    self.rows[idx].depth = 1;

                    // Mark first row of group as expandable.
                    if Some(&idx) == row_indices.first() {
                        self.rows[idx].can_expand = row_indices.len() > 1;
                        self.rows[idx].sub_row_ids =
                            row_indices.iter().skip(1).map(|&i| self.rows[i].id.clone()).collect();
                    }
                }
            }
        }
    }

    /// Applies expansion to show/hide sub-rows.
    ///
    /// # Parameters
    ///
    /// - `expanding_state`: The expanding state.
    fn apply_expansion(&mut self, expanding_state: &ExpandingState) {
        // If expand_all is set, show all rows.
        if expanding_state.is_expand_all() {
            self.expanded_indices = self.sorted_indices.clone();
            return;
        }

        // Check if any rows have parent-child relationships.
        let has_hierarchy = self
            .rows
            .iter()
            .any(|r| r.parent_id.is_some() || !r.sub_row_ids.is_empty());

        if !has_hierarchy {
            // No hierarchy, just copy sorted indices.
            self.expanded_indices = self.sorted_indices.clone();
            return;
        }

        // Build expanded indices based on expansion state.
        self.expanded_indices = Vec::new();

        for &idx in &self.sorted_indices {
            let row = &self.rows[idx];

            // Check if this row should be visible based on parent expansion.
            if let Some(parent_id) = &row.parent_id {
                // Only show if parent is expanded.
                if !expanding_state.is_expanded(parent_id) {
                    continue;
                }
            }

            // Add the row to expanded indices.
            self.expanded_indices.push(idx);
        }
    }

    /// Applies pagination to the expanded rows.
    ///
    /// # Parameters
    ///
    /// - `pagination_state`: The pagination state.
    fn apply_pagination(&mut self, pagination_state: &PaginationState) {
        // Skip pagination if disabled.
        if !pagination_state.is_enabled() {
            self.paginated_indices = self.expanded_indices.clone();
            return;
        }

        // Extract the page range from expanded indices.
        let range = pagination_state.row_range(self.expanded_indices.len());
        self.paginated_indices = self.expanded_indices[range].to_vec();
    }
}
