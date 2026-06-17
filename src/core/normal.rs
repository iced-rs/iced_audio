//! An `f32` value that is gauranteed to be constrained to the range of
//!
//! `0.0 >= value <= 1.0`

use std::fmt;

/// An error returned when trying to build a [`Normal`] from an out of range value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NormalOutOfRange(f32);

impl fmt::Display for NormalOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} out of `Normal` range (0.0..=1.0)", self.0)
    }
}

/// An `f32` value that is gauranteed to be constrained to the range of
///
/// `0.0 >= value <= 1.0`
///
/// # Example
///
/// ```
/// use iced_audio::Normal;
///
/// let mut normal = Normal::from_clipped(-1.0);
/// assert_eq!(normal.as_f32(), 0.0);
///
/// normal.set_clipped(3.0);
/// assert_eq!(normal.as_f32(), 1.0);
///
/// normal.set_clipped(0.5);
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
    /// A `Normal` with the value `0.0`.
    pub const MIN: Self = Self { value: 0.0 };

    /// A `Normal` with the value `0.5`.
    pub const CENTER: Self = Self { value: 0.5 };

    /// A `Normal` with the value `1.0`.
    pub const MAX: Self = Self { value: 1.0 };

    /// Creates a new `Normal`, clipping the provided value.
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
    pub fn from_clipped(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
        }
    }

    /// Sets a value for the `Normal`, clipping the provided value.
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
    pub fn set_clipped(&mut self, value: f32) {
        *self = Normal::from_clipped(value)
    }

    /// Tries to set a value for the `Normal`,
    /// erroring if the value is out of the range `(0.0..=1.0)`.
    #[inline]
    pub fn try_set(&mut self, value: f32) -> Result<(), NormalOutOfRange> {
        *self = Normal::try_from(value)?;

        Ok(())
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

impl std::error::Error for NormalOutOfRange {}

impl TryFrom<f32> for Normal {
    type Error = NormalOutOfRange;

    fn try_from(value: f32) -> Result<Self, NormalOutOfRange> {
        if !(0.0..=1.0).contains(&value) {
            return Err(NormalOutOfRange(value));
        }

        Ok(Normal { value })
    }
}

impl From<Normal> for f32 {
    fn from(normal: Normal) -> f32 {
        normal.value
    }
}

#[cfg(test)]
mod tests {
    use super::{Normal, NormalOutOfRange};

    #[test]
    fn from_clipped() {
        let min = Normal::from_clipped(0.0);
        assert_eq!(min, Normal::MIN);
        assert_eq!(min.as_f32(), 0.0);

        let max = Normal::from_clipped(1.0);
        assert_eq!(max, Normal::MAX);
        assert_eq!(max.as_f32(), 1.0);

        let less_than_min = Normal::from_clipped(-0.1);
        assert_eq!(less_than_min, Normal::MIN);
        assert_eq!(less_than_min.as_f32(), 0.0);

        let more_than_max = Normal::from_clipped(1.1);
        assert_eq!(more_than_max, Normal::MAX);
        assert_eq!(more_than_max.as_f32(), 1.0);
    }

    #[test]
    fn set_clipped() {
        let mut normal = Normal::MIN;
        normal.set_clipped(0.0);
        assert_eq!(normal.as_f32(), 0.0);

        normal.set_clipped(1.0);
        assert_eq!(normal.as_f32(), 1.0);

        normal.set_clipped(-0.1);
        assert_eq!(normal.as_f32(), 0.0);

        normal.set_clipped(1.1);
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
