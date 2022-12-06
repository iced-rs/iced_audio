//! The range between the minimum and maximum angle (in radians) a knob
//! will rotate.

use super::math::{PI_OVER_180, TWO_PI};

/// The default minimum angle of a rotating widget such as a Knob
pub const DEFAULT_ANGLE_MIN: f32 = 30.0 * PI_OVER_180;
/// The default maximum angle of a rotating widget such as a Knob
pub const DEFAULT_ANGLE_MAX: f32 = (360.0 - 30.0) * PI_OVER_180;

/// The range between the minimum and maximum angle (in radians) a knob
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
    /// The range between the `min` and `max` angle (in degrees) a knob
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

        if !(0.0..TWO_PI).contains(&min) {
            min = 0.0;
        }
        if !(0.0..TWO_PI).contains(&max) {
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
