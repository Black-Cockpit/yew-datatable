//! Pagination mode enumeration.
//!
//! Determines whether pagination is handled on the client side
//! with all data loaded, or on the server side with per-page fetching.

/// Pagination mode.
///
/// Controls whether data is paginated locally or fetched
/// per page from a server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PaginationMode {
    /// Client-side pagination (all data loaded).
    #[default]
    Client,

    /// Server-side pagination (data fetched per page).
    Server,
}
