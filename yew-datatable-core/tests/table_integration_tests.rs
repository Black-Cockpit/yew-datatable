//! Integration tests for the full table pipeline.

use yew_datatable_core::prelude::{
    ColumnDefBuilder, DataTable, DataTableOptions, DataTableOptionsBuilder, DataTableRowId, DataTableState,
    DataTableStateBuilder, PaginationState, SortingState,
};

// Mock employee data for testing
#[derive(Clone, Debug, PartialEq)]
struct Employee {
    // Employee ID
    id: usize,

    // Employee name
    name: String,

    // Employee department
    department: String,

    // Employee salary
    salary: f64,
}

// Mock data for testing
fn sample_data() -> Vec<Employee> {
    vec![
        Employee {
            id: 1,
            name: "Alice".into(),
            department: "Engineering".into(),
            salary: 90000.0,
        },
        Employee {
            id: 2,
            name: "Bob".into(),
            department: "Marketing".into(),
            salary: 70000.0,
        },
        Employee {
            id: 3,
            name: "Carol".into(),
            department: "Engineering".into(),
            salary: 95000.0,
        },
        Employee {
            id: 4,
            name: "David".into(),
            department: "Sales".into(),
            salary: 65000.0,
        },
        Employee {
            id: 5,
            name: "Eve".into(),
            department: "Engineering".into(),
            salary: 85000.0,
        },
        Employee {
            id: 6,
            name: "Frank".into(),
            department: "Marketing".into(),
            salary: 72000.0,
        },
        Employee {
            id: 7,
            name: "Grace".into(),
            department: "Sales".into(),
            salary: 68000.0,
        },
        Employee {
            id: 8,
            name: "Henry".into(),
            department: "Engineering".into(),
            salary: 92000.0,
        },
        Employee {
            id: 9,
            name: "Ivy".into(),
            department: "Marketing".into(),
            salary: 71000.0,
        },
        Employee {
            id: 10,
            name: "Jack".into(),
            department: "Sales".into(),
            salary: 67000.0,
        },
        Employee {
            id: 11,
            name: "Karen".into(),
            department: "Engineering".into(),
            salary: 88000.0,
        },
        Employee {
            id: 12,
            name: "Leo".into(),
            department: "Marketing".into(),
            salary: 75000.0,
        },
    ]
}

// Mock columns for testing
fn sample_columns() -> Vec<yew_datatable_core::column::column_def::ColumnDef<Employee>> {
    vec![
        ColumnDefBuilder::new("id", "ID")
            .accessor(|e: &Employee| e.id as i32)
            .build(),
        ColumnDefBuilder::new("name", "Name")
            .accessor(|e: &Employee| e.name.clone())
            .build(),
        ColumnDefBuilder::new("department", "Dept")
            .accessor(|e: &Employee| e.department.clone())
            .build(),
        ColumnDefBuilder::new("salary", "Salary")
            .accessor(|e: &Employee| e.salary)
            .build(),
    ]
}

/// Validates that DataTable::with_data creates a table with the correct row and column counts.
#[test]
fn test_table_creation() {
    let table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    assert_eq!(table.total_row_count(), 12);
    assert_eq!(table.columns().len(), 4);
}

/// Validates that processing with default state returns all rows unfiltered.
#[test]
fn test_table_process_no_state() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.process();
    assert_eq!(table.total_row_count(), 12);
    assert_eq!(table.filtered_row_count(), 12);
}

/// Validates that toggle_sort followed by process sorts visible rows correctly.
#[test]
fn test_table_sorting() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.toggle_sort("name", false);
    table.process();
    let names: Vec<&str> = table.visible_rows().map(|r| r.original.name.as_str()).collect();
    assert_eq!(names[0], "Alice");
    assert_eq!(names[1], "Bob");
}

/// Validates that set_column_filter followed by process reduces the filtered row count.
#[test]
fn test_table_filtering() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.set_column_filter("department", "Engineering");
    table.process();
    assert_eq!(table.filtered_row_count(), 5);
}

/// Validates that set_global_filter followed by process matches rows across all columns.
#[test]
fn test_table_global_filter() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.set_global_filter("alice");
    table.process();
    assert_eq!(table.filtered_row_count(), 1);
}

/// Validates that pagination limits the number of visible rows per page.
#[test]
fn test_table_pagination() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.state_mut().pagination = PaginationState::new().with_page_size(5);
    table.process();
    assert_eq!(table.page_row_count(), 5);
}

/// Validates page navigation through next_page across multiple pages.
#[test]
fn test_table_page_navigation() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.state_mut().pagination = PaginationState::new().with_page_size(5);
    table.process();
    assert_eq!(table.page_row_count(), 5);
    table.next_page();
    table.process();
    assert_eq!(table.page_row_count(), 5);
    table.next_page();
    table.process();
    assert_eq!(table.page_row_count(), 2);
}

/// Validates that toggle_row_selection alternates a row between selected and deselected.
#[test]
fn test_table_row_selection() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.toggle_row_selection(DataTableRowId::from_index(0));
    assert!(table.state().row_selection.is_selected(&DataTableRowId::from_index(0)));
    table.toggle_row_selection(DataTableRowId::from_index(0));
    assert!(!table.state().row_selection.is_selected(&DataTableRowId::from_index(0)));
}

/// Validates that select_all_rows selects every filtered row and clear_selection empties it.
#[test]
fn test_table_select_all() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.process();
    table.select_all_rows();
    assert_eq!(table.state().row_selection.selected_count(), 12);
    table.clear_selection();
    assert_eq!(table.state().row_selection.selected_count(), 0);
}

/// Validates that toggle_column_visibility removes a column from the visible list.
#[test]
fn test_table_column_visibility() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.toggle_column_visibility("salary".into());
    let visible = table.visible_column_ids();
    assert_eq!(visible.len(), 3);
    assert!(!visible.iter().any(|id| id.as_str() == "salary"));
}

/// Validates that visible_columns returns all columns when no visibility overrides exist.
#[test]
fn test_table_visible_columns() {
    let table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    let cols = table.visible_columns();
    assert_eq!(cols.len(), 4);
}

/// Validates that get_column retrieves a column definition by its identifier.
#[test]
fn test_table_get_column() {
    let table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    let col = table.get_column(&"name".into());
    assert!(col.is_some());
    assert_eq!(col.unwrap().header(), "Name");
}

/// Validates that get_row retrieves a row by its identifier after processing.
#[test]
fn test_table_get_row() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.process();
    let row = table.get_row(&DataTableRowId::from_index(0));
    assert!(row.is_some());
    assert_eq!(row.unwrap().original.name, "Alice");
}

/// Validates that set_data_indexed replaces the table data and updates row counts.
#[test]
fn test_table_set_data() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.process();
    assert_eq!(table.total_row_count(), 12);
    let new_data = vec![Employee {
        id: 1,
        name: "New".into(),
        department: "Test".into(),
        salary: 50000.0,
    }];
    table.set_data_indexed(new_data);
    table.process();
    assert_eq!(table.total_row_count(), 1);
}

/// Validates that set_page_size adjusts the number of visible rows per page.
#[test]
fn test_table_set_page_size() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.process();
    table.set_page_size(3);
    table.process();
    assert_eq!(table.page_row_count(), 3);
}

/// Validates that reset clears all sorting, filtering, and selection state.
#[test]
fn test_table_reset() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.toggle_sort("name", false);
    table.set_column_filter("department", "Engineering");
    table.toggle_row_selection(DataTableRowId::from_index(0));
    table.process();
    table.reset();
    table.process();
    assert_eq!(table.filtered_row_count(), 12);
    assert!(!table.state().row_selection.has_selection());
    assert!(!table.state().sorting.is_sorted());
}

/// Validates the combined pipeline: filter by department, sort by salary, and paginate.
#[test]
fn test_table_combined_filter_sort_paginate() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.set_column_filter("department", "Engineering");
    table.toggle_sort("salary", false);
    table.state_mut().pagination = PaginationState::new().with_page_size(3);
    table.process();
    assert_eq!(table.filtered_row_count(), 5);
    assert_eq!(table.page_row_count(), 3);
    let salaries: Vec<f64> = table.visible_rows().map(|r| r.original.salary).collect();
    assert!(salaries.windows(2).all(|w| w[0] <= w[1]));
}

/// Validates that DataTableOptions::default enables all features.
#[test]
fn test_table_options_default() {
    let options = DataTableOptions::default();
    assert!(options.enable_sorting);
    assert!(options.enable_filtering);
    assert!(options.enable_pagination);
}

/// Validates that DataTableOptionsBuilder can selectively disable features.
#[test]
fn test_table_options_builder() {
    let options = DataTableOptionsBuilder::new().sorting(false).pagination(false).build();
    assert!(!options.enable_sorting);
    assert!(!options.enable_pagination);
    assert!(options.enable_filtering);
}

/// Validates that DataTableState::new starts with no modifications.
#[test]
fn test_table_state_new() {
    let state = DataTableState::new();
    assert!(!state.sorting.is_sorted());
    assert!(!state.filtering.is_filtered());
    assert!(!state.has_modifications());
}

/// Validates that DataTableStateBuilder can configure initial sorting state.
#[test]
fn test_table_state_builder() {
    let sorting = SortingState::with_sorts(vec![yew_datatable_core::prelude::SortState::asc("name")]);
    let state = DataTableStateBuilder::new().sorting(sorting).build();
    assert!(state.sorting.is_sorted());
    assert!(state.has_modifications());
}

/// Validates that reset_all clears all state modifications.
#[test]
fn test_table_state_reset_all() {
    let mut state = DataTableState::new();
    state.sorting.toggle_sort("name", false);
    state.filtering.set_global_filter("test");
    assert!(state.has_modifications());
    state.reset_all();
    assert!(!state.has_modifications());
}

/// Validates that DataTable::builder returns an options builder instance.
#[test]
fn test_table_builder() {
    let _builder = DataTable::<Employee>::builder();
}

/// Validates that toggle_row_expansion marks a row as expanded.
#[test]
fn test_table_row_expansion() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.toggle_row_expansion(DataTableRowId::from_index(0));
    assert!(table.state().expanding.is_expanded(&DataTableRowId::from_index(0)));
}

/// Validates that previous_page decrements the page index.
#[test]
fn test_table_previous_page() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.state_mut().pagination = PaginationState::new().with_page_size(5);
    table.process();
    table.next_page();
    table.process();
    assert_eq!(table.state().pagination.page_index(), 1);
    table.previous_page();
    table.process();
    assert_eq!(table.state().pagination.page_index(), 0);
}

/// Validates that go_to_page navigates to a specific page index.
#[test]
fn test_table_go_to_page() {
    let mut table = DataTable::with_data(sample_columns(), sample_data(), |_, idx| {
        DataTableRowId::from_index(idx)
    });
    table.state_mut().pagination = PaginationState::new().with_page_size(5);
    table.process();
    table.go_to_page(2);
    table.process();
    assert_eq!(table.state().pagination.page_index(), 2);
}
