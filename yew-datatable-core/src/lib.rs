//! # yew-datatable-core
//!
//! A headless data table engine for Rust/WASM applications.
//! Inspired by TanStack Table v8, providing feature parity with idiomatic Rust design.
//!
//! ## Features
//!
//! - **Headless**: Pure logic, no UI assumptions
//! - **Type-safe**: Compile-time guarantees for column correctness
//! - **Zero unsafe**: 100% safe Rust
//! - **WASM-optimized**: Efficient performance for web applications
//!
//! ## Architecture
//!
//! The library is organized into:
//! - `column`: Column definitions and configuration
//! - `state`: Table state management
//! - `row`: Row models and pipeline
//! - `features`: Sorting, filtering, pagination, etc.
//! - `table`: Main table coordinator

/// Column definitions and configuration.
///
/// Provides column types, accessors, metadata, and builder pattern
/// for defining table columns with type-safe data access.
pub mod column;

/// Table features including sorting, filtering, pagination, and more.
///
/// Each feature is implemented as a separate submodule with its own
/// state and logic, following a plugin-based architecture.
pub mod features;

/// Row types and processing pipeline.
///
/// Provides the row data structures and the row model pipeline
/// that processes rows through filtering, sorting, grouping, and pagination.
pub mod row;

/// Combined table state management.
///
/// Aggregates all feature states into a single structure
/// for easier management and passing around.
pub mod state;

/// Main table type and configuration.
///
/// Provides the primary entry point for interacting with the data table,
/// coordinating columns, data, and state.
pub mod table;

/// Re-exports for convenient access to all public types.
///
/// Import this module to get access to the most frequently used
/// types, traits, and structs in the library.
pub mod prelude;
