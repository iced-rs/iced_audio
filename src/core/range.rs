//! Ranges of parameter values that map to a [`Normal`]
///
/// [`Normal`]: ../struct.Normal.html
use crate::core::Normal;
use crate::core::param::NormalParam;

use std::fmt::Debug;

/// A range that maps a continuous linear range of `f32` values
/// to a [`Normal`]
///
/// [`Normal`]: ../struct.Normal.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FloatRange {
    min: f32,
    max: f32,
}

impl FloatRange {
    /// A float range of `[0.0..=1.0]`
    pub const NORMAL: Self = Self { min: 0.0, max: 1.0 };
    /// A float range of `[-1.0..=1.0]`
    pub const NORMAL_BIPOLAR: Self = Self {
        min: -1.0,
        max: 1.0,
    };

    /// Creates a new `FloatRange`
    ///
    /// # Arguments
    ///
    /// * `min` - the minimum of the range (inclusive)
    /// * `max` - the maximum of the range (inclusive)
    ///
    /// # Panics
    ///
    /// This will panic if `max` < `min`
    pub const fn new(min: f32, max: f32) -> Self {
        assert!(max >= min);

        Self { min, max }
    }

    pub const fn min(&self) -> f32 {
        self.min
    }

    pub const fn max(&self) -> f32 {
        self.max
    }

    pub fn span(&self) -> f32 {
        self.max - self.min
    }

    pub fn clamp(&self, value: f32) -> f32 {
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
    pub fn param(&self, value: f32, default: f32) -> NormalParam {
        NormalParam {
            normal: self.map(value),
            default: self.map(default),
            ..Default::default()
        }
    }

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range where `value` and `default_value` is `0.0`.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    pub fn default_param(&self) -> NormalParam {
        NormalParam {
            normal: self.map(0.0),
            default: self.map(0.0),
            ..Default::default()
        }
    }

    /// Returns the corresponding [`Normal`] from the supplied value
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn map(&self, value: f32) -> Normal {
        let span = self.span();
        if span == 0.0 {
            Normal::MIN
        } else {
            let value = self.clamp(value);
            Normal::new((value - self.min) / self.span())
        }
    }

    /// Returns the corresponding value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn unmap_to_value(&self, normal: Normal) -> f32 {
        (normal.as_f32() * self.span()) + self.min
    }
}

impl Default for FloatRange {
    fn default() -> Self {
        FloatRange::new(0.0, 1.0)
    }
}

/// A range that defines a discrete linear range of i32 values
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IntRange {
    min: i32,
    max: i32,
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
    /// This will panic if `max` < `min`
    pub const fn new(min: i32, max: i32) -> Self {
        assert!(max >= min);

        Self { min, max }
    }

    pub const fn min(&self) -> i32 {
        self.min
    }

    pub const fn max(&self) -> i32 {
        self.max
    }

    pub const fn span(&self) -> i32 {
        self.max - self.min
    }

    pub const fn span_f32(&self) -> f32 {
        (self.max - self.min) as f32
    }

    pub fn clamp(&self, value: i32) -> i32 {
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
    pub fn param(&self, value: i32, default: i32) -> NormalParam {
        NormalParam {
            normal: self.map(value),
            default: self.map(default),
            ..Default::default()
        }
    }

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range where `value` and `default_value` is `0`.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    pub fn default_param(&self) -> NormalParam {
        NormalParam {
            normal: self.map(0),
            default: self.map(0),
            ..Default::default()
        }
    }

    /// Returns a [`Normal`] that is snapped to the closest integer
    /// value in this range.
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn snap(&self, normal: Normal) -> Normal {
        let value_int = self.unmap_to_value(normal);
        self.map(value_int)
    }

    /// Returns the corresponding [`Normal`] from the supplied value
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn map(&self, value: i32) -> Normal {
        let span = self.span_f32();
        if span == 0.0 {
            Normal::MIN
        } else {
            let value = self.clamp(value);
            Normal::new((value - self.min) as f32 / self.span_f32())
        }
    }

    /// Returns the corresponding value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn unmap_to_value(&self, normal: Normal) -> i32 {
        (normal.as_f32() * self.span_f32()).round() as i32 + self.min
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DBRange {
    min: f32,
    max: f32,
    skew_factor: f32,
    zero_position: Normal,
}

impl DBRange {
    pub const DEFAULT_SKEW_FACTOR: f32 = 1.38;

    pub const NEG_3_TO_3: Self = Self::new(-6.0, 6.0, Normal::CENTER, Self::DEFAULT_SKEW_FACTOR);
    pub const NEG_6_TO_6: Self = Self::new(-6.0, 6.0, Normal::CENTER, Self::DEFAULT_SKEW_FACTOR);
    pub const NEG_12_TO_12: Self =
        Self::new(-12.0, 12.0, Normal::CENTER, Self::DEFAULT_SKEW_FACTOR);
    pub const NEG_24_TO_24: Self =
        Self::new(-24.0, 24.0, Normal::CENTER, Self::DEFAULT_SKEW_FACTOR);

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
    /// * `skew_factor` - The exponent of the parameter curve. Larger values are
    ///   more skewed towards the zero position. Must be greater than or equal to `1.0`
    ///   A good default value is [`DBRange::DEFAULT_SKEW_FACTOR`] (1.38).
    ///
    /// # Panics
    ///
    /// This will panic if
    /// * `max < min`
    /// * `min > 0.0`
    /// * `max < 0.0`
    /// * `skew_factor < 1.0`
    ///
    pub const fn new(min: f32, max: f32, zero_position: Normal, skew_factor: f32) -> Self {
        assert!(max >= min, "max must be >= min");
        assert!(max >= 0.0, "max must be >= 0.0");
        assert!(min <= 0.0, "min must be <= 0.0");
        assert!(skew_factor >= 1.0, "skew_factor must be >= 1.0");

        Self {
            min,
            max,
            skew_factor,
            zero_position,
        }
    }

    pub fn clamp(&self, value: f32) -> f32 {
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
    pub fn param(&self, value: f32, default: f32) -> NormalParam {
        NormalParam {
            normal: self.map_db(value),
            default: self.map_db(default),
            ..Default::default()
        }
    }

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range where `value` and `default_value` is `0.0`.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    pub fn default_param(&self) -> NormalParam {
        NormalParam {
            normal: self.map_db(0.0),
            default: self.map_db(0.0),
            ..Default::default()
        }
    }

    /// Returns the corresponding [`Normal`] from the supplied `value`
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn map_db(&self, value: f32) -> Normal {
        let value = self.clamp(value);
        if value == 0.0 || self.min == self.max {
            self.zero_position
        } else if value < 0.0 {
            if self.min >= 0.0 {
                return Normal::MIN;
            }

            let min_recip = if self.min == 0.0 {
                0.0
            } else {
                self.min.recip()
            };

            let neg_normal = value * min_recip;

            let log_normal = 1.0 - neg_normal.abs().powf(self.skew_factor.recip());

            Normal::new(log_normal * self.zero_position.as_f32())
        } else {
            if self.max <= 0.0 {
                return Normal::MAX;
            }

            let max_recip = if self.max == 0.0 {
                0.0
            } else {
                self.max.recip()
            };

            let pos_normal = value * max_recip;

            let log_normal = pos_normal.abs().powf(self.skew_factor.recip());

            Normal::new(
                (log_normal * (1.0 - self.zero_position.as_f32())) + self.zero_position.as_f32(),
            )
        }
    }

    /// Returns the corresponding dB value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn unmap_to_db(&self, normal: Normal) -> f32 {
        if normal == self.zero_position || self.min == self.max {
            0.0
        } else if normal < self.zero_position {
            if self.min >= 0.0 {
                return self.min;
            }

            let zero_pos_recip = if self.zero_position.as_f32() == 0.0 {
                0.0
            } else {
                1.0 / self.zero_position.as_f32()
            };

            let neg_normal = 1.0 - (normal.as_f32() * zero_pos_recip);

            let log_normal = 1.0 - neg_normal.abs().powf(self.skew_factor);

            (1.0 - log_normal) * self.min
        } else {
            if self.zero_position.as_f32() == 1.0 || self.max <= 0.0 {
                return self.max;
            }

            let one_min_zero_pos_recip = if self.zero_position.as_f32() == 0.0 {
                0.0
            } else {
                1.0 / (1.0 - self.zero_position.as_f32())
            };

            let pos_normal =
                (normal.as_f32() - self.zero_position.as_f32()) * one_min_zero_pos_recip;

            let log_normal = pos_normal.abs().powf(self.skew_factor);

            log_normal * self.max
        }
    }
}

impl Default for DBRange {
    fn default() -> Self {
        DBRange::NEG_12_TO_12
    }
}

/// A [`NormalParam`] that defines a continuous logarithmic range of `f32` frequency
/// values, with each octave in the 10 octave spectrum spaced evenly.
///
/// Smaller frequencies will increment slower per slider movement than larger
/// ones.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FreqRange {
    min: f32,
    max: f32,
}

impl FreqRange {
    pub const HZ_20_TO_20K: Self = Self::new(20.0, 20_000.0);

    /// Creates a new `OctaveNormalParam`
    ///
    /// # Arguments
    ///
    /// * `min` - the minimum of the range in Hz (inclusive). Must be greater
    ///   than 0.0.
    /// * `max` - the maximum of the range in Hz (inclusive). Must be greater
    ///   than 0.0.
    /// # Panics
    ///
    /// This will panic if
    /// * `max < min`
    /// * `min <= 0.0`
    /// * `max <= 0.0`
    pub const fn new(min: f32, max: f32) -> Self {
        assert!(max >= min);
        assert!(min > 0.0);
        assert!(max > 0.0);

        Self { min, max }
    }

    pub fn clamp(&self, value: f32) -> f32 {
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
    pub fn param(&self, value: f32, default: f32) -> NormalParam {
        NormalParam {
            normal: self.map_freq(value),
            default: self.map_freq(default),
            ..Default::default()
        }
    }

    /// Creates a new [`NormalParam`] with values mapped
    /// from this range where `value` and `default_value` is `20480.0`.
    ///
    /// [`NormalParam`]: ../normal_param/struct.NormalParam.html
    pub fn default_param(&self) -> NormalParam {
        NormalParam {
            normal: self.map_freq(20_480.0),
            default: self.map_freq(20_480.0),
            ..Default::default()
        }
    }

    /// Returns the corresponding [`Normal`] from the supplied frequency value
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn map_freq(&self, value: f32) -> Normal {
        if self.min == self.max {
            Normal::MIN
        } else {
            let value = self.clamp(value);

            let spectrum_normal = octave_spectrum_map_to_normal(value);
            let min_spectrum_normal = octave_spectrum_map_to_normal(self.min);
            let max_spectrum_normal = octave_spectrum_map_to_normal(self.max);

            let spectrum_normal_span = max_spectrum_normal.as_f32() - min_spectrum_normal.as_f32();

            Normal::new(
                (spectrum_normal.as_f32() - min_spectrum_normal.as_f32()) / spectrum_normal_span,
            )
        }
    }

    /// Returns the corresponding frequency value from the supplied [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub fn unmap_to_freq(&self, normal: Normal) -> f32 {
        if self.min == self.max {
            self.min
        } else {
            let min_spectrum_normal = octave_spectrum_map_to_normal(self.min);
            let max_spectrum_normal = octave_spectrum_map_to_normal(self.max);

            let spectrum_normal_span = max_spectrum_normal.as_f32() - min_spectrum_normal.as_f32();

            let spectrum_normal =
                Normal::new(normal.as_f32() * spectrum_normal_span + min_spectrum_normal.as_f32());

            octave_normal_to_spectrum(spectrum_normal)
        }
    }
}

impl Default for FreqRange {
    fn default() -> Self {
        FreqRange::HZ_20_TO_20K
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
    Normal::new(((freq / 40.0).log2() + 1.0) * 0.1)
}
