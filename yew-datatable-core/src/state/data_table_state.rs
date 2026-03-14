//! Combined state for all table features.
//!
//! Aggregates all feature states into a single structure
//! for easier management and passing around.

use crate::features::aggregation::aggregation_state::AggregationState;
use crate::features::column_ordering::column_ordering_state::ColumnOrderingState;
use crate::features::column_pinning::column_pinning_state::ColumnPinningState;
use crate::features::column_sizing::column_sizing_state::ColumnSizingState;
use crate::features::column_visibility::column_visibility_state::ColumnVisibilityState;
use crate::features::expanding::expanding_state::ExpandingState;
use crate::features::filtering::filter_state::FilterState;
use crate::features::grouping::grouping_state::GroupingState;
use crate::features::pagination::pagination_state::PaginationState;
use crate::features::row_selection::row_selection_state::RowSelectionState;
use crate::features::sorting::sorting_state::SortingState;

/// Combined state for all table features.
///
/// `DataTableState` aggregates all feature states into a single structure
/// for easier management and passing around.
#[derive(Debug, Clone, Default)]
pub struct DataTableState {
    /// Sorting state.
    pub sorting: SortingState,

    /// Filtering state.
    pub filtering: FilterState,

    /// Pagination state.
    pub pagination: PaginationState,

    /// Row selection state.
    pub row_selection: RowSelectionState,

    /// Row expansion state.
    pub expanding: ExpandingState,

    /// Column visibility state.
    pub column_visibility: ColumnVisibilityState,

    /// Column ordering state.
    pub column_ordering: ColumnOrderingState,

    /// Column pinning state.
    pub column_pinning: ColumnPinningState,

    /// Column sizing state.
    pub column_sizing: ColumnSizingState,

    /// Row grouping state.
    pub grouping: GroupingState,

    /// Aggregation state.
    pub aggregation: AggregationState,
}

impl DataTableState {
    /// Creates a new default table state.
    ///
    /// # Returns
    ///
    /// - `DataTableState`: A new state with all defaults applied.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a table state builder.
    ///
    /// # Returns
    ///
    /// - `DataTableStateBuilder`: A new state builder instance.
    pub fn builder() -> super::data_table_state_builder::DataTableStateBuilder {
        super::data_table_state_builder::DataTableStateBuilder::new()
    }

    /// Resets all state to defaults.
    pub fn reset_all(&mut self) {
        // Reset each feature state individually.
        self.sorting.reset();
        self.filtering.reset();
        self.pagination.reset();
        self.row_selection.reset();
        self.expanding.reset();
        self.column_visibility.reset();
        self.column_ordering.reset();
        self.column_pinning.reset();
        self.column_sizing.reset();
        self.grouping.reset();
        self.aggregation.reset();
    }

    /// Checks if any state has been modified from defaults.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether any feature state differs from its default.
    pub fn has_modifications(&self) -> bool {
        // Check each feature state for modifications.
        self.sorting.is_sorted()
            || self.filtering.is_filtered()
            || self.row_selection.has_selection()
            || self.expanding.has_expanded()
            || self.grouping.is_grouped()
    }
}
