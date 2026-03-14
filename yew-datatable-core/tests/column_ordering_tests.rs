//! Tests for column ordering functionality.

use yew_datatable_core::prelude::{ColumnId, ColumnOrderingState};

/// Validates that a newly created ColumnOrderingState has no custom order.
#[test]
fn test_column_ordering_new() {
    let state = ColumnOrderingState::new();
    assert!(!state.has_custom_order());
    assert!(state.order().is_empty());
}

/// Validates that with_order sets the initial column order.
#[test]
fn test_column_ordering_with_order() {
    let order = vec![ColumnId::new("col1"), ColumnId::new("col2"), ColumnId::new("col3")];
    let state = ColumnOrderingState::with_order(order.clone());

    assert!(state.has_custom_order());
    assert_eq!(state.order().len(), 3);
}

/// Validates that get_index returns the position of a column in the custom order.
#[test]
fn test_column_ordering_get_index() {
    let state = ColumnOrderingState::with_order(vec![
        ColumnId::new("col1"),
        ColumnId::new("col2"),
        ColumnId::new("col3"),
    ]);

    assert_eq!(state.get_index(&ColumnId::new("col1")), Some(0));
    assert_eq!(state.get_index(&ColumnId::new("col2")), Some(1));
    assert_eq!(state.get_index(&ColumnId::new("col3")), Some(2));
    assert_eq!(state.get_index(&ColumnId::new("col4")), None);
}

/// Validates that apply_order reorders columns and appends unlisted ones at the end.
#[test]
fn test_column_ordering_apply_order() {
    let state = ColumnOrderingState::with_order(vec![
        ColumnId::new("col3"),
        ColumnId::new("col1"),
        ColumnId::new("col2"),
    ]);

    let columns = vec![ColumnId::new("col1"), ColumnId::new("col2"), ColumnId::new("col3")];

    let ordered = state.apply_order(&columns);

    assert_eq!(ordered[0], ColumnId::new("col3"));
    assert_eq!(ordered[1], ColumnId::new("col1"));
    assert_eq!(ordered[2], ColumnId::new("col2"));
}

/// Validates that apply_order appends columns not present in the custom order at the end.
#[test]
fn test_column_ordering_apply_order_with_extra_columns() {
    let state = ColumnOrderingState::with_order(vec![ColumnId::new("col2"), ColumnId::new("col1")]);

    let columns = vec![
        ColumnId::new("col1"),
        ColumnId::new("col2"),
        ColumnId::new("col3"),
        ColumnId::new("col4"),
    ];

    let ordered = state.apply_order(&columns);

    assert_eq!(ordered[0], ColumnId::new("col2"));
    assert_eq!(ordered[1], ColumnId::new("col1"));
    assert_eq!(ordered[2], ColumnId::new("col3"));
    assert_eq!(ordered[3], ColumnId::new("col4"));
}

/// Validates that apply_order returns the original order when no custom order is set.
#[test]
fn test_column_ordering_apply_order_empty() {
    let state = ColumnOrderingState::new();

    let columns = vec![ColumnId::new("col1"), ColumnId::new("col2")];

    let ordered = state.apply_order(&columns);

    assert_eq!(ordered, columns);
}

/// Validates that set_order replaces the current column order.
#[test]
fn test_column_ordering_set_order() {
    let mut state = ColumnOrderingState::new();

    state.set_order(vec![ColumnId::new("col2"), ColumnId::new("col1")]);

    assert!(state.has_custom_order());
    assert_eq!(state.order().len(), 2);
}

/// Validates that move_column relocates a column to the specified index.
#[test]
fn test_column_ordering_move_column() {
    let mut state = ColumnOrderingState::with_order(vec![
        ColumnId::new("col1"),
        ColumnId::new("col2"),
        ColumnId::new("col3"),
    ]);

    state.move_column(&ColumnId::new("col3"), 0);

    assert_eq!(state.order()[0], ColumnId::new("col3"));
    assert_eq!(state.order()[1], ColumnId::new("col1"));
    assert_eq!(state.order()[2], ColumnId::new("col2"));
}

/// Validates that swap_columns exchanges the positions of two columns.
#[test]
fn test_column_ordering_swap_columns() {
    let mut state = ColumnOrderingState::with_order(vec![
        ColumnId::new("col1"),
        ColumnId::new("col2"),
        ColumnId::new("col3"),
    ]);

    state.swap_columns(&ColumnId::new("col1"), &ColumnId::new("col3"));

    assert_eq!(state.order()[0], ColumnId::new("col3"));
    assert_eq!(state.order()[1], ColumnId::new("col2"));
    assert_eq!(state.order()[2], ColumnId::new("col1"));
}

/// Validates that move_before places a column immediately before the target.
#[test]
fn test_column_ordering_move_before() {
    let mut state = ColumnOrderingState::with_order(vec![
        ColumnId::new("col1"),
        ColumnId::new("col2"),
        ColumnId::new("col3"),
    ]);

    state.move_before(&ColumnId::new("col3"), &ColumnId::new("col1"));

    assert_eq!(state.order()[0], ColumnId::new("col3"));
    assert_eq!(state.order()[1], ColumnId::new("col1"));
    assert_eq!(state.order()[2], ColumnId::new("col2"));
}

/// Validates that move_after places a column immediately after the target.
#[test]
fn test_column_ordering_move_after() {
    let mut state = ColumnOrderingState::with_order(vec![
        ColumnId::new("col1"),
        ColumnId::new("col2"),
        ColumnId::new("col3"),
    ]);

    state.move_after(&ColumnId::new("col1"), &ColumnId::new("col3"));

    assert_eq!(state.order()[0], ColumnId::new("col2"));
    assert_eq!(state.order()[1], ColumnId::new("col3"));
    assert_eq!(state.order()[2], ColumnId::new("col1"));
}

/// Validates that initialize sets the order from a column list.
#[test]
fn test_column_ordering_initialize() {
    let mut state = ColumnOrderingState::new();

    state.initialize(&[ColumnId::new("col1"), ColumnId::new("col2")]);

    assert!(state.has_custom_order());
    assert_eq!(state.order().len(), 2);
}

/// Validates that reset clears the custom column order.
#[test]
fn test_column_ordering_reset() {
    let mut state = ColumnOrderingState::with_order(vec![ColumnId::new("col1"), ColumnId::new("col2")]);

    state.reset();

    assert!(!state.has_custom_order());
    assert!(state.order().is_empty());
}
