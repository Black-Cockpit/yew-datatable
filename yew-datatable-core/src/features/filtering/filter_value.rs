//! Filter value types that can be applied to a column.
//!
//! Represents different kinds of filter criteria including text,
//! numeric, boolean, multi-select, date range, and custom values.

/// Filter value that can be applied to a column.
///
/// Each variant represents a different type of filter criterion
/// that can be applied to column data.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FilterValue {
    /// Text filter value.
    Text(String),

    /// Numeric filter value.
    Number(f64),

    /// Numeric range filter.
    NumberRange {
        /// Minimum value (inclusive).
        min: Option<f64>,
        /// Maximum value (inclusive).
        max: Option<f64>,
    },

    /// Boolean filter value.
    Boolean(bool),

    /// Multiple values (for select filters).
    MultiSelect(Vec<String>),

    /// Date range filter (ISO 8601 strings).
    DateRange {
        /// Start date (inclusive).
        start: Option<String>,
        /// End date (inclusive).
        end: Option<String>,
    },

    /// Custom filter value.
    Custom(String),
}

impl FilterValue {
    /// Returns the value as a string for display.
    ///
    /// # Returns
    ///
    /// - `String`: A human-readable string representation of the filter value.
    pub fn as_display_string(&self) -> String {
        // Format the value based on its variant.
        match self {
            Self::Text(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::NumberRange { min, max } => {
                format!(
                    "{} - {}",
                    min.map(|n| n.to_string()).unwrap_or_default(),
                    max.map(|n| n.to_string()).unwrap_or_default()
                )
            }
            Self::Boolean(b) => b.to_string(),
            Self::MultiSelect(v) => v.join(", "),
            Self::DateRange { start, end } => {
                format!(
                    "{} - {}",
                    start.clone().unwrap_or_default(),
                    end.clone().unwrap_or_default()
                )
            }
            Self::Custom(s) => s.clone(),
        }
    }

    /// Returns true if this filter is empty or inactive.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the filter value represents an inactive state.
    pub fn is_empty(&self) -> bool {
        // Check each variant for its empty condition.
        match self {
            Self::Text(s) => s.is_empty(),
            Self::Number(_) => false,
            Self::NumberRange { min, max } => min.is_none() && max.is_none(),
            Self::Boolean(_) => false,
            Self::MultiSelect(v) => v.is_empty(),
            Self::DateRange { start, end } => start.is_none() && end.is_none(),
            Self::Custom(s) => s.is_empty(),
        }
    }
}

/// Creates a text `FilterValue` from a string slice.
impl From<&str> for FilterValue {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}

/// Creates a text `FilterValue` from an owned `String`.
impl From<String> for FilterValue {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}
