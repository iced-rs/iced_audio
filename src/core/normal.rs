//! An `f32` value that is gauranteed to be constrained to the range of
//!
//! `0.0 >= value <= 1.0`

use std::fmt;

/// An `f32` value that is guaranteed to be constrained to the range of
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
#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Normal(f32);

impl Normal {
    /// A `Normal` with the value `0.0`.
    pub const MIN: Self = Self(0.0);

    /// A `Normal` with the value `0.5`.
    pub const CENTER: Self = Self(0.5);

    /// A `Normal` with the value `1.0`.
    pub const MAX: Self = Self(1.0);

    /// Creates a new `Normal`, clamping the provided value to the valid range of `[0.0..=1.0]`.
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
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Tries to create a `Normal` from the given value,
    /// erroring if the value is out of the range `[0.0..=1.0]`.
    #[inline]
    pub fn try_from(value: f32) -> Result<Self, NormalOutOfRange> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(NormalOutOfRange(value))
        }
    }

    /// Sets a value for the `Normal`, clamping it to the valid range of `[0.0..=1.0]`.
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
    pub const fn set(&mut self, value: f32) {
        *self = Normal::new(value)
    }

    /// Tries to set a value for the `Normal`,
    /// erroring if the value is out of the range `[0.0..=1.0]`.
    #[inline]
    pub fn try_set(&mut self, value: f32) -> Result<(), NormalOutOfRange> {
        *self = Normal::try_from(value)?;
        Ok(())
    }

    /// Returns the value of the `Normal` as an `f32`
    #[inline]
    pub fn as_f32(&self) -> f32 {
        self.0
    }

    /// Returns the inverse value (`1.0 - value`) of the `Normal` as an `f32`
    #[inline]
    pub fn as_f32_inv(&self) -> f32 {
        1.0 - self.0
    }

    /// Returns the value of the `Normal` times the `scalar`
    #[inline]
    pub fn scale(&self, scalar: f32) -> f32 {
        self.0 * scalar
    }

    /// Returns the inverse value (`1.0 - value`) of the `Normal`
    /// times the `scalar`
    #[inline]
    pub fn scale_inv(&self, scalar: f32) -> f32 {
        (1.0 - self.0) * scalar
    }
}

/// An error returned when trying to build a [`Normal`] from an out of range value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NormalOutOfRange(f32);

impl fmt::Display for NormalOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} out of `Normal` range [0.0..=1.0]", self.0)
    }
}

impl std::error::Error for NormalOutOfRange {}

impl From<f32> for Normal {
    fn from(value: f32) -> Self {
        Normal::new(value)
    }
}

impl From<Normal> for f32 {
    fn from(normal: Normal) -> f32 {
        normal.0
    }
}

impl From<f64> for Normal {
    fn from(value: f64) -> Self {
        Normal::new(value as f32)
    }
}

impl From<Normal> for f64 {
    fn from(normal: Normal) -> f64 {
        normal.0 as f64
    }
}

impl fmt::Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::{Normal, NormalOutOfRange};

    #[test]
    fn from_clipped() {
        let min = Normal::new(0.0);
        assert_eq!(min, Normal::MIN);
        assert_eq!(min.as_f32(), 0.0);

        let max = Normal::new(1.0);
        assert_eq!(max, Normal::MAX);
        assert_eq!(max.as_f32(), 1.0);

        let less_than_min = Normal::new(-0.1);
        assert_eq!(less_than_min, Normal::MIN);
        assert_eq!(less_than_min.as_f32(), 0.0);

        let more_than_max = Normal::new(1.1);
        assert_eq!(more_than_max, Normal::MAX);
        assert_eq!(more_than_max.as_f32(), 1.0);
    }

    #[test]
    fn set_clipped() {
        let mut normal = Normal::MIN;
        normal.set(0.0);
        assert_eq!(normal.as_f32(), 0.0);

        normal.set(1.0);
        assert_eq!(normal.as_f32(), 1.0);

        normal.set(-0.1);
        assert_eq!(normal.as_f32(), 0.0);

        normal.set(1.1);
        assert_eq!(normal.as_f32(), 1.0);
    }

    #[test]
    fn try_from() {
        let min = Normal::try_from(0.0).unwrap();
        assert_eq!(min, Normal::MIN);
        assert_eq!(min.as_f32(), 0.0);

        let max = Normal::try_from(1.0).unwrap();
        assert_eq!(max, Normal::MAX);
        assert_eq!(max.as_f32(), 1.0);

        assert_eq!(Normal::try_from(-0.1).unwrap_err(), NormalOutOfRange(-0.1));
        assert_eq!(Normal::try_from(1.1).unwrap_err(), NormalOutOfRange(1.1));
    }

    #[test]
    fn try_set() {
        let mut normal = Normal::MIN;
        normal.try_set(0.0).unwrap();
        assert_eq!(normal.as_f32(), 0.0);

        normal.try_set(1.0).unwrap();
        assert_eq!(normal.as_f32(), 1.0);

        let err = normal.try_set(-0.1).unwrap_err();
        assert_eq!(err, NormalOutOfRange(-0.1));
        assert_eq!(normal.as_f32(), 1.0);

        let err = normal.try_set(1.1).unwrap_err();
        assert_eq!(err, NormalOutOfRange(1.1));
        assert_eq!(normal.as_f32(), 1.0);
    }
}
