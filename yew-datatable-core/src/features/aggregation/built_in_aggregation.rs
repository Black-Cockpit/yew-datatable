//! Built-in aggregation functions for grouped rows.
//!
//! Provides pre-built aggregation strategies for common
//! statistical operations on grouped row data.

use std::cmp::Ordering;

/// Built-in aggregation functions.
///
/// Each variant represents a different aggregation strategy
/// that can be applied to grouped column values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BuiltInAggregation {
    /// Sum of values.
    Sum,

    /// Average of values.
    Mean,

    /// Minimum value.
    Min,

    /// Maximum value.
    Max,

    /// Count of values.
    Count,

    /// Count of unique values.
    UniqueCount,

    /// Median value.
    Median,

    /// First value.
    First,

    /// Last value.
    Last,
}

impl BuiltInAggregation {
    /// Aggregates a list of f64 values.
    ///
    /// # Parameters
    ///
    /// - `values`: The numeric values to aggregate.
    ///
    /// # Returns
    ///
    /// - `Option<f64>`: The aggregated result, or None if the input is empty.
    pub fn aggregate_f64(&self, values: &[f64]) -> Option<f64> {
        // Return None for empty input.
        if values.is_empty() {
            return None;
        }

        // Dispatch to the appropriate aggregation strategy.
        match self {
            Self::Sum => Some(values.iter().sum()),
            Self::Mean => {
                // Calculate the arithmetic mean.
                Some(values.iter().sum::<f64>() / values.len() as f64)
            }
            Self::Min => values.iter().copied().reduce(f64::min),
            Self::Max => values.iter().copied().reduce(f64::max),
            Self::Count => Some(values.len() as f64),
            Self::UniqueCount => {
                // Sort and deduplicate to count unique values.
                let mut unique: Vec<_> = values.to_vec();
                unique.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
                unique.dedup();
                Some(unique.len() as f64)
            }
            Self::Median => {
                // Sort values and find the middle element.
                let mut sorted = values.to_vec();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
                let mid = sorted.len() / 2;

                // Average the two middle elements for even-length arrays.
                if sorted.len() % 2 == 0 {
                    Some((sorted[mid - 1] + sorted[mid]) / 2.0)
                } else {
                    Some(sorted[mid])
                }
            }
            Self::First => values.first().copied(),
            Self::Last => values.last().copied(),
        }
    }

    /// Aggregates a list of string values.
    ///
    /// # Parameters
    ///
    /// - `values`: The string values to aggregate.
    ///
    /// # Returns
    ///
    /// - `Option<String>`: The aggregated result, or None if the input is empty.
    pub fn aggregate_strings(&self, values: &[String]) -> Option<String> {
        // Return None for empty input.
        if values.is_empty() {
            return None;
        }

        // Dispatch to the appropriate aggregation strategy.
        match self {
            Self::Count => Some(values.len().to_string()),
            Self::UniqueCount => {
                // Sort and deduplicate to count unique values.
                let mut unique = values.to_vec();
                unique.sort();
                unique.dedup();
                Some(unique.len().to_string())
            }
            Self::First => values.first().cloned(),
            Self::Last => values.last().cloned(),
            Self::Min => values.iter().min().cloned(),
            Self::Max => values.iter().max().cloned(),
            _ => Some(values.len().to_string()),
        }
    }
}
