//! wgpu renderer for the [`ReductionMeter`] widget
//!
//! [`ReductionMeter`]: ../native/reduction_meter/struct.ReductionMeter.html

use crate::core::{Normal, TickMarkGroup, TickMarkTier};
use crate::native::reduction_meter;
use iced_native::{Background, Color, MouseCursor, Rectangle};
use iced_wgpu::{Primitive, Renderer};

pub use crate::native::reduction_meter::{Orientation, State};
pub use crate::style::reduction_meter::{
    Style, StyleSheet, TickMarkPlacement, TickMarkStyle,
};

/// This is an alias of a `crate::native` [`ReductionMeter`] with an
/// `iced_wgpu::Renderer`.
///
/// [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html
pub type ReductionMeter<'a> = reduction_meter::ReductionMeter<'a, Renderer>;

impl reduction_meter::Renderer for Renderer {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        bar_normal: Normal,
        peak_normal: Option<Normal>,
        orientation: &Orientation,
        tick_marks: Option<&TickMarkGroup>,
        style_sheet: &Self::Style,
    ) -> Self::Output {
        let bounds_x = bounds.x.floor();
        let bounds_y = bounds.y.floor();

        let bounds_width = bounds.width.floor();
        let bounds_height = bounds.height.floor();

        let style = style_sheet.style();

        let border_width = style.back_border_width as f32;

        let back = Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x,
                y: bounds_y,
                width: bounds_width,
                height: bounds_height,
            },
            background: Background::Color(style.back_color),
            border_radius: style.back_border_radius,
            border_width: style.back_border_width,
            border_color: style.back_border_color,
        };

        match orientation {
            Orientation::Vertical => {
                let tick_marks: Primitive = {
                    if let Some(tick_marks) = tick_marks {
                        if let Some(style) = style_sheet.tick_mark_style() {
                            let notch_span =
                                bounds_height - (border_width * 2.0);
                            let start_y =
                                bounds_y + bounds_height - border_width;

                            let mut primitives: Vec<Primitive> = Vec::new();

                            if style.placement != TickMarkPlacement::Right {
                                let notch_x = bounds_x - style.offset as f32;

                                for left_tick_mark in tick_marks.group.iter() {
                                    let y_offset = notch_span
                                        * left_tick_mark.position.value();

                                    let (length, height, color) =
                                        match left_tick_mark.tier {
                                            TickMarkTier::One => (
                                                style.length_tier_1,
                                                style.width_tier_1,
                                                style.color_tier_1,
                                            ),
                                            TickMarkTier::Two => (
                                                style.length_tier_2,
                                                style.width_tier_2,
                                                style.color_tier_2,
                                            ),
                                            TickMarkTier::Three => (
                                                style.length_tier_3,
                                                style.width_tier_3,
                                                style.color_tier_3,
                                            ),
                                        };

                                    let half_height = height as f32 / 2.0;

                                    let x = notch_x - length as f32;

                                    let mark = Primitive::Quad {
                                        bounds: Rectangle {
                                            x,
                                            y: (start_y
                                                - y_offset
                                                - half_height)
                                                .floor(),
                                            width: length as f32,
                                            height: height as f32,
                                        },
                                        background: Background::Color(color),
                                        border_radius: 0,
                                        border_width: 0,
                                        border_color: Color::TRANSPARENT,
                                    };

                                    primitives.push(mark);
                                }
                            }

                            if style.placement != TickMarkPlacement::Left {
                                let notch_x = bounds_x
                                    + bounds_width
                                    + style.offset as f32;

                                for right_tick_mark in tick_marks.group.iter() {
                                    let y_offset = notch_span
                                        * right_tick_mark.position.value();

                                    let (length, height, color) =
                                        match right_tick_mark.tier {
                                            TickMarkTier::One => (
                                                style.length_tier_1,
                                                style.width_tier_1,
                                                style.color_tier_1,
                                            ),
                                            TickMarkTier::Two => (
                                                style.length_tier_2,
                                                style.width_tier_2,
                                                style.color_tier_2,
                                            ),
                                            TickMarkTier::Three => (
                                                style.length_tier_3,
                                                style.width_tier_3,
                                                style.color_tier_3,
                                            ),
                                        };

                                    let half_height = height as f32 / 2.0;

                                    let mark = Primitive::Quad {
                                        bounds: Rectangle {
                                            x: notch_x,
                                            y: (start_y
                                                - y_offset
                                                - half_height)
                                                .floor(),
                                            width: length as f32,
                                            height: height as f32,
                                        },
                                        background: Background::Color(color),
                                        border_radius: 0,
                                        border_width: 0,
                                        border_color: Color::TRANSPARENT,
                                    };

                                    primitives.push(mark);
                                }
                            }

                            Primitive::Group { primitives }
                        } else {
                            Primitive::None
                        }
                    } else {
                        Primitive::None
                    }
                };

                let bar: Primitive = {
                    if bar_normal.value() != 0.0 {
                        Primitive::Quad {
                            bounds: Rectangle {
                                x: bounds_x,
                                y: bounds_y,
                                width: bounds_width,
                                height: bounds_height * bar_normal.value(),
                            },
                            background: Background::Color(style.color),
                            border_radius: style.back_border_radius,
                            border_width: style.back_border_width,
                            border_color: Color::TRANSPARENT,
                        }
                    } else {
                        Primitive::None
                    }
                };

                let peak_line: Primitive = {
                    if let Some(peak_normal) = peak_normal {
                        let peak_width =
                            style.peak_line_width as f32 + (border_width * 2.0);
                        let half_peak_width = peak_width / 2.0;

                        if peak_normal.value() != 0.0 {
                            Primitive::Quad {
                                bounds: Rectangle {
                                    x: bounds_x,
                                    y: (bounds_y
                                        + (bounds_height
                                            * peak_normal.value())
                                        - half_peak_width)
                                        .round(),
                                    width: bounds_width,
                                    height: peak_width,
                                },
                                background: Background::Color(
                                    style.peak_line_color,
                                ),
                                border_radius: style.back_border_radius,
                                border_width: style.back_border_width,
                                border_color: Color::TRANSPARENT,
                            }
                        } else {
                            Primitive::None
                        }
                    } else {
                        Primitive::None
                    }
                };

                (
                    Primitive::Group {
                        primitives: vec![tick_marks, back, bar, peak_line],
                    },
                    MouseCursor::default(),
                )
            }
            Orientation::Horizontal => {
                let tick_marks: Primitive = {
                    if let Some(tick_marks) = tick_marks {
                        if let Some(style) = style_sheet.tick_mark_style() {
                            let notch_span =
                                bounds_width - (border_width * 2.0);
                            let start_x =
                                bounds_x + bounds_width - border_width;

                            let mut primitives: Vec<Primitive> = Vec::new();

                            if style.placement != TickMarkPlacement::Right {
                                let notch_y = bounds_y - style.offset as f32;

                                for left_tick_mark in tick_marks.group.iter() {
                                    let x_offset = notch_span
                                        * (1.0
                                            - left_tick_mark.position.value());

                                    let (length, width, color) =
                                        match left_tick_mark.tier {
                                            TickMarkTier::One => (
                                                style.length_tier_1,
                                                style.width_tier_1,
                                                style.color_tier_1,
                                            ),
                                            TickMarkTier::Two => (
                                                style.length_tier_2,
                                                style.width_tier_2,
                                                style.color_tier_2,
                                            ),
                                            TickMarkTier::Three => (
                                                style.length_tier_3,
                                                style.width_tier_3,
                                                style.color_tier_3,
                                            ),
                                        };

                                    let half_width = width as f32 / 2.0;

                                    let y = notch_y - length as f32;

                                    let mark = Primitive::Quad {
                                        bounds: Rectangle {
                                            x: (start_x
                                                - x_offset
                                                - half_width)
                                                .floor(),
                                            y,
                                            width: width as f32,
                                            height: length as f32,
                                        },
                                        background: Background::Color(color),
                                        border_radius: 0,
                                        border_width: 0,
                                        border_color: Color::TRANSPARENT,
                                    };

                                    primitives.push(mark);
                                }
                            }

                            if style.placement != TickMarkPlacement::Left {
                                let notch_y = bounds_y
                                    + bounds_height
                                    + style.offset as f32;

                                for right_tick_mark in tick_marks.group.iter() {
                                    let x_offset = notch_span
                                        * (1.0
                                            - right_tick_mark.position.value());

                                    let (length, width, color) =
                                        match right_tick_mark.tier {
                                            TickMarkTier::One => (
                                                style.length_tier_1,
                                                style.width_tier_1,
                                                style.color_tier_1,
                                            ),
                                            TickMarkTier::Two => (
                                                style.length_tier_2,
                                                style.width_tier_2,
                                                style.color_tier_2,
                                            ),
                                            TickMarkTier::Three => (
                                                style.length_tier_3,
                                                style.width_tier_3,
                                                style.color_tier_3,
                                            ),
                                        };

                                    let half_width = width as f32 / 2.0;

                                    let mark = Primitive::Quad {
                                        bounds: Rectangle {
                                            x: (start_x
                                                - x_offset
                                                - half_width)
                                                .floor(),
                                            y: notch_y,
                                            width: width as f32,
                                            height: length as f32,
                                        },
                                        background: Background::Color(color),
                                        border_radius: 0,
                                        border_width: 0,
                                        border_color: Color::TRANSPARENT,
                                    };

                                    primitives.push(mark);
                                }
                            }

                            Primitive::Group { primitives }
                        } else {
                            Primitive::None
                        }
                    } else {
                        Primitive::None
                    }
                };

                let bar: Primitive = {
                    if bar_normal.value() != 0.0 {
                        let bar_offset =
                            (bounds_width * (1.0 - bar_normal.value())).round();

                        Primitive::Quad {
                            bounds: Rectangle {
                                x: bounds_x + bar_offset,
                                y: bounds_y,
                                width: bounds_width - bar_offset,
                                height: bounds_height,
                            },
                            background: Background::Color(style.color),
                            border_radius: style.back_border_radius,
                            border_width: style.back_border_width,
                            border_color: Color::TRANSPARENT,
                        }
                    } else {
                        Primitive::None
                    }
                };

                let peak_line: Primitive = {
                    if let Some(peak_normal) = peak_normal {
                        let peak_width =
                            style.peak_line_width as f32 + (border_width * 2.0);

                        let peak_offset = ((bounds_width - peak_width)
                            * (1.0 - peak_normal.value()))
                        .round();

                        if peak_normal.value() != 0.0 {
                            Primitive::Quad {
                                bounds: Rectangle {
                                    x: bounds_x + peak_offset,
                                    y: bounds_y,
                                    width: peak_width,
                                    height: bounds_height,
                                },
                                background: Background::Color(
                                    style.peak_line_color,
                                ),
                                border_radius: style.back_border_radius,
                                border_width: style.back_border_width,
                                border_color: Color::TRANSPARENT,
                            }
                        } else {
                            Primitive::None
                        }
                    } else {
                        Primitive::None
                    }
                };

                (
                    Primitive::Group {
                        primitives: vec![tick_marks, back, bar, peak_line],
                    },
                    MouseCursor::default(),
                )
            }
        }
    }
}
