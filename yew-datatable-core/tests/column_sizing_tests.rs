//! Tests for column sizing functionality.

use yew_datatable_core::prelude::{ColumnId, ColumnSize, ColumnSizingMode, ColumnSizingState};

/// Validates that a newly created ColumnSizingState starts enabled with fixed mode.
#[test]
fn test_column_sizing_state_new() {
    let state = ColumnSizingState::new();
    assert!(state.is_enabled());
    assert!(!state.is_resizing());
    assert_eq!(state.mode(), ColumnSizingMode::Fixed);
}

/// Validates that with_mode sets the sizing mode correctly.
#[test]
fn test_column_sizing_with_mode() {
    let state = ColumnSizingState::new().with_mode(ColumnSizingMode::Flex);
    assert_eq!(state.mode(), ColumnSizingMode::Flex);
}

/// Validates that a default ColumnSize has the expected width and minimum.
#[test]
fn test_column_size_defaults() {
    let size = ColumnSize::new();
    assert_eq!(size.width, ColumnSize::DEFAULT_WIDTH);
    assert_eq!(size.min_width, ColumnSize::DEFAULT_MIN_WIDTH);
}

/// Validates that clamp constrains values within min/max bounds.
#[test]
fn test_column_size_clamp() {
    let size = ColumnSize::new().with_min(50.0).with_max(300.0);
    assert_eq!(size.clamp(10.0), 50.0);
    assert_eq!(size.clamp(150.0), 150.0);
    assert_eq!(size.clamp(500.0), 300.0);
}

/// Validates that set_width and get_width store and retrieve column widths.
#[test]
fn test_column_sizing_get_set_width() {
    let mut state = ColumnSizingState::new();
    state.set_width("col1".into(), 250.0);
    assert_eq!(state.get_width(&"col1".into()), 250.0);
}

/// Validates the resize lifecycle: start, update delta, and end.
#[test]
fn test_column_sizing_resize_operations() {
    let mut state = ColumnSizingState::new();
    let col_id: ColumnId = "col1".into();
    state.set_width(col_id.clone(), 150.0);
    state.start_resize(col_id.clone());
    assert!(state.is_resizing());
    state.update_resize(50.0);
    assert_eq!(state.get_width(&col_id), 200.0);
    state.end_resize();
    assert!(!state.is_resizing());
}

/// Validates that initialize sets default widths for uninitialized columns.
#[test]
fn test_column_sizing_initialize() {
    let mut state = ColumnSizingState::new();
    let cols: Vec<ColumnId> = vec!["col1".into(), "col2".into()];
    state.initialize(&cols, Some(100.0));
    assert_eq!(state.get_width(&"col1".into()), 100.0);
    assert_eq!(state.get_width(&"col2".into()), 100.0);
}

/// Validates that total_width sums the widths of the specified columns.
#[test]
fn test_column_sizing_total_width() {
    let mut state = ColumnSizingState::new();
    state.set_width("col1".into(), 100.0);
    state.set_width("col2".into(), 200.0);
    let cols: Vec<ColumnId> = vec!["col1".into(), "col2".into()];
    assert_eq!(state.total_width(&cols), 300.0);
}

/// Validates that reset clears all stored sizes and stops any active resize.
#[test]
fn test_column_sizing_reset() {
    let mut state = ColumnSizingState::new();
    state.set_width("col1".into(), 200.0);
    state.start_resize("col1".into());
    state.reset();
    assert_eq!(state.get_width(&"col1".into()), ColumnSize::DEFAULT_WIDTH);
    assert!(!state.is_resizing());
}
