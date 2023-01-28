//! Display an interactive rotating knob that controls a [`Param`]
//!
//! [`Param`]: ../core/param/struct.Param.html

use std::cmp::Ordering;

use crate::core::{ModulationRange, Normal};
use crate::graphics::{text_marks, tick_marks};
use crate::native::knob;
use iced_graphics::widget::canvas::{self, path::Arc, Frame, Path, Stroke};
use iced_graphics::Primitive;
use iced_native::{Background, Point, Rectangle, Size, Vector};

pub use crate::style::knob::{
    Appearance, ArcAppearance, ArcBipolarAppearance, CircleAppearance,
    CircleNotch, LineCap, LineNotch, ModRangeArcAppearance, NotchShape,
    StyleLength, StyleSheet, TextMarksAppearance, TickMarksAppearance,
    ValueArcAppearance,
};

struct ValueMarkers<'a> {
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
    tick_marks_style: Option<TickMarksAppearance>,
    text_marks_style: Option<TextMarksAppearance>,
    value_arc_style: Option<ValueArcAppearance>,
    mod_range_style_1: Option<ModRangeArcAppearance>,
    mod_range_style_2: Option<ModRangeArcAppearance>,
}

struct KnobInfo {
    bounds: Rectangle,
    start_angle: f32,
    angle_span: f32,
    radius: f32,
    value: Normal,
    bipolar_center: Option<Normal>,
    value_angle: f32,
}

/// A rotating knob GUI widget that controls a [`Param`]
///
/// [`Param`]: ../../core/param/struct.Param.html
pub type Knob<'a, Message, Theme> =
    knob::Knob<'a, Message, crate::Renderer<Theme>>;

impl<Theme> knob::Renderer for crate::Renderer<Theme>
where
    Self::Theme: StyleSheet,
{
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        bipolar_center: Option<Normal>,
        is_dragging: bool,
        mod_range_1: Option<&ModulationRange>,
        mod_range_2: Option<&ModulationRange>,
        tick_marks: Option<&tick_marks::Group>,
        text_marks: Option<&text_marks::Group>,
        style_sheet: &dyn StyleSheet<
            Style = <Self::Theme as StyleSheet>::Style,
        >,
        style: &<Self::Theme as StyleSheet>::Style,
        tick_marks_cache: &tick_marks::PrimitiveCache,
        text_marks_cache: &text_marks::PrimitiveCache,
    ) {
        let is_mouse_over = bounds.contains(cursor_position);

        let angle_range = style_sheet.angle_range(style);

        let appearance = if is_dragging {
            style_sheet.dragging(style)
        } else if is_mouse_over {
            style_sheet.hovered(style)
        } else {
            style_sheet.active(style)
        };

        let value_markers = ValueMarkers {
            tick_marks,
            text_marks,
            mod_range_1,
            mod_range_2,
            tick_marks_style: style_sheet.tick_marks_appearance(style),
            text_marks_style: style_sheet.text_marks_appearance(style),
            value_arc_style: style_sheet.value_arc_appearance(style),
            mod_range_style_1: style_sheet.mod_range_arc_appearance(style),
            mod_range_style_2: style_sheet.mod_range_arc_appearance_2(style),
        };

        let bounds = {
            let bounds = Rectangle {
                x: bounds.x.round(),
                y: bounds.y.round(),
                width: bounds.width.round(),
                height: bounds.height.round(),
            };

            if bounds.width == bounds.height {
                bounds
            } else if bounds.width > bounds.height {
                Rectangle {
                    x: (bounds.x + (bounds.width - bounds.height) / 2.0)
                        .round(),
                    y: bounds.y,
                    width: bounds.height,
                    height: bounds.height,
                }
            } else {
                Rectangle {
                    x: bounds.x,
                    y: (bounds.y + (bounds.height - bounds.width) / 2.0)
                        .round(),
                    width: bounds.width,
                    height: bounds.width,
                }
            }
        };

        let radius = bounds.width / 2.0;

        let start_angle =
            if angle_range.min() >= crate::core::math::THREE_HALVES_PI {
                angle_range.min() - crate::core::math::THREE_HALVES_PI
            } else {
                angle_range.min() + std::f32::consts::FRAC_PI_2
            };
        let angle_span = angle_range.max() - angle_range.min();
        let value_angle = start_angle + (normal.scale(angle_span));

        let knob_info = KnobInfo {
            bounds,
            start_angle,
            angle_span,
            radius,
            value: normal,
            bipolar_center,
            value_angle,
        };

        self.draw_primitive(match appearance {
            Appearance::Circle(style) => draw_circle_style(
                &knob_info,
                style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
            Appearance::Arc(style) => draw_arc_style(
                &knob_info,
                style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
            Appearance::ArcBipolar(style) => draw_arc_bipolar_style(
                &knob_info,
                style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
        })
    }
}

fn draw_value_markers(
    knob_info: &KnobInfo,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> (Primitive, Primitive, Primitive, Primitive, Primitive) {
    (
        draw_tick_marks(
            knob_info,
            value_markers.tick_marks,
            &value_markers.tick_marks_style,
            tick_marks_cache,
        ),
        draw_text_marks(
            knob_info,
            value_markers.text_marks,
            &value_markers.text_marks_style,
            text_marks_cache,
        ),
        draw_value_arc(knob_info, &value_markers.value_arc_style),
        draw_mod_range_arc(
            knob_info,
            &value_markers.mod_range_style_1,
            value_markers.mod_range_1,
        ),
        draw_mod_range_arc(
            knob_info,
            &value_markers.mod_range_style_2,
            value_markers.mod_range_2,
        ),
    )
}

fn draw_tick_marks(
    knob_info: &KnobInfo,
    tick_marks: Option<&tick_marks::Group>,
    style: &Option<TickMarksAppearance>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
) -> Primitive {
    if let Some(tick_marks) = tick_marks {
        if let Some(style) = style {
            tick_marks::draw_radial_tick_marks(
                knob_info.bounds.center(),
                knob_info.radius + style.offset,
                knob_info.start_angle + std::f32::consts::FRAC_PI_2,
                knob_info.angle_span,
                false,
                tick_marks,
                &style.style,
                false,
                tick_marks_cache,
            )
        } else {
            Primitive::None
        }
    } else {
        Primitive::None
    }
}

fn draw_text_marks(
    knob_info: &KnobInfo,
    text_marks: Option<&text_marks::Group>,
    style: &Option<TextMarksAppearance>,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> Primitive {
    if let Some(text_marks) = text_marks {
        if let Some(style) = style {
            text_marks::draw_radial_text_marks(
                Point::new(
                    knob_info.bounds.center_x(),
                    knob_info.bounds.center_y() + style.v_offset,
                ),
                knob_info.radius + style.offset,
                knob_info.start_angle,
                knob_info.angle_span,
                text_marks,
                &style.style,
                style.h_char_offset,
                false,
                text_marks_cache,
            )
        } else {
            Primitive::None
        }
    } else {
        Primitive::None
    }
}

fn draw_value_arc(
    knob_info: &KnobInfo,
    style: &Option<ValueArcAppearance>,
) -> Primitive {
    if let Some(style) = style {
        let half_width = style.width / 2.0;

        let end_angle = knob_info.start_angle + knob_info.angle_span;
        let arc_radius = knob_info.radius + style.offset + half_width;

        let half_frame_size = (arc_radius + half_width).ceil();
        let frame_size = half_frame_size * 2.0;
        let frame_offset = half_frame_size - knob_info.radius;
        let center_point = Point::new(half_frame_size, half_frame_size);

        let mut frame = Frame::new(Size::new(frame_size, frame_size));

        if let Some(empty_color) = style.empty_color {
            let empty_stroke = Stroke {
                width: style.width,
                style: canvas::Style::Solid(empty_color),
                line_cap: style.cap,
                ..Stroke::default()
            };

            let empty_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle: knob_info.start_angle,
                end_angle,
            };

            let empty_path = Path::new(|path| path.arc(empty_arc));

            frame.stroke(&empty_path, empty_stroke);
        }

        if let Some(right_filled_color) = style.right_filled_color {
            if knob_info.value.as_f32() < 0.499
                || knob_info.value.as_f32() > 0.501
            {
                let half_angle =
                    knob_info.start_angle + (knob_info.angle_span / 2.0);

                if knob_info.value < Normal::CENTER {
                    let filled_stroke = Stroke {
                        width: style.width,
                        style: canvas::Style::Solid(style.left_filled_color),
                        line_cap: style.cap,
                        ..Stroke::default()
                    };

                    let filled_arc = Arc {
                        center: center_point,
                        radius: arc_radius,
                        start_angle: knob_info.value_angle,
                        end_angle: half_angle,
                    };

                    let filled_path = Path::new(|path| path.arc(filled_arc));

                    frame.stroke(&filled_path, filled_stroke);
                } else if knob_info.value > Normal::CENTER {
                    let filled_stroke = Stroke {
                        width: style.width,
                        style: canvas::Style::Solid(right_filled_color),
                        line_cap: style.cap,
                        ..Stroke::default()
                    };

                    let filled_arc = Arc {
                        center: center_point,
                        radius: arc_radius,
                        start_angle: half_angle,
                        end_angle: knob_info.value_angle,
                    };

                    let filled_path = Path::new(|path| path.arc(filled_arc));

                    frame.stroke(&filled_path, filled_stroke);
                }
            }
        } else if knob_info.value != Normal::MIN {
            let filled_stroke = Stroke {
                width: style.width,
                style: canvas::Style::Solid(style.left_filled_color),
                line_cap: style.cap,
                ..Stroke::default()
            };

            let filled_arc = Arc {
                center: center_point,
                radius: arc_radius,
                start_angle: knob_info.start_angle,
                end_angle: knob_info.value_angle,
            };

            let filled_path = Path::new(|path| path.arc(filled_arc));

            frame.stroke(&filled_path, filled_stroke);
        }

        Primitive::Translate {
            translation: Vector::new(
                knob_info.bounds.x - frame_offset,
                knob_info.bounds.y - frame_offset,
            ),
            content: Box::new(frame.into_geometry().into_primitive()),
        }
    } else {
        Primitive::None
    }
}

fn draw_mod_range_arc(
    knob_info: &KnobInfo,
    style: &Option<ModRangeArcAppearance>,
    mod_range: Option<&ModulationRange>,
) -> Primitive {
    if let Some(mod_range) = mod_range {
        if let Some(style) = style {
            let half_width = style.width / 2.0;
            let arc_radius = knob_info.radius + style.offset + half_width;

            let half_frame_size = (arc_radius + half_width).ceil();
            let frame_size = half_frame_size * 2.0;
            let frame_offset = half_frame_size - knob_info.radius;
            let center_point = Point::new(half_frame_size, half_frame_size);

            let mut frame = Frame::new(Size::new(frame_size, frame_size));

            if let Some(empty_color) = style.empty_color {
                let empty_stroke = Stroke {
                    width: style.width,
                    style: canvas::Style::Solid(empty_color),
                    line_cap: style.cap,
                    ..Stroke::default()
                };

                let empty_arc = Arc {
                    center: center_point,
                    radius: arc_radius,
                    start_angle: knob_info.start_angle,
                    end_angle: knob_info.start_angle + knob_info.angle_span,
                };

                let empty_path = Path::new(|path| path.arc(empty_arc));

                frame.stroke(&empty_path, empty_stroke);
            }

            if mod_range.filled_visible && (mod_range.start != mod_range.end) {
                let (start, end, color) =
                    if mod_range.start.as_f32() < mod_range.end.as_f32() {
                        (
                            mod_range.start.as_f32(),
                            mod_range.end.as_f32(),
                            style.filled_color,
                        )
                    } else {
                        (
                            mod_range.end.as_f32(),
                            mod_range.start.as_f32(),
                            style.filled_inverse_color,
                        )
                    };

                let filled_stroke = Stroke {
                    width: style.width,
                    style: canvas::Style::Solid(color),
                    line_cap: style.cap,
                    ..Stroke::default()
                };

                let filled_arc = Arc {
                    center: center_point,
                    radius: arc_radius,
                    start_angle: knob_info.start_angle
                        + (knob_info.angle_span * start),
                    end_angle: knob_info.start_angle
                        + (knob_info.angle_span * end),
                };

                let filled_path = Path::new(|path| path.arc(filled_arc));

                frame.stroke(&filled_path, filled_stroke);
            }

            Primitive::Translate {
                translation: Vector::new(
                    knob_info.bounds.x - frame_offset,
                    knob_info.bounds.y - frame_offset,
                ),
                content: Box::new(frame.into_geometry().into_primitive()),
            }
        } else {
            Primitive::None
        }
    } else {
        Primitive::None
    }
}

fn draw_circle_notch(knob_info: &KnobInfo, style: &CircleNotch) -> Primitive {
    let value_angle = knob_info.value_angle + std::f32::consts::FRAC_PI_2;

    let (dx, dy) = if !(-0.001..=0.001).contains(&value_angle) {
        value_angle.sin_cos()
    } else {
        (0.0, -1.0)
    };

    let notch_diameter =
        style.diameter.from_knob_diameter(knob_info.bounds.width);
    let notch_radius = notch_diameter / 2.0;

    let offset_radius = knob_info.radius
        - style.offset.from_knob_diameter(knob_info.bounds.width);

    Primitive::Quad {
        bounds: Rectangle {
            x: knob_info.bounds.center_x() + (dx * offset_radius)
                - notch_radius,
            y: knob_info.bounds.center_y()
                - (dy * offset_radius)
                - notch_radius,
            width: notch_diameter,
            height: notch_diameter,
        },
        background: Background::Color(style.color),
        border_radius: [notch_radius; 4],
        border_width: style.border_width,
        border_color: style.border_color,
    }
}

fn draw_line_notch(knob_info: &KnobInfo, style: &LineNotch) -> Primitive {
    let value_angle = knob_info.value_angle + std::f32::consts::FRAC_PI_2;

    let stroke = Stroke {
        width: style.width.from_knob_diameter(knob_info.bounds.width),
        style: canvas::Style::Solid(style.color),
        line_cap: style.cap,
        ..Stroke::default()
    };

    let stroke_begin_y = -(knob_info.radius
        - style.offset.from_knob_diameter(knob_info.bounds.width));
    let notch_height = style.length.from_knob_diameter(knob_info.bounds.width);

    let path = Path::line(
        Point::new(0.0, stroke_begin_y),
        Point::new(0.0, stroke_begin_y + notch_height),
    );

    let mut frame =
        Frame::new(Size::new(knob_info.bounds.width, knob_info.bounds.width));
    frame.translate(Vector::new(knob_info.radius, knob_info.radius));

    if !(-0.001..=0.001).contains(&value_angle) {
        frame.rotate(value_angle);
    }

    frame.stroke(&path, stroke);

    Primitive::Translate {
        translation: Vector::new(knob_info.bounds.x, knob_info.bounds.y),
        content: Box::new(frame.into_geometry().into_primitive()),
    }
}

fn draw_notch(knob_info: &KnobInfo, notch: &NotchShape) -> Primitive {
    match notch {
        NotchShape::None => Primitive::None,
        NotchShape::Circle(style) => draw_circle_notch(knob_info, style),
        NotchShape::Line(style) => draw_line_notch(knob_info, style),
    }
}

fn draw_circle_style(
    knob_info: &KnobInfo,
    style: CircleAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> Primitive {
    let (tick_marks, text_marks, value_arc, mod_range_arc_1, mod_range_arc_2) =
        draw_value_markers(
            knob_info,
            value_markers,
            tick_marks_cache,
            text_marks_cache,
        );

    let knob_back = Primitive::Quad {
        bounds: knob_info.bounds,
        background: Background::Color(style.color),
        border_radius: [knob_info.radius; 4],
        border_width: style.border_width,
        border_color: style.border_color,
    };

    let notch = draw_notch(knob_info, &style.notch);

    Primitive::Group {
        primitives: vec![
            tick_marks,
            text_marks,
            value_arc,
            mod_range_arc_1,
            mod_range_arc_2,
            knob_back,
            notch,
        ],
    }
}

fn draw_arc_style(
    knob_info: &KnobInfo,
    style: ArcAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> Primitive {
    let (tick_marks, text_marks, value_arc, mod_range_arc_1, mod_range_arc_2) =
        draw_value_markers(
            knob_info,
            value_markers,
            tick_marks_cache,
            text_marks_cache,
        );

    let arc: Primitive = {
        let width = style.width.from_knob_diameter(knob_info.bounds.width);

        let center_point = Point::new(knob_info.radius, knob_info.radius);
        let arc_radius = knob_info.radius - (width / 2.0);

        let mut frame = Frame::new(Size::new(
            knob_info.bounds.width,
            knob_info.bounds.width,
        ));

        let empty_stroke = Stroke {
            width,
            style: canvas::Style::Solid(style.empty_color),
            line_cap: style.cap,
            ..Stroke::default()
        };

        let empty_arc = Arc {
            center: center_point,
            radius: arc_radius,
            start_angle: knob_info.start_angle,
            end_angle: knob_info.start_angle + knob_info.angle_span,
        };

        let empty_path = Path::new(|path| path.arc(empty_arc));

        frame.stroke(&empty_path, empty_stroke);

        let filled_stroke = Stroke {
            width,
            style: canvas::Style::Solid(style.filled_color),
            line_cap: style.cap,
            ..Stroke::default()
        };

        let filled_arc = Arc {
            center: center_point,
            radius: arc_radius,
            start_angle: knob_info.start_angle,
            end_angle: knob_info.value_angle,
        };

        let filled_path = Path::new(|path| path.arc(filled_arc));

        frame.stroke(&filled_path, filled_stroke);

        Primitive::Translate {
            translation: Vector::new(knob_info.bounds.x, knob_info.bounds.y),
            content: Box::new(frame.into_geometry().into_primitive()),
        }
    };

    let notch = draw_notch(knob_info, &style.notch);

    Primitive::Group {
        primitives: vec![
            tick_marks,
            text_marks,
            arc,
            notch,
            value_arc,
            mod_range_arc_1,
            mod_range_arc_2,
        ],
    }
}

enum BipolarState {
    Left,
    Right,
    Center,
}

impl BipolarState {
    pub fn from_knob_info(knob_info: &KnobInfo) -> Self {
        if let Some(center) = knob_info.bipolar_center {
            match knob_info.value.partial_cmp(&center) {
                Some(Ordering::Less) => BipolarState::Left,
                Some(Ordering::Equal) => BipolarState::Center,
                Some(Ordering::Greater) => BipolarState::Right,
                None => BipolarState::Center,
            }
        } else if knob_info.value.as_f32() < 0.499 {
            BipolarState::Left
        } else if knob_info.value.as_f32() > 0.501 {
            BipolarState::Right
        } else {
            BipolarState::Center
        }
    }
}

fn draw_arc_bipolar_style(
    knob_info: &KnobInfo,
    style: ArcBipolarAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> Primitive {
    let (tick_marks, text_marks, value_arc, mod_range_arc_1, mod_range_arc_2) =
        draw_value_markers(
            knob_info,
            value_markers,
            tick_marks_cache,
            text_marks_cache,
        );

    let bipolar_state = BipolarState::from_knob_info(knob_info);

    let arc: Primitive = {
        let width = style.width.from_knob_diameter(knob_info.bounds.width);

        let center_point = Point::new(knob_info.radius, knob_info.radius);
        let arc_radius = knob_info.radius - (width / 2.0);

        let mut frame = Frame::new(Size::new(
            knob_info.bounds.width,
            knob_info.bounds.width,
        ));

        let empty_stroke = Stroke {
            width,
            style: canvas::Style::Solid(style.empty_color),
            line_cap: style.cap,
            ..Stroke::default()
        };

        let empty_arc = Arc {
            center: center_point,
            radius: arc_radius,
            start_angle: knob_info.start_angle,
            end_angle: knob_info.start_angle + knob_info.angle_span,
        };

        let empty_path = Path::new(|path| path.arc(empty_arc));

        frame.stroke(&empty_path, empty_stroke);

        let center_angle = knob_info.start_angle
            + knob_info
                .bipolar_center
                .unwrap_or_else(|| Normal::from_clipped(0.5))
                .scale(knob_info.angle_span);

        match bipolar_state {
            BipolarState::Left => {
                let filled_stroke = Stroke {
                    width,
                    style: canvas::Style::Solid(style.left_filled_color),
                    line_cap: style.cap,
                    ..Stroke::default()
                };

                let filled_arc = Arc {
                    center: center_point,
                    radius: arc_radius,
                    start_angle: knob_info.value_angle,
                    end_angle: center_angle,
                };

                let filled_path = Path::new(|path| path.arc(filled_arc));

                frame.stroke(&filled_path, filled_stroke);
            }
            BipolarState::Right => {
                let filled_stroke = Stroke {
                    width,
                    style: canvas::Style::Solid(style.right_filled_color),
                    line_cap: style.cap,
                    ..Stroke::default()
                };

                let filled_arc = Arc {
                    center: center_point,
                    radius: arc_radius,
                    start_angle: center_angle,
                    end_angle: knob_info.value_angle,
                };

                let filled_path = Path::new(|path| path.arc(filled_arc));

                frame.stroke(&filled_path, filled_stroke);
            }
            _ => {}
        }

        Primitive::Translate {
            translation: Vector::new(knob_info.bounds.x, knob_info.bounds.y),
            content: Box::new(frame.into_geometry().into_primitive()),
        }
    };

    let notch = if let Some((notch_left, notch_right)) = style.notch_left_right
    {
        match bipolar_state {
            BipolarState::Left => draw_notch(knob_info, &notch_left),
            BipolarState::Right => draw_notch(knob_info, &notch_right),
            BipolarState::Center => draw_notch(knob_info, &style.notch_center),
        }
    } else {
        draw_notch(knob_info, &style.notch_center)
    };

    Primitive::Group {
        primitives: vec![
            tick_marks,
            text_marks,
            arc,
            notch,
            value_arc,
            mod_range_arc_1,
            mod_range_arc_2,
        ],
    }
}
