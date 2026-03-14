//! Tests for column visibility functionality.

use yew_datatable_core::prelude::{ColumnId, ColumnVisibilityState};

/// Validates that a newly created ColumnVisibilityState has all columns visible by default.
#[test]
fn test_column_visibility_new() {
    let state = ColumnVisibilityState::new();
    assert!(state.is_visible(&ColumnId::new("any_column")));
}

/// Validates that with_hidden creates a state with the specified columns hidden.
#[test]
fn test_column_visibility_with_hidden() {
    let state = ColumnVisibilityState::with_hidden(vec![ColumnId::new("col1"), ColumnId::new("col2")]);

    assert!(!state.is_visible(&ColumnId::new("col1")));
    assert!(!state.is_visible(&ColumnId::new("col2")));
    assert!(state.is_visible(&ColumnId::new("col3")));
}

/// Validates that with_visible creates a state with only the specified columns visible.
#[test]
fn test_column_visibility_with_visible() {
    let state = ColumnVisibilityState::with_visible(vec![ColumnId::new("col1"), ColumnId::new("col2")]);

    assert!(state.is_visible(&ColumnId::new("col1")));
    assert!(state.is_visible(&ColumnId::new("col2")));
    assert!(!state.is_visible(&ColumnId::new("col3")));
}

/// Validates that set_visibility explicitly sets a column visible or hidden.
#[test]
fn test_column_visibility_set_visibility() {
    let mut state = ColumnVisibilityState::new();

    state.set_visibility(ColumnId::new("col1"), false);

    assert!(!state.is_visible(&ColumnId::new("col1")));
}

/// Validates that show makes a previously hidden column visible.
#[test]
fn test_column_visibility_show() {
    let mut state = ColumnVisibilityState::with_hidden(vec![ColumnId::new("col1")]);

    state.show(ColumnId::new("col1"));

    assert!(state.is_visible(&ColumnId::new("col1")));
}

/// Validates that hide makes a previously visible column hidden.
#[test]
fn test_column_visibility_hide() {
    let mut state = ColumnVisibilityState::new();

    state.hide(ColumnId::new("col1"));

    assert!(!state.is_visible(&ColumnId::new("col1")));
}

/// Validates that toggle alternates a column between visible and hidden.
#[test]
fn test_column_visibility_toggle() {
    let mut state = ColumnVisibilityState::new();

    state.toggle(ColumnId::new("col1"));
    assert!(!state.is_visible(&ColumnId::new("col1")));

    state.toggle(ColumnId::new("col1"));
    assert!(state.is_visible(&ColumnId::new("col1")));
}

/// Validates that show_many makes multiple columns visible at once.
#[test]
fn test_column_visibility_show_many() {
    let mut state = ColumnVisibilityState::with_hidden(vec![
        ColumnId::new("col1"),
        ColumnId::new("col2"),
        ColumnId::new("col3"),
    ]);

    state.show_many(vec![ColumnId::new("col1"), ColumnId::new("col2")]);

    assert!(state.is_visible(&ColumnId::new("col1")));
    assert!(state.is_visible(&ColumnId::new("col2")));
    assert!(!state.is_visible(&ColumnId::new("col3")));
}

/// Validates that hide_many hides multiple columns at once.
#[test]
fn test_column_visibility_hide_many() {
    let mut state = ColumnVisibilityState::new();

    state.hide_many(vec![ColumnId::new("col1"), ColumnId::new("col2")]);

    assert!(!state.is_visible(&ColumnId::new("col1")));
    assert!(!state.is_visible(&ColumnId::new("col2")));
    assert!(state.is_visible(&ColumnId::new("col3")));
}

/// Validates that show_all makes all columns visible.
#[test]
fn test_column_visibility_show_all() {
    let mut state = ColumnVisibilityState::with_hidden(vec![ColumnId::new("col1"), ColumnId::new("col2")]);

    state.show_all();

    assert!(state.is_visible(&ColumnId::new("col1")));
    assert!(state.is_visible(&ColumnId::new("col2")));
}

/// Validates that hide_all hides all columns.
#[test]
fn test_column_visibility_hide_all() {
    let mut state = ColumnVisibilityState::new();
    state.show(ColumnId::new("col1"));

    state.hide_all();

    assert!(!state.is_visible(&ColumnId::new("col1")));
    assert!(!state.is_visible(&ColumnId::new("any")));
}

/// Validates that visible_columns filters a column list to only visible ones.
#[test]
fn test_column_visibility_visible_columns() {
    let state = ColumnVisibilityState::with_hidden(vec![ColumnId::new("col2")]);
    let columns = vec![ColumnId::new("col1"), ColumnId::new("col2"), ColumnId::new("col3")];

    let visible = state.visible_columns(&columns);

    assert_eq!(visible.len(), 2);
    assert!(visible.contains(&&ColumnId::new("col1")));
    assert!(visible.contains(&&ColumnId::new("col3")));
}

/// Validates that hidden_columns filters a column list to only hidden ones.
#[test]
fn test_column_visibility_hidden_columns() {
    let state = ColumnVisibilityState::with_hidden(vec![ColumnId::new("col2")]);
    let columns = vec![ColumnId::new("col1"), ColumnId::new("col2"), ColumnId::new("col3")];

    let hidden = state.hidden_columns(&columns);

    assert_eq!(hidden.len(), 1);
    assert!(hidden.contains(&&ColumnId::new("col2")));
}

/// Validates that reset_column removes the visibility override for a specific column.
#[test]
fn test_column_visibility_reset_column() {
    let mut state = ColumnVisibilityState::new();
    state.hide(ColumnId::new("col1"));

    state.reset_column(&ColumnId::new("col1"));

    assert!(state.is_visible(&ColumnId::new("col1")));
}

/// Validates that reset restores all columns to default visible state.
#[test]
fn test_column_visibility_reset() {
    let mut state = ColumnVisibilityState::new();
    state.hide(ColumnId::new("col1"));
    state.hide(ColumnId::new("col2"));

    state.reset();

    assert!(state.is_visible(&ColumnId::new("col1")));
    assert!(state.is_visible(&ColumnId::new("col2")));
}

/// Validates that with_default_visible sets the default for unlisted columns.
#[test]
fn test_column_visibility_with_default_visible() {
    let state = ColumnVisibilityState::new().with_default_visible(false);

    assert!(!state.is_visible(&ColumnId::new("any")));
}

/// Validates that visibility_map returns the underlying visibility override map.
#[test]
fn test_column_visibility_map() {
    let mut state = ColumnVisibilityState::new();
    state.hide(ColumnId::new("col1"));
    state.hide(ColumnId::new("col2"));

    let map = state.visibility_map();

    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&ColumnId::new("col1")), Some(&false));
    assert_eq!(map.get(&ColumnId::new("col2")), Some(&false));
}
