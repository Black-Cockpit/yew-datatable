//! A row in the table with associated metadata.
//!
//! `DataTableRow` wraps the original data type `T` and adds metadata needed
//! for table operations like selection, expansion, and grouping.

use crate::row::data_table_row_id::DataTableRowId;

/// A row in the table with associated metadata.
///
/// Wraps the original data and adds metadata for table operations
/// like selection, expansion, grouping, and nested row support.
#[derive(Debug, Clone)]
pub struct DataTableRow<T> {
    /// Unique identifier for the row.
    pub id: DataTableRowId,

    /// The original row data.
    pub original: T,

    /// Index in the original data array.
    pub original_index: usize,

    /// Index in the current view (after filtering/sorting).
    pub view_index: usize,

    /// Depth level for nested/grouped rows.
    pub depth: usize,

    /// Parent row ID for nested rows.
    pub parent_id: Option<DataTableRowId>,

    /// Child row IDs for expandable rows.
    pub sub_row_ids: Vec<DataTableRowId>,

    /// Whether this is a group row (aggregated).
    pub is_group_row: bool,

    /// Group value for grouped rows.
    pub group_value: Option<String>,

    /// Whether this row can be expanded.
    pub can_expand: bool,

    /// Whether this row can be selected.
    pub can_select: bool,
}

impl<T> DataTableRow<T> {
    /// Creates a new row with the given ID and data.
    ///
    /// # Parameters
    ///
    /// - `id`: The unique row identifier.
    /// - `original`: The original row data.
    /// - `original_index`: The index in the original data array.
    ///
    /// # Returns
    ///
    /// - `DataTableRow<T>`: A new row with default metadata.
    pub fn new(id: impl Into<DataTableRowId>, original: T, original_index: usize) -> Self {
        Self {
            id: id.into(),
            original,
            original_index,
            view_index: original_index,
            depth: 0,
            parent_id: None,
            sub_row_ids: Vec::new(),
            is_group_row: false,
            group_value: None,
            can_expand: false,
            can_select: true,
        }
    }

    /// Creates a row from index (using index as ID).
    ///
    /// # Parameters
    ///
    /// - `original`: The original row data.
    /// - `index`: The row index used as both ID and original index.
    ///
    /// # Returns
    ///
    /// - `DataTableRow<T>`: A new row with index-based ID.
    pub fn from_index(original: T, index: usize) -> Self {
        // Create the row using the index as the ID.
        Self::new(DataTableRowId::from_index(index), original, index)
    }

    /// Sets the view index.
    ///
    /// # Parameters
    ///
    /// - `index`: The new view index.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified row.
    pub fn with_view_index(mut self, index: usize) -> Self {
        // Update the view index.
        self.view_index = index;
        self
    }

    /// Sets the depth level.
    ///
    /// # Parameters
    ///
    /// - `depth`: The nesting depth level.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified row.
    pub fn with_depth(mut self, depth: usize) -> Self {
        // Update the depth level.
        self.depth = depth;
        self
    }

    /// Sets the parent row ID.
    ///
    /// # Parameters
    ///
    /// - `parent_id`: The parent row identifier.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified row.
    pub fn with_parent(mut self, parent_id: DataTableRowId) -> Self {
        // Set the parent row reference.
        self.parent_id = Some(parent_id);
        self
    }

    /// Sets the sub-row IDs.
    ///
    /// # Parameters
    ///
    /// - `sub_row_ids`: The child row identifiers.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified row.
    pub fn with_sub_rows(mut self, sub_row_ids: Vec<DataTableRowId>) -> Self {
        // Set the child row IDs and update expandability.
        self.sub_row_ids = sub_row_ids;
        self.can_expand = !self.sub_row_ids.is_empty();
        self
    }

    /// Marks this as a group row.
    ///
    /// # Parameters
    ///
    /// - `group_value`: The group value label.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified row.
    pub fn as_group_row(mut self, group_value: String) -> Self {
        // Mark as group row and set the value.
        self.is_group_row = true;
        self.group_value = Some(group_value);
        self
    }

    /// Sets whether this row can be selected.
    ///
    /// # Parameters
    ///
    /// - `can_select`: Whether the row is selectable.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified row.
    pub fn with_can_select(mut self, can_select: bool) -> Self {
        // Update the selectable flag.
        self.can_select = can_select;
        self
    }

    /// Returns a reference to the original data.
    ///
    /// # Returns
    ///
    /// - `&T`: A reference to the original row data.
    pub fn data(&self) -> &T {
        &self.original
    }

    /// Returns a mutable reference to the original data.
    ///
    /// # Returns
    ///
    /// - `&mut T`: A mutable reference to the original row data.
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.original
    }

    /// Returns whether this row has sub-rows.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the row has child rows.
    pub fn has_sub_rows(&self) -> bool {
        !self.sub_row_ids.is_empty()
    }

    /// Returns whether this row is a leaf (no sub-rows).
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the row is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.sub_row_ids.is_empty() && !self.is_group_row
    }

    /// Returns whether this row is a root row (no parent).
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the row has no parent.
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Maps the row data to a new type.
    ///
    /// # Parameters
    ///
    /// - `f`: A function to transform the row data.
    ///
    /// # Returns
    ///
    /// - `DataTableRow<U>`: A new row with the transformed data.
    pub fn map<U, F>(self, f: F) -> DataTableRow<U>
    where
        F: FnOnce(T) -> U,
    {
        // Transform the data while preserving metadata.
        DataTableRow {
            id: self.id,
            original: f(self.original),
            original_index: self.original_index,
            view_index: self.view_index,
            depth: self.depth,
            parent_id: self.parent_id,
            sub_row_ids: self.sub_row_ids,
            is_group_row: self.is_group_row,
            group_value: self.group_value,
            can_expand: self.can_expand,
            can_select: self.can_select,
        }
    }
}

impl<T: Clone> DataTableRow<T> {
    /// Clones the row with a new view index.
    ///
    /// # Parameters
    ///
    /// - `view_index`: The new view index.
    ///
    /// # Returns
    ///
    /// - `Self`: A cloned row with the updated view index.
    pub fn clone_with_view_index(&self, view_index: usize) -> Self {
        // Clone the row and update the view index.
        let mut row = self.clone();
        row.view_index = view_index;
        row
    }
}
