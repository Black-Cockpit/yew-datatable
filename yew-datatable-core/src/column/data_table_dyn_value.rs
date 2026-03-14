//! Trait for dynamically typed values that support comparison and display.
//!
//! Provides a common interface for column values of different types,
//! enabling sorting, filtering, and display operations without
//! compile-time knowledge of the concrete type.

use std::any::Any;
use std::cmp::Ordering;
use std::fmt;

/// Trait for dynamically typed values that support comparison and display.
///
/// Implementations are provided for common Rust types including
/// `String`, `&'static str`, `i32`, `i64`, `f64`, `bool`, and `usize`.
pub trait DataTableDynValue: fmt::Display + Send + Sync {
    /// Returns the value as a string for filtering.
    fn as_string(&self) -> String {
        self.to_string()
    }

    /// Compares this value to another for sorting.
    ///
    /// # Parameters
    ///
    /// - `other`: The other value to compare against.
    ///
    /// # Returns
    ///
    /// - `Ordering`: The comparison result.
    fn compare(&self, other: &dyn DataTableDynValue) -> Ordering;

    /// Returns true if this value contains the given substring (case-insensitive).
    ///
    /// # Parameters
    ///
    /// - `needle`: The substring to search for.
    ///
    /// # Returns
    ///
    /// - `bool`: Whether the value contains the substring.
    fn contains_str(&self, needle: &str) -> bool {
        // Perform case-insensitive substring search.
        self.as_string().to_lowercase().contains(&needle.to_lowercase())
    }

    /// Clone into a boxed trait object.
    ///
    /// # Returns
    ///
    /// - `Box<dyn DataTableDynValue>`: A boxed clone of this value.
    fn clone_box(&self) -> Box<dyn DataTableDynValue>;

    /// Returns the value as Any for downcasting.
    ///
    /// # Returns
    ///
    /// - `&dyn Any`: A reference to the value as Any.
    fn as_any(&self) -> &dyn Any;

    /// Returns an optional f64 representation for numeric comparisons.
    ///
    /// # Returns
    ///
    /// - `Option<f64>`: The numeric value if available.
    fn as_f64(&self) -> Option<f64> {
        None
    }
}

/// Implements `DataTableDynValue` for `String` with downcast-optimized comparison.
impl DataTableDynValue for String {
    fn compare(&self, other: &dyn DataTableDynValue) -> Ordering {
        // Try to downcast for efficient comparison.
        if let Some(other_str) = other.as_any().downcast_ref::<String>() {
            self.cmp(other_str)
        } else {
            // Fall back to string comparison.
            self.as_str().cmp(&other.as_string())
        }
    }

    fn clone_box(&self) -> Box<dyn DataTableDynValue> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implements `DataTableDynValue` for static string slices.
impl DataTableDynValue for &'static str {
    fn compare(&self, other: &dyn DataTableDynValue) -> Ordering {
        // Compare using the string representation.
        (*self).cmp(&other.as_string())
    }

    fn clone_box(&self) -> Box<dyn DataTableDynValue> {
        Box::new(self.to_string())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implements `DataTableDynValue` for `i32` with numeric comparison support.
impl DataTableDynValue for i32 {
    fn compare(&self, other: &dyn DataTableDynValue) -> Ordering {
        // Try numeric comparison first.
        if let Some(other_f64) = other.as_f64() {
            (*self as f64).partial_cmp(&other_f64).unwrap_or(Ordering::Equal)
        } else if let Some(other_val) = other.as_any().downcast_ref::<i32>() {
            // Downcast for efficient comparison.
            self.cmp(other_val)
        } else {
            // Fallback to string parsing.
            let other_val: i32 = other.as_string().parse().unwrap_or(0);
            self.cmp(&other_val)
        }
    }

    fn clone_box(&self) -> Box<dyn DataTableDynValue> {
        Box::new(*self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_f64(&self) -> Option<f64> {
        Some(*self as f64)
    }
}

/// Implements `DataTableDynValue` for `i64` with numeric comparison support.
impl DataTableDynValue for i64 {
    fn compare(&self, other: &dyn DataTableDynValue) -> Ordering {
        // Try numeric comparison first.
        if let Some(other_f64) = other.as_f64() {
            (*self as f64).partial_cmp(&other_f64).unwrap_or(Ordering::Equal)
        } else if let Some(other_val) = other.as_any().downcast_ref::<i64>() {
            // Downcast for efficient comparison.
            self.cmp(other_val)
        } else {
            // Fallback to string parsing.
            let other_val: i64 = other.as_string().parse().unwrap_or(0);
            self.cmp(&other_val)
        }
    }

    fn clone_box(&self) -> Box<dyn DataTableDynValue> {
        Box::new(*self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_f64(&self) -> Option<f64> {
        Some(*self as f64)
    }
}

/// Implements `DataTableDynValue` for `f64` with floating-point comparison.
impl DataTableDynValue for f64 {
    fn compare(&self, other: &dyn DataTableDynValue) -> Ordering {
        // Try numeric comparison first.
        if let Some(other_f64) = other.as_f64() {
            self.partial_cmp(&other_f64).unwrap_or(Ordering::Equal)
        } else if let Some(other_val) = other.as_any().downcast_ref::<f64>() {
            // Downcast for efficient comparison.
            self.partial_cmp(other_val).unwrap_or(Ordering::Equal)
        } else {
            // Fallback to string parsing.
            let other_val: f64 = other.as_string().parse().unwrap_or(0.0);
            self.partial_cmp(&other_val).unwrap_or(Ordering::Equal)
        }
    }

    fn clone_box(&self) -> Box<dyn DataTableDynValue> {
        Box::new(*self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_f64(&self) -> Option<f64> {
        Some(*self)
    }
}

/// Implements `DataTableDynValue` for `bool` with downcast-optimized comparison.
impl DataTableDynValue for bool {
    fn compare(&self, other: &dyn DataTableDynValue) -> Ordering {
        // Try downcasting for efficient comparison.
        if let Some(other_val) = other.as_any().downcast_ref::<bool>() {
            self.cmp(other_val)
        } else {
            // Fallback to string parsing.
            let other_val: bool = other.as_string().parse().unwrap_or(false);
            self.cmp(&other_val)
        }
    }

    fn clone_box(&self) -> Box<dyn DataTableDynValue> {
        Box::new(*self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implements `DataTableDynValue` for `usize` with numeric comparison support.
impl DataTableDynValue for usize {
    fn compare(&self, other: &dyn DataTableDynValue) -> Ordering {
        // Try numeric comparison first.
        if let Some(other_f64) = other.as_f64() {
            (*self as f64).partial_cmp(&other_f64).unwrap_or(Ordering::Equal)
        } else if let Some(other_val) = other.as_any().downcast_ref::<usize>() {
            // Downcast for efficient comparison.
            self.cmp(other_val)
        } else {
            // Fallback to string parsing.
            let other_val: usize = other.as_string().parse().unwrap_or(0);
            self.cmp(&other_val)
        }
    }

    fn clone_box(&self) -> Box<dyn DataTableDynValue> {
        Box::new(*self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_f64(&self) -> Option<f64> {
        Some(*self as f64)
    }
}
