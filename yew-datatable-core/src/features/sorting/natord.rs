//! Natural ordering comparison for strings.
//!
//! Provides natural sort order where embedded numbers are compared
//! by their numeric value rather than lexicographically.
//! Supports both ASCII and Unicode numeric digits.

use std::cmp::Ordering;

/// Compares two strings using natural ordering.
///
/// Numbers embedded in strings are compared numerically,
/// so "item2" sorts before "item10".
///
/// # Parameters
///
/// - `a`: The first string to compare.
/// - `b`: The second string to compare.
///
/// # Returns
///
/// - `Ordering`: The natural comparison result.
pub fn compare(a: &str, b: &str) -> Ordering {
    // Create peekable character iterators for both strings.
    let mut a_chars = a.chars().peekable();
    let mut b_chars = b.chars().peekable();

    loop {
        match (a_chars.peek(), b_chars.peek()) {
            // Both strings exhausted, they are equal.
            (None, None) => return Ordering::Equal,
            // First string is shorter.
            (None, Some(_)) => return Ordering::Less,
            // Second string is shorter.
            (Some(_), None) => return Ordering::Greater,
            (Some(&ac), Some(&bc)) => {
                // Check if both characters are numeric digits.
                if is_numeric_char(ac) && is_numeric_char(bc) {
                    // Collect and compare full numbers.
                    let a_num = collect_number(&mut a_chars);
                    let b_num = collect_number(&mut b_chars);
                    match a_num.cmp(&b_num) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                } else {
                    // Compare characters individually.
                    a_chars.next();
                    b_chars.next();
                    match ac.cmp(&bc) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
            }
        }
    }
}

/// Compares two strings using case-insensitive natural ordering.
///
/// # Parameters
///
/// - `a`: The first string to compare.
/// - `b`: The second string to compare.
///
/// # Returns
///
/// - `Ordering`: The case-insensitive natural comparison result.
pub fn compare_ignore_case(a: &str, b: &str) -> Ordering {
    // Convert both strings to lowercase and compare.
    compare(&a.to_lowercase(), &b.to_lowercase())
}

/// Checks if a character is a numeric digit (Unicode-aware).
///
/// # Parameters
///
/// - `c`: The character to check.
///
/// # Returns
///
/// - `bool`: Whether the character is numeric.
fn is_numeric_char(c: char) -> bool {
    c.is_numeric()
}

/// Collects consecutive numeric digits into a number.
///
/// # Parameters
///
/// - `chars`: The peekable character iterator to consume digits from.
///
/// # Returns
///
/// - `u64`: The collected numeric value.
fn collect_number<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> u64 {
    // Accumulate digits into a number.
    let mut num = 0u64;
    while let Some(&c) = chars.peek() {
        if let Some(digit) = c.to_digit(10) {
            // Accumulate the digit value with overflow protection.
            num = num.saturating_mul(10).saturating_add(digit as u64);
            chars.next();
        } else if c.is_numeric() {
            // Handle non-ASCII numeric characters by skipping them.
            chars.next();
        } else {
            // Stop at the first non-numeric character.
            break;
        }
    }
    num
}
