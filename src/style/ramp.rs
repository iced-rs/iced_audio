//! Various styles for the [`Ramp`] widget
//!
//! [`Ramp`]: ../native/ramp/struct.Ramp.html

use iced_native::Color;

use crate::style::default_colors;

/// The appearance of a [`Ramp`],
///
/// [`Ramp`]: ../../native/ramp/struct.Ramp.html
#[derive(Debug, Clone)]
pub struct Appearance {
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

impl Default for Appearance {
    fn default() -> Self {
        Appearance {
            back_color: default_colors::LIGHT_BACK,
            back_border_width: 1.0,
            back_border_color: default_colors::BORDER,
            line_width: 2.0,
            line_center_color: default_colors::BORDER,
            line_up_color: default_colors::BORDER,
            line_down_color: default_colors::BORDER,
        }
    }
}

/// A set of rules that dictate the style of a [`Ramp`].
///
/// [`Ramp`]: ../../native/ramp/struct.Ramp.html
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the style of an active [`Ramp`].
    ///
    /// [`Ramp`]: ../../native/ramp/struct.Ramp.html
    fn active(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of a hovered [`Ramp`].
    ///
    /// [`Ramp`]: ../../native/ramp/struct.Ramp.html
    fn hovered(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of a [`Ramp`] that is being dragged.
    ///
    /// [`Ramp`]: ../../native/ramp/struct.Ramp.html
    fn dragging(&self, style: &Self::Style) -> Appearance;
}
