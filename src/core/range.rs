//! Ranges of parameter values that map to a [`Normal`]
use crate::core::param::Param;
///
/// [`Normal`]: ../struct.Normal.html
use crate::core::Normal;

use std::fmt::Debug;

/// A range that maps a continuous linear range of `f32` values
/// to a [`Normal`]
///
/// [`Normal`]: ../struct.Normal.html
#[derive(Debug, Copy, Clone)]
pub struct FloatRange {
    min: f32,
    max: f32,
    span: f32,
    span_recip: f32,
}

impl FloatRange {
    /// Creates a new `FloatRange`
    ///
    /// # Arguments
    ///
    /// * `min` - the minimum of the range (inclusive)
    /// * `max` - the maximum of the range (inclusive)
    ///
    /// # Panics
    ///
    /// This will panic if `max` <= `min`
    pub fn new(min: f32, max: f32) -> Self {
        assert!(max > min);

        let span = max - min;
        let span_recip = span.recip();

        Self {
            min,
            max,
            span,
            span_recip,
        }
    }

    /// A `FloatRange` with the range
    ///
    /// * `min` = -1.0
    /// * `max` = 1.0
    pub fn default_bipolar() -> Self {
        FloatRange::new(-1.0, 1.0)
    }

    fn constrain(&self, value: f32) -> f32 {
        if value <= self.min {
            self.min
        } else if value >= self.max {
            self.max
        } else {
            value
        }
    }

    /// Creates a new [`Param`] with values mapped
    /// from this range.
    ///
    /// [`Param`]: ../param/struct.Param.html
    ///
    /// * `id` - A unique user-defined identifier for the
    /// parameter. This can be an enum, i32, u32, String, etc.
    /// Each parameter must have a unique `ID`
    /// value!
    /// * `value` - The inital value of the parameter.
    /// * `default_value` - The default value of the parameter.
    pub fn create_param<ID: Debug + Copy + Clone>(
        &self,
        id: ID,
        value: f32,
        default_value: f32,
    ) -> Param<ID> {
        Param {
            id,
            normal: self.to_normal(value),
            default_normal: self.to_normal(default_value),
        }
    }

    /// Creates a new [`Param`] with values mapped
    /// from this range where `value` and `default_value` is `0.0`.
    ///
    /// [`Param`]: ../param/struct.Param.html
    ///
    /// * `id` - A unique user-defined identifier for the
    /// parameter. This can be an enum, i32, u32, String, etc.
    /// Each parameter must have a unique `ID`
    /// value!
    pub fn create_param_default<ID: Debug + Copy + Clone>(
        &self,
        id: ID,
    ) -> Param<ID> {
        Param {
            id,
            normal: self.to_normal(0.0),
            default_normal: self.to_normal(0.0),
        }
    }

    /// Returns the corresponding [`Normal`] from the supplied value
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn to_normal(&self, value: f32) -> Normal {
        let value = self.constrain(value);
        ((value - self.min) * self.span_recip).into()
    }

    /// Returns the corresponding value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn to_value(&self, normal: Normal) -> f32 {
        (normal.value() * self.span) + self.min
    }
}

impl Default for FloatRange {
    fn default() -> Self {
        FloatRange::new(0.0, 1.0)
    }
}

/// A range that defines a discrete linear range of i32 values
#[derive(Debug, Copy, Clone)]
pub struct IntRange {
    min: i32,
    max: i32,
    span: f32,
    span_recip: f32,
}

impl IntRange {
    /// Creates a new `IntRange`
    ///
    /// # Arguments
    ///
    /// * `min` - the minimum of the range (inclusive)
    /// * `max` - the maximum of the range (inclusive)
    ///
    /// # Panics
    ///
    /// This will panic if `max` <= `min`
    pub fn new(min: i32, max: i32) -> Self {
        assert!(max > min);

        let span = (max - min) as f32;
        let span_recip = span.recip();

        Self {
            min,
            max,
            span,
            span_recip,
        }
    }

    fn constrain(&self, value: i32) -> i32 {
        if value <= self.min {
            self.min
        } else if value >= self.max {
            self.max
        } else {
            value
        }
    }

    /// Creates a new [`Param`] with values mapped
    /// from this range.
    ///
    /// [`Param`]: ../param/struct.Param.html
    ///
    /// * `id` - A unique user-defined identifier for the
    /// parameter. This can be an enum, i32, u32, String, etc.
    /// Each parameter must have a unique `ID`
    /// value!
    /// * `value` - The inital value of the parameter.
    /// * `default_value` - The default value of the parameter.
    pub fn create_param<ID: Debug + Copy + Clone>(
        &self,
        id: ID,
        value: i32,
        default_value: i32,
    ) -> Param<ID> {
        Param {
            id,
            normal: self.to_normal(value),
            default_normal: self.to_normal(default_value),
        }
    }

    /// Creates a new [`Param`] with values mapped
    /// from this range where `value` and `default_value` is `0`.
    ///
    /// [`Param`]: ../param/struct.Param.html
    ///
    /// * `id` - A unique user-defined identifier for the
    /// parameter. This can be an enum, i32, u32, String, etc.
    /// Each parameter must have a unique `ID`
    /// value!
    pub fn create_param_default<ID: Debug + Copy + Clone>(
        &self,
        id: ID,
    ) -> Param<ID> {
        Param {
            id,
            normal: self.to_normal(0),
            default_normal: self.to_normal(0),
        }
    }

    /// Snaps a [`Normal`] to the closest integer value in this
    /// range.
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn snap_normal(&self, normal: &mut Normal) {
        let value = self.to_value(*normal);
        *normal = self.to_normal(value);
    }

    /// Returns the corresponding [`Normal`] from the supplied value
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn to_normal(&self, value: i32) -> Normal {
        let value = self.constrain(value);
        ((value - self.min) as f32 * self.span_recip).into()
    }

    /// Returns the corresponding value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn to_value(&self, normal: Normal) -> i32 {
        (normal.value() * self.span).round() as i32 + self.min
    }
}

impl Default for IntRange {
    fn default() -> Self {
        IntRange::new(0, 100)
    }
}

/// A range that defines a continuous logarithmic range of `dB` values,
/// with an inflection/stationary point at 0 dB
///
/// Values around 0 dB (positive and negative) will increment slower per
/// slider movement than values farther away from 0 dB.
#[derive(Debug, Copy, Clone)]
pub struct LogDBRange {
    min: f32,
    max: f32,
    zero_position: Normal,
    min_recip: f32,
    max_recip: f32,
    zero_pos_recip: f32,
    one_min_zero_pos_recip: f32,
}

impl LogDBRange {
    /// Creates a new `LogDBRange`
    ///
    /// # Arguments
    ///
    /// * `min` - the minimum of the range in dB (inclusive), must be <= 0.0
    /// * `max` - the maximum of the range in dB (inclusive), must be >= 0.0
    /// * `zero_position` - a normal that defines where on the slider 0 decibels
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
    pub fn new(min: f32, max: f32, zero_position: Normal) -> Self {
        assert!(max > min, "max must be greater than min");
        assert!(max >= 0.0, "max must be 0.0 or positive");
        assert!(min <= 0.0, "min must be 0.0 or negative");

        let min_recip = if min == 0.0 { 0.0 } else { 1.0 / min };

        let max_recip = if max == 0.0 { 0.0 } else { 1.0 / max };

        let zero_pos_recip = if zero_position.value() == 0.0 {
            0.0
        } else {
            1.0 / zero_position.value()
        };

        let one_min_zero_pos_recip = if zero_position.value() == 0.0 {
            0.0
        } else {
            1.0 / (1.0 - zero_position.value())
        };

        Self {
            min,
            max,
            zero_position,
            min_recip,
            max_recip,
            zero_pos_recip,
            one_min_zero_pos_recip,
        }
    }

    fn constrain(&self, value: f32) -> f32 {
        if value <= self.min {
            self.min
        } else if value >= self.max {
            self.max
        } else {
            value
        }
    }

    /// Creates a new [`Param`] with values mapped
    /// from this range.
    ///
    /// [`Param`]: ../param/struct.Param.html
    ///
    /// * `id` - A unique user-defined identifier for the
    /// parameter. This can be an enum, i32, u32, String, etc.
    /// Each parameter must have a unique `ID`
    /// value!
    /// * `value` - The inital value of the parameter.
    /// * `default_value` - The default value of the parameter.
    pub fn create_param<ID: Debug + Copy + Clone>(
        &self,
        id: ID,
        value: f32,
        default_value: f32,
    ) -> Param<ID> {
        Param {
            id,
            normal: self.to_normal(value),
            default_normal: self.to_normal(default_value),
        }
    }

    /// Creates a new [`Param`] with values mapped
    /// from this range where `value` and `default_value` is `0.0`.
    ///
    /// [`Param`]: ../param/struct.Param.html
    ///
    /// * `id` - A unique user-defined identifier for the
    /// parameter. This can be an enum, i32, u32, String, etc.
    /// Each parameter must have a unique `ID`
    /// value!
    pub fn create_param_default<ID: Debug + Copy + Clone>(
        &self,
        id: ID,
    ) -> Param<ID> {
        Param {
            id,
            normal: self.to_normal(0.0),
            default_normal: self.to_normal(0.0),
        }
    }

    /// Returns the corresponding [`Normal`] from the supplied `value`
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn to_normal(&self, value: f32) -> Normal {
        let value = self.constrain(value);
        if value == 0.0 {
            self.zero_position
        } else if value < 0.0 {
            if self.min >= 0.0 {
                return 0.0.into();
            }
            let neg_normal = value * self.min_recip;

            let log_normal = 1.0 - neg_normal.sqrt();

            (log_normal * self.zero_position.value()).into()
        } else {
            if self.max <= 0.0 {
                return 1.0.into();
            }
            let pos_normal = value * self.max_recip;

            let log_normal = pos_normal.sqrt();

            ((log_normal * (1.0 - self.zero_position.value()))
                + self.zero_position.value())
            .into()
        }
    }

    /// Returns the corresponding dB value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn to_value(&self, normal: Normal) -> f32 {
        if normal == self.zero_position {
            0.0
        } else if normal < self.zero_position {
            if self.min >= 0.0 {
                return self.min;
            }
            let neg_normal = 1.0 - (normal.value() * self.zero_pos_recip);

            let log_normal = 1.0 - (neg_normal * neg_normal);

            (1.0 - log_normal) * self.min
        } else {
            if self.zero_position.value() == 1.0 || self.max <= 0.0 {
                return self.max;
            }
            let pos_normal = (normal.value() - self.zero_position.value())
                * self.one_min_zero_pos_recip;

            let log_normal = pos_normal * pos_normal;

            log_normal * self.max
        }
    }
}

impl Default for LogDBRange {
    fn default() -> Self {
        LogDBRange::new(-12.0, 12.0, 0.5.into())
    }
}

/// A [`Param`] that defines a continuous logarithmic range of `f32` frequency
/// values, with each octave in the 10 octave spectrum spaced evenly.
///
/// Smaller frequencies will increment slower per slider movement than larger
/// ones.
#[derive(Debug, Copy, Clone)]
pub struct FreqRange {
    min: f32,
    max: f32,
    spectrum_normal_span: f32,
    spectrum_normal_span_recip: f32,
    min_spectrum_normal: Normal,
}

impl FreqRange {
    /// Creates a new `OctaveParam`
    ///
    /// # Arguments
    ///
    /// * `min` - the minimum of the range in Hz (inclusive), will be
    /// constrained to `20.0 Hz <= min <= 20480.0 Hz`
    /// * `max` - the maximum of the range in Hz (inclusive), will be
    /// constrained to `20.0 Hz <= max <= 20480.0 Hz`
    ///
    /// # Panics
    ///
    /// This will panic if
    /// * `max` <= `min`
    ///
    pub fn new(min: f32, max: f32) -> Self {
        assert!(max > min);

        let mut min = min;
        if min < 20.0 {
            min = 20.0;
        }

        let mut max = max;
        if max > 20480.0 {
            max = 20480.0;
        }

        let min_spectrum_normal = octave_spectrum_to_normal(min);
        let max_spectrum_normal = octave_spectrum_to_normal(max);

        let spectrum_normal_span =
            max_spectrum_normal.value() - min_spectrum_normal.value();

        let spectrum_normal_span_recip = 1.0 / spectrum_normal_span;

        Self {
            min,
            max,
            spectrum_normal_span,
            min_spectrum_normal,
            spectrum_normal_span_recip,
        }
    }

    fn constrain(&self, value: f32) -> f32 {
        if value <= self.min {
            self.min
        } else if value >= self.max {
            self.max
        } else {
            value
        }
    }

    /// Creates a new [`Param`] with values mapped
    /// from this range.
    ///
    /// [`Param`]: ../param/struct.Param.html
    ///
    /// * `id` - A unique user-defined identifier for the
    /// parameter. This can be an enum, i32, u32, String, etc.
    /// Each parameter must have a unique `ID`
    /// value!
    /// * `value` - The inital value of the parameter.
    /// * `default_value` - The default value of the parameter.
    pub fn create_param<ID: Debug + Copy + Clone>(
        &self,
        id: ID,
        value: f32,
        default_value: f32,
    ) -> Param<ID> {
        Param {
            id,
            normal: self.to_normal(value),
            default_normal: self.to_normal(default_value),
        }
    }

    /// Creates a new [`Param`] with values mapped
    /// from this range where `value` and `default_value` is `20480.0`.
    ///
    /// [`Param`]: ../param/struct.Param.html
    ///
    /// * `id` - A unique user-defined identifier for the
    /// parameter. This can be an enum, i32, u32, String, etc.
    /// Each parameter must have a unique `ID`
    /// value!
    pub fn create_param_default<ID: Debug + Copy + Clone>(
        &self,
        id: ID,
    ) -> Param<ID> {
        Param {
            id,
            normal: self.to_normal(20_480.0),
            default_normal: self.to_normal(20_480.0),
        }
    }

    /// Returns the corresponding [`Normal`] from the supplied frequency value
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn to_normal(&self, value: f32) -> Normal {
        let value = self.constrain(value);
        let spectrum_normal = octave_spectrum_to_normal(value);
        ((spectrum_normal.value() - self.min_spectrum_normal.value())
            * self.spectrum_normal_span_recip)
            .into()
    }

    /// Returns the corresponding frequency value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn to_value(&self, normal: Normal) -> f32 {
        let spectrum_normal = Normal::new(
            normal.value() * self.spectrum_normal_span
                + self.min_spectrum_normal.value(),
        );

        octave_normal_to_spectrum(spectrum_normal)
    }
}

impl Default for FreqRange {
    fn default() -> Self {
        FreqRange::new(20.0, 20_000.0)
    }
}

/// Returns the corresponding frequency for the whole 10 octave spectrum
/// (between 20 Hz and 20480 Hz)
fn octave_normal_to_spectrum(normal: Normal) -> f32 {
    40.0 * 2.0_f32.powf((10.0 * normal.value()) - 1.0)
}

/// Returns the corresponding [`Normal`] for a frequency in the whole
/// 10 octave spectrum (between 20 Hz and 20480 Hz)
///
/// [`Normal`]: ../struct.Normal.html
fn octave_spectrum_to_normal(freq: f32) -> Normal {
    (((freq / 40.0).log2() + 1.0) * 0.1).into()
}
