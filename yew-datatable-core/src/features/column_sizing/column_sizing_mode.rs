//! Column sizing mode enumeration.
//!
//! Determines how columns are sized within the table layout,
//! supporting fixed, fit-content, and flex sizing strategies.

/// Column sizing mode.
///
/// Controls the sizing strategy used for table columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ColumnSizingMode {
    /// Fixed sizing - columns have set widths.
    #[default]
    Fixed,

    /// Fit content - columns size to fit their content.
    FitContent,

    /// Flex sizing - columns share available space.
    Flex,
}
