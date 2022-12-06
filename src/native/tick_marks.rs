//! Structs for constructing a group of tick marks.

use std::fmt::Debug;

use crate::core::Normal;

/// A group of tick marks.
///
/// tick mark: struct.TickMark.html
#[derive(Debug, Clone)]
pub struct Group {
    tier_1_positions: Vec<Normal>,
    tier_2_positions: Vec<Normal>,
    tier_3_positions: Vec<Normal>,
    len: usize,
    hashed: u64,
}

impl Default for Group {
    fn default() -> Self {
        Group::center(Tier::One)
    }
}

impl Group {
    /// Constructs a new `Group` from an array of normalized values and tiers.
    ///
    /// [`Group`]: struct.Group.html
    pub fn from_normalized(tick_marks: &[(Normal, Tier)]) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = iced_native::Hasher::default();
        tick_marks.len().hash(&mut hasher);

        let len = tick_marks.len();

        let mut tier_1_positions: Vec<Normal> = Vec::new();
        let mut tier_2_positions: Vec<Normal> = Vec::new();
        let mut tier_3_positions: Vec<Normal> = Vec::new();

        for tick_mark in tick_marks.iter() {
            tick_mark.1.hash(&mut hasher);
            // Rust can't hash an f32 value.
            ((tick_mark.0.as_f32() * 10000000.0) as u64).hash(&mut hasher);

            match tick_mark.1 {
                Tier::One => {
                    tier_1_positions.push(tick_mark.0);
                }
                Tier::Two => {
                    tier_2_positions.push(tick_mark.0);
                }
                Tier::Three => {
                    tier_3_positions.push(tick_mark.0);
                }
            }
        }

        Self {
            tier_1_positions,
            tier_2_positions,
            tier_3_positions,
            len,
            hashed: hasher.finish(),
        }
    }

    /// Returns a new [`Group`] with a single tick mark in
    /// the center position.
    ///
    /// * `tier` - a [`Tier`] representing the size of the tick mark
    ///
    /// [`Group`]: struct.Group.html
    /// [`Tier`]: enum.Tier.html
    pub fn center(tier: Tier) -> Self {
        Self::from_normalized(&[(Normal::CENTER, tier)])
    }

    /// Returns a new [`Group`] with a tick mark in
    /// the min (`0.0`) position and max (`1.0`) position.
    ///
    /// * `tier` - a [`Tier`] representing the size of the tick mark
    ///
    /// [`Group`]: struct.Group.html
    /// [`Tier`]: enum.Tier.html
    pub fn min_max(tier: Tier) -> Self {
        Self::from_normalized(&[(Normal::MIN, tier), (Normal::MAX, tier)])
    }

    /// Returns a new [`Group`] with a tick mark in
    /// the min (`0.0`), the max (`1.0`), and center (`0.5`) positions.
    ///
    /// * `min_max_tier` - a [`Tier`] representing the size of the `min` and `max` tick marks
    /// * `center_tier` - a [`Tier`] representing the size of the `center` tick mark
    ///
    /// [`Group`]: struct.Group.html
    /// [`Tier`]: enum.Tier.html
    pub fn min_max_and_center(min_max_tier: Tier, center_tier: Tier) -> Self {
        Self::from_normalized(&[
            (Normal::MIN, min_max_tier),
            (Normal::CENTER, center_tier),
            (Normal::MAX, min_max_tier),
        ])
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
        one: usize,
        two: usize,
        three: usize,
        sides: Option<Tier>,
    ) -> Self {
        let mut tick_marks: Vec<(Normal, Tier)> =
            Vec::with_capacity(one + (two * one) + (three * two * one) + 2);

        let one_ranges = one + 1;
        let two_ranges = two + 1;
        let three_ranges = three + 1;

        let one_span = 1.0 / one_ranges as f32;
        let two_span = one_span / two_ranges as f32;
        let three_span = two_span / three_ranges as f32;

        for i_1 in 0..one_ranges {
            let one_pos = (i_1 as f32 * one_span) + one_span;

            if i_1 != one {
                tick_marks.push((Normal::from_clipped(one_pos), Tier::One));
            }

            for i_2 in 0..two_ranges {
                let two_pos = (i_2 as f32 * two_span) + two_span;

                if i_2 != two {
                    tick_marks.push((
                        Normal::from_clipped(one_pos - two_pos),
                        Tier::Two,
                    ));
                }

                for i_3 in 0..three {
                    let three_pos = (i_3 as f32 * three_span) + three_span;

                    tick_marks.push((
                        Normal::from_clipped(one_pos - two_pos + three_pos),
                        Tier::Three,
                    ));
                }
            }
        }

        if let Some(side_tier) = sides {
            tick_marks.push((Normal::MIN, side_tier));
            tick_marks.push((Normal::MAX, side_tier));
        }

        Self::from_normalized(&tick_marks)
    }

    /// Creates a [`Group`] of evenly spaced tick marks
    ///
    /// * `len` - the number of tick marks
    /// * `tier` - the [`Tier`] of the tick marks
    ///
    /// [`Group`]: struct.Group.html
    /// [`Tier`]: enum.Tier.html
    pub fn evenly_spaced(len: usize, tier: Tier) -> Self {
        let mut tick_marks: Vec<(Normal, Tier)> = Vec::with_capacity(len);

        if len == 1 {
            tick_marks.push((Normal::MIN, tier));
        } else if len != 0 {
            let len_min_1 = len - 1;
            let span = 1.0 / len_min_1 as f32;

            for i in 0..len_min_1 {
                let pos = i as f32 * span;

                tick_marks.push((Normal::from_clipped(pos), tier));
            }

            tick_marks.push((Normal::MAX, tier));
        }

        Self::from_normalized(&tick_marks)
    }

    /// Returns the positions of the tier 1 tick marks.
    /// Returns `None` if there are no tier 1 tick marks.
    pub fn tier_1(&self) -> Option<&Vec<Normal>> {
        if self.tier_1_positions.is_empty() {
            None
        } else {
            Some(&self.tier_1_positions)
        }
    }

    /// Returns the positions of the tier 2 tick marks.
    /// Returns `None` if there are no tier 2 tick marks.
    pub fn tier_2(&self) -> Option<&Vec<Normal>> {
        if self.tier_2_positions.is_empty() {
            None
        } else {
            Some(&self.tier_2_positions)
        }
    }

    /// Returns the positions of the tier 3 tick marks.
    /// Returns `None` if there are no tier 3 tick marks.
    pub fn tier_3(&self) -> Option<&Vec<Normal>> {
        if self.tier_3_positions.is_empty() {
            None
        } else {
            Some(&self.tier_3_positions)
        }
    }

    /// Returns the total number of tick marks.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whethere there are no tick marks.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the hashed value of the internal data.
    pub(crate) fn hashed(&self) -> u64 {
        self.hashed
    }
}

impl From<Vec<(Normal, Tier)>> for Group {
    fn from(vec: Vec<(Normal, Tier)>) -> Self {
        Self::from_normalized(&vec)
    }
}

impl From<&[(Normal, Tier)]> for Group {
    fn from(slice: &[(Normal, Tier)]) -> Self {
        Self::from_normalized(slice)
    }
}

/// Tier of sizes for a tick mark.
///
/// * One - large-sized tick mark
/// * Two - medium-sized tick mark
/// * Small - small-sized tick mark
#[derive(Debug, Copy, Clone, Eq, PartialEq, std::hash::Hash)]
pub enum Tier {
    /// large-sized tick mark
    One,
    /// medium-sized tick mark
    Two,
    /// small-sized tick mark
    Three,
}

impl Default for Tier {
    fn default() -> Self {
        Tier::One
    }
}
