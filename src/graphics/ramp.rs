//! Display a ramp control that controls a [`Param`]. It is usually used to
//! represent the easing of a parameter between two points in time.
//!
//! [`Param`]: ../core/param/trait.Param.html

use crate::core::Normal;
use crate::native::ramp;
use iced::widget::canvas::{Frame, LineCap, Path, Stroke};
use iced::{Background, Point, Rectangle, Size, Vector};
use iced_graphics::triangle;
use iced_graphics::Primitive;

pub use crate::native::ramp::RampDirection;
pub use crate::style::ramp::{Appearance, StyleSheet};

/// A ramp GUI widget that controls a [`Param`]. It is usually used to
/// represent the easing of a parameter between two points in time.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`Ramp`]: struct.Ramp.html
pub type Ramp<'a, Message, Theme> =
    ramp::Ramp<'a, Message, iced::Renderer<Theme>>;

impl ramp::Renderer for iced::Renderer
where
    Self::Theme: StyleSheet,
{
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        style_sheet: &dyn StyleSheet<
            Style = <Self::Theme as StyleSheet>::Style,
        >,
        style: &<Self::Theme as StyleSheet>::Style,
        direction: RampDirection,
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

        let bounds_width = bounds.width.floor();
        let bounds_height = bounds.height.floor();

        let back = Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x,
                y: bounds_y,
                width: bounds_width,
                height: bounds_height,
            },
            background: Background::Color(appearance.back_color),
            border_radius: 0.0,
            border_width: appearance.back_border_width,
            border_color: appearance.back_border_color,
        };

        let border_width = appearance.back_border_width as f32;
        let twice_border_width = border_width * 2.0;

        let range_width = bounds_width - twice_border_width;
        let range_height = bounds_height - twice_border_width;

        let line: Primitive = match direction {
            RampDirection::Up => {
                if normal.as_f32() < 0.449 {
                    let stroke = Stroke {
                        width: appearance.line_width as f32,
                        style: triangle::Style::Solid(
                            appearance.line_down_color,
                        ),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control = Point::new(
                        range_width * (1.0 - (normal.as_f32() * 2.0)),
                        0.0,
                    );
                    let to = Point::new(range_width, -range_height);

                    let path = Path::new(|p| p.quadratic_curve_to(control, to));

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
                } else if normal.as_f32() > 0.501 {
                    let stroke = Stroke {
                        width: appearance.line_width as f32,
                        style: triangle::Style::Solid(appearance.line_up_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control = Point::new(
                        range_width * (1.0 - ((normal.as_f32() - 0.5) * 2.0)),
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
                        width: appearance.line_width as f32,
                        style: triangle::Style::Solid(
                            appearance.line_center_color,
                        ),
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
            }
            RampDirection::Down => {
                if normal.as_f32() < 0.449 {
                    let stroke = Stroke {
                        width: appearance.line_width as f32,
                        style: triangle::Style::Solid(
                            appearance.line_down_color,
                        ),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control =
                        Point::new(range_width * (normal.as_f32() * 2.0), 0.0);
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
                } else if normal.as_f32() > 0.501 {
                    let stroke = Stroke {
                        width: appearance.line_width as f32,
                        style: triangle::Style::Solid(appearance.line_up_color),
                        line_cap: LineCap::Square,
                        ..Stroke::default()
                    };

                    let control = Point::new(
                        range_width * ((normal.as_f32() - 0.5) * 2.0),
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
                        width: appearance.line_width as f32,
                        style: triangle::Style::Solid(
                            appearance.line_center_color,
                        ),
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
            }
        };

        self.draw_primitive(Primitive::Group {
            primitives: vec![back, line],
        })
    }
}
