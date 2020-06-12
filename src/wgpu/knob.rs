//! wgpu renderer for the [`Knob`] widget
//!
//! [`Knob`]: ../native/knob/struct.Knob.html

use crate::core::{AutomationRange, Normal, TickMarkGroup, TickMarkTier};
use crate::native::knob;
use iced_native::{Background, Color, MouseCursor, Point, Rectangle, Vector};
use iced_wgpu::widget::canvas::{path::Arc, Frame, LineCap, Path, Stroke};
use iced_wgpu::{Primitive, Renderer};

pub use crate::native::knob::State;
pub use crate::style::knob::{
    ArcBipolarNotch, ArcBipolarStyle, ArcNotch, ArcStyle, AutoRangeRingStyle,
    CircleTickMarks, LineTickMarks, Style, StyleSheet, TickMarkStyle,
    ValueRingStyle, VectorCircleStyle, VectorLineStyle,
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
        auto_range: Option<AutomationRange>,
        tick_marks: Option<&TickMarkGroup>,
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

        let value_ring: Primitive = {
            if let Some(style) = style_sheet.value_ring_style() {
                let mut start_angle =
                    angle_range.min() + std::f32::consts::FRAC_PI_2;
                if start_angle >= crate::TAU {
                    start_angle -= crate::TAU
                }

                let angle_span = angle_range.max() - angle_range.min();

                let fill_angle_span = angle_span * normal.value();

                let frame_size =
                    bounds_size + style.offset + (style.width * 0.5);

                let mut frame = Frame::new(frame_size, frame_size);
                frame.translate(Vector::new(bounds_x, bounds_y));

                let center_point = Point::new(radius, radius);
                let arc_radius = radius + style.offset + (style.width * 0.5);

                let empty_stroke = Stroke {
                    width: style.width,
                    color: style.color_empty,
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

                if let Some(color_right) = style.color_right {
                    if normal.value() < 0.5 {
                        let filled_stroke = Stroke {
                            width: style.width,
                            color: style.color_left,
                            line_cap: LineCap::Butt,
                            ..Stroke::default()
                        };

                        let filled_arc = Arc {
                            center: center_point,
                            radius: arc_radius,
                            start_angle: start_angle + fill_angle_span,
                            end_angle: (angle_span * 0.5) - fill_angle_span,
                        };

                        let filled_path =
                            Path::new(|path| path.arc(filled_arc));

                        frame.stroke(&filled_path, filled_stroke);
                    } else {
                        let filled_stroke = Stroke {
                            width: style.width,
                            color: color_right,
                            line_cap: LineCap::Butt,
                            ..Stroke::default()
                        };

                        let filled_arc = Arc {
                            center: center_point,
                            radius: arc_radius,
                            start_angle: start_angle + (angle_span * 0.5),
                            end_angle: fill_angle_span - (angle_span * 0.5),
                        };

                        let filled_path =
                            Path::new(|path| path.arc(filled_arc));

                        frame.stroke(&filled_path, filled_stroke);
                    }
                } else {
                    let center_point = Point::new(radius, radius);
                    let arc_radius =
                        radius + style.offset + (style.width * 0.5);

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
                            color: style.color_left,
                            line_cap: LineCap::Butt,
                            ..Stroke::default()
                        };

                        let filled_arc = Arc {
                            center: center_point,
                            radius: arc_radius,
                            start_angle,
                            end_angle: fill_angle_span,
                        };

                        let filled_path =
                            Path::new(|path| path.arc(filled_arc));

                        frame.stroke(&filled_path, filled_stroke);
                    }
                }

                frame.into_primitive()
            } else {
                Primitive::None
            }
        };

        let auto_range_ring: Primitive = {
            if let Some(auto_range) = auto_range {
                if auto_range.visible {
                    if let Some(style) = style_sheet.auto_range_ring_style() {
                        let mut start_angle =
                            angle_range.min() + std::f32::consts::FRAC_PI_2;
                        if start_angle >= crate::TAU {
                            start_angle -= crate::TAU
                        }

                        let center_point = Point::new(radius, radius);
                        let arc_radius =
                            radius + style.offset + (style.width * 0.5);

                        let angle_span = angle_range.max() - angle_range.min();

                        let frame_size =
                            bounds_size + style.offset + (style.width * 0.5);

                        let mut frame = Frame::new(frame_size, frame_size);
                        frame.translate(Vector::new(bounds_x, bounds_y));

                        if let Some(color_empty) = style.color_empty {
                            let empty_stroke = Stroke {
                                width: style.width,
                                color: color_empty,
                                line_cap: LineCap::Butt,
                                ..Stroke::default()
                            };

                            let empty_arc = Arc {
                                center: center_point,
                                radius: arc_radius,
                                start_angle: start_angle,
                                end_angle: angle_span,
                            };

                            let empty_path =
                                Path::new(|path| path.arc(empty_arc));

                            frame.stroke(&empty_path, empty_stroke);
                        }

                        if auto_range.filled_visible
                            && (auto_range.start.value()
                                != auto_range.end.value())
                        {
                            let (start, end, color) =
                                if auto_range.start.value()
                                    < auto_range.end.value()
                                {
                                    (
                                        auto_range.start.value(),
                                        auto_range.end.value(),
                                        style.color,
                                    )
                                } else {
                                    (
                                        auto_range.end.value(),
                                        auto_range.start.value(),
                                        style.color_inverse,
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

                            let filled_path =
                                Path::new(|path| path.arc(filled_arc));

                            frame.stroke(&filled_path, filled_stroke);
                        }

                        frame.into_primitive()
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
                        TickMarkStyle::Circle(style) => {
                            let mut primitives: Vec<Primitive> = Vec::new();

                            let tick_mark_radius = radius + style.offset;

                            for tick_mark in tick_marks.group.iter() {
                                let (diameter, color) = match tick_mark.tier {
                                    TickMarkTier::One => (
                                        style.diameter_tier_1 as f32,
                                        style.color_tier_1,
                                    ),
                                    TickMarkTier::Two => (
                                        style.diameter_tier_2 as f32,
                                        style.color_tier_2,
                                    ),
                                    TickMarkTier::Three => (
                                        style.diameter_tier_3 as f32,
                                        style.color_tier_3,
                                    ),
                                };

                                let tick_radius = diameter / 2.0;

                                let angle = ((angle_range.max()
                                    - angle_range.min())
                                    * tick_mark.position.value())
                                    + angle_range.min()
                                    + std::f32::consts::PI;

                                let (dx, dy) = {
                                    if angle < -0.001 || angle > 0.001 {
                                        angle.sin_cos()
                                    } else {
                                        (0.0, -1.0)
                                    }
                                };

                                primitives.push(Primitive::Quad {
                                    bounds: Rectangle {
                                        x: (bounds_x
                                            + radius
                                            + (dx * tick_mark_radius)
                                            - tick_radius)
                                            .round(),
                                        y: (bounds_y + radius
                                            - (dy * tick_mark_radius)
                                            - tick_radius)
                                            .round(),
                                        width: diameter,
                                        height: diameter,
                                    },
                                    background: Background::Color(color),
                                    border_radius: tick_radius as u16,
                                    border_width: 0,
                                    border_color: Color::TRANSPARENT,
                                });
                            }

                            Primitive::Group { primitives }
                        }
                        TickMarkStyle::Line(style) => {
                            let tick_mark_offset = radius + style.offset;

                            /*
                            let mut frame = Frame::new(
                                Size::new(bounds_size, bounds_size));
                            */

                            let mut frame =
                                Frame::new(bounds_size, bounds_size);

                            frame.translate(Vector::new(
                                bounds_x + radius,
                                bounds_y + radius,
                            ));

                            for tick_mark in tick_marks.group.iter() {
                                let (width, length, color) =
                                    match tick_mark.tier {
                                        TickMarkTier::One => (
                                            style.width_tier_1,
                                            style.length_tier_1,
                                            style.color_tier_1,
                                        ),
                                        TickMarkTier::Two => (
                                            style.width_tier_2,
                                            style.length_tier_2,
                                            style.color_tier_2,
                                        ),
                                        TickMarkTier::Three => (
                                            style.width_tier_3,
                                            style.length_tier_3,
                                            style.color_tier_3,
                                        ),
                                    };

                                let angle = ((angle_range.max()
                                    - angle_range.min())
                                    * tick_mark.position.value())
                                    + angle_range.min();

                                let stroke = Stroke {
                                    width,
                                    color,
                                    line_cap: LineCap::Butt,
                                    ..Stroke::default()
                                };

                                let path = Path::line(
                                    Point::new(0.0, tick_mark_offset),
                                    Point::new(0.0, tick_mark_offset + length),
                                );

                                frame.with_save(|frame| {
                                    if angle < -0.001 || angle > 0.001 {
                                        frame.rotate(angle);
                                    }

                                    frame.stroke(&path, stroke);
                                });
                            }

                            frame.into_primitive()
                        }
                    }
                } else {
                    Primitive::None
                }
            } else {
                Primitive::None
            }
        };

        match style {
            /*
            Style::Texture(style) => {



                let knob_width = style.knob_width as f32;
                let knob_height = style.knob_height as f32;

                let knob = {
                    if let Some(pad) = style.texture_padding {
                        Primitive::Image {
                            handle: style.texture,
                            bounds: Rectangle {
                                x: bounds_x - pad.left as f32,
                                y: bounds_y - pad.top as f32,
                                width: knob_width +
                                    (pad.left + pad.right) as f32,
                                height: knob_height +
                                    (pad.top + pad.bottom) as f32,
                            }
                        }
                    } else {
                        Primitive::Image {
                            handle: style.texture,
                            bounds: Rectangle {
                                x: bounds_x,
                                y: bounds_y,
                                width: knob_width,
                                height: knob_height,
                            },
                        }
                    }
                };

                // not implemented yet

                (
                    knob,
                    MouseCursor::default(),
                )
            },
            */
            Style::VectorCircle(style) => {
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

                let angle = ((angle_range.max() - angle_range.min())
                    * normal.value())
                    + angle_range.min()
                    + std::f32::consts::PI;

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

                let notch = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + radius + (dx * offset_radius)
                            - notch_radius,
                        y: bounds_y + radius
                            - (dy * offset_radius)
                            - notch_radius,
                        width: notch_radius * 2.0,
                        height: notch_radius * 2.0,
                    },
                    background: Background::Color(style.notch_color),
                    border_radius: notch_radius as u16,
                    border_width: style.notch_border_width,
                    border_color: style.notch_border_color,
                };

                (
                    Primitive::Group {
                        primitives: vec![
                            tick_marks,
                            value_ring,
                            auto_range_ring,
                            knob_back,
                            notch,
                        ],
                    },
                    MouseCursor::default(),
                )
            }

            Style::VectorLine(style) => {
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

                let angle = ((angle_range.max() - angle_range.min())
                    * normal.value())
                    + angle_range.min()
                    + std::f32::consts::PI;

                let notch: Primitive = {
                    let stroke = Stroke {
                        width: style.notch_width as f32,
                        color: style.notch_color,
                        line_cap: LineCap::Butt,
                        ..Stroke::default()
                    };

                    let stroke_begin_y =
                        -(radius - (style.notch_offset.value() * radius));
                    let notch_height = style.notch_scale.value() * radius;

                    let path = Path::line(
                        Point::new(0.0, stroke_begin_y),
                        Point::new(0.0, stroke_begin_y + notch_height),
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

                (
                    Primitive::Group {
                        primitives: vec![
                            tick_marks,
                            value_ring,
                            auto_range_ring,
                            knob_back,
                            notch,
                        ],
                    },
                    MouseCursor::default(),
                )
            }

            Style::Arc(style) => {
                let radius = bounds_size / 2.0;

                let mut start_angle =
                    angle_range.min() + std::f32::consts::FRAC_PI_2;
                if start_angle >= crate::TAU {
                    start_angle -= crate::TAU
                }

                let angle_span = angle_range.max() - angle_range.min();

                let fill_angle_span = angle_span * normal.value();

                let arc: Primitive = {
                    let center_point = Point::new(radius, radius);
                    let arc_radius = radius - (style.arc_width * 0.5);

                    let filled_stroke = Stroke {
                        width: style.arc_width,
                        color: style.arc_filled_color,
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

                    let empty_stroke = Stroke {
                        width: style.arc_width,
                        color: style.arc_empty_color,
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

                    let mut frame = Frame::new(bounds_size, bounds_size);
                    frame.translate(Vector::new(bounds_x, bounds_y));

                    frame.stroke(&empty_path, empty_stroke);
                    frame.stroke(&filled_path, filled_stroke);

                    if let Some(notch) = style.notch {
                        let angle = start_angle
                            + fill_angle_span
                            + std::f32::consts::FRAC_PI_2;

                        let stroke = Stroke {
                            width: notch.width,
                            color: notch.color,
                            line_cap: LineCap::Square,
                            ..Stroke::default()
                        };

                        let stroke_begin_y = -arc_radius;
                        let notch_height =
                            notch.length_scale.value() * arc_radius;

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

                (
                    Primitive::Group {
                        primitives: vec![
                            tick_marks,
                            value_ring,
                            auto_range_ring,
                            arc,
                        ],
                    },
                    MouseCursor::default(),
                )
            }

            Style::ArcBipolar(style) => {
                let radius = bounds_size / 2.0;

                let mut start_angle =
                    angle_range.min() + std::f32::consts::FRAC_PI_2;
                if start_angle >= crate::TAU {
                    start_angle -= crate::TAU
                }

                let angle_span = angle_range.max() - angle_range.min();

                let fill_angle_span = angle_span * normal.value();

                let arc: Primitive = {
                    let mut frame = Frame::new(bounds_size, bounds_size);
                    frame.translate(Vector::new(bounds_x, bounds_y));

                    let center_point = Point::new(radius, radius);
                    let arc_radius = radius - (style.arc_width * 0.5);

                    let empty_stroke = Stroke {
                        width: style.arc_width,
                        color: style.arc_empty_color,
                        line_cap: LineCap::Butt,
                        ..Stroke::default()
                    };

                    let empty_arc = Arc {
                        center: center_point,
                        radius: arc_radius,
                        start_angle,
                        end_angle: angle_span,
                    };

                    let empty_path = Path::new(|path| path.arc(empty_arc));

                    frame.stroke(&empty_path, empty_stroke);

                    if normal.value() < 0.5 {
                        let filled_stroke = Stroke {
                            width: style.arc_width,
                            color: style.arc_left_color,
                            line_cap: LineCap::Butt,
                            ..Stroke::default()
                        };

                        let filled_arc = Arc {
                            center: center_point,
                            radius: arc_radius,
                            start_angle: start_angle + fill_angle_span,
                            end_angle: (angle_span * 0.5) - fill_angle_span,
                        };

                        let filled_path =
                            Path::new(|path| path.arc(filled_arc));

                        frame.stroke(&filled_path, filled_stroke);
                    } else {
                        let filled_stroke = Stroke {
                            width: style.arc_width,
                            color: style.arc_right_color,
                            line_cap: LineCap::Butt,
                            ..Stroke::default()
                        };

                        let filled_arc = Arc {
                            center: center_point,
                            radius: arc_radius,
                            start_angle: start_angle + (angle_span * 0.5),
                            end_angle: fill_angle_span - (angle_span * 0.5),
                        };

                        let filled_path =
                            Path::new(|path| path.arc(filled_arc));

                        frame.stroke(&filled_path, filled_stroke);
                    }

                    if let Some(notch) = style.notch {
                        let notch_color = {
                            if normal.value() < 0.499 {
                                notch.color_left
                            } else if normal.value() > 0.501 {
                                notch.color_right
                            } else {
                                notch.color_center
                            }
                        };

                        let angle = start_angle
                            + fill_angle_span
                            + std::f32::consts::FRAC_PI_2;

                        let stroke = Stroke {
                            width: notch.width,
                            color: notch_color,
                            line_cap: LineCap::Square,
                            ..Stroke::default()
                        };

                        let stroke_begin_y = -arc_radius;
                        let notch_height =
                            notch.length_scale.value() * arc_radius;

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

                (
                    Primitive::Group {
                        primitives: vec![
                            tick_marks,
                            value_ring,
                            auto_range_ring,
                            arc,
                        ],
                    },
                    MouseCursor::default(),
                )
            }
        }
    }
}
