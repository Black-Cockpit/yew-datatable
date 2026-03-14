/// Pagination mode enumeration.
///
/// Determines whether pagination is handled on the client side
/// or on the server side with per-page fetching.
pub mod pagination_mode;

/// Complete pagination state for the table.
///
/// Manages page index, page size, total rows, and navigation
/// with support for both client-side and server-side pagination.
pub mod pagination_state;

/// Re-exports for convenient access to pagination types.
///
/// Provides a centralized location for importing commonly used
/// pagination-related types and enums.
pub mod prelude;
