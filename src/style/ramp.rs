//! Various styles for the [`Ramp`] widget
//!
//! [`Ramp`]: ../native/ramp/struct.Ramp.html

use iced_native::Color;

use crate::style::default_colors;

/// The appearance of a [`Ramp`],
///
/// [`Ramp`]: ../../native/ramp/struct.Ramp.html
#[derive(Debug, Clone)]
pub struct Style {
    /// The color of the background rectangle
    pub back_color: Color,
    /// The width of the border of the background rectangle
    pub back_border_width: f32,
    /// The color of the border of the background rectangle
    pub back_border_color: Color,
    /// The width of the ramp line,
    pub line_width: f32,
    /// The color of the ramp line when it is in the center (straight) position
    pub line_center_color: Color,
    /// The color of the ramp line when it is in the up position
    pub line_up_color: Color,
    /// The color of the ramp line when it is in the down position
    pub line_down_color: Color,
}

/// A set of rules that dictate the style of a [`Ramp`].
///
/// [`Ramp`]: ../../native/ramp/struct.Ramp.html
pub trait StyleSheet {
    /// Produces the style of an active [`Ramp`].
    ///
    /// [`Ramp`]: ../../native/ramp/struct.Ramp.html
    fn active(&self) -> Style;

    /// Produces the style of a hovered [`Ramp`].
    ///
    /// [`Ramp`]: ../../native/ramp/struct.Ramp.html
    fn hovered(&self) -> Style;

    /// Produces the style of a [`Ramp`] that is being dragged.
    ///
    /// [`Ramp`]: ../../native/ramp/struct.Ramp.html
    fn dragging(&self) -> Style;
}

struct Default;
impl Default {
    const ACTIVE_STYLE: Style = Style {
        back_color: default_colors::LIGHT_BACK,
        back_border_width: 1.0,
        back_border_color: default_colors::BORDER,
        line_width: 2.0,
        line_center_color: default_colors::BORDER,
        line_up_color: default_colors::BORDER,
        line_down_color: default_colors::BORDER,
    };
}
impl StyleSheet for Default {
    fn active(&self) -> Style {
        Self::ACTIVE_STYLE
    }

    fn hovered(&self) -> Style {
        Style {
            back_color: default_colors::RAMP_BACK_HOVER,
            ..Self::ACTIVE_STYLE
        }
    }

    fn dragging(&self) -> Style {
        self.hovered()
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
