//! Complete pagination state for the table.
//!
//! Manages page index, page size, total rows, and navigation
//! with support for both client-side and server-side pagination.

use crate::features::pagination::pagination_mode::PaginationMode;

/// Complete pagination state for the table.
///
/// Tracks the current page, page size, available options,
/// and provides navigation methods.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PaginationState {
    /// Current page index (0-based).
    page_index: usize,

    /// Number of rows per page.
    page_size: usize,

    /// Total number of rows (for server-side pagination).
    total_rows: Option<usize>,

    /// Available page size options.
    page_size_options: Vec<usize>,

    /// Pagination mode.
    mode: PaginationMode,

    /// Whether pagination is enabled.
    enabled: bool,
}

impl PaginationState {
    /// Default page size.
    pub const DEFAULT_PAGE_SIZE: usize = 10;

    /// Default page size options.
    pub const DEFAULT_PAGE_SIZE_OPTIONS: &'static [usize] = &[10, 20, 30, 50, 100];

    /// Creates a new pagination state with default settings.
    ///
    /// # Returns
    ///
    /// - `PaginationState`: A new pagination state with page size 10 and enabled.
    pub fn new() -> Self {
        Self {
            page_index: 0,
            page_size: Self::DEFAULT_PAGE_SIZE,
            total_rows: None,
            page_size_options: Self::DEFAULT_PAGE_SIZE_OPTIONS.to_vec(),
            mode: PaginationMode::Client,
            enabled: true,
        }
    }

    /// Creates a disabled pagination state.
    ///
    /// # Returns
    ///
    /// - `PaginationState`: A new disabled pagination state.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::new()
        }
    }

    /// Sets the page size.
    ///
    /// # Parameters
    ///
    /// - `size`: The number of rows per page (minimum 1).
    ///
    /// # Returns
    ///
    /// - `Self`: The modified pagination state.
    pub fn with_page_size(mut self, size: usize) -> Self {
        // Enforce minimum page size of 1.
        self.page_size = size.max(1);
        self
    }

    /// Sets the page size options.
    ///
    /// # Parameters
    ///
    /// - `options`: The available page size options.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified pagination state.
    pub fn with_page_size_options(mut self, options: Vec<usize>) -> Self {
        // Set the page size options.
        self.page_size_options = options;
        self
    }

    /// Sets the pagination mode.
    ///
    /// # Parameters
    ///
    /// - `mode`: The pagination mode (client or server).
    ///
    /// # Returns
    ///
    /// - `Self`: The modified pagination state.
    pub fn with_mode(mut self, mode: PaginationMode) -> Self {
        // Set the pagination mode.
        self.mode = mode;
        self
    }

    /// Sets the total row count (for server-side pagination).
    ///
    /// # Parameters
    ///
    /// - `total`: The total number of rows on the server.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified pagination state.
    pub fn with_total_rows(mut self, total: usize) -> Self {
        // Set the total row count.
        self.total_rows = Some(total);
        self
    }

    /// Returns whether pagination is enabled.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether pagination is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the current page index (0-based).
    ///
    /// # Returns
    ///
    /// - `usize`: The zero-based page index.
    pub fn page_index(&self) -> usize {
        self.page_index
    }

    /// Returns the current page number (1-based).
    ///
    /// # Returns
    ///
    /// - `usize`: The one-based page number.
    pub fn page_number(&self) -> usize {
        // Convert zero-based index to one-based number.
        self.page_index + 1
    }

    /// Returns the page size.
    ///
    /// # Returns
    ///
    /// - `usize`: The number of rows per page.
    pub fn page_size(&self) -> usize {
        self.page_size
    }

    /// Returns the pagination mode.
    ///
    /// # Returns
    ///
    /// - `PaginationMode`: The current pagination mode.
    pub fn mode(&self) -> PaginationMode {
        self.mode
    }

    /// Returns the page size options.
    ///
    /// # Returns
    ///
    /// - `&[usize]`: A slice of available page size options.
    pub fn page_size_options(&self) -> &[usize] {
        &self.page_size_options
    }

    /// Calculates the total number of pages.
    ///
    /// # Parameters
    ///
    /// - `row_count`: The number of rows to paginate.
    ///
    /// # Returns
    ///
    /// - `usize`: The total number of pages (minimum 1).
    pub fn page_count(&self, row_count: usize) -> usize {
        // Use server-provided total or the given row count.
        let total = self.total_rows.unwrap_or(row_count);

        // Calculate page count with ceiling division.
        if total == 0 || self.page_size == 0 {
            1
        } else {
            total.div_ceil(self.page_size)
        }
    }

    /// Returns the start index for the current page.
    ///
    /// # Returns
    ///
    /// - `usize`: The zero-based start row index.
    pub fn start_index(&self) -> usize {
        // Calculate start from page index and size.
        self.page_index * self.page_size
    }

    /// Returns the end index for the current page (exclusive).
    ///
    /// # Parameters
    ///
    /// - `row_count`: The total number of rows.
    ///
    /// # Returns
    ///
    /// - `usize`: The exclusive end row index.
    pub fn end_index(&self, row_count: usize) -> usize {
        // Use server-provided total or the given row count.
        let total = self.total_rows.unwrap_or(row_count);

        // Clamp to the total row count.
        ((self.page_index + 1) * self.page_size).min(total)
    }

    /// Returns the range of row indices for the current page.
    ///
    /// # Parameters
    ///
    /// - `row_count`: The total number of rows.
    ///
    /// # Returns
    ///
    /// - `Range<usize>`: The range of row indices.
    pub fn row_range(&self, row_count: usize) -> std::ops::Range<usize> {
        // Build the range from start to end indices.
        self.start_index()..self.end_index(row_count)
    }

    /// Returns whether there is a previous page.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether navigation to a previous page is possible.
    pub fn can_go_previous(&self) -> bool {
        self.page_index > 0
    }

    /// Returns whether there is a next page.
    ///
    /// # Parameters
    ///
    /// - `row_count`: The total number of rows.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether navigation to a next page is possible.
    pub fn can_go_next(&self, row_count: usize) -> bool {
        // Check if the current page is before the last page.
        self.page_index < self.page_count(row_count).saturating_sub(1)
    }

    /// Goes to the first page.
    pub fn go_to_first(&mut self) {
        // Reset page index to zero.
        self.page_index = 0;
    }

    /// Goes to the last page.
    ///
    /// # Parameters
    ///
    /// - `row_count`: The total number of rows.
    pub fn go_to_last(&mut self, row_count: usize) {
        // Set page index to the last page.
        self.page_index = self.page_count(row_count).saturating_sub(1);
    }

    /// Goes to the previous page.
    pub fn go_to_previous(&mut self) {
        // Decrement page index if not on the first page.
        if self.can_go_previous() {
            self.page_index -= 1;
        }
    }

    /// Goes to the next page.
    ///
    /// # Parameters
    ///
    /// - `row_count`: The total number of rows.
    pub fn go_to_next(&mut self, row_count: usize) {
        // Increment page index if not on the last page.
        if self.can_go_next(row_count) {
            self.page_index += 1;
        }
    }

    /// Goes to a specific page (0-based index).
    ///
    /// # Parameters
    ///
    /// - `index`: The zero-based page index to navigate to.
    /// - `row_count`: The total number of rows.
    pub fn go_to_page(&mut self, index: usize, row_count: usize) {
        // Clamp the page index to the valid range.
        let max_page = self.page_count(row_count).saturating_sub(1);
        self.page_index = index.min(max_page);
    }

    /// Sets the page size and adjusts the current page if needed.
    ///
    /// # Parameters
    ///
    /// - `size`: The new page size.
    /// - `row_count`: The total number of rows.
    pub fn set_page_size(&mut self, size: usize, row_count: usize) {
        // Preserve the approximate scroll position.
        let old_first_row = self.start_index();

        // Update page size with minimum of 1.
        self.page_size = size.max(1);

        // Calculate new page index to preserve position.
        self.page_index = old_first_row / self.page_size;

        // Clamp to valid range.
        let max_page = self.page_count(row_count).saturating_sub(1);
        self.page_index = self.page_index.min(max_page);
    }

    /// Sets the total row count (for server-side pagination).
    ///
    /// # Parameters
    ///
    /// - `total`: The total number of rows on the server.
    pub fn set_total_rows(&mut self, total: usize) {
        // Update total rows and clamp page index.
        self.total_rows = Some(total);
        let max_page = self.page_count(total).saturating_sub(1);
        self.page_index = self.page_index.min(max_page);
    }

    /// Enables pagination.
    pub fn enable(&mut self) {
        // Set the enabled flag.
        self.enabled = true;
    }

    /// Disables pagination.
    pub fn disable(&mut self) {
        // Clear the enabled flag.
        self.enabled = false;
    }

    /// Resets to initial state.
    pub fn reset(&mut self) {
        // Reset page index to the first page.
        self.page_index = 0;
    }
}

/// Provides a default pagination state with standard settings.
impl Default for PaginationState {
    fn default() -> Self {
        Self::new()
    }
}
