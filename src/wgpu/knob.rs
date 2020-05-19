//! wgpu renderer for the [`Knob`] widget
//!
//! [`Knob`]: struct.Knob.html

use crate::core::{Normal};
use crate::native::knob;
use iced_native::{
    Background, MouseCursor, Point, Rectangle, Color
};
use iced_wgpu::{Primitive, Renderer};
//use iced_wgpu::widget::canvas::{Frame, Path, Stroke, LineCap};

pub use crate::native::knob::State;
pub use crate::style::knob::{Style, StyleSheet, VectorStyle, InnerCircle};

/// This is an alias of a `crate::native` HSlider with an `iced_wgpu::Renderer`.
pub type Knob<'a, Message, ID> =
    knob::Knob<'a, Message, Renderer, ID>;

impl knob::Renderer for Renderer {
    type Style = Box<dyn StyleSheet>;

    fn diameter(&self, style_sheet: &Self::Style) -> u16 {
        style_sheet.diameter()
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        style_sheet: &Self::Style,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let angle_range = style_sheet.angle_range();

        let style = if is_dragging {
            style_sheet.dragging()
        } else if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.active()
        };

        let bounds_x = bounds.x.floor();
        let bounds_y = bounds.y.floor();

        let bounds_size = bounds.width.floor();

        match style {



            Style::Texture => {


                // not implemented yet

                (
                    Primitive::None,
                    MouseCursor::default(),
                )
            }


            /*
            Style::Vector(style) => {


                let radius = bounds_size / 2.0;

                let knob_back = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: bounds_y,
                        width: bounds_size,
                        height: bounds_size,
                    },
                    background: Background::Color(style.knob_color),
                    border_radius: radius as u16,
                    border_width: style.knob_border_width,
                    border_color: style.knob_border_color,
                };

                let mut angle = ( (angle_range.max() - angle_range.min())
                                * normal.value()
                            ) + angle_range.min() + std::f32::consts::PI;
                
                if angle >= TAU { angle -= TAU; }

                let notch: Primitive = {
                    let stroke = Stroke {
                        width: style.notch_width as f32,
                        color: style.notch_color,
                        line_cap: LineCap::Round,
                        ..Stroke::default()
                    };

                    let stroke_begin_y = -( radius
                        - style.notch_offset as f32 );

                    let path = Path::line(
                        Point::new(0.0, stroke_begin_y),
                        Point::new(0.0, stroke_begin_y
                            + style.notch_height as f32)
                    );

                    let mut frame = Frame::new(bounds_size, bounds_size);
                    frame.translate(Vector::new(
                        bounds_x + radius,
                        bounds_y + radius,
                    ));

                    if angle < -0.001 || angle > 0.001 {
                        frame.rotate(angle);
                    }

                    frame.stroke(&path, stroke);

                    frame.into_primitive()
                };

                if let Some(inner_circle) = style.inner_circle {
                    let inner_radius = radius * inner_circle.scale;
                    let diameter = inner_radius * 2.0;
                    let offset = radius - inner_radius;

                    let inner_circle = Primitive::Quad {
                        bounds: Rectangle {
                            x: bounds_x + offset,
                            y: bounds_y + offset,
                            width: diameter,
                            height: diameter,
                        },
                        background: Background::Color(inner_circle.color),
                        border_radius: inner_radius as u16,
                        border_width: inner_circle.border_width,
                        border_color: inner_circle.border_color,
                    };

                    (
                        Primitive::Group {
                            primitives: vec![knob_back, inner_circle, notch],
                        },
                        MouseCursor::default(),
                    )
                } else {
                    (
                        Primitive::Group {
                            primitives: vec![knob_back, notch],
                        },
                        MouseCursor::default(),
                    )
                }
            }
            */



            Style::VectorCircle(style) => {


                let radius = bounds_size / 2.0;

                let knob_back = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x,
                        y: bounds_y,
                        width: bounds_size,
                        height: bounds_size,
                    },
                    background: Background::Color(style.knob_color),
                    border_radius: radius as u16,
                    border_width: style.knob_border_width,
                    border_color: style.knob_border_color,
                };

                let angle = ( (angle_range.max() - angle_range.min())
                                * normal.value()
                            ) + angle_range.min() + std::f32::consts::PI;
                
                let (dx, dy) = {
                    if angle < -0.001 || angle > 0.001 { angle.sin_cos() }
                    else { (0.0, -1.0) }
                };

                let notch_radius = style.notch_diameter as f32 / 2.0;
                let offset_radius = radius - style.notch_offset as f32;

                let notch = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + radius + (dx * offset_radius)
                            - notch_radius,
                        y: bounds_y + radius - (dy * offset_radius)
                            - notch_radius,
                        width: style.notch_diameter as f32,
                        height: style.notch_diameter as f32,
                    },
                    background: Background::Color(style.notch_color),
                    border_radius: notch_radius as u16,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                };

                (
                    Primitive::Group {
                        primitives: vec![knob_back, notch],
                    },
                    MouseCursor::default(),
                )
            }
        }
    }
}