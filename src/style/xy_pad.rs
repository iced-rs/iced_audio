//! Style for the [`XYPad`] widget
//! 
//! [`XYPad`]: ../native/xy_pad/struct.XYPad.html

use iced::Color;

/// The appearance of an [`XYPad`].
///
/// * `rail_width` - the width of the horizontal and vertical rail lines
/// * `h_rail_color` - color of the horizontal rail line
/// * `v_rail_color` - color of the vertical rail line
/// * `handle` - the [`HandleShape`] of the handle
/// * `back_color` - the color of the background square
/// * `border_width` - the width of the border of the background square
/// * `border_color` - the color of the border of the background square
/// * `center_line_width` - the width of the center line markings
/// * `center_line_color` - the color of the center line markings
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
/// * color - the color of the circle
/// * diameter - the diameter of the circle
/// * border_width - the width of the border of the circle
/// * border_color - the color of the border of the circle
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
/// * color - the color of the square
/// * size - the size of the square
/// * border_width - the width of the border of the square
/// * border_radius - the radius of the corners of the square
/// * border_color - the color of the border of the square
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

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            rail_width: 2,
            h_rail_color: [0.56, 0.56, 0.56, 0.75].into(),
            v_rail_color: [0.56, 0.56, 0.56, 0.75].into(),
            handle: HandleShape::Circle(HandleCircle {
                color: Color::from_rgb(0.97, 0.97, 0.97),
                diameter: 11,
                border_width: 2,
                border_color: Color::from_rgb(0.51, 0.51, 0.51),
            }),
            back_color: Color::from_rgb(0.97, 0.97, 0.97),
            border_width: 1,
            border_color: Color::from_rgb(0.51, 0.51, 0.51),
            center_line_width: 1,
            center_line_color: [0.56, 0.56, 0.56, 0.4].into(),
        }
    }

    fn hovered(&self) -> Style {
        let active = self.active();

        Style {
            handle: HandleShape::Circle(HandleCircle {
                color: Color::from_rgb(0.93, 0.93, 0.93),
                diameter: 11,
                border_width: 2,
                border_color: Color::from_rgb(0.51, 0.51, 0.51),
            }),
            ..active
        }
    }

    fn dragging(&self) -> Style {
        let active = self.active();

        Style {
            handle: HandleShape::Circle(HandleCircle {
                color: Color::from_rgb(0.92, 0.92, 0.92),
                diameter: 9,
                border_width: 2,
                border_color: Color::from_rgb(0.51, 0.51, 0.51),
            }),
            ..active
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