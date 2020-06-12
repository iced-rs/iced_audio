//! Various styles for the [`AutoRangeInput`] widget
//!
//! [`AutoRangeInput`]: ../../native/auto_range_input/struct.AutoRangeInput.html

use iced::Color;

use crate::style::default_colors;

/// The appearance of an [`AutoRangeInput`]
///
/// [`AutoRangeInput`]: ../../native/auto_range_input/struct.AutoRangeInput.html
#[derive(Debug, Clone)]
pub enum Style {
    /// A circle style
    Circle(CircleStyle),
    /// A square style
    Square(SquareStyle),
    /// Appearance is invisible, but still interactable. Useful if placed right
    /// on top of a [`Knob`] with an [`AutoRangeRingStyle`].
    ///
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    /// [`AutoRangeRingStyle`]: ../knob/struct.AutoRangeRingStyle.html
    Invisible,
}

/// A circle [`Style`] for an [`AutoRangeInput`]
///
/// [`Style`]: enum.Style.html
/// [`AutoRangeInput`]: ../../native/auto_range_input/struct.AutoRangeInput.html
#[derive(Debug, Clone)]
pub struct CircleStyle {
    /// Color of the circle
    pub color: Color,
    /// Width of the border
    pub border_width: u16,
    /// Color of the border
    pub border_color: Color,
}

/// A square [`Style`] for an [`AutoRangeInput`]
///
/// [`Style`]: enum.Style.html
/// [`AutoRangeInput`]: ../../native/auto_range_input/struct.AutoRangeInput.html
#[derive(Debug, Clone)]
pub struct SquareStyle {
    /// Color of the square
    pub color: Color,
    /// Width of the border
    pub border_width: u16,
    /// Radius of the border
    pub border_radius: u16,
    /// Color of the border
    pub border_color: Color,
}

/// A set of rules that dictate the style of a [`AutoRangeInput`].
///
/// [`AutoRangeInput`]: ../../native/auto_range_input/struct.AutoRangeInput.html
pub trait StyleSheet {
    /// Produces the style of an active [`AutoRangeInput`].
    ///
    /// [`AutoRangeInput`]: ../../native/auto_range_input/struct.AutoRangeInput.html
    fn active(&self) -> Style;

    /// Produces the style of a hovered [`AutoRangeInput`].
    ///
    /// [`AutoRangeInput`]: ../../native/auto_range_input/struct.AutoRangeInput.html
    fn hovered(&self) -> Style;

    /// Produces the style of a [`AutoRangeInput`] that is being dragged.
    ///
    /// [`AutoRangeInput`]: ../../native/auto_range_input/struct.AutoRangeInput.html
    fn dragging(&self) -> Style;
}

struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::Circle(CircleStyle {
            color: default_colors::LIGHT_BACK,
            border_width: 1,
            border_color: default_colors::BORDER,
        })
    }

    fn hovered(&self) -> Style {
        let active = self.active();
        if let Style::Circle(active) = self.active() {
            Style::Circle(CircleStyle {
                color: default_colors::KNOB_BACK_HOVER,
                ..active
            })
        } else {
            active
        }
    }

    fn dragging(&self) -> Style {
        self.hovered()
    }
}

/// An invisible [`StyleSheet`] for an [`AutoRangeInput`]
/// 
/// Appearance is invisible, but the input is still interactable. Useful
/// if placed right on top of a [`Knob`] with an [`AutoRangeRingStyle`].
///
/// [`StyleSheet`]: struct.StyleSheet.html
/// [`AutoRangeInput`]: ../../native/auto_range_input/struct.AutoRangeInput.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
/// [`AutoRangeRingStyle`]: ../knob/struct.AutoRangeRingStyle.html
#[allow(missing_debug_implementations)]
pub struct DefaultInvisible;

impl StyleSheet for DefaultInvisible {
    fn active(&self) -> Style {
        Style::Invisible
    }

    fn hovered(&self) -> Style {
        self.active()
    }

    fn dragging(&self) -> Style {
        self.active()
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
