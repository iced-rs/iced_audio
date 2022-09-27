use iced::widget::canvas::{Fill, Frame, LineCap, Path, Stroke};
use iced_graphics::triangle;
use iced_graphics::Primitive;
use iced_native::{Color, Point, Size, Vector};

use super::PrimitiveCache;
use crate::core::Normal;
use crate::native::tick_marks;
use crate::style::tick_marks::{Shape, Style};

fn draw_radial_circles(
    frame: &mut Frame,
    offset_radius: f32,
    start_angle: f32,
    angle_span: f32,
    tick_marks: &[Normal],
    color: Color,
    radius: f32,
    inverse: bool,
) {
    let path = Path::circle(Point::new(0.0, -offset_radius), radius);

    if inverse {
        for tick_mark in tick_marks {
            let angle = start_angle + tick_mark.scale_inv(angle_span);

            frame.with_save(|frame| {
                if angle < -0.001 || angle > 0.001 {
                    frame.rotate(angle);
                }

                frame.fill(
                    &path,
                    Fill {
                        style: triangle::Style::Solid(color),
                        ..Fill::default()
                    },
                );
            });
        }
    } else {
        for tick_mark in tick_marks {
            let angle = start_angle + tick_mark.scale(angle_span);

            frame.with_save(|frame| {
                if angle < -0.001 || angle > 0.001 {
                    frame.rotate(angle);
                }

                frame.fill(
                    &path,
                    Fill {
                        style: triangle::Style::Solid(color),
                        ..Fill::default()
                    },
                );
            });
        }
    }
}

fn draw_radial_lines(
    frame: &mut Frame,
    offset_radius: f32,
    start_angle: f32,
    angle_span: f32,
    tick_marks: &[Normal],
    color: Color,
    width: f32,
    length: f32,
    inverse: bool,
) {
    let path = Path::line(
        Point::new(0.0, -offset_radius),
        Point::new(0.0, -offset_radius - length),
    );

    if inverse {
        for tick_mark in tick_marks {
            let angle = start_angle + tick_mark.scale_inv(angle_span);

            frame.with_save(|frame| {
                if angle < -0.001 || angle > 0.001 {
                    frame.rotate(angle);
                }

                frame.stroke(
                    &path,
                    Stroke {
                        width,
                        style: triangle::Style::Solid(color),
                        line_cap: LineCap::Butt,
                        ..Stroke::default()
                    },
                );
            });
        }
    } else {
        for tick_mark in tick_marks {
            let angle = start_angle + tick_mark.scale(angle_span);

            frame.with_save(|frame| {
                if angle < -0.001 || angle > 0.001 {
                    frame.rotate(angle);
                }

                frame.stroke(
                    &path,
                    Stroke {
                        width,
                        style: triangle::Style::Solid(color),
                        line_cap: LineCap::Butt,
                        ..Stroke::default()
                    },
                );
            });
        }
    }
}

#[inline]
fn draw_tier(
    frame: &mut Frame,
    offset_radius: f32,
    start_angle: f32,
    angle_span: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    inside: bool,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => return,
            Shape::Line {
                length,
                width,
                color,
            } => {
                let length = f32::from(*length);
                let width = f32::from(*width);

                if inside {
                    draw_radial_lines(
                        frame,
                        offset_radius - length,
                        start_angle,
                        angle_span,
                        tick_marks,
                        *color,
                        width,
                        length,
                        inverse,
                    );
                } else {
                    draw_radial_lines(
                        frame,
                        offset_radius,
                        start_angle,
                        angle_span,
                        tick_marks,
                        *color,
                        width,
                        length,
                        inverse,
                    );
                }
            }
            Shape::Circle { diameter, color } => {
                let radius = f32::from(*diameter) / 2.0;

                if inside {
                    draw_radial_circles(
                        frame,
                        offset_radius - radius,
                        start_angle,
                        angle_span,
                        tick_marks,
                        *color,
                        radius,
                        inverse,
                    );
                } else {
                    draw_radial_circles(
                        frame,
                        offset_radius + radius,
                        start_angle,
                        angle_span,
                        tick_marks,
                        *color,
                        radius,
                        inverse,
                    );
                }
            }
        }
    }
}

fn max_length(style: &Style) -> f32 {
    let length_1 = match style.tier_1 {
        Shape::None => 0.0,
        Shape::Line { length, .. } => length,
        Shape::Circle { diameter, .. } => diameter,
    };

    let length_2 = match style.tier_1 {
        Shape::None => 0.0,
        Shape::Line { length, .. } => length,
        Shape::Circle { diameter, .. } => diameter,
    };

    let length_3 = match style.tier_1 {
        Shape::None => 0.0,
        Shape::Line { length, .. } => length,
        Shape::Circle { diameter, .. } => diameter,
    };

    f32::from(length_1.max(length_2).max(length_3))
}

/// Draws tick marks around an arc.
///
/// * `center` - The center point of the arc.
/// * `radius` - The radius of the arc where the tick marks start
/// * `start_angle` - The starting angle of the arc in radians
/// * `angle_span` - The span of the angle in radians
/// * `inside` - Whether to place the tick marks inside the radius (true),
/// or outside the radius (false).
/// * `tick_marks` - The group of tick marks.
/// * `style` - The tick marks style.
/// * `inverse` - Whether to inverse the positions of the tick marks (true) or
/// not (false).
pub fn draw_radial_tick_marks(
    center: Point,
    radius: f32,
    start_angle: f32,
    angle_span: f32,
    inside: bool,
    tick_marks: &tick_marks::Group,
    style: &Style,
    inverse: bool,
    cache: &PrimitiveCache,
) -> Primitive {
    cache.cached_radial(
        center,
        radius,
        start_angle,
        angle_span,
        inside,
        tick_marks,
        *style,
        inverse,
        || {
            let frame_radius = if inside {
                radius
            } else {
                radius + max_length(style)
            };

            let frame_size = frame_radius * 2.0;

            let mut frame = Frame::new(Size::new(frame_size, frame_size));

            frame.translate(Vector::new(frame_radius, frame_radius));

            draw_tier(
                &mut frame,
                radius,
                start_angle,
                angle_span,
                tick_marks.tier_1(),
                &style.tier_1,
                inside,
                inverse,
            );
            draw_tier(
                &mut frame,
                radius,
                start_angle,
                angle_span,
                tick_marks.tier_2(),
                &style.tier_2,
                inside,
                inverse,
            );
            draw_tier(
                &mut frame,
                radius,
                start_angle,
                angle_span,
                tick_marks.tier_3(),
                &style.tier_3,
                inside,
                inverse,
            );

            Primitive::Translate {
                translation: Vector::new(
                    center.x - frame_radius,
                    center.y - frame_radius,
                ),
                content: Box::new(frame.into_geometry().into_primitive()),
            }
        },
    )
}
