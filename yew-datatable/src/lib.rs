//! # yew-datatable
//!
//! A data table component library for Yew applications.
//! Built on top of `yew-datatable-core`, providing idiomatic Yew components and hooks.
//!
//! ## Features
//!
//! - Fully-featured data table with sorting, filtering, pagination
//! - Type-safe column definitions
//! - Headless design with customizable rendering
//! - Yew hooks for state management
//!
//! ## Example
//!
//! ```rust,ignore
//! use yew::prelude::*;
//! use yew_datatable::prelude::*;
//!
//! #[function_component(MyTable)]
//! fn my_table() -> Html {
//!     let columns = vec![
//!         ColumnDefBuilder::new("name", "Name")
//!             .accessor(|row: &Person| row.name.clone())
//!             .build(),
//!         ColumnDefBuilder::new("age", "Age")
//!             .accessor(|row: &Person| row.age as i32)
//!             .build(),
//!     ];
//!
//!     let data = vec![
//!         Person { name: "Alice".into(), age: 30 },
//!         Person { name: "Bob".into(), age: 25 },
//!     ];
//!
//!     html! {
//!         <DataTable<Person> {columns} {data} />
//!     }
//! }
//! ```

/// Yew components for rendering data tables.
///
/// Provides pre-built table components including the main table,
/// header, body, and pagination components.
pub mod components;

/// Yew hooks for table state management.
///
/// Provides the `use_table` hook for creating and managing
/// table instances within Yew components.
pub mod hooks;

/// Re-exports for convenient access to all public types.
///
/// Import this module to get access to the most frequently used
/// types from both the core engine and UI components.
pub mod prelude;

/// Re-export the core crate for direct access to all types.
pub use yew_datatable_core as core;
