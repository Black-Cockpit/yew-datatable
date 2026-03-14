//! Metadata for a table column that affects its behavior and display.
//!
//! Contains configuration such as header text, footer text, sortable/filterable
//! flags, resize constraints, and group column settings.

use crate::column::column_id::ColumnId;

/// Metadata for a column that affects its behavior.
///
/// Controls column display properties, interaction capabilities,
/// and sizing constraints.
#[derive(Debug, Clone)]
pub struct ColumnMeta {
    /// The column ID.
    pub id: ColumnId,

    /// Display header text.
    pub header: String,

    /// Footer text.
    pub footer: Option<String>,

    /// Whether the column is sortable.
    pub sortable: bool,

    /// Whether the column is filterable.
    pub filterable: bool,

    /// Whether the column is resizable.
    pub resizable: bool,

    /// Whether the column is visible by default.
    pub visible: bool,

    /// Minimum column width in pixels.
    pub min_width: Option<f64>,

    /// Maximum column width in pixels.
    pub max_width: Option<f64>,

    /// Default column width in pixels.
    pub default_width: Option<f64>,

    /// Whether this is a group column (contains sub-columns).
    pub is_group: bool,

    /// Parent column ID for nested columns.
    pub parent_id: Option<ColumnId>,

    /// Placeholder text for column filter input.
    pub filter_placeholder: Option<String>,
}

impl ColumnMeta {
    /// Creates new column metadata with the given ID and header.
    ///
    /// # Parameters
    ///
    /// - `id`: The column identifier.
    /// - `header`: The display header text.
    ///
    /// # Returns
    ///
    /// - `ColumnMeta`: A new column metadata with default settings.
    pub fn new(id: impl Into<ColumnId>, header: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            header: header.into(),
            footer: None,
            sortable: true,
            filterable: true,
            resizable: true,
            visible: true,
            min_width: None,
            max_width: None,
            default_width: None,
            is_group: false,
            parent_id: None,
            filter_placeholder: None,
        }
    }

    /// Sets the footer text.
    ///
    /// # Parameters
    ///
    /// - `footer`: The footer text to display.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn with_footer(mut self, footer: impl Into<String>) -> Self {
        // Set the footer text.
        self.footer = Some(footer.into());
        self
    }

    /// Sets whether the column is sortable.
    ///
    /// # Parameters
    ///
    /// - `sortable`: Whether sorting is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn with_sortable(mut self, sortable: bool) -> Self {
        // Update the sortable flag.
        self.sortable = sortable;
        self
    }

    /// Sets whether the column is filterable.
    ///
    /// # Parameters
    ///
    /// - `filterable`: Whether filtering is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn with_filterable(mut self, filterable: bool) -> Self {
        // Update the filterable flag.
        self.filterable = filterable;
        self
    }

    /// Sets whether the column is resizable.
    ///
    /// # Parameters
    ///
    /// - `resizable`: Whether resizing is enabled.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        // Update the resizable flag.
        self.resizable = resizable;
        self
    }

    /// Sets whether the column is visible by default.
    ///
    /// # Parameters
    ///
    /// - `visible`: Whether the column is visible.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn with_visible(mut self, visible: bool) -> Self {
        // Update the visible flag.
        self.visible = visible;
        self
    }

    /// Sets the minimum column width.
    ///
    /// # Parameters
    ///
    /// - `width`: The minimum width in pixels.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn with_min_width(mut self, width: f64) -> Self {
        // Set the minimum width constraint.
        self.min_width = Some(width);
        self
    }

    /// Sets the maximum column width.
    ///
    /// # Parameters
    ///
    /// - `width`: The maximum width in pixels.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn with_max_width(mut self, width: f64) -> Self {
        // Set the maximum width constraint.
        self.max_width = Some(width);
        self
    }

    /// Sets the default column width.
    ///
    /// # Parameters
    ///
    /// - `width`: The default width in pixels.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn with_default_width(mut self, width: f64) -> Self {
        // Set the default width.
        self.default_width = Some(width);
        self
    }

    /// Sets the filter placeholder text.
    ///
    /// # Parameters
    ///
    /// - `placeholder`: The placeholder text for the filter input.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn with_filter_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        // Set the filter placeholder text.
        self.filter_placeholder = Some(placeholder.into());
        self
    }

    /// Marks this column as a group column.
    ///
    /// Group columns contain sub-columns and are not sortable or filterable.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn as_group(mut self) -> Self {
        // Mark as group and disable sorting and filtering.
        self.is_group = true;
        self.sortable = false;
        self.filterable = false;
        self
    }

    /// Sets the parent column ID.
    ///
    /// # Parameters
    ///
    /// - `parent_id`: The parent column identifier.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column metadata.
    pub fn with_parent(mut self, parent_id: impl Into<ColumnId>) -> Self {
        // Set the parent column reference.
        self.parent_id = Some(parent_id.into());
        self
    }
}

/// Provides a default column metadata with empty ID and header.
impl Default for ColumnMeta {
    fn default() -> Self {
        Self::new("", "")
    }
}
