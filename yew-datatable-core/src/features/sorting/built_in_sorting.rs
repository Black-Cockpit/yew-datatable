//! Built-in sorting functions for common sorting strategies.
//!
//! Provides pre-built comparison functions for alphanumeric,
//! numeric, date/time, and basic string sorting.

use std::cmp::Ordering;

use crate::features::sorting::natord;

/// Built-in sorting functions.
///
/// Each variant represents a different sorting strategy
/// that can be applied to string values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltInSorting {
    /// Alphabetical string sorting with natural number ordering.
    Alphanumeric,

    /// Case-insensitive alphabetical sorting with natural number ordering.
    AlphanumericCaseSensitive,

    /// Numeric sorting (parses strings as numbers).
    Numeric,

    /// Date/time sorting (ISO 8601 strings).
    DateTime,

    /// Basic lexicographic comparison sorting.
    Basic,
}

impl BuiltInSorting {
    /// Compares two string values using this sorting method.
    ///
    /// # Parameters
    ///
    /// - `a`: The first string value.
    /// - `b`: The second string value.
    ///
    /// # Returns
    ///
    /// - `Ordering`: The comparison result.
    pub fn compare_strings(&self, a: &str, b: &str) -> Ordering {
        // Dispatch to the appropriate comparison strategy.
        match self {
            Self::Alphanumeric => natord::compare(a, b),
            Self::AlphanumericCaseSensitive => natord::compare_ignore_case(a, b),
            Self::Numeric => {
                // Parse both values as floating point numbers.
                let a_num: f64 = a.parse().unwrap_or(f64::NAN);
                let b_num: f64 = b.parse().unwrap_or(f64::NAN);

                // Compare the parsed numeric values.
                a_num.partial_cmp(&b_num).unwrap_or(Ordering::Equal)
            }
            Self::DateTime => a.cmp(b),
            Self::Basic => a.cmp(b),
        }
    }
}
