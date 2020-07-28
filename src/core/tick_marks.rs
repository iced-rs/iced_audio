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
    tier_1_positions: Vec<Normal>,
    tier_2_positions: Vec<Normal>,
    tier_3_positions: Vec<Normal>,
    len: usize,
    has_tier_1: bool,
    has_tier_2: bool,
    has_tier_3: bool,
}

impl Default for TickMarkGroup {
    fn default() -> Self {
        TickMarkGroup::center(TickMarkTier::One)
    }
}

impl TickMarkGroup {
    /// Constructs a new `TickMarkGroup` from a vector of [`TickMark`]s.
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    /// [`TickMark`]: struct.TickMark.html
    pub fn new(tick_marks: Vec<TickMark>) -> Self {
        let len = tick_marks.len();

        let mut tier_1_positions: Vec<Normal> = Vec::new();
        let mut tier_2_positions: Vec<Normal> = Vec::new();
        let mut tier_3_positions: Vec<Normal> = Vec::new();

        for tick_mark in tick_marks.iter() {
            match tick_mark.tier {
                TickMarkTier::One => {
                    tier_1_positions.push(tick_mark.position);
                }
                TickMarkTier::Two => {
                    tier_2_positions.push(tick_mark.position);
                }
                TickMarkTier::Three => {
                    tier_3_positions.push(tick_mark.position);
                }
            }
        }

        let has_tier_1 = !tier_1_positions.is_empty();
        let has_tier_2 = !tier_2_positions.is_empty();
        let has_tier_3 = !tier_3_positions.is_empty();

        Self {
            tier_1_positions,
            tier_2_positions,
            tier_3_positions,
            len,
            has_tier_1,
            has_tier_2,
            has_tier_3,
        }
    }

    /// Returns a new [`TickMarkGroup`] with a single [`TickMark`] in
    /// the center position.
    ///
    /// * `tier` - a [`TickMarkTier`] representing the size of the tick mark
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    /// [`TickMark`]: struct.TickMark.html
    pub fn center(tier: TickMarkTier) -> Self {
        let tick_marks = vec![TickMark::center(tier)];
        Self::new(tick_marks)
    }

    /// Returns a new [`TickMarkGroup`] with a [`TickMark`] in
    /// the min (`0.0`) position and max (`1.0`) position.
    ///
    /// * `tier` - a [`TickMarkTier`] representing the size of the tick mark
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    /// [`TickMark`]: struct.TickMark.html
    pub fn min_max(tier: TickMarkTier) -> Self {
        let tick_marks = vec![TickMark::min(tier), TickMark::max(tier)];
        Self::new(tick_marks)
    }

    /// Returns a new [`TickMarkGroup`] with a [`TickMark`] in
    /// the min (`0.0`), the max (`1.0`), and center (`0.5`) positions.
    ///
    /// * `min_max_tier` - a [`TickMarkTier`] representing the size of the `min` and `max` tick marks
    /// * `center_tier` - a [`TickMarkTier`] representing the size of the `center` tick mark
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    /// [`TickMark`]: struct.TickMark.html
    pub fn min_max_and_center(
        min_max_tier: TickMarkTier,
        center_tier: TickMarkTier,
    ) -> Self {
        let tick_marks = vec![
            TickMark::min(min_max_tier),
            TickMark::max(min_max_tier),
            TickMark::center(center_tier),
        ];
        Self::new(tick_marks)
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

        Self::new(vec)
    }

    /// Creates a [`TickMarkGroup`] of evenly spaced [`TickMark`]s
    ///
    /// * `len` - the number of tick marks
    /// * `tier` - the [`TickMarkTier`] of the tick marks
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    /// [`TickMark`]: struct.TickMark.html
    pub fn evenly_spaced(len: usize, tier: TickMarkTier) -> Self {
        let mut vec: Vec<TickMark> = Vec::new();
        vec.reserve_exact(len);

        if len == 1 {
            vec.push(TickMark::min(tier));
        } else if len != 0 {
            let len_min_1 = len - 1;
            let span = 1.0 / len_min_1 as f32;

            for i in 0..len_min_1 {
                let pos = i as f32 * span;

                vec.push(TickMark {
                    position: pos.into(),
                    tier,
                });
            }

            vec.push(TickMark::max(tier));
        }

        Self::new(vec)
    }

    /// Returns `true` if the `TickMarkGroup` contains a tier 1 `TickMark`.
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    /// [`TickMark`]: struct.TickMark.html
    pub fn has_tier_1(&self) -> bool {
        self.has_tier_1
    }

    /// Returns `true` if the `TickMarkGroup` contains a tier 2 `TickMark`.
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    /// [`TickMark`]: struct.TickMark.html
    pub fn has_tier_2(&self) -> bool {
        self.has_tier_2
    }

    /// Returns `true` if the `TickMarkGroup` contains a tier 3 `TickMark`.
    ///
    /// [`TickMarkGroup`]: struct.TickMarkGroup.html
    /// [`TickMark`]: struct.TickMark.html
    pub fn has_tier_3(&self) -> bool {
        self.has_tier_3
    }

    /// Returns a vec with the positions of the tier 1 tick marks.
    pub fn tier_1_positions(&self) -> &Vec<Normal> {
        &self.tier_1_positions
    }

    /// Returns a vec with the positions of the tier 2 tick marks.
    pub fn tier_2_positions(&self) -> &Vec<Normal> {
        &self.tier_2_positions
    }

    /// Returns a vec with the positions of the tier 3 tick marks.
    pub fn tier_3_positions(&self) -> &Vec<Normal> {
        &self.tier_3_positions
    }

    /// Returns the total number of tick marks.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl From<Vec<TickMark>> for TickMarkGroup {
    fn from(tick_marks: Vec<TickMark>) -> Self {
        TickMarkGroup::new(tick_marks)
    }
}

/// Tier of sizes for a [`TickMark`].
///
/// * One - large-sized tick mark
/// * Two - medium-sized tick mark
/// * Small - small-sized tick mark
///
/// [`TickMark`]: struct.TickMark.html
#[derive(Debug, Copy, Clone, PartialEq)]
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
    /// Returns a new tick mark at the center (`0.5`) position.
    ///
    /// * `position` - a [`Normal`] value that represents the position of the tick
    /// mark. For example, a value of `0.0` is at the minimum position, `1.0` is
    /// at the maximum position, and `0.5` is at the center position.
    /// * `tier` - a [`TickMarkTier`] representing the size of the tick mark
    ///
    /// [`Normal`]: ../struct.Normal.html
    /// [`TickMarkTier`]: enum.TickMarkTier.html
    pub fn new(position: Normal, tier: TickMarkTier) -> Self {
        Self { position, tier }
    }

    /// Returns a new tick mark at the center (`0.5`) position.
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

    /// Returns a new tick mark at the minimum (`0.0`) position.
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

    /// Returns a new tick mark at the maximum (`1.0`) position.
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
