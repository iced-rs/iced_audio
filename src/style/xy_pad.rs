//! Style for the [`XYPad`] widget
//!
//! [`XYPad`]: ../native/xy_pad/struct.XYPad.html

use iced_native::Color;

use crate::style::default_colors;

/// The appearance of an [`XYPad`].
///
/// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
/// [`HandleShape`]: enum.HandleShape.html
#[derive(Debug, Clone)]
pub struct Style {
    /// the width of the horizontal and vertical rail lines
    pub rail_width: u16,
    /// color of the horizontal rail line
    pub h_rail_color: Color,
    /// color of the vertical rail line
    pub v_rail_color: Color,
    /// the [`HandleShape`] of the handle
    ///
    /// [`HandleShape`]: enum.HandleShape.html
    pub handle: HandleShape,
    /// the color of the background square
    pub back_color: Color,
    /// the width of the border of the background square
    pub border_width: u16,
    /// the color of the border of the background square
    pub border_color: Color,
    /// the width of the center line markings
    pub center_line_width: u16,
    /// the color of the center line markings
    pub center_line_color: Color,
}

/// The shape of the handle for the [`Style`] of an [`XYPad`]
///
/// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
/// [`Style`]: struct.Style.html
#[derive(Debug, Clone)]
pub enum HandleShape {
    /// a circular handle
    Circle(HandleCircle),
    /// a square handle
    Square(HandleSquare),
}

/// a circular handle style for the [`Style`] of an [`XYPad`]
///
/// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
/// [`Style`]: struct.Style.html
#[derive(Debug, Clone)]
pub struct HandleCircle {
    /// the color of the circle
    pub color: Color,
    /// the diameter of the circle
    pub diameter: u16,
    /// the width of the border of the circle
    pub border_width: u16,
    /// the color of the border of the circle
    pub border_color: Color,
}

/// a square handle style for the [`Style`] of an [`XYPad`]
///
/// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
/// [`Style`]: struct.Style.html
#[derive(Debug, Clone)]
pub struct HandleSquare {
    /// the color of the square
    pub color: Color,
    /// the size of the square
    pub size: u16,
    /// the width of the border of the square
    pub border_width: u16,
    /// the radius of the corners of the square
    pub border_radius: u16,
    /// the color of the border of the square
    pub border_color: Color,
}

/// A set of rules that dictate the style of an [`XYPad`].
///
/// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
pub trait StyleSheet {
    /// Produces the style of an active [`XYPad`].
    ///
    /// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
    fn active(&self) -> Style;

    /// Produces the style of a hovered [`XYPad`].
    ///
    /// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
    fn hovered(&self) -> Style;

    /// Produces the style of an [`XYPad`] that is being dragged.
    ///
    /// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
    fn dragging(&self) -> Style;
}

struct Default;
impl Default {
    const ACTIVE_HANDLE: HandleCircle = HandleCircle {
        color: default_colors::LIGHT_BACK,
        diameter: 11,
        border_width: 2,
        border_color: default_colors::BORDER,
    };
    const ACTIVE_STYLE: Style = Style {
        rail_width: 2,
        h_rail_color: default_colors::XY_PAD_RAIL,
        v_rail_color: default_colors::XY_PAD_RAIL,
        handle: HandleShape::Circle(Self::ACTIVE_HANDLE),
        back_color: default_colors::LIGHT_BACK,
        border_width: 1,
        border_color: default_colors::BORDER,
        center_line_width: 1,
        center_line_color: default_colors::XY_PAD_CENTER_LINE,
    };
}
impl StyleSheet for Default {
    fn active(&self) -> Style {
        Self::ACTIVE_STYLE
    }

    fn hovered(&self) -> Style {
        Style {
            handle: HandleShape::Circle(HandleCircle {
                color: default_colors::LIGHT_BACK_HOVER,
                ..Self::ACTIVE_HANDLE
            }),
            ..Self::ACTIVE_STYLE
        }
    }

    fn dragging(&self) -> Style {
        Style {
            handle: HandleShape::Circle(HandleCircle {
                color: default_colors::LIGHT_BACK_DRAG,
                diameter: 9,
                ..Self::ACTIVE_HANDLE
            }),
            ..Self::ACTIVE_STYLE
        }
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
