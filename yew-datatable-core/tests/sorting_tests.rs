//! Tests for sorting functionality.

use yew_datatable_core::prelude::{
    ColumnDefBuilder, DataTableRowId, DataTableRowModel, SortDirection, SortState, SortingState,
};

#[derive(Clone, Debug, PartialEq)]
struct TestRow {
    id: usize,
    name: String,
    age: i32,
    score: f64,
}

fn create_test_data() -> Vec<TestRow> {
    vec![
        TestRow {
            id: 1,
            name: "Alice".into(),
            age: 30,
            score: 85.5,
        },
        TestRow {
            id: 2,
            name: "Bob".into(),
            age: 25,
            score: 92.0,
        },
        TestRow {
            id: 3,
            name: "Carol".into(),
            age: 35,
            score: 78.5,
        },
        TestRow {
            id: 4,
            name: "David".into(),
            age: 28,
            score: 88.0,
        },
        TestRow {
            id: 5,
            name: "Eve".into(),
            age: 32,
            score: 95.5,
        },
    ]
}

fn create_columns() -> Vec<yew_datatable_core::column::column_def::ColumnDef<TestRow>> {
    vec![
        ColumnDefBuilder::new("id", "ID")
            .accessor(|r: &TestRow| r.id as i32)
            .build(),
        ColumnDefBuilder::new("name", "Name")
            .accessor(|r: &TestRow| r.name.clone())
            .build(),
        ColumnDefBuilder::new("age", "Age")
            .accessor(|r: &TestRow| r.age)
            .build(),
        ColumnDefBuilder::new("score", "Score")
            .accessor(|r: &TestRow| r.score)
            .build(),
    ]
}

/// Validates that a newly created `SortingState` starts with no active sorts.
#[test]
fn test_sorting_state_new() {
    let state = SortingState::new();
    assert!(!state.is_sorted());
    assert!(state.sorts().is_empty());
}

/// Validates that `SortingState::with_sorts` initializes with the provided sort list.
#[test]
fn test_sorting_state_with_sorts() {
    let sorts = vec![SortState::asc("name")];
    let state = SortingState::with_sorts(sorts);
    assert!(state.is_sorted());
    assert_eq!(state.sorts().len(), 1);
}

/// Validates that toggling sort on a new column adds an ascending sort entry.
#[test]
fn test_toggle_sort_adds_sort() {
    let mut state = SortingState::new();
    state.toggle_sort("name", false);

    assert!(state.is_sorted());
    assert_eq!(state.sorts().len(), 1);
    assert_eq!(state.get_direction(&"name".into()), Some(SortDirection::Asc));
}

/// Validates that toggling sort on an already-sorted column changes direction from ascending to descending.
#[test]
fn test_toggle_sort_changes_direction() {
    let mut state = SortingState::new();
    state.toggle_sort("name", false);
    assert_eq!(state.get_direction(&"name".into()), Some(SortDirection::Asc));

    state.toggle_sort("name", false);
    assert_eq!(state.get_direction(&"name".into()), Some(SortDirection::Desc));
}

/// Validates that toggling sort past descending removes the sort when sort removal is enabled.
#[test]
fn test_toggle_sort_removes_sort() {
    let mut state = SortingState::new().with_sort_removal(true);
    state.toggle_sort("name", false);
    state.toggle_sort("name", false);
    state.toggle_sort("name", false);

    assert!(!state.is_sorted());
    assert!(state.get_direction(&"name".into()).is_none());
}

/// Validates that multi-column sorting adds sorts in priority order when shift-toggling.
#[test]
fn test_multi_sort() {
    let mut state = SortingState::new().with_multi_sort(true);
    state.toggle_sort("name", true);
    state.toggle_sort("age", true);

    assert_eq!(state.sorts().len(), 2);
    assert_eq!(state.get_sort_index(&"name".into()), Some(0));
    assert_eq!(state.get_sort_index(&"age".into()), Some(1));
}

/// Validates that single-column sort mode replaces the previous sort with the new column.
#[test]
fn test_single_sort_replaces() {
    let mut state = SortingState::new();
    state.toggle_sort("name", false);
    state.toggle_sort("age", false);

    assert_eq!(state.sorts().len(), 1);
    assert!(state.get_direction(&"name".into()).is_none());
    assert_eq!(state.get_direction(&"age".into()), Some(SortDirection::Asc));
}

/// Validates that clearing a specific column's sort removes only that column from the sort list.
#[test]
fn test_clear_sort() {
    let mut state = SortingState::new();
    state.toggle_sort("name", false);
    state.toggle_sort("age", true);

    state.clear_sort(&"name".into());

    assert_eq!(state.sorts().len(), 1);
    assert!(state.get_direction(&"name".into()).is_none());
}

/// Validates that clearing all sorts resets the sorting state to empty.
#[test]
fn test_clear_all_sorts() {
    let mut state = SortingState::new();
    state.toggle_sort("name", true);
    state.toggle_sort("age", true);

    state.clear_all();

    assert!(!state.is_sorted());
    assert!(state.sorts().is_empty());
}

/// Validates that `set_sort` directly sets a column's sort direction without toggling.
#[test]
fn test_set_sort_directly() {
    let mut state = SortingState::new();
    state.set_sort("name", SortDirection::Desc);

    assert_eq!(state.get_direction(&"name".into()), Some(SortDirection::Desc));
}

/// Validates that exceeding the maximum multi-sort column limit evicts the oldest sort.
#[test]
fn test_max_multi_sort_columns() {
    let mut state = SortingState::new().with_multi_sort(true).with_max_multi_sort_columns(2);

    state.toggle_sort("col1", true);
    state.toggle_sort("col2", true);
    state.toggle_sort("col3", true);

    assert_eq!(state.sorts().len(), 2);
    assert!(state.get_direction(&"col1".into()).is_none());
    assert!(state.get_direction(&"col2".into()).is_some());
    assert!(state.get_direction(&"col3".into()).is_some());
}

/// Validates that `SortDirection::toggle` correctly swaps between ascending and descending.
#[test]
fn test_sort_direction_toggle() {
    let asc = SortDirection::Asc;
    let desc = SortDirection::Desc;

    assert_eq!(asc.toggle(), SortDirection::Desc);
    assert_eq!(desc.toggle(), SortDirection::Asc);
}

/// Validates that `SortDirection::apply` reverses the ordering for descending and preserves it for ascending.
#[test]
fn test_sort_direction_apply() {
    use std::cmp::Ordering;

    let asc = SortDirection::Asc;
    let desc = SortDirection::Desc;

    assert_eq!(asc.apply(Ordering::Less), Ordering::Less);
    assert_eq!(asc.apply(Ordering::Greater), Ordering::Greater);
    assert_eq!(desc.apply(Ordering::Less), Ordering::Greater);
    assert_eq!(desc.apply(Ordering::Greater), Ordering::Less);
}

/// Validates that the row model pipeline sorts rows in ascending order by a numeric column.
#[test]
fn test_row_model_sorting_ascending() {
    let data = create_test_data();
    let columns = create_columns();
    let mut model = DataTableRowModel::new(data, |_, idx| DataTableRowId::from_index(idx));

    let mut sorting = SortingState::new();
    sorting.toggle_sort("age", false);

    model.process(
        &columns,
        &Default::default(),
        &sorting,
        &Default::default(),
        &Default::default(),
        &yew_datatable_core::prelude::PaginationState::disabled(),
        &Default::default(),
    );

    let sorted: Vec<i32> = model.visible_rows().map(|r| r.original.age).collect();
    assert_eq!(sorted, vec![25, 28, 30, 32, 35]);
}

/// Validates that the row model pipeline sorts rows in descending order by a numeric column.
#[test]
fn test_row_model_sorting_descending() {
    let data = create_test_data();
    let columns = create_columns();
    let mut model = DataTableRowModel::new(data, |_, idx| DataTableRowId::from_index(idx));

    let mut sorting = SortingState::new();
    sorting.set_sort("age", SortDirection::Desc);

    model.process(
        &columns,
        &Default::default(),
        &sorting,
        &Default::default(),
        &Default::default(),
        &yew_datatable_core::prelude::PaginationState::disabled(),
        &Default::default(),
    );

    let sorted: Vec<i32> = model.visible_rows().map(|r| r.original.age).collect();
    assert_eq!(sorted, vec![35, 32, 30, 28, 25]);
}

/// Validates that the row model pipeline sorts rows alphabetically by a string column.
#[test]
fn test_row_model_string_sorting() {
    let data = create_test_data();
    let columns = create_columns();
    let mut model = DataTableRowModel::new(data, |_, idx| DataTableRowId::from_index(idx));

    let mut sorting = SortingState::new();
    sorting.toggle_sort("name", false);

    model.process(
        &columns,
        &Default::default(),
        &sorting,
        &Default::default(),
        &Default::default(),
        &yew_datatable_core::prelude::PaginationState::disabled(),
        &Default::default(),
    );

    let sorted: Vec<&str> = model.visible_rows().map(|r| r.original.name.as_str()).collect();
    assert_eq!(sorted, vec!["Alice", "Bob", "Carol", "David", "Eve"]);
}

/// Validates that sorting is stable — rows with equal sort keys preserve their original relative order.
#[test]
fn test_stable_sort() {
    #[derive(Clone, Debug, PartialEq)]
    struct Item {
        category: String,
        order: usize,
    }

    let data = vec![
        Item {
            category: "A".into(),
            order: 1,
        },
        Item {
            category: "B".into(),
            order: 2,
        },
        Item {
            category: "A".into(),
            order: 3,
        },
        Item {
            category: "B".into(),
            order: 4,
        },
        Item {
            category: "A".into(),
            order: 5,
        },
    ];

    let columns = vec![
        ColumnDefBuilder::new("category", "Category")
            .accessor(|r: &Item| r.category.clone())
            .build(),
    ];

    let mut model = DataTableRowModel::new(data, |_, idx| DataTableRowId::from_index(idx));

    let mut sorting = SortingState::new();
    sorting.toggle_sort("category", false);

    model.process(
        &columns,
        &Default::default(),
        &sorting,
        &Default::default(),
        &Default::default(),
        &yew_datatable_core::prelude::PaginationState::disabled(),
        &Default::default(),
    );

    let orders: Vec<usize> = model.visible_rows().map(|r| r.original.order).collect();
    assert_eq!(orders, vec![1, 3, 5, 2, 4]);
}
