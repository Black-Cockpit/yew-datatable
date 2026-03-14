//! Tests for pagination functionality.

use yew_datatable_core::prelude::{PaginationMode, PaginationState};

/// Validates that a newly created `PaginationState` starts enabled on page 0 with default page size.
#[test]
fn test_pagination_state_new() {
    let state = PaginationState::new();
    assert!(state.is_enabled());
    assert_eq!(state.page_index(), 0);
    assert_eq!(state.page_size(), PaginationState::DEFAULT_PAGE_SIZE);
}

/// Validates that `PaginationState::disabled` creates a disabled state.
#[test]
fn test_pagination_state_disabled() {
    let state = PaginationState::disabled();
    assert!(!state.is_enabled());
}

/// Validates that `with_page_size` sets the page size correctly.
#[test]
fn test_pagination_with_page_size() {
    let state = PaginationState::new().with_page_size(25);
    assert_eq!(state.page_size(), 25);
}

/// Validates that `with_page_size(0)` clamps the page size to a minimum of 1.
#[test]
fn test_pagination_with_page_size_minimum() {
    let state = PaginationState::new().with_page_size(0);
    assert_eq!(state.page_size(), 1);
}

/// Validates that `page_number` returns the 1-based page number.
#[test]
fn test_pagination_page_number() {
    let state = PaginationState::new();
    assert_eq!(state.page_number(), 1);
}

/// Validates page count calculation with ceiling division for various row counts.
#[test]
fn test_pagination_page_count() {
    let state = PaginationState::new().with_page_size(10);

    assert_eq!(state.page_count(0), 1);
    assert_eq!(state.page_count(5), 1);
    assert_eq!(state.page_count(10), 1);
    assert_eq!(state.page_count(11), 2);
    assert_eq!(state.page_count(20), 2);
    assert_eq!(state.page_count(25), 3);
    assert_eq!(state.page_count(100), 10);
}

/// Validates that start and end indices are correctly computed for the first page.
#[test]
fn test_pagination_start_end_index() {
    let state = PaginationState::new().with_page_size(10);

    assert_eq!(state.start_index(), 0);
    assert_eq!(state.end_index(100), 10);
}

/// Validates the row range for the first page.
#[test]
fn test_pagination_row_range() {
    let state = PaginationState::new().with_page_size(10);
    let range = state.row_range(100);

    assert_eq!(range.start, 0);
    assert_eq!(range.end, 10);
}

/// Validates that the row range on the last page only includes remaining rows.
#[test]
fn test_pagination_row_range_last_page() {
    let mut state = PaginationState::new().with_page_size(10);
    state.go_to_last(25);

    let range = state.row_range(25);
    assert_eq!(range.start, 20);
    assert_eq!(range.end, 25);
}

/// Validates that `can_go_previous` is false on the first page and true after navigating forward.
#[test]
fn test_pagination_can_go_previous() {
    let mut state = PaginationState::new();

    assert!(!state.can_go_previous());

    state.go_to_page(1, 100);
    assert!(state.can_go_previous());
}

/// Validates that `can_go_next` is true when more pages exist and false on the last page.
#[test]
fn test_pagination_can_go_next() {
    let state = PaginationState::new().with_page_size(10);

    assert!(state.can_go_next(100));
    assert!(state.can_go_next(11));
    assert!(!state.can_go_next(10));
    assert!(!state.can_go_next(5));
}

/// Validates that `go_to_first` resets the page index to 0.
#[test]
fn test_pagination_go_to_first() {
    let mut state = PaginationState::new().with_page_size(10);
    state.go_to_page(5, 100);

    state.go_to_first();

    assert_eq!(state.page_index(), 0);
}

/// Validates that `go_to_last` navigates to the final page for various row counts.
#[test]
fn test_pagination_go_to_last() {
    let mut state = PaginationState::new().with_page_size(10);

    state.go_to_last(100);
    assert_eq!(state.page_index(), 9);

    state.go_to_last(25);
    assert_eq!(state.page_index(), 2);
}

/// Validates that `go_to_previous` decrements the page and does not go below 0.
#[test]
fn test_pagination_go_to_previous() {
    let mut state = PaginationState::new().with_page_size(10);
    state.go_to_page(5, 100);

    state.go_to_previous();
    assert_eq!(state.page_index(), 4);

    state.go_to_first();
    state.go_to_previous();
    assert_eq!(state.page_index(), 0);
}

/// Validates that `go_to_next` increments the page and does not exceed the last page.
#[test]
fn test_pagination_go_to_next() {
    let mut state = PaginationState::new().with_page_size(10);

    state.go_to_next(100);
    assert_eq!(state.page_index(), 1);

    state.go_to_last(100);
    state.go_to_next(100);
    assert_eq!(state.page_index(), 9);
}

/// Validates that `go_to_page` navigates correctly and clamps to valid bounds.
#[test]
fn test_pagination_go_to_page() {
    let mut state = PaginationState::new().with_page_size(10);

    state.go_to_page(5, 100);
    assert_eq!(state.page_index(), 5);

    state.go_to_page(100, 100);
    assert_eq!(state.page_index(), 9);

    state.go_to_page(0, 100);
    assert_eq!(state.page_index(), 0);
}

/// Validates that `set_page_size` updates the size and adjusts the page to preserve scroll position.
#[test]
fn test_pagination_set_page_size() {
    let mut state = PaginationState::new().with_page_size(10);
    state.go_to_page(5, 100);

    state.set_page_size(20, 100);

    assert_eq!(state.page_size(), 20);
    assert_eq!(state.page_index(), 2);
}

/// Validates that increasing page size clamps the current page to the valid range.
#[test]
fn test_pagination_set_page_size_adjusts_page() {
    let mut state = PaginationState::new().with_page_size(10);
    state.go_to_page(9, 100);

    state.set_page_size(50, 100);

    assert_eq!(state.page_index(), 1);
}

/// Validates that `set_total_rows` adjusts page count and clamps the page index for server-side mode.
#[test]
fn test_pagination_set_total_rows() {
    let mut state = PaginationState::new()
        .with_page_size(10)
        .with_mode(PaginationMode::Server);

    state.go_to_page(9, 100);
    state.set_total_rows(50);

    assert_eq!(state.page_count(0), 5);
    assert_eq!(state.page_index(), 4);
}

/// Validates that enable and disable toggle the pagination state.
#[test]
fn test_pagination_enable_disable() {
    let mut state = PaginationState::new();

    state.disable();
    assert!(!state.is_enabled());

    state.enable();
    assert!(state.is_enabled());
}

/// Validates that reset returns the page index to 0.
#[test]
fn test_pagination_reset() {
    let mut state = PaginationState::new().with_page_size(10);
    state.go_to_page(5, 100);

    state.reset();

    assert_eq!(state.page_index(), 0);
}

/// Validates that `with_mode` sets the pagination mode to client or server.
#[test]
fn test_pagination_mode() {
    let client = PaginationState::new();
    assert_eq!(client.mode(), PaginationMode::Client);

    let server = PaginationState::new().with_mode(PaginationMode::Server);
    assert_eq!(server.mode(), PaginationMode::Server);
}

/// Validates that `with_page_size_options` stores the custom page size list.
#[test]
fn test_pagination_page_size_options() {
    let state = PaginationState::new().with_page_size_options(vec![5, 10, 15, 20]);

    assert_eq!(state.page_size_options(), &[5, 10, 15, 20]);
}

/// Validates that the default page size options match the built-in constant.
#[test]
fn test_pagination_default_page_size_options() {
    let state = PaginationState::new();
    assert_eq!(state.page_size_options(), PaginationState::DEFAULT_PAGE_SIZE_OPTIONS);
}
