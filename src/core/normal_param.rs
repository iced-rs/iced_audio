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
