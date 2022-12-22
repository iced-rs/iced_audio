//! Display an interactive dot that controls an [`Param`]
//!
//! [`Param`]: ../core/param/struct.Param.html

use iced_graphics::Primitive;
use iced_native::{Background, Point, Rectangle};

use crate::native::mod_range_input;
pub use crate::style::mod_range_input::{
    Appearance, CircleAppearance, SquareAppearance, StyleSheet,
};

/// An interactive dot that controls an [`Param`]
///
/// [`Param`]: ../core/param/struct.Param.html
pub type ModRangeInput<'a, Message, Theme> =
    mod_range_input::ModRangeInput<'a, Message, crate::Renderer<Theme>>;

impl<Theme> mod_range_input::Renderer for crate::Renderer<Theme>
where
    Self::Theme: StyleSheet,
{
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        is_dragging: bool,
        style_sheet: &dyn StyleSheet<
            Style = <Self::Theme as StyleSheet>::Style,
        >,
        style: &<Self::Theme as StyleSheet>::Style,
    ) {
        let is_mouse_over = bounds.contains(cursor_position);

        let appearance = if is_dragging {
            style_sheet.dragging(style)
        } else if is_mouse_over {
            style_sheet.hovered(style)
        } else {
            style_sheet.active(style)
        };

        let dot: Primitive = match appearance {
            Appearance::Circle(style) => {
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
                    border_radius: [radius; 4],
                    border_width: style.border_width,
                    border_color: style.border_color,
                }
            }
            Appearance::Square(style) => {
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
                    border_radius: [style.border_radius; 4],
                    border_width: style.border_width,
                    border_color: style.border_color,
                }
            }
            Appearance::Invisible => Primitive::None,
        };

        self.draw_primitive(dot)
    }
}
