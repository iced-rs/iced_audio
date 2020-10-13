//! An `f32` value that is gauranteed to be constrained to the range of
//!
//! `0.0 >= value <= 1.0`

/// An `f32` value that is gauranteed to be constrained to the range of
///
/// `0.0 >= value <= 1.0`
///
/// # Example
///
/// ```
/// use iced_audio::Normal;
///
/// let mut normal = Normal::new(-1.0);
/// assert_eq!(normal.as_f32(), 0.0);
///
/// normal.set(3.0);
/// assert_eq!(normal.as_f32(), 1.0);
///
/// normal.set(0.5);
/// assert_eq!(normal.as_f32(), 0.5);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Normal {
    value: f32,
}

impl Default for Normal {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl Normal {
    /// Creates a new `Normal`.
    ///
    /// # Arguments
    ///
    /// * `value` - the value to initialize the `Normal` with
    ///
    /// if `value < 0.0`, then `normal.value` is set to `0.0`
    ///
    /// else if `value > 1.0`, then `normal.value` is set to `1.0`
    ///
    /// else `normal.value` is set to `value`
    pub fn new(value: f32) -> Self {
        Self {
            value: {
                if value < 0.0 {
                    0.0
                } else if value > 1.0 {
                    1.0
                } else {
                    value
                }
            },
        }
    }

    /// Returns a `Normal` with the value `0.0`.
    pub fn min() -> Self {
        Self { value: 0.0 }
    }

    /// Returns a `Normal` with the value `1.0`.
    pub fn max() -> Self {
        Self { value: 1.0 }
    }

    /// Returns a `Normal` with the value `0.5`.
    pub fn center() -> Self {
        Self { value: 0.5 }
    }

    /// Set a value for the `Normal`.
    ///
    /// # Arguments
    ///
    /// * `value` - the value to set the `Normal` with
    ///
    /// if `value < 0.0`, then `normal.value` is set to `0.0`
    ///
    /// else if `value > 1.0`, then `normal.value` is set to `1.0`
    ///
    /// else `normal.value` is set to `value`
    #[inline]
    pub fn set(&mut self, value: f32) {
        self.value = {
            if value < 0.0 {
                0.0
            } else if value > 1.0 {
                1.0
            } else {
                value
            }
        }
    }

    /// Returns the value of the `Normal` as an `f32`
    #[inline]
    pub fn as_f32(&self) -> f32 {
        self.value
    }

    /// Returns the inverse value (`1.0 - value`) of the `Normal` as an `f32`
    #[inline]
    pub fn as_f32_inv(&self) -> f32 {
        1.0 - self.value
    }

    /// Returns the value of the `Normal` times the `scalar`
    #[inline]
    pub fn scale(&self, scalar: f32) -> f32 {
        self.value * scalar
    }

    /// Returns the inverse value (`1.0 - value`) of the `Normal`
    /// times the `scalar`
    #[inline]
    pub fn scale_inv(&self, scalar: f32) -> f32 {
        (1.0 - self.value) * scalar
    }
}

impl From<f32> for Normal {
    fn from(value: f32) -> Self {
        Normal::new(value)
    }
}

impl From<Normal> for f32 {
    fn from(normal: Normal) -> f32 {
        normal.value
    }
}
