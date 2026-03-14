//! Complete aggregation state for the table.
//!
//! Manages column-level aggregation function assignments
//! and provides configuration for aggregation behavior.

use std::collections::HashMap;

use crate::column::column_id::ColumnId;
use crate::features::aggregation::built_in_aggregation::BuiltInAggregation;

/// Complete aggregation state for the table.
///
/// Tracks which columns have aggregation functions assigned
/// and whether aggregation processing is enabled.
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AggregationState {
    /// Map of column ID to aggregation function.
    column_aggregations: HashMap<ColumnId, BuiltInAggregation>,

    /// Whether aggregation is enabled.
    enabled: bool,
}

impl AggregationState {
    /// Creates a new empty aggregation state.
    ///
    /// # Returns
    ///
    /// - `AggregationState`: A new aggregation state with no functions assigned.
    pub fn new() -> Self {
        Self {
            column_aggregations: HashMap::new(),
            enabled: true,
        }
    }

    /// Sets whether aggregation is enabled.
    ///
    /// # Parameters
    ///
    /// - `enabled`: Whether aggregation processing is active.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified aggregation state.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        // Update the enabled flag.
        self.enabled = enabled;
        self
    }

    /// Returns whether aggregation is enabled.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether aggregation processing is active.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the aggregation function for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to look up.
    ///
    /// # Returns
    ///
    /// - `Option<BuiltInAggregation>`: The assigned aggregation function.
    pub fn get_aggregation(&self, column_id: &ColumnId) -> Option<BuiltInAggregation> {
        // Look up the column in the aggregation map.
        self.column_aggregations.get(column_id).copied()
    }

    /// Sets the aggregation function for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `function`: The aggregation function to assign.
    pub fn set_aggregation(&mut self, column_id: ColumnId, function: BuiltInAggregation) {
        // Insert or update the aggregation function.
        self.column_aggregations.insert(column_id, function);
    }

    /// Removes the aggregation function for a column.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier to remove.
    pub fn remove_aggregation(&mut self, column_id: &ColumnId) {
        // Remove the column from the aggregation map.
        self.column_aggregations.remove(column_id);
    }

    /// Clears all aggregation settings.
    pub fn clear(&mut self) {
        // Remove all aggregation assignments.
        self.column_aggregations.clear();
    }

    /// Resets to initial state.
    pub fn reset(&mut self) {
        // Clear all aggregation assignments.
        self.column_aggregations.clear();
    }
}
