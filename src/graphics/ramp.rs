//! `iced_graphics` renderer for the [`Ramp`] widget
//!
//! [`Ramp`]: ../native/ramp/struct.Ramp.html

use crate::core::Normal;
use crate::native::ramp;
use iced_graphics::canvas::{Frame, LineCap, Path, Stroke};
use iced_graphics::{Backend, Primitive, Renderer};
use iced_native::{mouse, Background, Point, Rectangle, Size, Vector};

pub use crate::native::ramp::{RampDirection, State};
pub use crate::style::ramp::{Style, StyleSheet};

/// This is an alias of a `crate::native` [`Ramp`] with an
/// `iced_graphics::Renderer`.
///
/// [`Ramp`]: ../../native/ramp/struct.Ramp.html
pub type Ramp<'a, Message, ID, Backend> =
    ramp::Ramp<'a, Message, Renderer<Backend>, ID>;

impl<B: Backend> ramp::Renderer for Renderer<B> {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        style_sheet: &Self::Style,
        direction: RampDirection,
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

        let bounds_width = bounds.width.floor();
        let bounds_height = bounds.height.floor();

        let back = Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x,
                y: bounds_y,
                width: bounds_width,
                height: bounds_height,
            },
            background: Background::Color(style.back_color),
            border_radius: 0,
            border_width: style.back_border_width,
            border_color: style.back_border_color,
        };

        let border_width = style.back_border_width as f32;
        let twice_border_width = border_width * 2.0;

        let range_width = bounds_width - twice_border_width;
        let range_height = bounds_height - twice_border_width;

        let line: Primitive = match direction {
            RampDirection::Up => {
                let primitive = {
                    if normal.value() < 0.449 {
                        let stroke = Stroke {
                            width: style.line_width as f32,
                            color: style.line_down_color,
                            line_cap: LineCap::Square,
                            ..Stroke::default()
                        };

                        let control = Point::new(
                            range_width * (1.0 - (normal.value() * 2.0)),
                            0.0,
                        );
                        let to = Point::new(range_width, -range_height);

                        let path =
                            Path::new(|p| p.quadratic_curve_to(control, to));

                        let mut frame =
                            Frame::new(Size::new(range_width, range_height));

                        frame.translate(Vector::new(0.0, range_height));

                        frame.stroke(&path, stroke);

                        Primitive::Translate {
                            translation: Vector::new(
                                bounds_x + border_width,
                                bounds_y + border_width,
                            ),
                            content: Box::new(
                                frame.into_geometry().into_primitive(),
                            ),
                        }
                    } else if normal.value() > 0.501 {
                        let stroke = Stroke {
                            width: style.line_width as f32,
                            color: style.line_up_color,
                            line_cap: LineCap::Square,
                            ..Stroke::default()
                        };

                        let control = Point::new(
                            range_width
                                * (1.0 - ((normal.value() - 0.5) * 2.0)),
                            -range_height,
                        );
                        let to = Point::new(range_width, -range_height);

                        let path = Path::new(|p| {
                            p.move_to(to);
                            p.quadratic_curve_to(control, Point::ORIGIN)
                        });

                        let mut frame =
                            Frame::new(Size::new(range_width, range_height));

                        frame.translate(Vector::new(0.0, range_height));

                        frame.stroke(&path, stroke);

                        Primitive::Translate {
                            translation: Vector::new(
                                bounds_x + border_width,
                                bounds_y + border_width,
                            ),
                            content: Box::new(
                                frame.into_geometry().into_primitive(),
                            ),
                        }
                    } else {
                        let stroke = Stroke {
                            width: style.line_width as f32,
                            color: style.line_center_color,
                            line_cap: LineCap::Square,
                            ..Stroke::default()
                        };

                        let path = Path::line(
                            Point::new(0.0, 0.0),
                            Point::new(range_width, -range_height),
                        );

                        let mut frame =
                            Frame::new(Size::new(range_width, range_height));

                        frame.translate(Vector::new(0.0, range_height));

                        frame.stroke(&path, stroke);

                        Primitive::Translate {
                            translation: Vector::new(
                                bounds_x + border_width,
                                bounds_y + border_width,
                            ),
                            content: Box::new(
                                frame.into_geometry().into_primitive(),
                            ),
                        }
                    }
                };

                primitive
            }
            RampDirection::Down => {
                let primitive = {
                    if normal.value() < 0.449 {
                        let stroke = Stroke {
                            width: style.line_width as f32,
                            color: style.line_down_color,
                            line_cap: LineCap::Square,
                            ..Stroke::default()
                        };

                        let control = Point::new(
                            range_width * (normal.value() * 2.0),
                            0.0,
                        );
                        let from = Point::new(0.0, -range_height);
                        let to = Point::new(range_width, 0.0);

                        let path = Path::new(|p| {
                            p.move_to(from);
                            p.quadratic_curve_to(control, to)
                        });

                        let mut frame =
                            Frame::new(Size::new(range_width, range_height));

                        frame.translate(Vector::new(0.0, range_height));

                        frame.stroke(&path, stroke);

                        Primitive::Translate {
                            translation: Vector::new(
                                bounds_x + border_width,
                                bounds_y + border_width,
                            ),
                            content: Box::new(
                                frame.into_geometry().into_primitive(),
                            ),
                        }
                    } else if normal.value() > 0.501 {
                        let stroke = Stroke {
                            width: style.line_width as f32,
                            color: style.line_up_color,
                            line_cap: LineCap::Square,
                            ..Stroke::default()
                        };

                        let control = Point::new(
                            range_width * ((normal.value() - 0.5) * 2.0),
                            -range_height,
                        );
                        let from = Point::new(0.0, -range_height);
                        let to = Point::new(range_width, 0.0);

                        let path = Path::new(|p| {
                            p.move_to(to);
                            p.quadratic_curve_to(control, from)
                        });

                        let mut frame =
                            Frame::new(Size::new(range_width, range_height));

                        frame.translate(Vector::new(0.0, range_height));

                        frame.stroke(&path, stroke);

                        Primitive::Translate {
                            translation: Vector::new(
                                bounds_x + border_width,
                                bounds_y + border_width,
                            ),
                            content: Box::new(
                                frame.into_geometry().into_primitive(),
                            ),
                        }
                    } else {
                        let stroke = Stroke {
                            width: style.line_width as f32,
                            color: style.line_center_color,
                            line_cap: LineCap::Square,
                            ..Stroke::default()
                        };

                        let path = Path::line(
                            Point::new(0.0, -range_height),
                            Point::new(range_width, 0.0),
                        );

                        let mut frame =
                            Frame::new(Size::new(range_width, range_height));

                        frame.translate(Vector::new(0.0, range_height));

                        frame.stroke(&path, stroke);

                        Primitive::Translate {
                            translation: Vector::new(
                                bounds_x + border_width,
                                bounds_y + border_width,
                            ),
                            content: Box::new(
                                frame.into_geometry().into_primitive(),
                            ),
                        }
                    }
                };

                primitive
            }
        };

        (
            Primitive::Group {
                primitives: vec![back, line],
            },
            mouse::Interaction::default(),
        )
    }
}
