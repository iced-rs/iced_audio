//! Various styles for the [`Ramp`] widget
//!
//! [`Ramp`]: ../native/ramp/struct.Ramp.html

use crate::style::default_colors;
use iced_core::Color;

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
    type Style;

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

/// The style of a Ramp.
#[derive(Default)]
pub enum Ramp {
    /// The default style.
    #[default]
    Default,
    /// A custom style.
    Custom(Box<dyn StyleSheet<Style = iced_core::Theme>>),
}

impl<S> From<S> for Ramp
where
    S: 'static + StyleSheet<Style = iced_core::Theme>,
{
    fn from(val: S) -> Self {
        Ramp::Custom(Box::new(val))
    }
}

impl StyleSheet for iced_core::Theme {
    type Style = Ramp;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            Ramp::Default => Default::default(),
            Ramp::Custom(custom) => custom.active(self),
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            Ramp::Default => Appearance {
                back_color: default_colors::RAMP_BACK_HOVER,
                ..Default::default()
            },
            Ramp::Custom(custom) => custom.active(self),
        }
    }

    fn dragging(&self, style: &Self::Style) -> Appearance {
        self.hovered(style)
    }
}
