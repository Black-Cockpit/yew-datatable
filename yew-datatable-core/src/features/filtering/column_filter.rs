//! Filter state for a single column.
//!
//! Combines a column identifier with a filter value to represent
//! the active filter applied to that column.

use crate::column::column_id::ColumnId;
use crate::features::filtering::filter_value::FilterValue;

/// Filter state for a single column.
///
/// Tracks the column being filtered and the filter value applied to it.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColumnFilter {
    /// The column being filtered.
    pub column_id: ColumnId,

    /// The filter value.
    pub value: FilterValue,
}

impl ColumnFilter {
    /// Creates a new column filter.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `value`: The filter value to apply.
    ///
    /// # Returns
    ///
    /// - `ColumnFilter`: A new column filter instance.
    pub fn new(column_id: impl Into<ColumnId>, value: FilterValue) -> Self {
        Self {
            column_id: column_id.into(),
            value,
        }
    }

    /// Creates a text filter.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `value`: The text value to filter by.
    ///
    /// # Returns
    ///
    /// - `ColumnFilter`: A new text column filter.
    pub fn text(column_id: impl Into<ColumnId>, value: impl Into<String>) -> Self {
        // Create a text filter value and wrap it.
        Self::new(column_id, FilterValue::Text(value.into()))
    }

    /// Creates a number filter.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `value`: The numeric value to filter by.
    ///
    /// # Returns
    ///
    /// - `ColumnFilter`: A new numeric column filter.
    pub fn number(column_id: impl Into<ColumnId>, value: f64) -> Self {
        // Create a number filter value and wrap it.
        Self::new(column_id, FilterValue::Number(value))
    }

    /// Creates a number range filter.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `min`: The minimum value (inclusive), or None for unbounded.
    /// - `max`: The maximum value (inclusive), or None for unbounded.
    ///
    /// # Returns
    ///
    /// - `ColumnFilter`: A new numeric range column filter.
    pub fn number_range(column_id: impl Into<ColumnId>, min: Option<f64>, max: Option<f64>) -> Self {
        // Create a number range filter value and wrap it.
        Self::new(column_id, FilterValue::NumberRange { min, max })
    }

    /// Creates a boolean filter.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `value`: The boolean value to filter by.
    ///
    /// # Returns
    ///
    /// - `ColumnFilter`: A new boolean column filter.
    pub fn boolean(column_id: impl Into<ColumnId>, value: bool) -> Self {
        // Create a boolean filter value and wrap it.
        Self::new(column_id, FilterValue::Boolean(value))
    }

    /// Creates a multi-select filter.
    ///
    /// # Parameters
    ///
    /// - `column_id`: The column identifier.
    /// - `values`: The list of values to filter by.
    ///
    /// # Returns
    ///
    /// - `ColumnFilter`: A new multi-select column filter.
    pub fn multi_select(column_id: impl Into<ColumnId>, values: Vec<String>) -> Self {
        // Create a multi-select filter value and wrap it.
        Self::new(column_id, FilterValue::MultiSelect(values))
    }
}
