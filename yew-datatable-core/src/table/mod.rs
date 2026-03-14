/// Main table instance that coordinates all table functionality.
///
/// Manages columns, data, and state, providing methods for sorting,
/// filtering, pagination, selection, and all other table operations.
pub mod data_table;

/// Options for table configuration.
///
/// Controls which features are enabled or disabled in the table,
/// including sorting, filtering, pagination, selection, and more.
pub mod data_table_options;

/// Builder for table options with a fluent API.
///
/// Provides a convenient way to construct `DataTableOptions` instances
/// with selective feature enablement.
pub mod data_table_options_builder;

/// Re-exports for convenient access to table types.
///
/// Provides a centralized location for importing commonly used
/// table-related types and structs.
pub mod prelude;
