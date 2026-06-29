//! Various styles for the [`Ramp`] widget
//!
//! [`Ramp`]: ../native/ramp/struct.Ramp.html

use crate::virtual_slider::Status;
use iced_core::{Background, Border, Color, theme::palette};

/// The appearance of a [`Ramp`].
///
/// [`Ramp`]: ../../native/ramp/struct.Ramp.html
#[derive(Debug, Clone)]
pub struct Style {
    /// The [`Background`] of the ramp.
    ///
    /// Default is `None`.
    pub background: Option<Background>,
    /// The [`Border`] of the ramp's background.
    ///
    /// Default is `Border::default()`.
    pub border: Border,
    /// The width of the ramp line.
    ///
    /// Default is `2.0`.
    pub line_width: f32,
    /// The color of the ramp line.
    ///
    /// Default is [`Color::BLACK`].
    pub line_color: Color,
    /// The color of the ramp line when it is in the up position. If `None`,
    /// then `line_color` is used.
    ///
    /// Default is `None`.
    pub line_up_color: Option<Color>,
    /// The color of the ramp line when it is in the down position. If `None`,
    /// then `line_color` is used.
    ///
    /// Default is `None`.
    pub line_down_color: Option<Color>,
}

impl Style {
    /// Updates the [`Style`] with the given [`Background`].
    pub fn with_background(self, background: impl Into<Background>) -> Self {
        Self {
            background: Some(background.into()),
            ..self
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            background: None,
            border: Border::default(),
            line_width: 2.0,
            line_color: Color::BLACK,
            line_up_color: None,
            line_down_color: None,
        }
    }
}

/// The theme catalog of a [`Ramp`].
///
/// [`Ramp`]: ../../native/ramp/struct.Ramp.html
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Ramp`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for iced_core::Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`Ramp`].
pub fn default(theme: &iced_core::Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    match status {
        Status::Idle => styled(palette.background.neutral),
        Status::Hovered | Status::Gesturing => styled(palette.background.stronger),
        Status::Disabled => disabled(styled(palette.background.neutral)),
    }
}

fn styled(pair: palette::Pair) -> Style {
    Style {
        background: Some(Background::Color(pair.color)),
        line_color: pair.text,
        ..Style::default()
    }
}

fn disabled(style: Style) -> Style {
    Style {
        background: style.background.map(|b| b.scale_alpha(0.5)),
        border: Border {
            color: style.border.color.scale_alpha(0.5),
            ..style.border
        },
        line_color: style.line_color.scale_alpha(0.5),
        line_up_color: style.line_up_color.map(|c| c.scale_alpha(0.5)),
        line_down_color: style.line_down_color.map(|c| c.scale_alpha(0.5)),
        ..style
    }
}
