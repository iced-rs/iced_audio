//! Display an interactive dot that controls an [`Param`]
//!
//! [`Param`]: ../core/param/struct.Param.html

use crate::native::mod_range_input;

use iced_graphics::{Backend, Primitive, Renderer};
use iced_native::{Background, Point, Rectangle};

pub use crate::native::mod_range_input::State;
pub use crate::style::mod_range_input::{
    CircleStyle, DefaultInvisible, SquareStyle, Style, StyleSheet,
};

/// An interactive dot that controls an [`Param`]
///
/// [`Param`]: ../core/param/struct.Param.html
pub type ModRangeInput<'a, Message, Backend> =
    mod_range_input::ModRangeInput<'a, Message, Renderer<Backend>>;

impl<B: Backend> mod_range_input::Renderer for Renderer<B> {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        is_dragging: bool,
        style_sheet: &Self::Style,
    ) {
        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_dragging {
            style_sheet.dragging()
        } else if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.active()
        };

        let dot: Primitive = match style {
            Style::Circle(style) => {
                let bounds_x = bounds.x.floor();
                let bounds_y = bounds.y.floor();
                let bounds_size = bounds.width.floor();

                let radius = bounds_size / 2.0;

                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: bounds_y,
                        width: bounds_size,
                        height: bounds_size,
                    },
                    background: Background::Color(style.color),
                    border_radius: radius,
                    border_width: style.border_width,
                    border_color: style.border_color,
                }
            }
            Style::Square(style) => {
                let bounds_x = bounds.x.floor();
                let bounds_y = bounds.y.floor();
                let bounds_size = bounds.width.floor();

                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: bounds_y,
                        width: bounds_size,
                        height: bounds_size,
                    },
                    background: Background::Color(style.color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: style.border_color,
                }
            }
            Style::Invisible => Primitive::None,
        };

        self.draw_primitive(dot)
    }
}
