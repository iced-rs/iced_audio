//! Module for the [`NormalParam`] struct
//!
//! [`NormalParam`]: struct.NormalParam.html

use crate::core::Normal;

#[cfg(feature = "nice-plug")]
use crate::Gesture;

use std::fmt::Debug;

/// A paramater that contains a normalized `value` and a `default_value`.
///
/// The values are stored as the [`Normal`] type.
///
/// [`Normal`]: ../struct.Normal.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NormalParam {
    /// The value of the parameter represented as a [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub normal: Normal,

    /// The default value of the parameter represented as a [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub default: Normal,

    /// An optional value of the parameter represented as a [`Normal`] before any
    /// modulation from the host is applied. This may be useful for displaying
    /// modulation differently in plugin GUIs. Right now only CLAP plugins in
    /// Bitwig Studio is known to use this feature.
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub unmodulated_normal: Option<Normal>,
}

impl Default for NormalParam {
    fn default() -> Self {
        Self {
            normal: Normal::MIN,
            default: Normal::MIN,
            unmodulated_normal: None,
        }
    }
}

impl NormalParam {
    pub fn new(normal: impl Into<Normal>, default: impl Into<Normal>) -> Self {
        Self {
            normal: normal.into(),
            unmodulated_normal: None,
            default: default.into(),
        }
    }

    /// Updates the [`Normal`] value of this `NormalParam`
    ///
    /// [`Normal`]: ../struct.Normal.html
    #[inline]
    pub fn set(&mut self, normal: impl Into<Normal>) {
        self.normal = normal.into();
    }

    #[cfg(feature = "nice-plug")]
    pub fn from_nice(param: &impl nice_plug_core::params::Param) -> Self {
        Self {
            normal: param.modulated_normalized_value().into(),
            default: param.default_normalized_value().into(),
            unmodulated_normal: Some(param.unmodulated_normalized_value().into()),
        }
    }
}

/// A shorthand for [`NormalParam::from_nice()`]
#[cfg(feature = "nice-plug")]
pub fn nice_to_iced(param: &impl nice_plug_core::params::Param) -> NormalParam {
    NormalParam::from_nice(param)
}

/// A convenience method to set the nice-plug parameter from a [`Gesture`].
#[cfg(feature = "nice-plug")]
pub fn set_nice_param(
    param: &impl nice_plug_core::params::Param,
    gesture: Gesture,
    setter: nice_plug_core::context::gui::ParamSetter,
) {
    match gesture {
        Gesture::GestureStart => {
            setter.begin_set_parameter(param);
        }
        Gesture::Gesturing(new_normal) => {
            setter.set_parameter_normalized(param, new_normal.as_f32());
        }
        Gesture::GestureEnd => {
            setter.end_set_parameter(param);
        }
    }
}

impl From<f32> for NormalParam {
    fn from(normal: f32) -> Self {
        Self {
            normal: Normal::new(normal),
            ..Default::default()
        }
    }
}

impl From<f64> for NormalParam {
    fn from(normal: f64) -> Self {
        Self {
            normal: Normal::new(normal as f32),
            ..Default::default()
        }
    }
}

impl From<Normal> for NormalParam {
    fn from(normal: Normal) -> Self {
        Self {
            normal,
            ..Default::default()
        }
    }
}
