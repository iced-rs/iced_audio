//! Module for the [`Param`] trait
//!
//! [`Param`]: struct.Param.html

use crate::core::Normal;

use std::fmt::Debug;

/// A paramater that contains an `ID`, `value`, and `default_value`.
///
/// The values are stored as a universal [`Normal`] type.
///
/// The unique identifier of user supplied type `ID`. This can be an
/// `enum`, `u32`, `i32`, `String`, etc. Each parameter must have a unique `ID`
/// value!
///
/// [`Normal`]: ../struct.Normal.html
#[derive(Debug, Copy, Clone)]
pub struct Param<ID: Debug + Copy + Clone> {
    /// The unique identifier of a user supplied type `ID`. This can be an
    /// `enum`, `u32`, `i32`, `String`, etc. Each parameter must have a unique `ID`
    /// value!
    pub id: ID,

    /// The value of the parameter represented as a [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub normal: Normal,

    /// The default value of the parameter represented as a [`Normal`]
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub default_normal: Normal,
}
