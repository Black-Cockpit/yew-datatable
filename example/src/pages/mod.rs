/// Sorting feature demonstration page.
///
/// Demonstrates single and multi-column sorting with visual
/// direction indicators and shift-click multi-sort.
pub mod sorting;

/// Filtering feature demonstration page.
///
/// Demonstrates global cross-column search and column-specific
/// text filtering with real-time updates.
pub mod filtering;

/// Pagination feature demonstration page.
///
/// Demonstrates page navigation, page size selection, and
/// row range information display.
pub mod pagination;

/// Row selection feature demonstration page.
///
/// Demonstrates single and multi-row selection with toggle,
/// select-all, and clear operations.
pub mod selection;

/// Column features demonstration page.
///
/// Demonstrates column visibility toggling and column ordering
/// operations.
pub mod column_features;

/// Full-featured demonstration page.
///
/// Combines sorting, filtering, pagination, selection, and column
/// features into a single comprehensive demo.
pub mod full_demo;

/// Async data synchronization demonstration page.
///
/// Demonstrates server-like delayed updates where parent rows and
/// table rows stay synchronized.
pub mod async_data;
