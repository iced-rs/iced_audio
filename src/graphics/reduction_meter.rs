//! `iced_graphics` renderer for the [`ReductionMeter`] widget
//!
//! [`ReductionMeter`]: ../native/reduction_meter/struct.ReductionMeter.html

use crate::core::{Normal, TickMarkGroup};
use crate::graphics::bar_tick_marks;
use crate::native::reduction_meter;
use iced_graphics::{Backend, Primitive, Renderer};
use iced_native::{mouse, Background, Color, Rectangle};

pub use crate::native::reduction_meter::{Orientation, State};
pub use crate::style::reduction_meter::{Style, StyleSheet};

/// This is an alias of a `crate::native` [`ReductionMeter`] with an
/// `iced_graphics::Renderer`.
///
/// [`ReductionMeter`]: ../../native/reduction_meter/struct.ReductionMeter.html
pub type ReductionMeter<'a, Backend> =
    reduction_meter::ReductionMeter<'a, Renderer<Backend>>;

impl<B: Backend> reduction_meter::Renderer for Renderer<B> {
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
                            bar_tick_marks::draw_vertical_tick_marks(
                                bounds_x,
                                bounds_y + border_width,
                                bounds_width,
                                bounds_height - (border_width * 2.0),
                                tick_marks,
                                &style,
                                false,
                            )
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
                    mouse::Interaction::default(),
                )
            }
            Orientation::Horizontal => {
                let tick_marks: Primitive = {
                    if let Some(tick_marks) = tick_marks {
                        if let Some(style) = style_sheet.tick_mark_style() {
                            bar_tick_marks::draw_horizontal_tick_marks(
                                bounds_x + border_width,
                                bounds_y,
                                bounds_width - (border_width * 2.0),
                                bounds_height,
                                tick_marks,
                                &style,
                                false,
                            )
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
                    mouse::Interaction::default(),
                )
            }
        }
    }
}
