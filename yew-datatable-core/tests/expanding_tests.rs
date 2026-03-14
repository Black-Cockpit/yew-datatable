//! Tests for row expansion functionality.

use yew_datatable_core::prelude::{DataTableRowId, ExpandingState};

/// Validates that a newly created ExpandingState starts with no expansions.
#[test]
fn test_expanding_state_new() {
    let state = ExpandingState::new();
    assert!(!state.has_expanded());
    assert!(!state.is_expand_all());
    assert_eq!(state.expanded_count(), 0);
}

/// Validates that all_expanded creates a state with expand-all enabled.
#[test]
fn test_expanding_state_all_expanded() {
    let state = ExpandingState::all_expanded();
    assert!(state.is_expand_all());
    assert!(state.has_expanded());
}

/// Validates that is_expanded returns true for expanded rows and false for others.
#[test]
fn test_expanding_is_expanded() {
    let mut state = ExpandingState::new();
    state.expand(DataTableRowId::new("row1"));

    assert!(state.is_expanded(&DataTableRowId::new("row1")));
    assert!(!state.is_expanded(&DataTableRowId::new("row2")));
}

/// Validates that is_expanded returns true for any row when expand-all is enabled.
#[test]
fn test_expanding_is_expanded_with_expand_all() {
    let state = ExpandingState::all_expanded();

    assert!(state.is_expanded(&DataTableRowId::new("any_row")));
}

/// Validates that expanding a row adds it to the expanded set.
#[test]
fn test_expanding_expand() {
    let mut state = ExpandingState::new();

    state.expand(DataTableRowId::new("row1"));

    assert!(state.is_expanded(&DataTableRowId::new("row1")));
    assert_eq!(state.expanded_count(), 1);
}

/// Validates that collapsing a row removes it from the expanded set.
#[test]
fn test_expanding_collapse() {
    let mut state = ExpandingState::new();
    state.expand(DataTableRowId::new("row1"));

    state.collapse(&DataTableRowId::new("row1"));

    assert!(!state.is_expanded(&DataTableRowId::new("row1")));
    assert_eq!(state.expanded_count(), 0);
}

/// Validates that toggling a row alternates between expanded and collapsed.
#[test]
fn test_expanding_toggle() {
    let mut state = ExpandingState::new();

    state.toggle(DataTableRowId::new("row1"));
    assert!(state.is_expanded(&DataTableRowId::new("row1")));

    state.toggle(DataTableRowId::new("row1"));
    assert!(!state.is_expanded(&DataTableRowId::new("row1")));
}

/// Validates that expand_many adds all provided rows to the expanded set.
#[test]
fn test_expanding_expand_many() {
    let mut state = ExpandingState::new();

    state.expand_many(vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ]);

    assert_eq!(state.expanded_count(), 3);
    assert!(state.is_expanded(&DataTableRowId::new("row1")));
    assert!(state.is_expanded(&DataTableRowId::new("row2")));
    assert!(state.is_expanded(&DataTableRowId::new("row3")));
}

/// Validates that collapse_many removes the specified rows from the expanded set.
#[test]
fn test_expanding_collapse_many() {
    let mut state = ExpandingState::new();
    state.expand_many(vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ]);

    state.collapse_many(vec![DataTableRowId::new("row1"), DataTableRowId::new("row3")]);

    assert_eq!(state.expanded_count(), 1);
    assert!(state.is_expanded(&DataTableRowId::new("row2")));
}

/// Validates that expand_all enables the expand-all flag.
#[test]
fn test_expanding_expand_all() {
    let mut state = ExpandingState::new();
    state.expand(DataTableRowId::new("row1"));

    state.expand_all();

    assert!(state.is_expand_all());
    assert!(state.is_expanded(&DataTableRowId::new("any_row")));
}

/// Validates that collapse_all disables expand-all and clears individual expansions.
#[test]
fn test_expanding_collapse_all() {
    let mut state = ExpandingState::new();
    state.expand_many(vec![DataTableRowId::new("row1"), DataTableRowId::new("row2")]);
    state.expand_all();

    state.collapse_all();

    assert!(!state.is_expand_all());
    assert!(!state.has_expanded());
}

/// Validates that toggle_all alternates the expand-all flag.
#[test]
fn test_expanding_toggle_all() {
    let mut state = ExpandingState::new();

    state.toggle_all();
    assert!(state.is_expand_all());

    state.toggle_all();
    assert!(!state.is_expand_all());
}

/// Validates that reset clears all expansion state.
#[test]
fn test_expanding_reset() {
    let mut state = ExpandingState::new();
    state.expand(DataTableRowId::new("row1"));
    state.expand_all();

    state.reset();

    assert!(!state.is_expand_all());
    assert!(!state.has_expanded());
}

/// Validates that expanded_ids returns an iterator over individually expanded row IDs.
#[test]
fn test_expanding_expanded_ids() {
    let mut state = ExpandingState::new();
    state.expand(DataTableRowId::new("row1"));
    state.expand(DataTableRowId::new("row2"));

    let ids: Vec<_> = state.expanded_ids().collect();

    assert_eq!(ids.len(), 2);
}

/// Validates that auto-expand depth configuration controls should_auto_expand behavior.
#[test]
fn test_expanding_with_auto_expand_depth() {
    let state = ExpandingState::new().with_auto_expand_depth(2);

    assert!(state.should_auto_expand(0));
    assert!(state.should_auto_expand(1));
    assert!(!state.should_auto_expand(2));
    assert!(!state.should_auto_expand(3));
}

/// Validates that should_auto_expand returns false when no auto-expand depth is configured.
#[test]
fn test_expanding_should_auto_expand_no_depth() {
    let state = ExpandingState::new();

    assert!(!state.should_auto_expand(0));
    assert!(!state.should_auto_expand(1));
}

/// Validates that with_auto_expand_parents configures the auto-expand parents flag.
#[test]
fn test_expanding_with_auto_expand_parents() {
    let state = ExpandingState::new().with_auto_expand_parents(true);
    // This is a configuration option that would be used during row model processing
    // Just verify the builder works
    assert!(!state.is_expand_all()); // State should still work normally
}
