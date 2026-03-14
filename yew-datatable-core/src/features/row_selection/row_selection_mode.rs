//! Row selection mode enumeration.
//!
//! Determines whether row selection is disabled, limited to a single row,
//! or allows multiple rows to be selected simultaneously.

/// Row selection mode.
///
/// Controls the selection behavior of the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RowSelectionMode {
    /// No selection allowed.
    None,

    /// Single row selection.
    Single,

    /// Multiple row selection.
    #[default]
    Multi,
}
