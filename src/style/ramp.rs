//! Various styles for the [`Ramp`] widget
//! 
//! [`Ramp`]: ../native/ramp/struct.Ramp.html

use iced::Color;

/// The appearance of a [`Ramp`],
///
/// [`Ramp`]: ../../native/ramp/struct.Ramp.html
#[derive(Debug, Clone)]
pub struct Style {
    /// The color of the background rectangle
    pub background_color: Color,
    /// The width of the border of the background rectangle
    pub border_width: u16,
    /// The color of the border of the background rectangle
    pub border_color: Color,
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

impl StyleSheet for Default {
    fn active(&self) -> Style {
        let line_color = Color::from_rgb(0.4, 0.4, 0.4);

        Style {
            background_color: Color::from_rgb(0.97, 0.97, 0.97),
            border_width: 1,
            border_color: line_color,
            line_width: 2.0,
            line_center_color: line_color,
            line_up_color: line_color,
            line_down_color: line_color,
        }
    }

    fn hovered(&self) -> Style {
        let active = self.active();

        Style {
            background_color: Color::from_rgb(0.95, 0.95, 0.95),
            ..active
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