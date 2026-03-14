//! Size info for a single column.
//!
//! Stores the current width, minimum and maximum constraints,
//! and flex grow factor for a column.

/// Size info for a single column.
///
/// Tracks width constraints and the current size of a column.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColumnSize {
    /// Current width in pixels.
    pub width: f64,

    /// Minimum width in pixels.
    pub min_width: f64,

    /// Maximum width in pixels.
    pub max_width: f64,

    /// Flex grow factor.
    pub flex: f64,
}

impl ColumnSize {
    /// Default column width.
    pub const DEFAULT_WIDTH: f64 = 150.0;
    /// Default minimum width.
    pub const DEFAULT_MIN_WIDTH: f64 = 50.0;
    /// Default maximum width.
    pub const DEFAULT_MAX_WIDTH: f64 = f64::MAX;

    /// Creates a new column size with defaults.
    ///
    /// # Returns
    ///
    /// - `ColumnSize`: A new column size with default width and constraints.
    pub fn new() -> Self {
        Self {
            width: Self::DEFAULT_WIDTH,
            min_width: Self::DEFAULT_MIN_WIDTH,
            max_width: Self::DEFAULT_MAX_WIDTH,
            flex: 1.0,
        }
    }

    /// Creates a column size with a specific width.
    ///
    /// # Parameters
    ///
    /// - `width`: The initial width in pixels.
    ///
    /// # Returns
    ///
    /// - `ColumnSize`: A new column size with the specified width.
    pub fn with_width(width: f64) -> Self {
        Self { width, ..Self::new() }
    }

    /// Sets the minimum width.
    ///
    /// # Parameters
    ///
    /// - `min`: The minimum width in pixels.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column size.
    pub fn with_min(mut self, min: f64) -> Self {
        // Set the minimum width constraint.
        self.min_width = min;
        self
    }

    /// Sets the maximum width.
    ///
    /// # Parameters
    ///
    /// - `max`: The maximum width in pixels.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column size.
    pub fn with_max(mut self, max: f64) -> Self {
        // Set the maximum width constraint.
        self.max_width = max;
        self
    }

    /// Sets the flex factor.
    ///
    /// # Parameters
    ///
    /// - `flex`: The flex grow factor.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified column size.
    pub fn with_flex(mut self, flex: f64) -> Self {
        // Set the flex grow factor.
        self.flex = flex;
        self
    }

    /// Clamps the width to the min/max constraints.
    ///
    /// # Parameters
    ///
    /// - `width`: The width to clamp.
    ///
    /// # Returns
    ///
    /// - `f64`: The clamped width.
    pub fn clamp(&self, width: f64) -> f64 {
        // Clamp the width between min and max constraints.
        width.clamp(self.min_width, self.max_width)
    }

    /// Sets the width with clamping.
    ///
    /// # Parameters
    ///
    /// - `width`: The desired width in pixels.
    pub fn set_width(&mut self, width: f64) {
        // Clamp and set the width.
        self.width = self.clamp(width);
    }
}

/// Provides a default column size with standard width and constraints.
impl Default for ColumnSize {
    fn default() -> Self {
        Self::new()
    }
}
