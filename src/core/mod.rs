pub mod param;

pub use param::*;

/// An `f32` value that is gauranteed to be constrained to the range of
///
/// `0.0 >= value >= 1.0`
///
/// # Example
///
/// ```
/// use iced_audio::Normal;
///
/// let mut normal = Normal::new(-1.0);
/// assert_eq!(normal.value(), 0.0);
///
/// normal.set(3.0);
/// assert_eq!(normal.value(), 1.0);
///
/// normal.set(0.5);
/// assert_eq!(normal.value(), 0.5);
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
                if value < 0.0 { 0.0 }
                else if value > 1.0 { 1.0 }
                else { value }
            }
        }
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
    pub fn set(&mut self, value: f32) {
        self.value = {
            if value < 0.0 { 0.0 }
            else if value > 1.0 { 1.0 }
            else { value }
        }
    }

    /// Returns the value of the `Normal`
    pub fn value(&self) -> f32 {
        self.value
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

/// The texture padding around a bounding rectangle. This is useful when the
/// texture is larger than the intended bounds of the widget, such as a glowing
/// button texture or a slider with a drop shadow, etc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TexturePadding {
    pub top: u16,
    pub bottom: u16,
    pub left: u16,
    pub right: u16,
}

impl Default for TexturePadding {
    fn default() -> Self {
        Self {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        }
    }
}

impl TexturePadding {
    /// Creates a new `TexturePadding` with `top`, `bottom`, `left`, and `right`
    /// all set to `padding`.
    pub fn from_single(padding: u16) -> Self {
        Self {
            top: padding,
            bottom: padding,
            left: padding,
            right: padding,
        }
    }

    /// Creates a new `TexturePadding`
    ///
    /// # Arguments
    /// * `vertical_pad` - padding for `top` and `bottom`
    /// * `horizontal_pad` - padding for `left` and `right`
    pub fn from_v_h(vertical_pad: u16, horizontal_pad: u16) -> Self {
        Self {
            top: vertical_pad,
            bottom: vertical_pad,
            left: horizontal_pad,
            right: horizontal_pad,
        }
    }
}


static PI_OVER_180: f32 = std::f32::consts::PI / 180.0;
pub static TAU: f32 = std::f32::consts::PI * 2.0;
pub static DEFAULT_ANGLE_MIN: f32 = 30.0 * PI_OVER_180;
pub static DEFAULT_ANGLE_MAX: f32 = (360.0 - 30.0) * PI_OVER_180;

/// The range between the minimum and maximum angle (in radians) the knob
/// will rotate.
///
/// `0.0` radians points straight down at the bottom of the knob, with the
/// angles rotating clockwise towards `TAU` (`2*PI`).
///
/// Values < `0.0` and >= `TAU` are not allowed.
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
    /// angles rotating clockwise towards `TAU` (`2*PI`) radians.
    ///
    /// Values < `0.0` and >= `TAU` will be set to `0.0`.
    ///
    /// The default minimum (converted to degrees) is `30` degrees, and the
    /// default maximum is `330` degrees, giving a span of `300` degrees, and
    /// a halfway point pointing strait up.
    ///
    /// # Panics
    ///
    /// This will panic if `min` > `max`.
    pub fn from_rad(min: f32, max: f32) -> Self {
        assert!(min <= max);

        let mut min = min;
        let mut max = max;

        if min < 0.0 || min >= TAU { min = 0.0; }
        if max < 0.0 || max >= TAU { max = 0.0; }

        Self { min, max }
    }

    /// returns the minimum angle (between `0.0` and `TAU` in radians)
    pub fn min(&self) -> f32 { self.min }
    /// returns the maximum angle (between `0.0` and `TAU` in radians)
    pub fn max(&self) -> f32 { self.max }
}