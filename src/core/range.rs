//! Ranges of parameter values that map to a [`Normal`]
///
/// [`Normal`]: ../struct.Normal.html
use crate::core::Normal;
use crate::core::normal_param::NormalParam;

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

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    ///
    /// * `value` - The inital value of the parameter.
    /// * `default_value` - The default value of the parameter.
    pub fn normal_param(&self, value: f32, default: f32) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(value),
            default: self.map_to_normal(default),
        }
    }

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range where `value` and `default_value` is `0.0`.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    pub fn default_normal_param(&self) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(0.0),
            default: self.map_to_normal(0.0),
        }
    }

    /// Returns the corresponding [`Normal`] from the supplied value
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn map_to_normal(&self, value: f32) -> Normal {
        let value = self.constrain(value);
        Normal::from_clipped((value - self.min) * self.span_recip)
    }

    /// Returns the corresponding value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn unmap_to_value(&self, normal: Normal) -> f32 {
        (normal.as_f32() * self.span) + self.min
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

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    ///
    /// * `value` - The inital value of the parameter.
    /// * `default_value` - The default value of the parameter.
    pub fn normal_param(&self, value: i32, default: i32) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(value),
            default: self.map_to_normal(default),
        }
    }

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range where `value` and `default_value` is `0`.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    pub fn default_normal_param(&self) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(0),
            default: self.map_to_normal(0),
        }
    }

    /// Returns a [`Normal`] that is snapped to the closest integer
    /// value in this range.
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn snapped(&self, normal: Normal) -> Normal {
        let value_int = self.unmap_to_value(normal);
        self.map_to_normal(value_int)
    }

    /// Returns the corresponding [`Normal`] from the supplied value
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn map_to_normal(&self, value: i32) -> Normal {
        let value = self.constrain(value);
        Normal::from_clipped((value - self.min) as f32 * self.span_recip)
    }

    /// Returns the corresponding value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn unmap_to_value(&self, normal: Normal) -> i32 {
        (normal.as_f32() * self.span).round() as i32 + self.min
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
    ///   should be. For example, `Normal::CENTER` will have 0 dB at the center
    ///   of the slider. Normals of `1.0` and `0.0` can be used for only negative
    ///   or only positive decibels respectively
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

        let zero_pos_recip = if zero_position.as_f32() == 0.0 {
            0.0
        } else {
            1.0 / zero_position.as_f32()
        };

        let one_min_zero_pos_recip = if zero_position.as_f32() == 0.0 {
            0.0
        } else {
            1.0 / (1.0 - zero_position.as_f32())
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

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    ///
    /// * `value` - The inital value of the parameter.
    /// * `default_value` - The default value of the parameter.
    pub fn normal_param(&self, value: f32, default: f32) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(value),
            default: self.map_to_normal(default),
        }
    }

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range where `value` and `default_value` is `0.0`.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    pub fn default_normal_param(&self) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(0.0),
            default: self.map_to_normal(0.0),
        }
    }

    /// Returns the corresponding [`Normal`] from the supplied `value`
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn map_to_normal(&self, value: f32) -> Normal {
        let value = self.constrain(value);
        if value == 0.0 {
            self.zero_position
        } else if value < 0.0 {
            if self.min >= 0.0 {
                return Normal::MIN;
            }
            let neg_normal = value * self.min_recip;

            let log_normal = 1.0 - neg_normal.sqrt();

            Normal::from_clipped(log_normal * self.zero_position.as_f32())
        } else {
            if self.max <= 0.0 {
                return Normal::MAX;
            }
            let pos_normal = value * self.max_recip;

            let log_normal = pos_normal.sqrt();

            Normal::from_clipped(
                (log_normal * (1.0 - self.zero_position.as_f32())) + self.zero_position.as_f32(),
            )
        }
    }

    /// Returns the corresponding dB value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn unmap_to_value(&self, normal: Normal) -> f32 {
        if normal == self.zero_position {
            0.0
        } else if normal < self.zero_position {
            if self.min >= 0.0 {
                return self.min;
            }
            let neg_normal = 1.0 - (normal.as_f32() * self.zero_pos_recip);

            let log_normal = 1.0 - (neg_normal * neg_normal);

            (1.0 - log_normal) * self.min
        } else {
            if self.zero_position.as_f32() == 1.0 || self.max <= 0.0 {
                return self.max;
            }
            let pos_normal =
                (normal.as_f32() - self.zero_position.as_f32()) * self.one_min_zero_pos_recip;

            let log_normal = pos_normal * pos_normal;

            log_normal * self.max
        }
    }
}

impl Default for LogDBRange {
    fn default() -> Self {
        LogDBRange::new(-12.0, 12.0, Normal::CENTER)
    }
}

/// A [`NormalParam`] that defines a continuous logarithmic range of `f32` frequency
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
    /// Creates a new `OctaveNormalParam`
    ///
    /// # Arguments
    ///
    /// * `min` - the minimum of the range in Hz (inclusive), will be
    ///   constrained to `20.0 Hz <= min <= 20480.0 Hz`
    /// * `max` - the maximum of the range in Hz (inclusive), will be
    ///   constrained to `20.0 Hz <= max <= 20480.0 Hz`
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

        let min_spectrum_normal = octave_spectrum_map_to_normal(min);
        let max_spectrum_normal = octave_spectrum_map_to_normal(max);

        let spectrum_normal_span = max_spectrum_normal.as_f32() - min_spectrum_normal.as_f32();

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

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    ///
    /// * `value` - The inital value of the parameter.
    /// * `default_value` - The default value of the parameter.
    pub fn normal_param(&self, value: f32, default: f32) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(value),
            default: self.map_to_normal(default),
        }
    }

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range where `value` and `default_value` is `20480.0`.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    pub fn default_normal_param(&self) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(20_480.0),
            default: self.map_to_normal(20_480.0),
        }
    }

    /// Returns the corresponding [`Normal`] from the supplied frequency value
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn map_to_normal(&self, value: f32) -> Normal {
        let value = self.constrain(value);
        let spectrum_normal = octave_spectrum_map_to_normal(value);
        Normal::from_clipped(
            (spectrum_normal.as_f32() - self.min_spectrum_normal.as_f32())
                * self.spectrum_normal_span_recip,
        )
    }

    /// Returns the corresponding frequency value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn unmap_to_value(&self, normal: Normal) -> f32 {
        let spectrum_normal = Normal::from_clipped(
            normal.as_f32() * self.spectrum_normal_span + self.min_spectrum_normal.as_f32(),
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
#[inline]
fn octave_normal_to_spectrum(value: Normal) -> f32 {
    40.0 * 2.0_f32.powf((10.0 * value.as_f32()) - 1.0)
}

/// Returns the corresponding [`Normal`] for a frequency in the whole
/// 10 octave spectrum (between 20 Hz and 20480 Hz)
///
/// [`Normal`]: ../struct.Normal.html
#[inline]
fn octave_spectrum_map_to_normal(freq: f32) -> Normal {
    Normal::from_clipped(((freq / 40.0).log2() + 1.0) * 0.1)
}
