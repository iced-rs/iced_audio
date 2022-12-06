//! Module for the [`NormalParam`] struct
//!
//! [`NormalParam`]: struct.NormalParam.html

use crate::core::Normal;

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
    pub value: Normal,

    /// The default value of the parameter represented as a [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub default: Normal,
}

impl Default for NormalParam {
    fn default() -> Self {
        Self {
            value: Normal::MIN,
            default: Normal::MIN,
        }
    }
}

impl NormalParam {
    /// Updates the [`Normal`] value of this `NormalParam`
    ///
    /// [`Normal`]: ../struct.Normal.html
    #[inline]
    pub fn update(&mut self, normal: Normal) {
        self.value = normal;
    }
}
