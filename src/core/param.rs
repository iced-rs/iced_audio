use crate::core::Normal;

use std::f32;

/// A paramater that maps a range of values to a [`Normal`], which is used by
/// GUI widgets.
/// It also contains a [`Normal`] for the default value of the parameter.
///
/// [`Normal`]: struct.Normal.html
pub trait Param {
    /// returns the unique identifier of the parameter
    fn id(&self) -> u32;
    /// returns the value of the parameter represented as a [`Normal`]
    ///
    /// [`Normal`]: struct.Normal.html
    fn normal(&self) -> Normal;
    /// returns the default value of the parameter represented as a [`Normal`]
    ///
    /// [`Normal`]: struct.Normal.html
    fn default_normal(&self) -> Normal;
}

/// A [`Param`] that defines a continuous linear range of `f32` values
///
/// [`Param`]: struct.Param.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FloatParam {
    id: u32,
    value: f32,
    default_value: f32,
    normal: Normal,
    default_normal: Normal,
    min: f32,
    max: f32,
    range: f32,
    range_recip: f32,
}

impl FloatParam {
    /// Creates a new `FloatParam`
    /// 
    /// # Arguments
    ///
    /// * `id` - an identifier for the parameter (must be unique!)
    /// * `min` - the minimum of the range (inclusive)
    /// * `max` - the maximum of the range (inclusive)
    /// * `value` - the initial value of the parameter (if `value` falls outside
    /// the range given by `min` and `max`, then `min` or `max` will be used
    /// instead)
    /// * `default_value` - the default value of the parameter (if
    /// `default_value` falls outside the range given by `min` and `max`, then
    /// `min` or `max` will be used instead)
    ///
    /// # Panics
    ///
    /// This will panic if `max` <= `min`
    pub fn new(id: u32, min: f32, max: f32, value: f32, default_value: f32)
    -> Self {
        assert!(max > min);

        let range = max - min;
        let range_recip = range.recip();

        let mut new_self = Self {
            id,
            value,
            default_value,
            normal: Normal::default(),
            default_normal: Normal::default(),
            min,
            max,
            range,
            range_recip,
        };

        new_self.value = new_self.constrain(value);
        new_self.default_value = new_self.constrain(default_value);

        new_self.normal = new_self.value_to_normal(new_self.value);
        new_self.default_normal = new_self.value_to_normal(
                                    new_self.default_value);

        new_self
    }

    /// Sets the parameter's value to `value`
    ///
    /// If `value` falls outside the range given by `min` and `max` from
    /// `FloatParam::new()`, then
    /// `min` or `max` will be used instead.
    /// 
    /// Value to [`Normal`] calculactions will not be recalculated if the value
    /// has not changed.
    ///
    /// [`Normal`]: struct.Normal.html
    pub fn set_from_value(&mut self, value: f32) {
        if self.value != value {
            self.value = self.constrain(value);
            self.normal = self.value_to_normal(self.value);
        }
    }

    /// Sets the parameter's value from a [`Normal`]
    /// 
    /// [`Normal`] to value calculactions will not be recalculated if the Normal
    /// has not changed.
    ///
    /// [`Normal`]: struct.Normal.html
    pub fn set_from_normal(&mut self, normal: Normal) {
        if self.normal != normal {
            self.value = self.normal_to_value(normal);
            self.normal = normal;
        }
    }

    /// Returns the parameter's value
    pub fn value(&self) -> f32 { self.value }
    /// Returns the parameter's default value
    pub fn default_value(&self) -> f32 { self.default_value }

    fn constrain(&self, value: f32) -> f32 {
        if value <= self.min { self.min }
        else if value >= self.max { self.max }
        else { value }
    }

    fn value_to_normal(&self, value: f32) -> Normal {
        ((value - self.min) * self.range_recip).into()
    }

    fn normal_to_value(&self, normal: Normal) -> f32 {
        let value = (normal.value() * self.range) + self.min;
        if value > self.default_value - 0.001 &&
            value < self.default_value + 0.001 {
            self.default_value
        } else { value }
    }
}

impl Param for FloatParam {
    fn id(&self) -> u32 { self.id }
    fn normal(&self) -> Normal { self.normal }
    fn default_normal(&self) -> Normal { self.default_normal }
}


/// A parameter that defines a discrete linear range of i32 values
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IntParam {
    id: u32,
    value: i32,
    default_value: i32,
    normal: Normal,
    default_normal: Normal,
    min: i32,
    max: i32,
    range: f32,
    range_recip: f32,
}

impl IntParam {
    /// Creates a new `IntParam`
    /// 
    /// # Arguments
    ///
    /// * `id` - an identifier for the parameter (must be unique!)
    /// * `min` - the minimum of the range (inclusive)
    /// * `max` - the maximum of the range (inclusive)
    /// * `value` - the initial value of the parameter (if `value` falls outside
    /// the range given by `min` and `max`, then `min` or `max` will be used
    /// instead)
    /// * `default_value` - the default value of the parameter (if
    /// `default_value` falls outside the range given by `min` and `max`, then
    /// `min` or `max` will be used instead)
    ///
    /// # Panics
    ///
    /// This will panic if `max` <= `min`
    pub fn new(id: u32, min: i32, max: i32, value: i32, default_value: i32)
    -> Self {
        assert!(max > min);

        let range = (max - min) as f32;
        let range_recip = range.recip();

        let mut new_self = Self {
            id,
            value,
            default_value,
            normal: Normal::default(),
            default_normal: Normal::default(),
            min,
            max,
            range,
            range_recip,
        };

        new_self.value = new_self.constrain(value);
        new_self.default_value = new_self.constrain(default_value);

        new_self.normal = new_self.value_to_normal(new_self.value);
        new_self.default_normal = new_self.value_to_normal(
                                    new_self.default_value);

        new_self
    }

    /// Sets the parameter's value to `value`
    ///
    /// If `value` falls outside the range given by `min` and `max` from
    /// `IntParam::new()`, then
    /// `min` or `max` will be used instead.
    /// 
    /// Value to [`Normal`] calculactions will not be recalculated if the value
    /// has not changed.
    ///
    /// [`Normal`]: struct.Normal.html
    pub fn set_from_value(&mut self, value: i32) {
        if self.value != value {
            self.value = self.constrain(value);
            self.normal = self.value_to_normal(self.value);
        }
    }

    /// Sets the parameter's value from a [`Normal`]
    /// 
    /// The parameter's resulting [`Normal`] will be calculated from the
    /// resulting parameter's value rounded to the nearest integer.
    ///
    /// # Example
    ///
    /// ```
    /// use iced_audio::{Param, IntParam, Normal};
    ///
    /// // id: 0, min: 0, max: 2, value: 1, default_value: 1
    /// let mut param = IntParam::new(0, 0, 2, 1, 1);
    /// param.set_from_normal((0.8).into());
    ///
    /// assert_eq!(param.value(), 2);
    /// assert_eq!(param.normal().value(), 1.0);
    /// ```
    ///
    /// [`Normal`]: struct.Normal.html
    pub fn set_from_normal(&mut self, normal: Normal) {
        if self.normal != normal {
            self.value = self.constrain(self.normal_to_value(normal));
            self.normal = self.value_to_normal(self.value);
        }
    }

    /// Returns the parameter's value
    pub fn value(&self) -> i32 { self.value }
    /// Returns the parameter's default value
    pub fn default_value(&self) -> i32 { self.default_value }

    fn constrain(&self, value: i32) -> i32 {
        if value <= self.min { self.min }
        else if value >= self.max { self.max }
        else { value }
    }

    fn value_to_normal(&self, value: i32) -> Normal {
        ((value - self.min) as f32 * self.range_recip).into()
    }

    fn normal_to_value(&self, normal: Normal) -> i32 {
        (normal.value() * self.range).round() as i32 + self.min
    }
}

impl Param for IntParam {
    fn id(&self) -> u32 { self.id }
    fn normal(&self) -> Normal { self.normal }
    fn default_normal(&self) -> Normal { self.default_normal }
}


/// A [`Param`] that defines a continuous logarithmic range of `dB` values,
/// with an inflection/stationary point at 0 dB
///
/// Values around 0 dB (positive and negative) will increment slower per
/// slider movement than values farther away from 0 dB.
///
/// [`Param`]: struct.Param.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LogDBParam {
    id: u32,
    value: f32,
    default_value: f32,
    normal: Normal,
    default_normal: Normal,
    zero_normal: Normal,
    min: f32,
    max: f32,
    range: f32,
    range_recip: f32,
}

impl LogDBParam {
    /// Creates a new `LogDBParam`
    /// 
    /// # Arguments
    ///
    /// * `id` - an identifier for the parameter (must be unique!)
    /// * `min` - the minimum of the range in dB (inclusive), must be <= 0.0
    /// * `max` - the maximum of the range in dB (inclusive), must be >= 0.0
    /// * `value` - the initial value of the parameter in dB (if `value` falls
    /// outside the range given by `min` and `max`, then `min` or `max` will be
    /// used instead)
    /// * `default_value` - the default value of the parameter in dB (if
    /// `default_value` falls outside the range given by `min` and `max`, then
    /// `min` or `max` will be used instead)
    /// * `zero_normal` - a normal that defines where on the slider 0 decibels
    /// should be. For example, `Normal::new(0.5)` will have 0 dB at the center
    /// of the slider. Normals of `1.0` and `0.0` can be used for only negative
    /// or only positive decibels respectively
    ///
    /// # Panics
    ///
    /// This will panic if
    /// * `max` <= `min`
    /// * `min` > `0.0`
    /// * `max` < `0.0`
    ///
    pub fn new(id: u32, min: f32, max: f32, value: f32, default_value: f32,
    zero_normal: Normal)
    -> Self {
        assert!(max > min, "max must be greater than min");
        assert!(max >= 0.0, "max must be 0.0 or positive");
        assert!(min <= 0.0, "min must be 0.0 or negative");

        let range = max - min;
        let range_recip = range.recip();

        let mut new_self = Self {
            id,
            value,
            default_value,
            normal: Normal::default(),
            default_normal: Normal::default(),
            zero_normal,
            min,
            max,
            range,
            range_recip,
        };

        new_self.value = new_self.constrain(value);
        new_self.default_value = new_self.constrain(default_value);

        new_self.normal = new_self.value_to_normal(new_self.value);
        new_self.default_normal = new_self.value_to_normal(
                                    new_self.default_value);

        new_self
    }

    /// Sets the parameter's value to `value`
    ///
    /// If `value` falls outside the range given by `min` and `max` from
    /// `FloatParam::new()`, then
    /// `min` or `max` will be used instead.
    /// 
    /// Value to [`Normal`] calculactions will not be recalculated if the value
    /// has not changed.
    ///
    /// [`Normal`]: struct.Normal.html
    pub fn set_from_value(&mut self, value: f32) {
        if self.value != value {
            self.value = self.constrain(value);
            self.normal = self.value_to_normal(self.value);
        }
    }

    /// Sets the parameter's value from a [`Normal`]
    /// 
    /// [`Normal`] to value calculactions will not be recalculated if the Normal
    /// has not changed.
    ///
    /// [`Normal`]: struct.Normal.html
    pub fn set_from_normal(&mut self, normal: Normal) {
        if self.normal != normal {
            self.value = self.normal_to_value(normal);
            self.normal = normal;
        }
    }

    /// Returns the parameter's value
    pub fn value(&self) -> f32 { self.value }
    /// Returns the parameter's default value
    pub fn default_value(&self) -> f32 { self.default_value }

    fn constrain(&self, value: f32) -> f32 {
        if value <= self.min { self.min }
        else if value >= self.max { self.max }
        else { value }
    }

    fn value_to_normal(&self, value: f32) -> Normal {
        if value == 0.0 { self.zero_normal }
        else if value < 0.0 {
            if self.min >= 0.0 { return 0.0.into(); }
            let neg_normal = value / self.min;

            let log_normal = 1.0 - (1.0 - neg_normal).sqrt();

            (log_normal * self.zero_normal.value()).into()
        }
        else {
            if self.max <= 0.0 { return 1.0.into(); }
            let pos_normal = value / self.max;

            let log_normal = pos_normal.sqrt();

            ( (log_normal * (1.0 - self.zero_normal.value()))
                + self.zero_normal.value()).into()
        }
    }

    fn normal_to_value(&self, normal: Normal) -> f32 {
        if normal == self.zero_normal { 0.0 }
        else if normal < self.zero_normal {
            if self.min >= 0.0 { return self.min; }
            let neg_normal = normal.value() / self.zero_normal.value();

            let log_normal = 1.0 - (1.0 - neg_normal).powi(2);

            (1.0 - log_normal) * self.min
        }
        else {
            if self.zero_normal.value() == 1.0 || self.max <= 0.0 {
                return self.max;
            }
            let pos_normal = (normal.value() - self.zero_normal.value())
                                    / (1.0 - self.zero_normal.value());

            let log_normal = pos_normal.powi(2);

            log_normal * self.max
        }
    }
}

impl Param for LogDBParam {
    fn id(&self) -> u32 { self.id }
    fn normal(&self) -> Normal { self.normal }
    fn default_normal(&self) -> Normal { self.default_normal }
}


/// A [`Param`] that defines a continuous logarithmic range of `f32` frequency
/// values, with each octave in the 10 octave spectrum spaced evenly.
///
/// Smaller frequencies will increment slower per slider movement than larger
/// ones.
///
/// [`Param`]: struct.Param.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OctaveParam {
    id: u32,
    value: f32,
    default_value: f32,
    normal: Normal,
    default_normal: Normal,
    min_spectrum_normal: Normal,
    spectrum_normal_range: f32,
    min: f32,
    max: f32,
    range: f32,
    range_recip: f32,

}

impl OctaveParam {
    /// Creates a new `OctaveParam`
    /// 
    /// # Arguments
    ///
    /// * `id` - an identifier for the parameter (must be unique!)
    /// * `min` - the minimum of the range in Hz (inclusive), will be
    /// constrained to `20.0 Hz <= min <= 20480.0 Hz`
    /// * `max` - the maximum of the range in Hz (inclusive), will be
    /// constrained to `20.0 Hz <= max <= 20480.0 Hz`
    /// * `value` - the initial value of the parameter in Hz (if `value` falls
    /// outside the range given by `min` and `max`, then `min` or `max` will be
    /// used instead)
    /// * `default_value` - the default value of the parameter in Hz (if
    /// `default_value` falls outside the range given by `min` and `max`, then
    /// `min` or `max` will be used instead)
    ///
    /// # Panics
    ///
    /// This will panic if
    /// * `max` <= `min`
    ///
    pub fn new(id: u32, min: f32, max: f32, value: f32, default_value: f32)
    -> Self {
        assert!(max > min, "max must be greater than min");

        let mut min = min;
        if min < 20.0 { min = 20.0; }

        let mut max = max;
        if max > 20480.0 { max = 20480.0; }

        let range = max - min;
        let range_recip = range.recip();

        let min_spectrum_normal = OctaveParam::spectrum_to_normal(min);
        let max_spectrum_normal = OctaveParam::spectrum_to_normal(max);

        let spectrum_normal_range = max_spectrum_normal.value() -
                                        min_spectrum_normal.value();

        let mut new_self = Self {
            id,
            value,
            default_value,
            normal: Normal::default(),
            default_normal: Normal::default(),
            min_spectrum_normal,
            spectrum_normal_range,
            min,
            max,
            range,
            range_recip,
        };

        new_self.value = new_self.constrain(value);
        new_self.default_value = new_self.constrain(default_value);

        new_self.normal = new_self.value_to_normal(new_self.value);
        new_self.default_normal = new_self.value_to_normal(
                                    new_self.default_value);

        new_self
    }

    /// Sets the parameter's value to `value`
    ///
    /// If `value` falls outside the range given by `min` and `max` from
    /// `FloatParam::new()`, then
    /// `min` or `max` will be used instead.
    /// 
    /// Value to [`Normal`] calculactions will not be recalculated if the value
    /// has not changed.
    ///
    /// [`Normal`]: struct.Normal.html
    pub fn set_from_value(&mut self, value: f32) {
        if self.value != value {
            self.value = self.constrain(value);
            self.normal = self.value_to_normal(self.value);
        }
    }

    /// Sets the parameter's value from a [`Normal`]
    /// 
    /// [`Normal`] to value calculactions will not be recalculated if the Normal
    /// has not changed.
    ///
    /// [`Normal`]: struct.Normal.html
    pub fn set_from_normal(&mut self, normal: Normal) {
        if self.normal != normal {
            self.value = self.normal_to_value(normal);
            self.normal = normal;
        }
    }

    /// Returns the parameter's value
    pub fn value(&self) -> f32 { self.value }
    /// Returns the parameter's default value
    pub fn default_value(&self) -> f32 { self.default_value }

    fn constrain(&self, value: f32) -> f32 {
        if value <= self.min { self.min }
        else if value >= self.max { self.max }
        else { value }
    }

    fn value_to_normal(&self, value: f32) -> Normal {
        let spectrum_normal = OctaveParam::spectrum_to_normal(value);
        ( (spectrum_normal.value() - self.min_spectrum_normal.value())
             / self.spectrum_normal_range ).into()
    }

    fn normal_to_value(&self, normal: Normal) -> f32 {
        let spectrum_normal = Normal::new(
                                normal.value() * self.spectrum_normal_range
                                + self.min_spectrum_normal.value()
        );

        OctaveParam::normal_to_spectrum(spectrum_normal)
    }

    /// Returns the corresponding frequency for the whole 10 octave spectrum
    /// (between 20 Hz and 20480 Hz)
    fn normal_to_spectrum(normal: Normal) -> f32 {
        40.0 * 2.0_f32.powf((10.0 * normal.value()) - 1.0)
    }

    /// Returns the corresponding [`Normal`] for a frequency in the whole
    /// 10 octave spectrum (between 20 Hz and 20480 Hz)
    ///
    /// [`Normal`]: struct.Normal.html
    fn spectrum_to_normal(freq: f32) -> Normal {
        (((freq / 40.0).log2() + 1.0) * 0.1).into()
    }
}

impl Param for OctaveParam {
    fn id(&self) -> u32 { self.id }
    fn normal(&self) -> Normal { self.normal }
    fn default_normal(&self) -> Normal { self.default_normal }
}