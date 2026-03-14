//! Tests for aggregation functionality.

use yew_datatable_core::prelude::{AggregationState, BuiltInAggregation, ColumnId};

/// Validates that a newly created AggregationState starts enabled with no column assignments.
#[test]
fn test_aggregation_state_new() {
    let state = AggregationState::new();
    assert!(state.is_enabled());
}

/// Validates that with_enabled configures the enabled flag.
#[test]
fn test_aggregation_state_with_enabled() {
    let state = AggregationState::new().with_enabled(false);
    assert!(!state.is_enabled());
}

/// Validates that set_aggregation and get_aggregation store and retrieve functions correctly.
#[test]
fn test_aggregation_set_aggregation() {
    let mut state = AggregationState::new();

    state.set_aggregation(ColumnId::new("col1"), BuiltInAggregation::Sum);

    assert_eq!(
        state.get_aggregation(&ColumnId::new("col1")),
        Some(BuiltInAggregation::Sum)
    );
}

/// Validates that remove_aggregation removes a column assignment.
#[test]
fn test_aggregation_remove_aggregation() {
    let mut state = AggregationState::new();
    state.set_aggregation(ColumnId::new("col1"), BuiltInAggregation::Sum);

    state.remove_aggregation(&ColumnId::new("col1"));

    assert!(state.get_aggregation(&ColumnId::new("col1")).is_none());
}

/// Validates that clear removes all column aggregation assignments.
#[test]
fn test_aggregation_clear() {
    let mut state = AggregationState::new();
    state.set_aggregation(ColumnId::new("col1"), BuiltInAggregation::Sum);
    state.set_aggregation(ColumnId::new("col2"), BuiltInAggregation::Mean);

    state.clear();

    assert!(state.get_aggregation(&ColumnId::new("col1")).is_none());
    assert!(state.get_aggregation(&ColumnId::new("col2")).is_none());
}

/// Validates that reset clears all aggregation state.
#[test]
fn test_aggregation_reset() {
    let mut state = AggregationState::new();
    state.set_aggregation(ColumnId::new("col1"), BuiltInAggregation::Sum);

    state.reset();

    assert!(state.get_aggregation(&ColumnId::new("col1")).is_none());
}

/// Validates the Sum aggregation function for f64 values.
#[test]
fn test_builtin_aggregation_sum() {
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = BuiltInAggregation::Sum.aggregate_f64(&values);

    assert_eq!(result, Some(15.0));
}

/// Validates the Mean aggregation function for f64 values.
#[test]
fn test_builtin_aggregation_mean() {
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = BuiltInAggregation::Mean.aggregate_f64(&values);

    assert_eq!(result, Some(3.0));
}

/// Validates the Min aggregation function for f64 values.
#[test]
fn test_builtin_aggregation_min() {
    let values = vec![3.0, 1.0, 4.0, 1.0, 5.0];
    let result = BuiltInAggregation::Min.aggregate_f64(&values);

    assert_eq!(result, Some(1.0));
}

/// Validates the Max aggregation function for f64 values.
#[test]
fn test_builtin_aggregation_max() {
    let values = vec![3.0, 1.0, 4.0, 1.0, 5.0];
    let result = BuiltInAggregation::Max.aggregate_f64(&values);

    assert_eq!(result, Some(5.0));
}

/// Validates the Count aggregation function for f64 values.
#[test]
fn test_builtin_aggregation_count() {
    let values = vec![1.0, 2.0, 3.0];
    let result = BuiltInAggregation::Count.aggregate_f64(&values);

    assert_eq!(result, Some(3.0));
}

/// Validates the UniqueCount aggregation function for f64 values.
#[test]
fn test_builtin_aggregation_unique_count() {
    let values = vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0];
    let result = BuiltInAggregation::UniqueCount.aggregate_f64(&values);

    assert_eq!(result, Some(3.0));
}

/// Validates the Median aggregation function for an odd-length f64 array.
#[test]
fn test_builtin_aggregation_median_odd() {
    let values = vec![1.0, 3.0, 5.0, 7.0, 9.0];
    let result = BuiltInAggregation::Median.aggregate_f64(&values);

    assert_eq!(result, Some(5.0));
}

/// Validates the Median aggregation function for an even-length f64 array.
#[test]
fn test_builtin_aggregation_median_even() {
    let values = vec![1.0, 2.0, 3.0, 4.0];
    let result = BuiltInAggregation::Median.aggregate_f64(&values);

    assert_eq!(result, Some(2.5));
}

/// Validates the First aggregation function returns the first f64 value.
#[test]
fn test_builtin_aggregation_first() {
    let values = vec![5.0, 3.0, 1.0];
    let result = BuiltInAggregation::First.aggregate_f64(&values);

    assert_eq!(result, Some(5.0));
}

/// Validates the Last aggregation function returns the last f64 value.
#[test]
fn test_builtin_aggregation_last() {
    let values = vec![5.0, 3.0, 1.0];
    let result = BuiltInAggregation::Last.aggregate_f64(&values);

    assert_eq!(result, Some(1.0));
}

/// Validates that all f64 aggregation functions return None for empty input.
#[test]
fn test_builtin_aggregation_empty_values() {
    let values: Vec<f64> = vec![];

    assert_eq!(BuiltInAggregation::Sum.aggregate_f64(&values), None);
    assert_eq!(BuiltInAggregation::Mean.aggregate_f64(&values), None);
    assert_eq!(BuiltInAggregation::Min.aggregate_f64(&values), None);
    assert_eq!(BuiltInAggregation::Max.aggregate_f64(&values), None);
    assert_eq!(BuiltInAggregation::First.aggregate_f64(&values), None);
    assert_eq!(BuiltInAggregation::Last.aggregate_f64(&values), None);
}

/// Validates the Count aggregation function for string values.
#[test]
fn test_builtin_aggregation_strings_count() {
    let values = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let result = BuiltInAggregation::Count.aggregate_strings(&values);

    assert_eq!(result, Some("3".to_string()));
}

/// Validates the UniqueCount aggregation function for string values.
#[test]
fn test_builtin_aggregation_strings_unique_count() {
    let values = vec!["a".to_string(), "b".to_string(), "a".to_string(), "c".to_string()];
    let result = BuiltInAggregation::UniqueCount.aggregate_strings(&values);

    assert_eq!(result, Some("3".to_string()));
}

/// Validates the First aggregation function for string values.
#[test]
fn test_builtin_aggregation_strings_first() {
    let values = vec!["first".to_string(), "second".to_string()];
    let result = BuiltInAggregation::First.aggregate_strings(&values);

    assert_eq!(result, Some("first".to_string()));
}

/// Validates the Last aggregation function for string values.
#[test]
fn test_builtin_aggregation_strings_last() {
    let values = vec!["first".to_string(), "last".to_string()];
    let result = BuiltInAggregation::Last.aggregate_strings(&values);

    assert_eq!(result, Some("last".to_string()));
}

/// Validates the Min aggregation function for string values.
#[test]
fn test_builtin_aggregation_strings_min() {
    let values = vec!["banana".to_string(), "apple".to_string(), "cherry".to_string()];
    let result = BuiltInAggregation::Min.aggregate_strings(&values);

    assert_eq!(result, Some("apple".to_string()));
}

/// Validates the Max aggregation function for string values.
#[test]
fn test_builtin_aggregation_strings_max() {
    let values = vec!["banana".to_string(), "apple".to_string(), "cherry".to_string()];
    let result = BuiltInAggregation::Max.aggregate_strings(&values);

    assert_eq!(result, Some("cherry".to_string()));
}

/// Validates that string aggregation functions return None for empty input.
#[test]
fn test_builtin_aggregation_strings_empty() {
    let values: Vec<String> = vec![];

    assert_eq!(BuiltInAggregation::Count.aggregate_strings(&values), None);
    assert_eq!(BuiltInAggregation::First.aggregate_strings(&values), None);
    assert_eq!(BuiltInAggregation::Last.aggregate_strings(&values), None);
}
