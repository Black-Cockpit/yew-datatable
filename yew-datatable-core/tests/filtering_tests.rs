//! Tests for filtering functionality.

use yew_datatable_core::prelude::{
    BuiltInFilter, ColumnDefBuilder, ColumnFilter, DataTableRowId, DataTableRowModel, FilterState, FilterValue,
    GlobalFilter, PaginationState,
};

#[derive(Clone, Debug, PartialEq)]
struct TestRow {
    id: usize,
    name: String,
    category: String,
    price: f64,
    active: bool,
}

fn create_test_data() -> Vec<TestRow> {
    vec![
        TestRow {
            id: 1,
            name: "Apple".into(),
            category: "Fruit".into(),
            price: 1.50,
            active: true,
        },
        TestRow {
            id: 2,
            name: "Banana".into(),
            category: "Fruit".into(),
            price: 0.75,
            active: true,
        },
        TestRow {
            id: 3,
            name: "Carrot".into(),
            category: "Vegetable".into(),
            price: 0.50,
            active: true,
        },
        TestRow {
            id: 4,
            name: "Donut".into(),
            category: "Snack".into(),
            price: 2.00,
            active: false,
        },
        TestRow {
            id: 5,
            name: "Eggplant".into(),
            category: "Vegetable".into(),
            price: 1.25,
            active: true,
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
        ColumnDefBuilder::new("category", "Category")
            .accessor(|r: &TestRow| r.category.clone())
            .build(),
        ColumnDefBuilder::new("price", "Price")
            .accessor(|r: &TestRow| r.price)
            .build(),
    ]
}

/// Validates that a newly created `FilterState` starts with no active filters.
#[test]
fn test_filter_state_new() {
    let state = FilterState::new();
    assert!(!state.is_filtered());
    assert!(state.column_filters().is_empty());
    assert!(state.global_filter().is_empty());
}

/// Validates that setting a text filter marks the state as filtered.
#[test]
fn test_set_text_filter() {
    let mut state = FilterState::new();
    state.set_text_filter("name", "apple");

    assert!(state.is_filtered());
    assert!(state.get_column_filter(&"name".into()).is_some());
}

/// Validates that setting a text filter to an empty string removes the filter.
#[test]
fn test_clear_text_filter_with_empty_string() {
    let mut state = FilterState::new();
    state.set_text_filter("name", "apple");
    state.set_text_filter("name", "");

    assert!(!state.is_filtered());
    assert!(state.get_column_filter(&"name".into()).is_none());
}

/// Validates that setting a global filter marks the state as filtered with the correct value.
#[test]
fn test_set_global_filter() {
    let mut state = FilterState::new();
    state.set_global_filter("search term");

    assert!(state.is_filtered());
    assert_eq!(state.global_filter().value, "search term");
}

/// Validates that clearing the global filter resets the state to unfiltered.
#[test]
fn test_clear_global_filter() {
    let mut state = FilterState::new();
    state.set_global_filter("search");
    state.clear_global_filter();

    assert!(!state.is_filtered());
    assert!(state.global_filter().is_empty());
}

/// Validates that clearing all filters removes both column and global filters.
#[test]
fn test_clear_all_filters() {
    let mut state = FilterState::new();
    state.set_text_filter("name", "test");
    state.set_text_filter("category", "test");
    state.set_global_filter("global");

    state.clear_all();

    assert!(!state.is_filtered());
    assert!(state.column_filters().is_empty());
    assert!(state.global_filter().is_empty());
}

/// Validates that a text `FilterValue` displays correctly and is not empty.
#[test]
fn test_filter_value_text() {
    let value = FilterValue::Text("hello".into());
    assert_eq!(value.as_display_string(), "hello");
    assert!(!value.is_empty());
}

/// Validates that an empty text `FilterValue` reports as empty.
#[test]
fn test_filter_value_text_empty() {
    let value = FilterValue::Text("".into());
    assert!(value.is_empty());
}

/// Validates that a numeric `FilterValue` displays correctly and is not empty.
#[test]
fn test_filter_value_number() {
    let value = FilterValue::Number(42.5);
    assert_eq!(value.as_display_string(), "42.5");
    assert!(!value.is_empty());
}

/// Validates that a number range `FilterValue` is empty only when both bounds are `None`.
#[test]
fn test_filter_value_number_range() {
    let value = FilterValue::NumberRange {
        min: Some(10.0),
        max: Some(100.0),
    };
    assert!(!value.is_empty());

    let empty_range = FilterValue::NumberRange { min: None, max: None };
    assert!(empty_range.is_empty());
}

/// Validates that a boolean `FilterValue` displays correctly and is not empty.
#[test]
fn test_filter_value_boolean() {
    let value = FilterValue::Boolean(true);
    assert_eq!(value.as_display_string(), "true");
    assert!(!value.is_empty());
}

/// Validates that a multi-select `FilterValue` displays as comma-separated and reports empty when the list is empty.
#[test]
fn test_filter_value_multi_select() {
    let value = FilterValue::MultiSelect(vec!["A".into(), "B".into()]);
    assert_eq!(value.as_display_string(), "A, B");
    assert!(!value.is_empty());

    let empty = FilterValue::MultiSelect(vec![]);
    assert!(empty.is_empty());
}

/// Validates that `ColumnFilter` constructors produce the correct `FilterValue` variants.
#[test]
fn test_column_filter_constructors() {
    let text = ColumnFilter::text("col", "value");
    assert!(matches!(text.value, FilterValue::Text(_)));

    let num = ColumnFilter::number("col", 42.0);
    assert!(matches!(num.value, FilterValue::Number(_)));

    let bool_filter = ColumnFilter::boolean("col", true);
    assert!(matches!(bool_filter.value, FilterValue::Boolean(true)));

    let range = ColumnFilter::number_range("col", Some(0.0), Some(100.0));
    assert!(matches!(range.value, FilterValue::NumberRange { .. }));
}

/// Validates that a global filter with explicit column list only includes the specified columns.
#[test]
fn test_global_filter_with_columns() {
    let filter = GlobalFilter::new("search").with_columns(vec!["col1".into(), "col2".into()]);

    assert!(filter.includes_column(&"col1".into()));
    assert!(filter.includes_column(&"col2".into()));
    assert!(!filter.includes_column(&"col3".into()));
}

/// Validates that a global filter with no explicit columns includes all columns.
#[test]
fn test_global_filter_empty_columns_includes_all() {
    let filter = GlobalFilter::new("search");

    assert!(filter.includes_column(&"any_column".into()));
}

/// Validates the case-insensitive substring match filter.
#[test]
fn test_builtin_filter_includes_string() {
    let filter = BuiltInFilter::IncludesString;
    let value = FilterValue::Text("app".into());

    assert!(filter.test_string("Apple", &value));
    assert!(filter.test_string("APPLE", &value));
    assert!(filter.test_string("pineapple", &value));
    assert!(!filter.test_string("Banana", &value));
}

/// Validates the case-sensitive substring match filter.
#[test]
fn test_builtin_filter_includes_string_sensitive() {
    let filter = BuiltInFilter::IncludesStringSensitive;
    let value = FilterValue::Text("App".into());

    assert!(filter.test_string("Apple", &value));
    assert!(!filter.test_string("apple", &value));
}

/// Validates the case-insensitive exact match filter.
#[test]
fn test_builtin_filter_equals() {
    let filter = BuiltInFilter::Equals;
    let value = FilterValue::Text("apple".into());

    assert!(filter.test_string("Apple", &value));
    assert!(filter.test_string("APPLE", &value));
    assert!(!filter.test_string("Apples", &value));
}

/// Validates the case-insensitive starts-with filter.
#[test]
fn test_builtin_filter_starts_with() {
    let filter = BuiltInFilter::StartsWith;
    let value = FilterValue::Text("app".into());

    assert!(filter.test_string("Apple", &value));
    assert!(!filter.test_string("Pineapple", &value));
}

/// Validates the case-insensitive ends-with filter.
#[test]
fn test_builtin_filter_ends_with() {
    let filter = BuiltInFilter::EndsWith;
    let value = FilterValue::Text("ple".into());

    assert!(filter.test_string("Apple", &value));
    assert!(filter.test_string("Pineapple", &value));
    assert!(!filter.test_string("Apples", &value));
}

/// Validates the case-insensitive not-equals filter.
#[test]
fn test_builtin_filter_not_equals() {
    let filter = BuiltInFilter::NotEquals;
    let value = FilterValue::Text("apple".into());

    assert!(!filter.test_string("Apple", &value));
    assert!(filter.test_string("Banana", &value));
}

/// Validates the case-insensitive set membership filter.
#[test]
fn test_builtin_filter_in_array() {
    let filter = BuiltInFilter::InArray;
    let value = FilterValue::MultiSelect(vec!["Apple".into(), "Banana".into()]);

    assert!(filter.test_string("apple", &value));
    assert!(filter.test_string("Banana", &value));
    assert!(!filter.test_string("Carrot", &value));
}

/// Validates the is-empty filter for empty and non-empty strings.
#[test]
fn test_builtin_filter_is_empty() {
    let filter = BuiltInFilter::IsEmpty;
    let value = FilterValue::Text("".into());

    assert!(filter.test_string("", &value));
    assert!(!filter.test_string("something", &value));
}

/// Validates the numeric greater-than filter.
#[test]
fn test_builtin_filter_greater_than() {
    let filter = BuiltInFilter::GreaterThan;
    let value = FilterValue::Number(10.0);

    assert!(filter.test_number(15.0, &value));
    assert!(!filter.test_number(10.0, &value));
    assert!(!filter.test_number(5.0, &value));
}

/// Validates the numeric less-than filter.
#[test]
fn test_builtin_filter_less_than() {
    let filter = BuiltInFilter::LessThan;
    let value = FilterValue::Number(10.0);

    assert!(filter.test_number(5.0, &value));
    assert!(!filter.test_number(10.0, &value));
    assert!(!filter.test_number(15.0, &value));
}

/// Validates the numeric between (inclusive range) filter.
#[test]
fn test_builtin_filter_between() {
    let filter = BuiltInFilter::Between;
    let value = FilterValue::NumberRange {
        min: Some(10.0),
        max: Some(20.0),
    };

    assert!(filter.test_number(15.0, &value));
    assert!(filter.test_number(10.0, &value));
    assert!(filter.test_number(20.0, &value));
    assert!(!filter.test_number(5.0, &value));
    assert!(!filter.test_number(25.0, &value));
}

/// Validates that the row model pipeline filters rows by a column-specific text filter.
#[test]
fn test_row_model_column_filtering() {
    let data = create_test_data();
    let columns = create_columns();
    let mut model = DataTableRowModel::new(data, |_, idx| DataTableRowId::from_index(idx));

    let mut filter = FilterState::new();
    filter.set_text_filter("category", "Fruit");

    model.process(
        &columns,
        &filter,
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &PaginationState::disabled(),
        &Default::default(),
    );

    let names: Vec<&str> = model.visible_rows().map(|r| r.original.name.as_str()).collect();
    assert_eq!(names, vec!["Apple", "Banana"]);
}

/// Validates that the row model pipeline filters rows by a global cross-column search.
#[test]
fn test_row_model_global_filtering() {
    let data = create_test_data();
    let columns = create_columns();
    let mut model = DataTableRowModel::new(data, |_, idx| DataTableRowId::from_index(idx));

    let mut filter = FilterState::new();
    filter.set_global_filter("an");

    model.process(
        &columns,
        &filter,
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &PaginationState::disabled(),
        &Default::default(),
    );

    let names: Vec<&str> = model.visible_rows().map(|r| r.original.name.as_str()).collect();
    assert_eq!(names, vec!["Banana", "Eggplant"]);
}

/// Validates that the row model pipeline applies both column and global filters simultaneously.
#[test]
fn test_row_model_combined_filtering() {
    let data = create_test_data();
    let columns = create_columns();
    let mut model = DataTableRowModel::new(data, |_, idx| DataTableRowId::from_index(idx));

    let mut filter = FilterState::new();
    filter.set_text_filter("category", "Vegetable");
    filter.set_global_filter("egg");

    model.process(
        &columns,
        &filter,
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &PaginationState::disabled(),
        &Default::default(),
    );

    let names: Vec<&str> = model.visible_rows().map(|r| r.original.name.as_str()).collect();
    assert_eq!(names, vec!["Eggplant"]);
}

/// Validates that the row model returns zero rows when no data matches the filter.
#[test]
fn test_row_model_no_matches() {
    let data = create_test_data();
    let columns = create_columns();
    let mut model = DataTableRowModel::new(data, |_, idx| DataTableRowId::from_index(idx));

    let mut filter = FilterState::new();
    filter.set_global_filter("xyz123");

    model.process(
        &columns,
        &filter,
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &PaginationState::disabled(),
        &Default::default(),
    );

    assert_eq!(model.filtered_row_count(), 0);
}

/// Validates that the case sensitivity setting is correctly stored and retrieved.
#[test]
fn test_filter_state_case_sensitivity() {
    let state = FilterState::new().with_case_sensitive(true);
    assert!(state.is_case_sensitive());

    let state2 = FilterState::new().with_case_sensitive(false);
    assert!(!state2.is_case_sensitive());
}
