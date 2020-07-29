//! `iced_wgpu` renderer for the [`Knob`] widget
//!
//! [`Knob`]: ../native/knob/struct.Knob.html

use crate::core::{ModulationRange, Normal, TextMarkGroup, TickMarkGroup};
use crate::native::knob;
use iced_native::{
    Background, Color, HorizontalAlignment, MouseCursor, Point, Rectangle,
    Vector, VerticalAlignment,
};
use iced_wgpu::canvas::{path::Arc, Frame, LineCap, Path, Stroke};
use iced_wgpu::{Primitive, Renderer};

pub use crate::native::knob::State;
pub use crate::style::knob::{
    ArcBipolarNotch, ArcBipolarStyle, ArcNotch, ArcStyle, CircleTickMarkStyle,
    ClassicCircleStyle, ClassicLineStyle, LineTickMarkStyle, ModRangeRingStyle,
    Style, StyleSheet, TextMarkStyle, TickMarkStyle, ValueRingStyle,
};

/// This is an alias of a `crate::native` [`Knob`] with an
/// `iced_wgpu::Renderer`.
///
/// [`Knob`]: ../../native/knob/struct.Knob.html
pub type Knob<'a, Message, ID> = knob::Knob<'a, Message, Renderer, ID>;

impl knob::Renderer for Renderer {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        mod_range: Option<ModulationRange>,
        tick_marks: Option<&TickMarkGroup>,
        text_marks: Option<&TextMarkGroup>,
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

        let radius = bounds_size / 2.0;

        let mut start_angle = angle_range.min() + std::f32::consts::FRAC_PI_2;
        if start_angle >= crate::TAU {
            start_angle -= crate::TAU
        }

        let angle_span = angle_range.max() - angle_range.min();

        let value_ring: Primitive = {
            if let Some(style) = style_sheet.value_ring_style() {
                draw_value_ring(
                    start_angle,
                    angle_span,
                    normal,
                    bounds_x,
                    bounds_y,
                    bounds_size,
                    radius,
                    &style,
                )
            } else {
                Primitive::None
            }
        };

        let mod_range_ring: Primitive = {
            if let Some(mod_range) = mod_range {
                if mod_range.visible {
                    if let Some(style) = style_sheet.mod_range_ring_style() {
                        draw_mod_range_ring(
                            start_angle,
                            angle_span,
                            bounds_x,
                            bounds_y,
                            bounds_size,
                            radius,
                            &style,
                            &mod_range,
                        )
                    } else {
                        Primitive::None
                    }
                } else {
                    Primitive::None
                }
            } else {
                Primitive::None
            }
        };

        let tick_marks: Primitive = {
            if let Some(tick_marks) = tick_marks {
                if let Some(style) = style_sheet.tick_mark_style() {
                    match style {
                        TickMarkStyle::Circle(style) => draw_circle_tick_marks(
                            start_angle,
                            angle_span,
                            radius,
                            bounds_x,
                            bounds_y,
                            &tick_marks,
                            &style,
                        ),
                        TickMarkStyle::Line(style) => draw_line_tick_marks(
                            start_angle,
                            angle_span,
                            radius,
                            bounds_x,
                            bounds_y,
                            bounds_size,
                            &tick_marks,
                            &style,
                        ),
                    }
                } else {
                    Primitive::None
                }
            } else {
                Primitive::None
            }
        };

        let text_marks: Primitive = {
            if let Some(text_marks) = text_marks {
                if let Some(style) = style_sheet.text_mark_style() {
                    draw_text_marks(
                        &style,
                        bounds_x,
                        bounds_y,
                        radius,
                        start_angle,
                        angle_span,
                        text_marks,
                    )
                } else {
                    Primitive::None
                }
            } else {
                Primitive::None
            }
        };

        (
            match style {
                Style::ClassicCircle(style) => draw_classic_circle_style(
                    start_angle,
                    angle_span,
                    bounds_x,
                    bounds_y,
                    bounds_size,
                    radius,
                    normal,
                    &style,
                    value_ring,
                    mod_range_ring,
                    tick_marks,
                    text_marks,
                ),
                Style::ClassicLine(style) => draw_classic_line_style(
                    start_angle,
                    angle_span,
                    bounds_x,
                    bounds_y,
                    bounds_size,
                    radius,
                    normal,
                    &style,
                    value_ring,
                    mod_range_ring,
                    tick_marks,
                    text_marks,
                ),
                Style::Arc(style) => draw_arc_style(
                    start_angle,
                    angle_span,
                    bounds_x,
                    bounds_y,
                    bounds_size,
                    radius,
                    normal,
                    &style,
                    value_ring,
                    mod_range_ring,
                    tick_marks,
                    text_marks,
                ),
                Style::ArcBipolar(style) => draw_arc_bipolar_style(
                    start_angle,
                    angle_span,
                    bounds_x,
                    bounds_y,
                    bounds_size,
                    radius,
                    normal,
                    &style,
                    value_ring,
                    mod_range_ring,
                    tick_marks,
                    text_marks,
                ),
            },
            MouseCursor::default(),
        )
    }
}

fn draw_value_ring(
    start_angle: f32,
    angle_span: f32,
    normal: Normal,
    bounds_x: f32,
    bounds_y: f32,
    bounds_size: f32,
    radius: f32,
    style: &ValueRingStyle,
) -> Primitive {
    let fill_angle_span = angle_span * normal.value();
    let half_angle_span = angle_span / 2.0;
    let half_width = style.width / 2.0;

    let frame_size = bounds_size + style.offset + half_width;

    let mut frame = Frame::new(frame_size, frame_size);
    frame.translate(Vector::new(bounds_x, bounds_y));

    let center_point = Point::new(radius, radius);
    let arc_radius = radius + style.offset + half_width;

    let empty_stroke = Stroke {
        width: style.width,
        color: style.empty_color,
        line_cap: LineCap::Butt,
        ..Stroke::default()
    };

    let empty_arc = Arc {
        center: center_point,
        radius: arc_radius,
        start_angle: start_angle,
        end_angle: angle_span,
    };

    let empty_path = Path::new(|path| path.arc(empty_arc));

    frame.stroke(&empty_path, empty_stroke);

    if let Some(right_filled_color) = style.right_filled_color {
        if normal.value() < 0.5 {
            let filled_stroke = Stroke {
                width: style.width,
                color: style.left_filled_color,
                line_cap: LineCap::Butt,
                ..Stroke::default()
            };

            let filled_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle: start_angle + fill_angle_span,
                end_angle: half_angle_span - fill_angle_span,
            };

            let filled_path = Path::new(|path| path.arc(filled_arc));

            frame.stroke(&filled_path, filled_stroke);
        } else {
            let filled_stroke = Stroke {
                width: style.width,
                color: right_filled_color,
                line_cap: LineCap::Butt,
                ..Stroke::default()
            };

            let filled_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle: start_angle + half_angle_span,
                end_angle: fill_angle_span - half_angle_span,
            };

            let filled_path = Path::new(|path| path.arc(filled_arc));

            frame.stroke(&filled_path, filled_stroke);
        }
    } else {
        let center_point = Point::new(radius, radius);
        let arc_radius = radius + style.offset + half_width;

        let empty_arc = Arc {
            center: center_point,
            radius: arc_radius,
            start_angle: start_angle + fill_angle_span,
            end_angle: angle_span - fill_angle_span,
        };

        let empty_path = Path::new(|path| path.arc(empty_arc));

        frame.stroke(&empty_path, empty_stroke);

        if normal.value() != 0.0 {
            let filled_stroke = Stroke {
                width: style.width,
                color: style.left_filled_color,
                line_cap: LineCap::Butt,
                ..Stroke::default()
            };

            let filled_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle,
                end_angle: fill_angle_span,
            };

            let filled_path = Path::new(|path| path.arc(filled_arc));

            frame.stroke(&filled_path, filled_stroke);
        }
    }

    frame.into_primitive()
}

fn draw_mod_range_ring(
    start_angle: f32,
    angle_span: f32,
    bounds_x: f32,
    bounds_y: f32,
    bounds_size: f32,
    radius: f32,
    style: &ModRangeRingStyle,
    mod_range: &ModulationRange,
) -> Primitive {
    let half_width = style.width / 2.0;

    let center_point = Point::new(radius, radius);
    let arc_radius = radius + style.offset + half_width;

    let frame_size = bounds_size + style.offset + half_width;

    let mut frame = Frame::new(frame_size, frame_size);
    frame.translate(Vector::new(bounds_x, bounds_y));

    if let Some(empty_color) = style.empty_color {
        let empty_stroke = Stroke {
            width: style.width,
            color: empty_color,
            line_cap: LineCap::Butt,
            ..Stroke::default()
        };

        let empty_arc = Arc {
            center: center_point,
            radius: arc_radius,
            start_angle: start_angle,
            end_angle: angle_span,
        };

        let empty_path = Path::new(|path| path.arc(empty_arc));

        frame.stroke(&empty_path, empty_stroke);
    }

    if mod_range.filled_visible
        && (mod_range.start.value() != mod_range.end.value())
    {
        let (start, end, color) =
            if mod_range.start.value() < mod_range.end.value() {
                (
                    mod_range.start.value(),
                    mod_range.end.value(),
                    style.filled_color,
                )
            } else {
                (
                    mod_range.end.value(),
                    mod_range.start.value(),
                    style.filled_inverse_color,
                )
            };

        let start_span = angle_span * start;
        let span = (angle_span * end) - start_span;

        let filled_stroke = Stroke {
            width: style.width,
            color: color,
            line_cap: LineCap::Butt,
            ..Stroke::default()
        };

        let filled_arc = Arc {
            center: center_point,
            radius: arc_radius,
            start_angle: start_angle + start_span,
            end_angle: span,
        };

        let filled_path = Path::new(|path| path.arc(filled_arc));

        frame.stroke(&filled_path, filled_stroke);
    }

    frame.into_primitive()
}

fn draw_classic_circle_style(
    start_angle: f32,
    angle_span: f32,
    bounds_x: f32,
    bounds_y: f32,
    bounds_size: f32,
    radius: f32,
    normal: Normal,
    style: &ClassicCircleStyle,
    value_ring: Primitive,
    mod_range_ring: Primitive,
    tick_marks: Primitive,
    text_marks: Primitive,
) -> Primitive {
    let start_angle = start_angle + std::f32::consts::FRAC_PI_2;

    let knob_back = Primitive::Quad {
        bounds: Rectangle {
            x: bounds_x,
            y: bounds_y,
            width: bounds_size,
            height: bounds_size,
        },
        background: Background::Color(style.color),
        border_radius: radius as u16,
        border_width: style.border_width,
        border_color: style.border_color,
    };

    let angle = (angle_span * normal.value()) + start_angle;

    let (dx, dy) = {
        if angle < -0.001 || angle > 0.001 {
            angle.sin_cos()
        } else {
            (0.0, -1.0)
        }
    };

    let notch_radius = radius * style.notch_scale.value();

    let offset_radius = (radius - (notch_radius * 2.0))
        * (1.0 - style.notch_offset.value())
        + notch_radius;

    let notch_diameter = notch_radius * 2.0;

    let radius_offset = radius - notch_radius;

    let notch = Primitive::Quad {
        bounds: Rectangle {
            x: bounds_x + radius_offset + (dx * offset_radius),
            y: bounds_y + radius_offset - (dy * offset_radius),
            width: notch_diameter,
            height: notch_diameter,
        },
        background: Background::Color(style.notch_color),
        border_radius: notch_radius as u16,
        border_width: style.notch_border_width,
        border_color: style.notch_border_color,
    };

    Primitive::Group {
        primitives: vec![
            tick_marks,
            text_marks,
            value_ring,
            mod_range_ring,
            knob_back,
            notch,
        ],
    }
}

fn draw_classic_line_style(
    angle_start: f32,
    angle_span: f32,
    bounds_x: f32,
    bounds_y: f32,
    bounds_size: f32,
    radius: f32,
    normal: Normal,
    style: &ClassicLineStyle,
    value_ring: Primitive,
    mod_range_ring: Primitive,
    tick_marks: Primitive,
    text_marks: Primitive,
) -> Primitive {
    let angle_start = angle_start + std::f32::consts::FRAC_PI_2;

    let knob_back = Primitive::Quad {
        bounds: Rectangle {
            x: bounds_x,
            y: bounds_y,
            width: bounds_size,
            height: bounds_size,
        },
        background: Background::Color(style.color),
        border_radius: radius as u16,
        border_width: style.border_width,
        border_color: style.border_color,
    };

    let angle = (angle_span * normal.value()) + angle_start;

    let notch: Primitive = {
        let stroke = Stroke {
            width: style.notch_width as f32,
            color: style.notch_color,
            line_cap: LineCap::Butt,
            ..Stroke::default()
        };

        let stroke_begin_y = -(radius - (style.notch_offset.value() * radius));
        let notch_height = style.notch_scale.value() * radius;

        let path = Path::line(
            Point::new(0.0, stroke_begin_y),
            Point::new(0.0, stroke_begin_y + notch_height),
        );

        let mut frame = Frame::new(bounds_size, bounds_size);
        frame.translate(Vector::new(bounds_x + radius, bounds_y + radius));

        if angle < -0.001 || angle > 0.001 {
            frame.rotate(angle);
        }

        frame.stroke(&path, stroke);

        frame.into_primitive()
    };

    Primitive::Group {
        primitives: vec![
            tick_marks,
            text_marks,
            value_ring,
            mod_range_ring,
            knob_back,
            notch,
        ],
    }
}

fn draw_arc_style(
    angle_start: f32,
    angle_span: f32,
    bounds_x: f32,
    bounds_y: f32,
    bounds_size: f32,
    radius: f32,
    normal: Normal,
    style: &ArcStyle,
    value_ring: Primitive,
    mod_range_ring: Primitive,
    tick_marks: Primitive,
    text_marks: Primitive,
) -> Primitive {
    let fill_angle_span = angle_span * normal.value();

    let arc: Primitive = {
        let center_point = Point::new(radius, radius);
        let arc_radius = radius - (style.width / 2.0);

        let filled_stroke = Stroke {
            width: style.width,
            color: style.filled_color,
            line_cap: LineCap::Butt,
            ..Stroke::default()
        };

        let filled_arc = Arc {
            center: center_point,
            radius: arc_radius,
            start_angle: angle_start,
            end_angle: fill_angle_span,
        };

        let filled_path = Path::new(|path| path.arc(filled_arc));

        let empty_stroke = Stroke {
            width: style.width,
            color: style.empty_color,
            line_cap: LineCap::Butt,
            ..Stroke::default()
        };

        let empty_arc = Arc {
            center: center_point,
            radius: arc_radius,
            start_angle: angle_start,
            end_angle: angle_span,
        };

        let empty_path = Path::new(|path| path.arc(empty_arc));

        let mut frame = Frame::new(bounds_size, bounds_size);
        frame.translate(Vector::new(bounds_x, bounds_y));

        frame.stroke(&empty_path, empty_stroke);
        frame.stroke(&filled_path, filled_stroke);

        if let Some(notch) = &style.notch {
            let angle =
                angle_start + fill_angle_span + std::f32::consts::FRAC_PI_2;

            let stroke = Stroke {
                width: notch.width,
                color: notch.color,
                line_cap: LineCap::Square,
                ..Stroke::default()
            };

            let stroke_begin_y = -arc_radius;
            let notch_height = notch.length_scale.value() * arc_radius;

            let path = Path::line(
                Point::new(0.0, stroke_begin_y),
                Point::new(0.0, stroke_begin_y + notch_height),
            );

            frame.translate(Vector::new(radius, radius));

            if angle < -0.001 || angle > 0.001 {
                frame.rotate(angle);
            }

            frame.stroke(&path, stroke);
        }

        frame.into_primitive()
    };

    Primitive::Group {
        primitives: vec![
            tick_marks,
            text_marks,
            value_ring,
            mod_range_ring,
            arc,
        ],
    }
}

fn draw_arc_bipolar_style(
    angle_start: f32,
    angle_span: f32,
    bounds_x: f32,
    bounds_y: f32,
    bounds_size: f32,
    radius: f32,
    normal: Normal,
    style: &ArcBipolarStyle,
    value_ring: Primitive,
    mod_range_ring: Primitive,
    tick_marks: Primitive,
    text_marks: Primitive,
) -> Primitive {
    let fill_angle_span = angle_span * normal.value();
    let half_angle_span = angle_span / 2.0;

    let arc: Primitive = {
        let mut frame = Frame::new(bounds_size, bounds_size);
        frame.translate(Vector::new(bounds_x, bounds_y));

        let center_point = Point::new(radius, radius);
        let arc_radius = radius - (style.width / 2.0);

        let empty_stroke = Stroke {
            width: style.width,
            color: style.empty_color,
            line_cap: LineCap::Butt,
            ..Stroke::default()
        };

        let empty_arc = Arc {
            center: center_point,
            radius: arc_radius,
            start_angle: angle_start,
            end_angle: angle_span,
        };

        let empty_path = Path::new(|path| path.arc(empty_arc));

        frame.stroke(&empty_path, empty_stroke);

        if normal.value() < 0.5 {
            let filled_stroke = Stroke {
                width: style.width,
                color: style.left_filled_color,
                line_cap: LineCap::Butt,
                ..Stroke::default()
            };

            let filled_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle: angle_start + fill_angle_span,
                end_angle: half_angle_span - fill_angle_span,
            };

            let filled_path = Path::new(|path| path.arc(filled_arc));

            frame.stroke(&filled_path, filled_stroke);
        } else {
            let filled_stroke = Stroke {
                width: style.width,
                color: style.right_filled_color,
                line_cap: LineCap::Butt,
                ..Stroke::default()
            };

            let filled_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle: angle_start + half_angle_span,
                end_angle: fill_angle_span - half_angle_span,
            };

            let filled_path = Path::new(|path| path.arc(filled_arc));

            frame.stroke(&filled_path, filled_stroke);
        }

        if let Some(notch) = &style.notch {
            let notch_color = {
                if normal.value() < 0.499 {
                    notch.left_color
                } else if normal.value() > 0.501 {
                    notch.right_color
                } else {
                    notch.center_color
                }
            };

            let angle =
                angle_start + fill_angle_span + std::f32::consts::FRAC_PI_2;

            let stroke = Stroke {
                width: notch.width,
                color: notch_color,
                line_cap: LineCap::Square,
                ..Stroke::default()
            };

            let stroke_begin_y = -arc_radius;
            let notch_height = notch.length_scale.value() * arc_radius;

            let path = Path::line(
                Point::new(0.0, stroke_begin_y),
                Point::new(0.0, stroke_begin_y + notch_height),
            );

            frame.translate(Vector::new(radius, radius));

            if angle < -0.001 || angle > 0.001 {
                frame.rotate(angle);
            }

            frame.stroke(&path, stroke);
        }

        frame.into_primitive()
    };

    Primitive::Group {
        primitives: vec![
            tick_marks,
            text_marks,
            value_ring,
            mod_range_ring,
            arc,
        ],
    }
}

fn draw_circle_tick_mark_tier(
    primitives: &mut Vec<Primitive>,
    tick_mark_positions: &Vec<Normal>,
    diameter: f32,
    color: &Color,
    start_angle: f32,
    angle_span: f32,
    center_x: f32,
    center_y: f32,
    tick_mark_radius: f32,
) {
    let tier_radius = diameter / 2.0;
    let tier_radius_u16 = tier_radius as u16;
    let color = Background::Color(*color);

    for tick_mark_position in tick_mark_positions.iter() {
        let angle = (angle_span * tick_mark_position.value()) + start_angle;

        let (dx, dy) = {
            if angle < -0.001 || angle > 0.001 {
                angle.sin_cos()
            } else {
                (0.0, -1.0)
            }
        };

        primitives.push(Primitive::Quad {
            bounds: Rectangle {
                x: (center_x + (dx * tick_mark_radius) - tier_radius).round(),
                y: (center_y - (dy * tick_mark_radius) - tier_radius).round(),
                width: diameter,
                height: diameter,
            },
            background: color,
            border_radius: tier_radius_u16,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        });
    }
}

fn draw_circle_tick_marks(
    start_angle: f32,
    angle_span: f32,
    radius: f32,
    bounds_x: f32,
    bounds_y: f32,
    tick_marks: &TickMarkGroup,
    style: &CircleTickMarkStyle,
) -> Primitive {
    let start_angle = start_angle + std::f32::consts::FRAC_PI_2;

    let mut primitives: Vec<Primitive> = Vec::new();
    primitives.reserve_exact(tick_marks.len());

    let tick_mark_radius = radius + style.offset;

    let center_x = bounds_x + radius;
    let center_y = bounds_y + radius;

    if tick_marks.has_tier_1() {
        draw_circle_tick_mark_tier(
            &mut primitives,
            &tick_marks.tier_1_positions(),
            style.diameter_tier_1 as f32,
            &style.color_tier_1,
            start_angle,
            angle_span,
            center_x,
            center_y,
            tick_mark_radius,
        );
    }
    if tick_marks.has_tier_2() {
        draw_circle_tick_mark_tier(
            &mut primitives,
            &tick_marks.tier_2_positions(),
            style.diameter_tier_2 as f32,
            &style.color_tier_2,
            start_angle,
            angle_span,
            center_x,
            center_y,
            tick_mark_radius,
        );
    }
    if tick_marks.has_tier_3() {
        draw_circle_tick_mark_tier(
            &mut primitives,
            &tick_marks.tier_3_positions(),
            style.diameter_tier_3 as f32,
            &style.color_tier_3,
            start_angle,
            angle_span,
            center_x,
            center_y,
            tick_mark_radius,
        );
    }

    Primitive::Group { primitives }
}

fn draw_line_tick_marks(
    angle_start: f32,
    angle_span: f32,
    radius: f32,
    bounds_x: f32,
    bounds_y: f32,
    bounds_size: f32,
    tick_marks: &TickMarkGroup,
    style: &LineTickMarkStyle,
) -> Primitive {
    let angle_start = angle_start - std::f32::consts::FRAC_PI_2;

    let tick_mark_offset = radius + style.offset;

    let mut frame = Frame::new(bounds_size, bounds_size);

    frame.translate(Vector::new(bounds_x + radius, bounds_y + radius));

    if tick_marks.has_tier_1() {
        for tick_mark_position in tick_marks.tier_1_positions().iter() {
            let angle = (angle_span * tick_mark_position.value()) + angle_start;

            let stroke = Stroke {
                width: style.width_tier_1,
                color: style.color_tier_1,
                line_cap: LineCap::Butt,
                ..Stroke::default()
            };

            let path = Path::line(
                Point::new(0.0, tick_mark_offset),
                Point::new(0.0, tick_mark_offset + style.length_tier_1),
            );

            frame.with_save(|frame| {
                if angle < -0.001 || angle > 0.001 {
                    frame.rotate(angle);
                }

                frame.stroke(&path, stroke);
            });
        }
    }
    if tick_marks.has_tier_2() {
        for tick_mark_position in tick_marks.tier_2_positions().iter() {
            let angle = (angle_span * tick_mark_position.value()) + angle_start;

            let stroke = Stroke {
                width: style.width_tier_2,
                color: style.color_tier_2,
                line_cap: LineCap::Butt,
                ..Stroke::default()
            };

            let path = Path::line(
                Point::new(0.0, tick_mark_offset),
                Point::new(0.0, tick_mark_offset + style.length_tier_2),
            );

            frame.with_save(|frame| {
                if angle < -0.001 || angle > 0.001 {
                    frame.rotate(angle);
                }

                frame.stroke(&path, stroke);
            });
        }
    }
    if tick_marks.has_tier_3() {
        for tick_mark_position in tick_marks.tier_3_positions().iter() {
            let angle = (angle_span * tick_mark_position.value()) + angle_start;

            let stroke = Stroke {
                width: style.width_tier_3,
                color: style.color_tier_3,
                line_cap: LineCap::Butt,
                ..Stroke::default()
            };

            let path = Path::line(
                Point::new(0.0, tick_mark_offset),
                Point::new(0.0, tick_mark_offset + style.length_tier_3),
            );

            frame.with_save(|frame| {
                if angle < -0.001 || angle > 0.001 {
                    frame.rotate(angle);
                }

                frame.stroke(&path, stroke);
            });
        }
    }

    frame.into_primitive()
}

fn draw_text_marks(
    style: &TextMarkStyle,
    bounds_x: f32,
    bounds_y: f32,
    radius: f32,
    start_angle: f32,
    angle_span: f32,
    text_marks: &TextMarkGroup,
) -> Primitive {
    let mut primitives: Vec<Primitive> = Vec::new();

    let color = style.color;
    let font = style.font;
    let text_size = style.text_size as f32;
    let text_bounds_width = style.bounds_width as f32;
    let text_bounds_height = style.bounds_height as f32;

    let text_mark_radius = radius + (style.offset as f32);

    let center_x = bounds_x + radius;
    let center_y = bounds_y + radius;

    let start_angle = start_angle + std::f32::consts::FRAC_PI_2;

    for text_mark in text_marks.group.iter() {
        let angle = (angle_span * text_mark.position.value()) + start_angle;

        let (dx, dy) = {
            if angle < -0.001 || angle > 0.001 {
                angle.sin_cos()
            } else {
                (0.0, -1.0)
            }
        };

        primitives.push(Primitive::Text {
            content: text_mark.text.clone(),
            size: text_size,
            bounds: Rectangle {
                x: (center_x + (dx * text_mark_radius)).round(),
                y: (center_y - (dy * text_mark_radius)).round(),
                width: text_bounds_width,
                height: text_bounds_height,
            },
            color,
            font,
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        });
    }

    Primitive::Group { primitives }
}
