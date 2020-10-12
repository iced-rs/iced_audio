//! Display an interactive 2D XY Pad that controls two [`Param`] parameters at
//! once. One in the `x` coordinate and one in the `y` coordinate.
//!
//! [`Param`]: ../core/param/trait.Param.html

use crate::core::Normal;
use crate::native::xy_pad;
use iced_graphics::{Backend, Primitive, Renderer};
use iced_native::{mouse, Background, Color, Point, Rectangle};

pub use crate::native::xy_pad::State;
pub use crate::style::xy_pad::{
    HandleCircle, HandleShape, HandleSquare, Style, StyleSheet,
};

/// A 2D XY pad GUI widget that controls two [`Param`] parameters at
/// once. One in the `x` coordinate and one in the `y` coordinate.
///
/// an [`XYPad`] will try to fill the space of its container while keeping a
/// square aspect ratio.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`XYPad`]: struct.XYPad.html
pub type XYPad<'a, Message, Backend> =
    xy_pad::XYPad<'a, Message, Renderer<Backend>>;

impl<B: Backend> xy_pad::Renderer for Renderer<B> {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal_x: Normal,
        normal_y: Normal,
        is_dragging: bool,
        style_sheet: &Self::Style,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_dragging {
            style_sheet.dragging()
        } else if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.active()
        };

        let bounds_x = bounds.x.floor();
        let bounds_y = bounds.y.floor();

        let bounds_size = {
            if bounds.width <= bounds.height {
                bounds.width.floor()
            } else {
                bounds.height.floor()
            }
        };

        let back = Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x,
                y: bounds_y,
                width: bounds_size,
                height: bounds_size,
            },
            background: Background::Color(style.back_color),
            border_radius: 0,
            border_width: style.border_width,
            border_color: style.border_color,
        };

        let handle_x = (bounds_x + (bounds_size * normal_x.as_f32())).floor();
        let handle_y =
            (bounds_y + (bounds_size * (1.0 - normal_y.as_f32()))).floor();

        let bounds_center = (bounds_size / 2.0).floor();

        let (h_center_line, v_center_line) = if style.center_line_color
            != Color::TRANSPARENT
        {
            let center_line_width = style.center_line_width as f32;
            let half_center_line_width = (center_line_width / 2.0).floor();

            (
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: bounds_y + bounds_center - half_center_line_width,
                        width: bounds_size,
                        height: center_line_width,
                    },
                    background: Background::Color(style.center_line_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                },
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + bounds_center - half_center_line_width,
                        y: bounds_y,
                        width: center_line_width,
                        height: bounds_size,
                    },
                    background: Background::Color(style.center_line_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                },
            )
        } else {
            (Primitive::None, Primitive::None)
        };

        let (h_rail, v_rail) = if style.rail_width != 0 {
            let rail_width = style.rail_width as f32;
            let half_rail_width = (rail_width / 2.0).floor();
            (
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: handle_y - half_rail_width,
                        width: bounds_size,
                        height: style.rail_width as f32,
                    },
                    background: Background::Color(style.h_rail_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                },
                Primitive::Quad {
                    bounds: Rectangle {
                        x: handle_x - half_rail_width,
                        y: bounds_y,
                        width: style.rail_width as f32,
                        height: bounds_size,
                    },
                    background: Background::Color(style.v_rail_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                },
            )
        } else {
            (Primitive::None, Primitive::None)
        };

        let handle = {
            match style.handle {
                HandleShape::Circle(circle) => {
                    let diameter = circle.diameter as f32;
                    let radius = diameter / 2.0;

                    Primitive::Quad {
                        bounds: Rectangle {
                            x: handle_x - radius,
                            y: handle_y - radius,
                            width: diameter,
                            height: diameter,
                        },
                        background: Background::Color(circle.color),
                        border_radius: radius as u16,
                        border_width: circle.border_width,
                        border_color: circle.border_color,
                    }
                }
                HandleShape::Square(square) => {
                    let size = square.size as f32;
                    let half_size = (size / 2.0).floor();

                    Primitive::Quad {
                        bounds: Rectangle {
                            x: handle_x - half_size,
                            y: handle_y - half_size,
                            width: size,
                            height: size,
                        },
                        background: Background::Color(square.color),
                        border_radius: square.border_radius,
                        border_width: square.border_width,
                        border_color: square.border_color,
                    }
                }
            }
        };

        (
            Primitive::Group {
                primitives: vec![
                    back,
                    h_center_line,
                    v_center_line,
                    h_rail,
                    v_rail,
                    handle,
                ],
            },
            mouse::Interaction::default(),
        )
    }
}
