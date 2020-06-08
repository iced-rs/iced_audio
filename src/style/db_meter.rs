//! Various styles for the [`DBMeter`] widget
//!
//! [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html

use iced::Color;

/// The appearance of a [`DBMeter`].
///
/// [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html
#[derive(Debug, Clone)]
pub struct Style {
    ///
    pub back_color: Color,
    ///
    pub back_border_width: u16,
    ///
    pub back_border_color: Color,
    ///
    pub low_color: Color,
    ///
    pub med_color: Color,
    ///
    pub high_color: Color,
    ///
    pub clip_color: Color,
    ///
    pub peak_line_color: Option<Color>,
    ///
    pub peak_line_width: u16,
    ///
    pub color_all_clip_color: bool,
    /// 
    pub clip_marker_width: u16,
    /// 
    pub clip_marker_color: Color,
    ///
    pub inner_gap: u16,
    ///
    pub inner_gap_color: Color,
}
/// 
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TickMarkPosition {
    /// 
    LeftAndRight,
    /// 
    Left,
    /// 
    Right,
}

impl std::default::Default for TickMarkPosition {
    fn default() -> Self {
        TickMarkPosition::LeftAndRight
    }
}

/// The style of a [`TickMarkGroup`] for a [`DBMeter`]
///
/// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
/// [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html
#[derive(Debug, Copy, Clone)]
pub struct TickMarkStyle {
    /// 
    pub size_tier_1: u16,
    /// 
    pub size_tier_2: u16,
    /// 
    pub size_tier_3: u16,

    /// The height (thickness) of a tier 1 tick mark
    pub height_tier_1: u16,
    /// The height (thickness) of a tier 2 tick mark
    pub height_tier_2: u16,
    /// The height (thickness) of a tier 3 tick mark
    pub height_tier_3: u16,

    /// The color of a tier 1 tick mark
    pub color_tier_1: Color,
    /// The color of a tier 2 tick mark
    pub color_tier_2: Color,
    /// The color of a tier 3 tick mark
    pub color_tier_3: Color,

    /// 
    pub offset: u16,

    /// 
    pub position: TickMarkPosition,
}

impl std::default::Default for TickMarkStyle {
    fn default() -> Self {
        Self {
            size_tier_1: 4,
            size_tier_2: 3,
            size_tier_3: 2,

            height_tier_1: 2,
            height_tier_2: 2,
            height_tier_3: 1,

            color_tier_1: [0.56, 0.56, 0.56, 0.85].into(),
            color_tier_2: [0.56, 0.56, 0.56, 0.73].into(),
            color_tier_3: [0.56, 0.56, 0.56, 0.63].into(),

            offset: 2,

            position: TickMarkPosition::default(),
        }
    }
}

/// A set of rules that dictate the style of a [`DBMeter`].
///
/// [`DBMeter`]: ../../native/DBMeter/struct.DBMeter.html
pub trait StyleSheet {
    /// Produces the style of an active [`DBMeter`].
    ///
    /// [`DBMeter`]: ../../native/DBMeter/struct.DBMeter.html
    fn style(&self) -> Style;

    /// The style of a [`TickMarkGroup`] for a [`DBMeter`]
    ///
    /// For no tick marks, don't override this or set this to return `None`.
    ///
    /// [`TickMarkGroup`]: ../../core/tick_marks/struct.TickMarkGroup.html
    /// [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html
    fn tick_mark_style(&self) -> Option<TickMarkStyle> {
        None
    }
}

struct Default;

impl StyleSheet for Default {
    fn style(&self) -> Style {
        Style {
            back_color: Color::from_rgb(0.45, 0.45, 0.45),
            back_border_width: 1,
            back_border_color: Color::from_rgb(0.2, 0.2, 0.2),
            low_color: Color::from_rgb(0.435, 0.886, 0.11),
            med_color: Color::from_rgb(0.737, 1.0, 0.145),
            high_color: Color::from_rgb(1.0, 0.945, 0.0),
            clip_color: Color::from_rgb(1.0, 0.071, 0.071),
            peak_line_color: None,
            peak_line_width: 2,
            color_all_clip_color: true,
            clip_marker_width: 2,
            clip_marker_color: Color::from_rgba8(200, 200, 200, 0.28),
            inner_gap: 2,
            inner_gap_color: Color::from_rgb(0.25, 0.25, 0.25),
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
