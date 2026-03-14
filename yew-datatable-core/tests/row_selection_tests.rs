//! Tests for row selection functionality.

use yew_datatable_core::prelude::{DataTableRowId, RowSelectionMode, RowSelectionState};

/// Validates that a newly created RowSelectionState starts with multi mode and no selection.
#[test]
fn test_row_selection_state_new() {
    let state = RowSelectionState::new();
    assert!(state.is_enabled());
    assert!(!state.has_selection());
    assert_eq!(state.selected_count(), 0);
    assert_eq!(state.mode(), RowSelectionMode::Multi);
}

/// Validates that None mode disables all selection.
#[test]
fn test_row_selection_mode_none() {
    let state = RowSelectionState::with_mode(RowSelectionMode::None);
    assert!(!state.is_enabled());
}

/// Validates that Single mode enables single-row selection.
#[test]
fn test_row_selection_mode_single() {
    let state = RowSelectionState::with_mode(RowSelectionMode::Single);
    assert!(state.is_enabled());
    assert_eq!(state.mode(), RowSelectionMode::Single);
}

/// Validates that selecting a row adds it to the selection set.
#[test]
fn test_row_selection_select() {
    let mut state = RowSelectionState::new();

    state.select(DataTableRowId::new("row1"));

    assert!(state.has_selection());
    assert!(state.is_selected(&DataTableRowId::new("row1")));
    assert_eq!(state.selected_count(), 1);
}

/// Validates that selecting multiple rows in multi mode adds all of them.
#[test]
fn test_row_selection_select_multiple() {
    let mut state = RowSelectionState::new();

    state.select(DataTableRowId::new("row1"));
    state.select(DataTableRowId::new("row2"));
    state.select(DataTableRowId::new("row3"));

    assert_eq!(state.selected_count(), 3);
    assert!(state.is_selected(&DataTableRowId::new("row1")));
    assert!(state.is_selected(&DataTableRowId::new("row2")));
    assert!(state.is_selected(&DataTableRowId::new("row3")));
}

/// Validates that selecting a new row in single mode replaces the previous selection.
#[test]
fn test_row_selection_single_mode_replaces() {
    let mut state = RowSelectionState::with_mode(RowSelectionMode::Single);

    state.select(DataTableRowId::new("row1"));
    state.select(DataTableRowId::new("row2"));

    assert_eq!(state.selected_count(), 1);
    assert!(!state.is_selected(&DataTableRowId::new("row1")));
    assert!(state.is_selected(&DataTableRowId::new("row2")));
}

/// Validates that selection operations are ignored in none mode.
#[test]
fn test_row_selection_none_mode_ignores() {
    let mut state = RowSelectionState::with_mode(RowSelectionMode::None);

    state.select(DataTableRowId::new("row1"));

    assert!(!state.has_selection());
    assert_eq!(state.selected_count(), 0);
}

/// Validates that deselecting a row removes it from the selection set.
#[test]
fn test_row_selection_deselect() {
    let mut state = RowSelectionState::new();
    state.select(DataTableRowId::new("row1"));
    state.select(DataTableRowId::new("row2"));

    state.deselect(&DataTableRowId::new("row1"));

    assert_eq!(state.selected_count(), 1);
    assert!(!state.is_selected(&DataTableRowId::new("row1")));
    assert!(state.is_selected(&DataTableRowId::new("row2")));
}

/// Validates that toggling a row alternates between selected and deselected.
#[test]
fn test_row_selection_toggle() {
    let mut state = RowSelectionState::new();

    state.toggle(DataTableRowId::new("row1"));
    assert!(state.is_selected(&DataTableRowId::new("row1")));

    state.toggle(DataTableRowId::new("row1"));
    assert!(!state.is_selected(&DataTableRowId::new("row1")));
}

/// Validates that select_many adds all provided rows in multi mode.
#[test]
fn test_row_selection_select_many() {
    let mut state = RowSelectionState::new();

    state.select_many(vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ]);

    assert_eq!(state.selected_count(), 3);
}

/// Validates that select_many in single mode only selects the first row.
#[test]
fn test_row_selection_select_many_single_mode() {
    let mut state = RowSelectionState::with_mode(RowSelectionMode::Single);

    state.select_many(vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ]);

    assert_eq!(state.selected_count(), 1);
    assert!(state.is_selected(&DataTableRowId::new("row1")));
}

/// Validates that deselect_many removes the specified rows from the selection.
#[test]
fn test_row_selection_deselect_many() {
    let mut state = RowSelectionState::new();
    state.select_many(vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ]);

    state.deselect_many(vec![DataTableRowId::new("row1"), DataTableRowId::new("row3")]);

    assert_eq!(state.selected_count(), 1);
    assert!(state.is_selected(&DataTableRowId::new("row2")));
}

/// Validates that select_all adds all provided rows in multi mode.
#[test]
fn test_row_selection_select_all() {
    let mut state = RowSelectionState::new();
    let rows = vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ];

    state.select_all(rows);

    assert_eq!(state.selected_count(), 3);
}

/// Validates that toggle_all selects all rows when none are selected.
#[test]
fn test_row_selection_toggle_all_select() {
    let mut state = RowSelectionState::new();
    let rows = vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ];

    state.toggle_all(rows.clone());

    assert!(state.is_all_selected(&rows));
}

/// Validates that toggle_all deselects all rows when all are already selected.
#[test]
fn test_row_selection_toggle_all_deselect() {
    let mut state = RowSelectionState::new();
    let rows = vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ];

    state.select_all(rows.clone());
    state.toggle_all(rows.clone());

    assert!(!state.has_selection());
}

/// Validates that is_all_selected returns true only when every provided row is selected.
#[test]
fn test_row_selection_is_all_selected() {
    let mut state = RowSelectionState::new();
    let rows = vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ];

    state.select(DataTableRowId::new("row1"));
    state.select(DataTableRowId::new("row2"));

    assert!(!state.is_all_selected(&rows));

    state.select(DataTableRowId::new("row3"));
    assert!(state.is_all_selected(&rows));
}

/// Validates that is_some_selected detects partial selection for indeterminate state.
#[test]
fn test_row_selection_is_some_selected() {
    let mut state = RowSelectionState::new();
    let rows = vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ];

    assert!(!state.is_some_selected(&rows));

    state.select(DataTableRowId::new("row1"));
    assert!(state.is_some_selected(&rows));

    state.select(DataTableRowId::new("row2"));
    state.select(DataTableRowId::new("row3"));
    assert!(!state.is_some_selected(&rows));
}

/// Validates that clear removes all selections.
#[test]
fn test_row_selection_clear() {
    let mut state = RowSelectionState::new();
    state.select_many(vec![DataTableRowId::new("row1"), DataTableRowId::new("row2")]);

    state.clear();

    assert!(!state.has_selection());
    assert_eq!(state.selected_count(), 0);
}

/// Validates that switching to single mode trims the selection to one row.
#[test]
fn test_row_selection_set_mode() {
    let mut state = RowSelectionState::new();
    state.select_many(vec![
        DataTableRowId::new("row1"),
        DataTableRowId::new("row2"),
        DataTableRowId::new("row3"),
    ]);

    state.set_mode(RowSelectionMode::Single);

    assert_eq!(state.selected_count(), 1);
}

/// Validates that switching to none mode clears all selections.
#[test]
fn test_row_selection_set_mode_none_clears() {
    let mut state = RowSelectionState::new();
    state.select(DataTableRowId::new("row1"));

    state.set_mode(RowSelectionMode::None);

    assert!(!state.has_selection());
}

/// Validates that selected_ids returns an iterator over all selected row IDs.
#[test]
fn test_row_selection_selected_ids() {
    let mut state = RowSelectionState::new();
    state.select(DataTableRowId::new("row1"));
    state.select(DataTableRowId::new("row2"));

    let ids: Vec<_> = state.selected_ids().collect();
    assert_eq!(ids.len(), 2);
}

/// Validates that selected_ids_vec returns a vector of all selected row IDs.
#[test]
fn test_row_selection_selected_ids_vec() {
    let mut state = RowSelectionState::new();
    state.select(DataTableRowId::new("row1"));
    state.select(DataTableRowId::new("row2"));

    let ids = state.selected_ids_vec();
    assert_eq!(ids.len(), 2);
}

/// Validates that reset clears all selections.
#[test]
fn test_row_selection_reset() {
    let mut state = RowSelectionState::new();
    state.select(DataTableRowId::new("row1"));

    state.reset();

    assert!(!state.has_selection());
}

/// Validates that with_row_click configures the row click selection behavior.
#[test]
fn test_row_selection_with_row_click() {
    let state = RowSelectionState::new().with_row_click(false);
    assert!(!state.is_row_click_enabled());

    let state2 = RowSelectionState::new().with_row_click(true);
    assert!(state2.is_row_click_enabled());
}

/// Validates that is_all_selected returns false for an empty row list.
#[test]
fn test_row_selection_is_all_selected_empty() {
    let state = RowSelectionState::new();
    let empty: Vec<DataTableRowId> = vec![];

    assert!(!state.is_all_selected(&empty));
}
