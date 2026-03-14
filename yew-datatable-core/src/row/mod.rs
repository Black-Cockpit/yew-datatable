/// Row wrapper type with metadata.
///
/// Wraps the original data and adds metadata for table operations
/// like selection, expansion, grouping, and nested row support.
pub mod data_table_row;

/// Unique identifier for a table row.
///
/// Row IDs are used to reference rows throughout the table API
/// for operations like selection and expansion.
pub mod data_table_row_id;

/// Row model that manages data processing through the table pipeline.
///
/// Handles filtering, sorting, grouping, expansion, and pagination
/// of row data, maintaining index mappings at each processing stage.
pub mod data_table_row_model;

/// Re-exports for convenient access to row types.
///
/// Provides a centralized location for importing commonly used
/// row-related types and structs.
pub mod prelude;
