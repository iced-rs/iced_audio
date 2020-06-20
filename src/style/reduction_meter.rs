//! Various styles for the [`ReductionMeter`] widget
//!
//! [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html

use iced::Color;

use crate::style::{bar_tick_marks, default_colors};

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
            back_border_radius: 0,
            back_border_color: default_colors::DB_METER_BORDER,
            color: default_colors::DB_METER_LOW,
            peak_line_color: default_colors::DB_METER_LOW,
            peak_line_width: 2,
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
