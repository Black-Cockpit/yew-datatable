//! Builder for creating table state with specific initial values.
//!
//! Provides a fluent API for constructing `DataTableState` instances
//! with pre-configured feature states.

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
use crate::state::data_table_state::DataTableState;

/// Builder for creating table state with specific initial values.
///
/// Allows step-by-step configuration of each feature state
/// before building the final `DataTableState` instance.
#[derive(Debug, Default)]
pub struct DataTableStateBuilder {
    /// The state being constructed.
    state: DataTableState,
}

impl DataTableStateBuilder {
    /// Creates a new builder.
    ///
    /// # Returns
    ///
    /// - `DataTableStateBuilder`: A new builder with default state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the sorting state.
    ///
    /// # Parameters
    ///
    /// - `sorting`: The initial sorting state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn sorting(mut self, sorting: SortingState) -> Self {
        self.state.sorting = sorting;
        self
    }

    /// Sets the filtering state.
    ///
    /// # Parameters
    ///
    /// - `filtering`: The initial filtering state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn filtering(mut self, filtering: FilterState) -> Self {
        self.state.filtering = filtering;
        self
    }

    /// Sets the pagination state.
    ///
    /// # Parameters
    ///
    /// - `pagination`: The initial pagination state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn pagination(mut self, pagination: PaginationState) -> Self {
        self.state.pagination = pagination;
        self
    }

    /// Sets the row selection state.
    ///
    /// # Parameters
    ///
    /// - `row_selection`: The initial row selection state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn row_selection(mut self, row_selection: RowSelectionState) -> Self {
        self.state.row_selection = row_selection;
        self
    }

    /// Sets the expanding state.
    ///
    /// # Parameters
    ///
    /// - `expanding`: The initial expanding state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn expanding(mut self, expanding: ExpandingState) -> Self {
        self.state.expanding = expanding;
        self
    }

    /// Sets the column visibility state.
    ///
    /// # Parameters
    ///
    /// - `visibility`: The initial column visibility state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn column_visibility(mut self, visibility: ColumnVisibilityState) -> Self {
        self.state.column_visibility = visibility;
        self
    }

    /// Sets the column ordering state.
    ///
    /// # Parameters
    ///
    /// - `ordering`: The initial column ordering state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn column_ordering(mut self, ordering: ColumnOrderingState) -> Self {
        self.state.column_ordering = ordering;
        self
    }

    /// Sets the column pinning state.
    ///
    /// # Parameters
    ///
    /// - `pinning`: The initial column pinning state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn column_pinning(mut self, pinning: ColumnPinningState) -> Self {
        self.state.column_pinning = pinning;
        self
    }

    /// Sets the column sizing state.
    ///
    /// # Parameters
    ///
    /// - `sizing`: The initial column sizing state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn column_sizing(mut self, sizing: ColumnSizingState) -> Self {
        self.state.column_sizing = sizing;
        self
    }

    /// Sets the grouping state.
    ///
    /// # Parameters
    ///
    /// - `grouping`: The initial grouping state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn grouping(mut self, grouping: GroupingState) -> Self {
        self.state.grouping = grouping;
        self
    }

    /// Sets the aggregation state.
    ///
    /// # Parameters
    ///
    /// - `aggregation`: The initial aggregation state.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified builder.
    pub fn aggregation(mut self, aggregation: AggregationState) -> Self {
        self.state.aggregation = aggregation;
        self
    }

    /// Builds the table state.
    ///
    /// # Returns
    ///
    /// - `DataTableState`: The constructed table state.
    pub fn build(self) -> DataTableState {
        // Return the constructed state.
        self.state
    }
}
