//! Various styles for the [`ReductionMeter`] widget
//!
//! [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html

use iced::Color;

use crate::style::default_colors;

/// The appearance of a [`ReductionMeter`].
///
/// [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html
#[derive(Debug, Copy, Clone)]
pub struct Style {
    /// The color of the background rectangle
    pub back_color: Color,
    /// The width of the border of the background rectangle
    pub back_border_width: u16,
    /// The radius of the border of the background rectangle
    pub back_border_radius: u16,
    /// The color of the border of the background rectangle
    pub back_border_color: Color,
    /// The color of the meter bar.
    pub color: Color,
    /// The color of the peak line.
    pub peak_line_color: Color,
    /// The width of the peak line
    pub peak_line_width: u16,
}

/// The placement of tick marks for a [`ReductionMeter`].
///
/// [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TickMarkPlacement {
    /// Tick marks on both sides
    LeftAndRight,
    /// Tick marks only on the left/top side
    Left,
    /// Tick marks only on the right/bottom side
    Right,
}

impl std::default::Default for TickMarkPlacement {
    fn default() -> Self {
        TickMarkPlacement::LeftAndRight
    }
}

/// The style of a [`TickMarkGroup`] for a [`ReductionMeter`]
///
/// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
/// [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html
#[derive(Debug, Copy, Clone)]
pub struct TickMarkStyle {
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

    /// The offset of the tick marks from the side of the [`ReductionMeter`]
    ///
    /// [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html
    pub offset: u16,

    /// The placement of the tick marks
    pub placement: TickMarkPlacement,
}

impl std::default::Default for TickMarkStyle {
    fn default() -> Self {
        Self {
            length_tier_1: 4,
            length_tier_2: 3,
            length_tier_3: 2,

            width_tier_1: 2,
            width_tier_2: 2,
            width_tier_3: 1,

            color_tier_1: default_colors::DB_METER_TICK_TIER_1,
            color_tier_2: default_colors::DB_METER_TICK_TIER_2,
            color_tier_3: default_colors::DB_METER_TICK_TIER_3,

            offset: 2,

            placement: TickMarkPlacement::default(),
        }
    }
}

/// A set of rules that dictate the style of a [`ReductionMeter`].
///
/// [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html
pub trait StyleSheet {
    /// Produces the style of a [`ReductionMeter`].
    ///
    /// [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html
    fn style(&self) -> Style;

    /// The style of a [`TickMarkGroup`] for a [`ReductionMeter`]
    ///
    /// For no tick marks, don't override this or set this to return `None`.
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html
    fn tick_mark_style(&self) -> Option<TickMarkStyle> {
        None
    }
}

struct Default;

impl StyleSheet for Default {
    fn style(&self) -> Style {
        Style {
            back_color: default_colors::DB_METER_BACK,
            back_border_width: 1,
            back_border_radius: 0,
            back_border_color: default_colors::DB_METER_BORDER,
            color: default_colors::DB_METER_LOW,
            peak_line_color: default_colors::DB_METER_LOW,
            peak_line_width: 2,
        }
    }

    fn tick_mark_style(&self) -> Option<TickMarkStyle> {
        Some(TickMarkStyle::default())
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
