/// Context provided when rendering a table cell.
///
/// Contains all information needed to render a cell value,
/// including the row data, column information, and table state.
pub mod cell_context;

/// Definition of a table column.
///
/// Provides all configuration needed to display and interact
/// with a column, including accessor, sorting, filtering, and aggregation.
pub mod column_def;

/// Builder pattern for creating column definitions with a fluent API.
///
/// Provides a convenient way to construct `ColumnDef` instances
/// with various configuration options.
pub mod column_def_builder;

/// Unique identifier for a table column.
///
/// Column IDs are used to reference columns throughout the table API
/// for operations like sorting, filtering, visibility, and pinning.
pub mod column_id;

/// Metadata for a table column that affects its behavior and display.
///
/// Contains configuration such as header text, footer text, sortable/filterable
/// flags, resize constraints, and group column settings.
pub mod column_meta;

/// Type-safe accessor for extracting values from row data.
///
/// Provides compile-time guarantees that columns access the correct
/// fields from row data without string-based property access.
pub mod data_table_accessor;

/// Dynamic accessor that returns boxed trait objects for sorting and filtering.
///
/// Erases the concrete value type, allowing columns with different
/// value types to be stored and processed uniformly.
pub mod data_table_dyn_accessor;

/// Trait for dynamically typed values that support comparison and display.
///
/// Provides a common interface for column values of different types,
/// enabling sorting, filtering, and display operations.
pub mod data_table_dyn_value;

/// Context provided when rendering a column header.
///
/// Contains sort state, filter capability, resize state,
/// pinning status, and column width information.
pub mod header_context;

/// Sort direction indicator for header context rendering.
///
/// Represents the visual sort direction state displayed in column
/// headers when sorting is active.
pub mod sort_direction;

/// Re-exports for convenient access to column types.
///
/// Provides a centralized location for importing commonly used
/// column-related types, traits, and structs.
pub mod prelude;
