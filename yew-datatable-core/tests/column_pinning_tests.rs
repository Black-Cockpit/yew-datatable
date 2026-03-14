//! Tests for column pinning functionality.

use yew_datatable_core::prelude::{ColumnId, ColumnPinningPosition, ColumnPinningState};

/// Validates that a newly created ColumnPinningState has no pinned columns.
#[test]
fn test_column_pinning_new() {
    let state = ColumnPinningState::new();
    assert!(!state.has_pinned());
    assert!(state.left().is_empty());
    assert!(state.right().is_empty());
}

/// Validates that with_left creates a state with left-pinned columns.
#[test]
fn test_column_pinning_with_left() {
    let state = ColumnPinningState::with_left(vec![ColumnId::new("col1"), ColumnId::new("col2")]);

    assert!(state.has_pinned());
    assert_eq!(state.left().len(), 2);
    assert!(state.right().is_empty());
}

/// Validates that with_right creates a state with right-pinned columns.
#[test]
fn test_column_pinning_with_right() {
    let state = ColumnPinningState::with_right(vec![ColumnId::new("col1"), ColumnId::new("col2")]);

    assert!(state.has_pinned());
    assert!(state.left().is_empty());
    assert_eq!(state.right().len(), 2);
}

/// Validates that is_pinned detects columns in both left and right lists.
#[test]
fn test_column_pinning_is_pinned() {
    let state = ColumnPinningState::with_left(vec![ColumnId::new("col1")]);

    assert!(state.is_pinned(&ColumnId::new("col1")));
    assert!(!state.is_pinned(&ColumnId::new("col2")));
}

/// Validates that get_position returns the correct pinning position.
#[test]
fn test_column_pinning_get_position() {
    let mut state = ColumnPinningState::new();
    state.pin_left(ColumnId::new("col1"));
    state.pin_right(ColumnId::new("col2"));

    assert_eq!(
        state.get_position(&ColumnId::new("col1")),
        Some(ColumnPinningPosition::Left)
    );
    assert_eq!(
        state.get_position(&ColumnId::new("col2")),
        Some(ColumnPinningPosition::Right)
    );
    assert_eq!(state.get_position(&ColumnId::new("col3")), None);
}

/// Validates that pin adds a column to the specified side and removes it from the other.
#[test]
fn test_column_pinning_pin() {
    let mut state = ColumnPinningState::new();

    state.pin(ColumnId::new("col1"), ColumnPinningPosition::Left);
    state.pin(ColumnId::new("col2"), ColumnPinningPosition::Right);

    assert_eq!(state.left().len(), 1);
    assert_eq!(state.right().len(), 1);
}

/// Validates that pinning a column to a new position removes it from the old one.
#[test]
fn test_column_pinning_pin_moves_between_positions() {
    let mut state = ColumnPinningState::new();
    state.pin_left(ColumnId::new("col1"));

    state.pin_right(ColumnId::new("col1"));

    assert!(state.left().is_empty());
    assert_eq!(state.right().len(), 1);
}

/// Validates that unpin removes a column from both left and right pinned lists.
#[test]
fn test_column_pinning_unpin() {
    let mut state = ColumnPinningState::with_left(vec![ColumnId::new("col1"), ColumnId::new("col2")]);

    state.unpin(&ColumnId::new("col1"));

    assert_eq!(state.left().len(), 1);
    assert!(!state.is_pinned(&ColumnId::new("col1")));
}

/// Validates that toggle alternates a column between pinned and unpinned.
#[test]
fn test_column_pinning_toggle() {
    let mut state = ColumnPinningState::new();

    state.toggle(ColumnId::new("col1"), ColumnPinningPosition::Left);
    assert!(state.is_pinned(&ColumnId::new("col1")));

    state.toggle(ColumnId::new("col1"), ColumnPinningPosition::Left);
    assert!(!state.is_pinned(&ColumnId::new("col1")));
}

/// Validates that set_left replaces the left-pinned column list and removes duplicates from right.
#[test]
fn test_column_pinning_set_left() {
    let mut state = ColumnPinningState::new();
    state.pin_left(ColumnId::new("col1"));

    state.set_left(vec![ColumnId::new("col2"), ColumnId::new("col3")]);

    assert_eq!(state.left().len(), 2);
    assert!(!state.is_pinned(&ColumnId::new("col1")));
}

/// Validates that set_right replaces the right-pinned column list and removes duplicates from left.
#[test]
fn test_column_pinning_set_right() {
    let mut state = ColumnPinningState::new();
    state.pin_right(ColumnId::new("col1"));

    state.set_right(vec![ColumnId::new("col2"), ColumnId::new("col3")]);

    assert_eq!(state.right().len(), 2);
    assert!(!state.is_pinned(&ColumnId::new("col1")));
}

/// Validates that set_left removes duplicates from the right-pinned list.
#[test]
fn test_column_pinning_set_left_removes_from_right() {
    let mut state = ColumnPinningState::new();
    state.pin_right(ColumnId::new("col1"));

    state.set_left(vec![ColumnId::new("col1")]);

    assert!(state.right().is_empty());
    assert_eq!(state.left().len(), 1);
}

/// Validates that apply_pinning reorders columns as left, center, right.
#[test]
fn test_column_pinning_apply_pinning() {
    let mut state = ColumnPinningState::new();
    state.pin_left(ColumnId::new("col1"));
    state.pin_right(ColumnId::new("col4"));

    let columns = vec![
        ColumnId::new("col1"),
        ColumnId::new("col2"),
        ColumnId::new("col3"),
        ColumnId::new("col4"),
    ];

    let pinned = state.apply_pinning(&columns);

    assert_eq!(pinned[0], ColumnId::new("col1"));
    assert_eq!(pinned[1], ColumnId::new("col2"));
    assert_eq!(pinned[2], ColumnId::new("col3"));
    assert_eq!(pinned[3], ColumnId::new("col4"));
}

/// Validates that apply_pinning reorders columns as left, center, right with multiple pinned columns.
#[test]
fn test_column_pinning_apply_pinning_multiple() {
    let mut state = ColumnPinningState::new();
    state.set_left(vec![ColumnId::new("col3"), ColumnId::new("col1")]);
    state.set_right(vec![ColumnId::new("col4")]);

    let columns = vec![
        ColumnId::new("col1"),
        ColumnId::new("col2"),
        ColumnId::new("col3"),
        ColumnId::new("col4"),
    ];

    let pinned = state.apply_pinning(&columns);

    assert_eq!(pinned[0], ColumnId::new("col3"));
    assert_eq!(pinned[1], ColumnId::new("col1"));
    assert_eq!(pinned[2], ColumnId::new("col2"));
    assert_eq!(pinned[3], ColumnId::new("col4"));
}

/// Validates that all_pinned returns a set of all pinned column IDs.
#[test]
fn test_column_pinning_all_pinned() {
    let mut state = ColumnPinningState::new();
    state.pin_left(ColumnId::new("col1"));
    state.pin_right(ColumnId::new("col2"));

    let all = state.all_pinned();

    assert_eq!(all.len(), 2);
    assert!(all.contains(&ColumnId::new("col1")));
    assert!(all.contains(&ColumnId::new("col2")));
}

/// Validates that clear removes all pinned columns.
#[test]
fn test_column_pinning_clear() {
    let mut state = ColumnPinningState::new();
    state.pin_left(ColumnId::new("col1"));
    state.pin_right(ColumnId::new("col2"));

    state.clear();

    assert!(!state.has_pinned());
}

/// Validates that reset clears all pinning state.
#[test]
fn test_column_pinning_reset() {
    let mut state = ColumnPinningState::new();
    state.pin_left(ColumnId::new("col1"));

    state.reset();

    assert!(!state.has_pinned());
}
