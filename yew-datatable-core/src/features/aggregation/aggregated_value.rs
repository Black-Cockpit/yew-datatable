//! Aggregated value for a column in grouped rows.
//!
//! Represents the result of an aggregation operation on a column,
//! storing the computed value and the function used to compute it.

use crate::features::aggregation::built_in_aggregation::BuiltInAggregation;

/// Aggregated value for a column.
///
/// Stores the string representation of an aggregated result
/// along with the optional aggregation function used.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AggregatedValue {
    /// The aggregated value as a string.
    pub value: String,

    /// The aggregation function used.
    pub function: Option<BuiltInAggregation>,
}

impl AggregatedValue {
    /// Creates a new aggregated value.
    ///
    /// # Parameters
    ///
    /// - `value`: The aggregated value string.
    ///
    /// # Returns
    ///
    /// - `AggregatedValue`: A new aggregated value without a function reference.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            function: None,
        }
    }

    /// Creates an aggregated value with a function reference.
    ///
    /// # Parameters
    ///
    /// - `value`: The aggregated value string.
    /// - `function`: The aggregation function that produced this value.
    ///
    /// # Returns
    ///
    /// - `AggregatedValue`: A new aggregated value with a function reference.
    pub fn with_function(value: impl Into<String>, function: BuiltInAggregation) -> Self {
        Self {
            value: value.into(),
            function: Some(function),
        }
    }
}
