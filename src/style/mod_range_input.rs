//! Various styles for the [`ModRangeInput`] widget
//!
//! [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html

use iced_native::Color;

use crate::style::default_colors;

/// The appearance of an [`ModRangeInput`]
///
/// [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html
#[derive(Debug, Clone)]
pub enum Appearance {
    /// A circle style
    Circle(CircleAppearance),
    /// A square style
    Square(SquareAppearance),
    /// Appearance is invisible, but still interactable. Useful if placed right
    /// on top of a [`Knob`] with an [`ModRangeRingStyle`].
    ///
    /// [`Knob`]: ../../native/knob/struct.Knob.html
    /// [`ModRangeRingStyle`]: ../knob/struct.ModRangeRingStyle.html
    Invisible,
}

/// A circle [`Appearance`] for an [`ModRangeInput`]
///
/// [`Appearance`]: enum.Appearance.html
/// [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html
#[derive(Debug, Clone)]
pub struct CircleAppearance {
    /// Color of the circle
    pub color: Color,
    /// Width of the border
    pub border_width: f32,
    /// Color of the border
    pub border_color: Color,
}

impl Default for CircleAppearance {
    fn default() -> Self {
        CircleAppearance {
            color: default_colors::LIGHT_BACK,
            border_width: 1.0,
            border_color: default_colors::BORDER,
        }
    }
}

/// A square [`Appearance`] for an [`ModRangeInput`]
///
/// [`Appearance`]: enum.Appearance.html
/// [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html
#[derive(Debug, Clone)]
pub struct SquareAppearance {
    /// Color of the square
    pub color: Color,
    /// Width of the border
    pub border_width: f32,
    /// Radius of the border
    pub border_radius: f32,
    /// Color of the border
    pub border_color: Color,
}

/// A set of rules that dictate the style of a [`ModRangeInput`].
///
/// [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the style of an active [`ModRangeInput`].
    ///
    /// [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html
    fn active(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of a hovered [`ModRangeInput`].
    ///
    /// [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html
    fn hovered(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of a [`ModRangeInput`] that is being dragged.
    ///
    /// [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html
    fn dragging(&self, style: &Self::Style) -> Appearance;
}
