//! Various styles for the [`PhaseMeter`] widget
//!
//! [`PhaseMeter`]: ../../native/phase_meter/struct.PhaseMeter.html

use iced::Color;

use crate::style::{bar_tick_marks, default_colors};

/// The appearance of a [`PhaseMeter`].
///
/// [`PhaseMeter`]: ../../native/phase_meter/struct.PhaseMeter.html
#[derive(Debug, Copy, Clone)]
pub struct Style {
    /// The color of the background rectangle
    pub back_color: Color,
    /// The width of the border of the background rectangle
    pub back_border_width: u16,
    /// The color of the border of the background rectangle
    pub back_border_color: Color,
    /// The color of the meter bar in the bad phase correlation position
    pub bad_color: Color,
    /// The color of the meter bar in the poor phase correlation position
    pub poor_color: Color,
    /// The color of the meter bar in the okay phase correlation position
    pub okay_color: Color,
    /// The color of the meter bar in the good phase correlation position
    pub good_color: Color,
    /// The width of the line marking the center position
    pub center_line_width: u16,
    /// The color of the line marking the center position
    pub center_line_color: Color,
}

/// A set of rules that dictate the style of a [`PhaseMeter`].
///
/// [`PhaseMeter`]: ../../native/phase_meter/struct.PhaseMeter.html
pub trait StyleSheet {
    /// Produces the style of a [`PhaseMeter`].
    ///
    /// [`PhaseMeter`]: ../../native/phase_meter/struct.PhaseMeter.html
    fn style(&self) -> Style;

    /// The style of a [`TickMarkGroup`] for a [`PhaseMeter`]
    ///
    /// For no tick marks, don't override this or set this to return `None`.
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`PhaseMeter`]: ../../native/phase_meter/struct.PhaseMeter.html
    fn tick_mark_style(&self) -> Option<bar_tick_marks::Style> {
        None
    }
}

struct Default;

impl StyleSheet for Default {
    fn style(&self) -> Style {
        Style {
            back_color: default_colors::DB_METER_BACK,
            back_border_width: 1,
            back_border_color: default_colors::DB_METER_BORDER,
            bad_color: default_colors::DB_METER_CLIP,
            poor_color: default_colors::DB_METER_HIGH,
            okay_color: default_colors::DB_METER_MED,
            good_color: default_colors::DB_METER_LOW,
            center_line_width: 1,
            center_line_color: default_colors::PHASE_METER_CENTER_LINE,
        }
    }

    fn tick_mark_style(&self) -> Option<bar_tick_marks::Style> {
        Some(bar_tick_marks::Style::default())
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
