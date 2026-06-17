//! Various styles for the [`ModRangeInput`] widget
//!
//! [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html

use crate::style::default_colors;
use iced::{Color, Theme};

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
    type Style;

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

    /// Produces the style of a [`ModRangeInput`] that is currently disabled.
    ///
    /// [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html
    fn disabled(&self, style: &Self::Style) -> Appearance;
}

/// A style for a [`ModRangeInput`] that makes it invisible but still interactable.
/// Useful if placed right on top of a [`Knob`] with an [`ModRangeRingStyle`].
///
/// [`ModRangeInput`]: ../../native/mod_range_input/struct.ModRangeInput.html
/// [`Knob`]: ../../native/knob/struct.Knob.html
/// [`ModRangeRingStyle`]: ../knob/struct.ModRangeRingStyle.html
pub struct InvisibleStyle;
impl StyleSheet for InvisibleStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance::Invisible
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        Appearance::Invisible
    }

    fn dragging(&self, _style: &Self::Style) -> Appearance {
        Appearance::Invisible
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        Appearance::Invisible
    }
}

/// The style of a [`ModRangeInput`].
#[derive(Default)]
pub enum ModRangeInput {
    /// The default style.
    #[default]
    Default,
    /// The invisible style.
    Invisible,
    /// A custom style.
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}

impl<S> From<S> for ModRangeInput
where
    S: 'static + StyleSheet<Style = Theme>,
{
    fn from(val: S) -> Self {
        ModRangeInput::Custom(Box::new(val))
    }
}

impl StyleSheet for Theme {
    type Style = ModRangeInput;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            ModRangeInput::Default => Appearance::Circle(Default::default()),
            ModRangeInput::Invisible => Appearance::Invisible,
            ModRangeInput::Custom(custom) => custom.active(self),
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            ModRangeInput::Default => Appearance::Circle(CircleAppearance {
                color: default_colors::KNOB_BACK_HOVER,
                ..Default::default()
            }),
            ModRangeInput::Invisible => self.active(style),
            ModRangeInput::Custom(custom) => custom.active(self),
        }
    }

    fn dragging(&self, style: &Self::Style) -> Appearance {
        match style {
            ModRangeInput::Default => self.hovered(style),
            ModRangeInput::Invisible => self.active(style),
            ModRangeInput::Custom(custom) => custom.active(self),
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        match style {
            ModRangeInput::Default => Appearance::Circle(Default::default()),
            ModRangeInput::Invisible => Appearance::Invisible,
            ModRangeInput::Custom(custom) => custom.disabled(self),
        }
    }
}
