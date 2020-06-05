//! Structs for constructing a group of [`TickMark`]s.
//!
//! [`TickMark`]: struct.TickMark.html

use std::fmt::Debug;

use crate::core::Normal;

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

    /// Creates a group of tick marks by subdividing the range.
    ///
    /// * `one` - The number of tier 1 tick marks. For example, `1` will put
    /// a single tier 1 tick mark at the `0.5` (center) position. `3` will put
    /// three tick marks at `0.25`, `0.5`, `0.75`. For no tier 1 tick marks,
    /// put `0`.
    /// * `two` - The number of tier 2 tick marks in each range between tier 1
    /// tick marks. If there are no tier 1 tick marks, then it will behave the
    /// same as tier 1 tick marks.
    /// * `three` - The number of tier 3 tick marks in each range between tier
    /// 2 tick marks. If there are no tier 2 tick marks, then it will behave the
    /// same as tier 2 tick marks.
    /// * `sides` - The tier of tick marks to put on the two sides (`0.0` and
    /// `1.0`). For no tick marks on the sides, put `None`.
    pub fn subdivided(
        one: u16,
        two: u16,
        three: u16,
        sides: Option<TickMarkTier>,
    ) -> Self {
        let mut vec: Vec<TickMark> = Vec::new();

        let one_ranges = one + 1;
        let two_ranges = two + 1;
        let three_ranges = three + 1;

        let one_span = 1.0 / one_ranges as f32;
        let two_span = one_span / two_ranges as f32;
        let three_span = two_span / three_ranges as f32;

        for i_1 in 0..one_ranges {
            let one_pos = (i_1 as f32 * one_span) + one_span;

            if i_1 != one {
                vec.push(TickMark {
                    position: one_pos.into(),
                    tier: TickMarkTier::One,
                });
            }

            for i_2 in 0..two_ranges {
                let two_pos = (i_2 as f32 * two_span) + two_span;

                if i_2 != two {
                    vec.push(TickMark {
                        position: (one_pos - two_pos).into(),
                        tier: TickMarkTier::Two,
                    });
                }

                for i_3 in 0..three {
                    let three_pos = (i_3 as f32 * three_span) + three_span;

                    vec.push(TickMark {
                        position: (one_pos - two_pos + three_pos).into(),
                        tier: TickMarkTier::Three,
                    });
                }
            }
        }

        if let Some(side_tier) = sides {
            vec.push(TickMark {
                position: 0.0.into(),
                tier: side_tier,
            });
            vec.push(TickMark {
                position: 1.0.into(),
                tier: side_tier,
            });
        }

        vec.into()
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
        Self {
            position: 0.5.into(),
            tier,
        }
    }

    /// Returns a tick mark at the minimum position.
    ///
    /// * `tier` - a [`TickMarkTier`] representing the size of the tick mark
    ///
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    pub fn min(tier: TickMarkTier) -> Self {
        Self {
            position: 0.0.into(),
            tier,
        }
    }

    /// Returns a tick mark at the maximum position.
    ///
    /// * `tier` - a [`TickMarkTier`] representing the size of the tick mark
    ///
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    pub fn max(tier: TickMarkTier) -> Self {
        Self {
            position: 1.0.into(),
            tier,
        }
    }
}
