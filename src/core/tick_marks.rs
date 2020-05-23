//! Structs for constructing a group of [`TickMark`]s.
//!
//! [`TickMark`]: struct.TickMark.html

use crate::core::Normal;

use std::fmt::Debug;

/// A group of [`TickMark`]s.
///
/// [`TickMark`]: struct.TickMark.html
#[derive(Debug, Clone)]
pub struct TickMarkGroup {
    /// A group of [`TickMark`]s.
    ///
    /// [`TickMark`]: struct.TickMark.html
    pub group: Vec<TickMark>,
}

impl Default for TickMarkGroup {
    fn default() -> Self {
        vec![TickMark::center(TickMarkTier::One)].into()
    }
}

impl TickMarkGroup {
    /// Constructs an empty [`TickMarkGroup`].
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    pub fn new() -> Self {
        Self { group: Vec::new() }
    }

    /// Constructs a new `TickMarkGroup` from a vector of [`TickMark`]s.
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    /// [`TickMark`]: struct.TickMark.html
    pub fn from_vec(tick_marks: Vec<TickMark>) -> Self {
        Self { group: tick_marks }
    }

    /// Pushes a new [`TickMark`] into the [`TickMarkGroup`].
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    /// [`TickMark`]: struct.TickMark.html
    pub fn push(&mut self, tick_mark: TickMark) {
        self.group.push(tick_mark);
    }
}

impl From<Vec<TickMark>> for TickMarkGroup {
    fn from(tick_marks: Vec<TickMark>) -> Self {
        TickMarkGroup { group: tick_marks }
    }
}

/// Tier of sizes for a [`TickMark`].
///
/// * One - large-sized tick mark
/// * Two - medium-sized tick mark
/// * Small - small-sized tick mark
///
/// [`TickMark`]: struct.TickMark.html
#[derive(Debug, Copy, Clone)]
pub enum TickMarkTier {
    /// large-sized tick mark
    One,
    /// medium-sized tick mark
    Two,
    /// small-sized tick mark
    Three,
}

impl Default for TickMarkTier {
    fn default() -> Self {
        TickMarkTier::One
    }
}

/// Data of a tick mark. It includes:
///
/// * `position` - a [`Normal`] value that represents the position of the tick
/// mark. For example, a value of `0.0` is at the minimum position, `1.0` is
/// at the maximum position, and `0.5` is at the center position. The default
/// is `0.5`.
/// * `tier` - a [`TickMarkTier`] representing the size of the tick mark.
///
/// [`TickMarkTier`]: enum.TickMarkTier.html
/// [`Normal`]: ../struct.Normal.html
#[derive(Debug, Copy, Clone)]
pub struct TickMark {
    /// a [`Normal`] value that represents the position of the tick
    /// mark. For example, a value of `0.0` is at the minimum position, `1.0` is
    /// at the maximum position, and `0.5` is at the center position. The
    /// default is `0.5`.
    ///
    /// [`Normal`]: ../struct.Normal.html
    pub position: Normal,
    /// a [`TickMarkTier`] representing the size of the tick mark. The default
    /// is `TickMarkTier::One`.
    ///
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    pub tier: TickMarkTier,
}

impl Default for TickMark {
    fn default() -> Self {
        TickMark::center(TickMarkTier::One)
    }
}

impl TickMark {
    /// Returns a tick mark at the center position.
    ///
    /// * `tier` - a [`TickMarkTier`] representing the size of the tick mark
    ///
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    pub fn center(tier: TickMarkTier) -> Self {
        Self { position: 0.5.into(), tier }
    }

    /// Returns a tick mark at the minimum position.
    ///
    /// * `tier` - a [`TickMarkTier`] representing the size of the tick mark
    ///
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    pub fn min(tier: TickMarkTier) -> Self {
        Self { position: 0.0.into(), tier }
    }

    /// Returns a tick mark at the maximum position.
    ///
    /// * `tier` - a [`TickMarkTier`] representing the size of the tick mark
    ///
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    pub fn max(tier: TickMarkTier) -> Self {
        Self { position: 1.0.into(), tier }
    }
}