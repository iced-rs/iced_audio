//! Various styles for the [`DBMeter`] widget
//!
//! [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html

use iced::Color;

use crate::style::bar_tick_marks;
use crate::style::default_colors;

/// The appearance of a [`DBMeter`].
///
/// [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html
#[derive(Debug, Copy, Clone)]
pub struct Style {
    /// The color of the background rectangle
    pub back_color: Color,
    /// The width of the border of the background rectangle
    pub back_border_width: u16,
    /// The color of the border of the background rectangle
    pub back_border_color: Color,
    /// The color of the bar in the low tier
    pub low_color: Color,
    /// The color of the bar in the medium tier
    pub med_color: Color,
    /// The color of the bar in the high tier
    pub high_color: Color,
    /// The color of the bar/peak line in the clipping tier
    pub clip_color: Color,
    /// The color of the peak line. Set this to `None` to use
    /// the same colors as the bar.
    pub peak_line_color: Option<Color>,
    /// The width of the peak line
    pub peak_line_width: u16,
    /// If true, this will color the entire bar `clip_color` when
    /// the clipping tier is reached.
    pub color_all_clip_color: bool,
    /// The width of the line that marks where clipping starts
    pub clip_marker_width: u16,
    /// The color of the line that marks where clipping starts
    pub clip_marker_color: Color,
    /// The width of the gap between the left and right bar. This has
    /// no effect if the [`DBMeter`] is in mono mode.
    ///
    /// [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html
    pub inner_gap: u16,
    /// The color of the gap between the left and right bar. This has
    /// no effect if the [`DBMeter`] is in mono mode.
    ///
    /// [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html
    pub inner_gap_color: Color,
}

/// A set of rules that dictate the style of a [`DBMeter`].
///
/// [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html
pub trait StyleSheet {
    /// Produces the style of a [`DBMeter`].
    ///
    /// [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html
    fn style(&self) -> Style;

    /// The style of a [`TickMarkGroup`] for a [`DBMeter`]
    ///
    /// For no tick marks, don't override this or set this to return `None`.
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html
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
            low_color: default_colors::DB_METER_LOW,
            med_color: default_colors::DB_METER_MED,
            high_color: default_colors::DB_METER_HIGH,
            clip_color: default_colors::DB_METER_CLIP,
            peak_line_color: None,
            peak_line_width: 2,
            color_all_clip_color: true,
            clip_marker_width: 2,
            clip_marker_color: default_colors::DB_METER_CLIP_MARKER,
            inner_gap: 2,
            inner_gap_color: default_colors::DB_METER_GAP,
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
