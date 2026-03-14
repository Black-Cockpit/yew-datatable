//! Built-in filter functions for common filtering strategies.
//!
//! Provides pre-built filter functions for string matching,
//! numeric comparisons, and set membership operations.

use crate::features::filtering::filter_value::FilterValue;

/// Built-in filter functions.
///
/// Each variant represents a different filtering strategy
/// that can be applied to column values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltInFilter {
    /// Case-insensitive substring match.
    IncludesString,

    /// Case-sensitive substring match.
    IncludesStringSensitive,

    /// Exact match.
    Equals,

    /// Case-sensitive exact match.
    EqualsSensitive,

    /// Starts with.
    StartsWith,

    /// Ends with.
    EndsWith,

    /// Not equal.
    NotEquals,

    /// Greater than (numeric).
    GreaterThan,

    /// Greater than or equal (numeric).
    GreaterThanOrEqual,

    /// Less than (numeric).
    LessThan,

    /// Less than or equal (numeric).
    LessThanOrEqual,

    /// Between (numeric range).
    Between,

    /// In array.
    InArray,

    /// Not in array.
    NotInArray,

    /// Is empty/null.
    IsEmpty,

    /// Is not empty/null.
    IsNotEmpty,
}

impl BuiltInFilter {
    /// Tests if a string value passes this filter.
    ///
    /// # Parameters
    ///
    /// - `value`: The string value to test.
    /// - `filter`: The filter value to test against.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the string value passes the filter.
    pub fn test_string(&self, value: &str, filter: &FilterValue) -> bool {
        // Dispatch to the appropriate filter strategy.
        match (self, filter) {
            (Self::IncludesString, FilterValue::Text(f)) => {
                // Perform case-insensitive substring match.
                value.to_lowercase().contains(&f.to_lowercase())
            }
            (Self::IncludesStringSensitive, FilterValue::Text(f)) => {
                // Perform case-sensitive substring match.
                value.contains(f.as_str())
            }
            (Self::Equals, FilterValue::Text(f)) => {
                // Perform case-insensitive exact match.
                value.eq_ignore_ascii_case(f)
            }
            (Self::EqualsSensitive, FilterValue::Text(f)) => {
                // Perform case-sensitive exact match.
                value == f
            }
            (Self::StartsWith, FilterValue::Text(f)) => {
                // Check if value starts with the filter text (case-insensitive).
                value.to_lowercase().starts_with(&f.to_lowercase())
            }
            (Self::EndsWith, FilterValue::Text(f)) => {
                // Check if value ends with the filter text (case-insensitive).
                value.to_lowercase().ends_with(&f.to_lowercase())
            }
            (Self::NotEquals, FilterValue::Text(f)) => {
                // Check for inequality (case-insensitive).
                !value.eq_ignore_ascii_case(f)
            }
            (Self::InArray, FilterValue::MultiSelect(arr)) => {
                // Check if value is in the provided list (case-insensitive).
                arr.iter().any(|a| a.eq_ignore_ascii_case(value))
            }
            (Self::NotInArray, FilterValue::MultiSelect(arr)) => {
                // Check if value is not in the provided list (case-insensitive).
                !arr.iter().any(|a| a.eq_ignore_ascii_case(value))
            }
            (Self::IsEmpty, _) => {
                // Check if value is empty.
                value.is_empty()
            }
            (Self::IsNotEmpty, _) => {
                // Check if value is not empty.
                !value.is_empty()
            }
            _ => true,
        }
    }

    /// Tests if a numeric value passes this filter.
    ///
    /// # Parameters
    ///
    /// - `value`: The numeric value to test.
    /// - `filter`: The filter value to test against.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the numeric value passes the filter.
    pub fn test_number(&self, value: f64, filter: &FilterValue) -> bool {
        // Dispatch to the appropriate numeric filter strategy.
        match (self, filter) {
            (Self::Equals, FilterValue::Number(f)) => {
                // Check for numeric equality within epsilon.
                (value - f).abs() < f64::EPSILON
            }
            (Self::NotEquals, FilterValue::Number(f)) => {
                // Check for numeric inequality.
                (value - f).abs() >= f64::EPSILON
            }
            (Self::GreaterThan, FilterValue::Number(f)) => {
                // Check if value is greater than the filter.
                value > *f
            }
            (Self::GreaterThanOrEqual, FilterValue::Number(f)) => {
                // Check if value is greater than or equal to the filter.
                value >= *f
            }
            (Self::LessThan, FilterValue::Number(f)) => {
                // Check if value is less than the filter.
                value < *f
            }
            (Self::LessThanOrEqual, FilterValue::Number(f)) => {
                // Check if value is less than or equal to the filter.
                value <= *f
            }
            (Self::Between, FilterValue::NumberRange { min, max }) => {
                // Check if value is within the range.
                let above_min = min.is_none_or(|m| value >= m);
                let below_max = max.is_none_or(|m| value <= m);
                above_min && below_max
            }
            _ => true,
        }
    }
}
