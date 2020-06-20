//! Various styles for a [`TickMarkGroup`] in a bar meter widget
///
/// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
use iced::Color;

use crate::style::default_colors;

/// The placement of tick marks relative to the widget
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Placement {
    /// Tick marks on both sides
    BothSides,
    /// Tick marks only on the left/top side
    LeftOrTop,
    /// Tick marks only on the right/bottom side
    RightOrBottom,
}

impl std::default::Default for Placement {
    fn default() -> Self {
        Placement::BothSides
    }
}

/// The style of a [`TickMarkGroup`] for a bar meter widget
///
/// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
#[derive(Debug, Copy, Clone)]
pub struct Style {
    /// The length of a tier 1 tick mark
    pub length_tier_1: u16,
    /// The length of a tier 2 tick mark
    pub length_tier_2: u16,
    /// The length of a tier 3 tick mark
    pub length_tier_3: u16,

    /// The width (thickness) of a tier 1 tick mark
    pub width_tier_1: u16,
    /// The width (thickness) of a tier 2 tick mark
    pub width_tier_2: u16,
    /// The width (thickness) of a tier 3 tick mark
    pub width_tier_3: u16,

    /// The color of a tier 1 tick mark
    pub color_tier_1: Color,
    /// The color of a tier 2 tick mark
    pub color_tier_2: Color,
    /// The color of a tier 3 tick mark
    pub color_tier_3: Color,

    /// The offset of the tick marks from the side of the widget
    pub offset: u16,

    /// The placement of the tick marks
    pub placement: Placement,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            length_tier_1: 4,
            length_tier_2: 3,
            length_tier_3: 2,

            width_tier_1: 2,
            width_tier_2: 2,
            width_tier_3: 1,

            color_tier_1: default_colors::TICK_TIER_1,
            color_tier_2: default_colors::TICK_TIER_2,
            color_tier_3: default_colors::TICK_TIER_3,

            offset: 2,

            placement: Placement::default(),
        }
    }
}
