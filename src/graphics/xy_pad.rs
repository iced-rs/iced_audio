//! Display an interactive 2D XY Pad that controls two [`Param`] parameters at
//! once. One in the `x` coordinate and one in the `y` coordinate.
//!
//! [`Param`]: ../core/param/trait.Param.html

use crate::core::Normal;
use crate::native::xy_pad;
use iced_graphics::Primitive;
use iced_native::{Background, Color, Point, Rectangle};

pub use crate::style::xy_pad::{
    Appearance, HandleCircle, HandleShape, HandleSquare, StyleSheet,
};

/// A 2D XY pad GUI widget that controls two [`Param`] parameters at
/// once. One in the `x` coordinate and one in the `y` coordinate.
///
/// an [`XYPad`] will try to fill the space of its container while keeping a
/// square aspect ratio.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`XYPad`]: struct.XYPad.html
pub type XYPad<'a, Message, Theme> =
    xy_pad::XYPad<'a, Message, crate::Renderer<Theme>>;

impl<Theme> xy_pad::Renderer for crate::Renderer<Theme>
where
    Self::Theme: StyleSheet,
{
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal_x: Normal,
        normal_y: Normal,
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
            background: Background::Color(appearance.back_color),
            border_radius: 0.0,
            border_width: appearance.border_width,
            border_color: appearance.border_color,
        };

        let handle_x = (bounds_x + (bounds_size * normal_x.as_f32())).floor();
        let handle_y =
            (bounds_y + (bounds_size * (1.0 - normal_y.as_f32()))).floor();

        let bounds_center = (bounds_size / 2.0).floor();

        let (h_center_line, v_center_line) = if appearance.center_line_color
            != Color::TRANSPARENT
        {
            let center_line_width = appearance.center_line_width as f32;
            let half_center_line_width = (center_line_width / 2.0).floor();

            (
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: bounds_y + bounds_center - half_center_line_width,
                        width: bounds_size,
                        height: center_line_width,
                    },
                    background: Background::Color(appearance.center_line_color),
                    border_radius: 0.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + bounds_center - half_center_line_width,
                        y: bounds_y,
                        width: center_line_width,
                        height: bounds_size,
                    },
                    background: Background::Color(appearance.center_line_color),
                    border_radius: 0.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            )
        } else {
            (Primitive::None, Primitive::None)
        };

        let (h_rail, v_rail) = if appearance.rail_width != 0.0 {
            let rail_width = appearance.rail_width as f32;
            let half_rail_width = (rail_width / 2.0).floor();
            (
                Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: handle_y - half_rail_width,
                        width: bounds_size,
                        height: appearance.rail_width as f32,
                    },
                    background: Background::Color(appearance.h_rail_color),
                    border_radius: 0.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                Primitive::Quad {
                    bounds: Rectangle {
                        x: handle_x - half_rail_width,
                        y: bounds_y,
                        width: appearance.rail_width as f32,
                        height: bounds_size,
                    },
                    background: Background::Color(appearance.v_rail_color),
                    border_radius: 0.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            )
        } else {
            (Primitive::None, Primitive::None)
        };

        let handle = {
            match appearance.handle {
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
                        border_radius: radius,
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

        self.draw_primitive(Primitive::Group {
            primitives: vec![
                back,
                h_center_line,
                v_center_line,
                h_rail,
                v_rail,
                handle,
            ],
        })
    }
}
