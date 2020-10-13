//! The core module of `Iced Audio`.
//!
//! This module holds basic types that can be reused and re-exported in
//! different runtime implementations.

pub mod math;
pub mod normal_param;
pub mod offset;
pub mod range;

pub use normal_param::*;
pub use offset::*;
pub use range::*;

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

/// pi / 180.0
pub static PI_OVER_180: f32 = std::f32::consts::PI / 180.0;
/// 2.0 * pi
pub static TWO_PI: f32 = std::f32::consts::PI * 2.0;
/// pi * (3.0 / 2.0)
pub static THREE_HALVES_PI: f32 = std::f32::consts::PI * 2.0;

/// The default minimum angle of a rotating widget such as a Knob
pub static DEFAULT_ANGLE_MIN: f32 = 30.0 * PI_OVER_180;
/// The default maximum angle of a rotating widget such as a Knob
pub static DEFAULT_ANGLE_MAX: f32 = (360.0 - 30.0) * PI_OVER_180;

/// The range between the minimum and maximum angle (in radians) the knob
/// will rotate.
///
/// `0.0` radians points straight down at the bottom of the knob, with the
/// angles rotating clockwise towards `TWO_PI` (`2*PI`).
///
/// Values < `0.0` and >= `TWO_PI` are not allowed.
///
/// The default minimum (converted to degrees) is `30` degrees, and the default
/// maximum is `330` degrees, giving a span of `300` degrees, and a halfway
/// point pointing strait up.
#[derive(Debug, Clone)]
pub struct KnobAngleRange {
    min: f32,
    max: f32,
}

impl std::default::Default for KnobAngleRange {
    fn default() -> Self {
        Self {
            min: DEFAULT_ANGLE_MIN,
            max: DEFAULT_ANGLE_MAX,
        }
    }
}

impl KnobAngleRange {
    /// The range between the `min` and `max` angle (in degrees) the knob
    /// will rotate.
    ///
    /// `0.0` degrees points straight down at the bottom of the knob, with the
    /// angles rotating clockwise towards `360` degrees.
    ///
    /// Values < `0.0` and >= `360.0` will be set to `0.0`.
    ///
    /// The default minimum is `30` degrees, and the default maximum is `330`
    /// degrees, giving a span of `300` degrees, and a halfway point pointing
    /// strait up.
    ///
    /// # Panics
    ///
    /// This will panic if `min` > `max`.
    pub fn from_deg(min: f32, max: f32) -> Self {
        let min_rad = min * PI_OVER_180;
        let max_rad = max * PI_OVER_180;

        Self::from_rad(min_rad, max_rad)
    }

    /// The span between the `min` and `max` angle (in radians) the knob
    /// will rotate.
    ///
    /// `0.0` radians points straight down at the bottom of the knob, with the
    /// angles rotating clockwise towards `TWO_PI` (`2*PI`) radians.
    ///
    /// Values < `0.0` and >= `TWO_PI` will be set to `0.0`.
    ///
    /// The default minimum (converted to degrees) is `30` degrees, and the
    /// default maximum is `330` degrees, giving a span of `300` degrees, and
    /// a halfway point pointing strait up.
    ///
    /// # Panics
    ///
    /// This will panic if `min` > `max`.
    pub fn from_rad(min: f32, max: f32) -> Self {
        debug_assert!(min <= max);

        let mut min = min;
        let mut max = max;

        if min < 0.0 || min >= TWO_PI {
            min = 0.0;
        }
        if max < 0.0 || max >= TWO_PI {
            max = 0.0;
        }

        Self { min, max }
    }

    /// returns the minimum angle (between `0.0` and `TWO_PI` in radians)
    pub fn min(&self) -> f32 {
        self.min
    }
    /// returns the maximum angle (between `0.0` and `TWO_PI` in radians)
    pub fn max(&self) -> f32 {
        self.max
    }
}

/// The state of a modulation range
#[derive(Debug, Clone)]
pub struct ModulationRange {
    /// Where the modulation range starts.
    /// `0.0.into()` is all the way minimum, and `1.0.into()` is all the way maximum.
    pub start: Normal,
    /// Where the modulation range ends.
    /// `0.0.into()` is all the way minimum, and `1.0.into()` is all the way maximum.
    pub end: Normal,
    /// Whether the filled portion of the modulation range is visible or not, while keeping
    /// the empty portion visible.
    pub filled_visible: bool,
}

impl ModulationRange {
    /// Creates a new `ModulationRange`
    ///
    /// * start - Where the modulation range starts.
    /// `0.0.into()` is all the way minimum, and `1.0.into()` is all the way maximum.
    /// * ends - Where the modulation range ends.
    /// `0.0.into()` is all the way minimum, and `1.0.into()` is all the way maximum.
    pub fn new(start: Normal, end: Normal) -> Self {
        Self {
            start,
            end,
            filled_visible: true,
        }
    }
}

impl Default for ModulationRange {
    fn default() -> Self {
        Self {
            start: 0.0.into(),
            end: 0.0.into(),
            filled_visible: true,
        }
    }
}
