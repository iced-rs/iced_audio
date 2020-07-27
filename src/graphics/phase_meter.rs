//! `iced_graphics` renderer for the [`PhaseMeter`] widget
//!
//! [`PhaseMeter`]: ../native/phase_meter/struct.PhaseMeter.html

use crate::core::{Normal, TextMarkGroup, TickMarkGroup};
use crate::graphics::{bar_text_marks, bar_tick_marks};
use crate::native::phase_meter;
use iced_graphics::{Backend, Primitive, Renderer};
use iced_native::{mouse, Background, Color, Rectangle};

pub use crate::native::phase_meter::{Orientation, State, TierPositions};
pub use crate::style::phase_meter::{Style, StyleSheet};

/// This is an alias of a `crate::native` [`PhaseMeter`] with an
/// `iced_graphics::Renderer`.
///
/// [`PhaseMeter`]: ../../native/phase_meter/struct.PhaseMeter.html
pub type PhaseMeter<'a, Backend> =
    phase_meter::PhaseMeter<'a, Renderer<Backend>>;

#[derive(PartialEq)]
enum PhaseTier {
    Bad,
    Poor,
    Okay,
    Good,
}

fn tier(
    normal: Normal,
    tier_positions: phase_meter::TierPositions,
) -> PhaseTier {
    let value = normal.value();

    if value >= (0.5 + (tier_positions.good.value() / 2.0)) {
        PhaseTier::Good
    } else if value >= 0.5 {
        PhaseTier::Okay
    } else if value >= (tier_positions.poor.value() / 2.0) {
        PhaseTier::Poor
    } else {
        PhaseTier::Bad
    }
}

impl<B: Backend> phase_meter::Renderer for Renderer<B> {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        normal: Normal,
        tier_positions: TierPositions,
        orientation: &Orientation,
        tick_marks: Option<&TickMarkGroup>,
        text_marks: Option<&TextMarkGroup>,
        style_sheet: &Self::Style,
    ) -> Self::Output {
        let bounds_x = bounds.x.floor();
        let bounds_y = bounds.y.floor();

        let bounds_width = bounds.width.floor();
        let bounds_height = bounds.height.floor();

        let style = style_sheet.style();

        let border_width = style.back_border_width as f32;
        let twice_border_width = border_width * 2.0;

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

        let center_line_width = style.center_line_width as f32;

        match orientation {
            Orientation::Horizontal => {
                let h_center = (bounds_width / 2.0).floor();

                let bar_y = bounds_y + border_width;
                let bar_height = bounds_height - twice_border_width;

                let bar_x = bounds_x + border_width;
                let bar_width = bounds_width - twice_border_width;

                let tick_marks: Primitive = {
                    if let Some(tick_marks) = tick_marks {
                        if let Some(style) = style_sheet.tick_mark_style() {
                            bar_tick_marks::draw_horizontal_tick_marks(
                                bar_x,
                                bounds_y,
                                bar_width,
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

                let text_marks: Primitive = {
                    if let Some(text_marks) = text_marks {
                        if let Some(style) = style_sheet.text_mark_style() {
                            bar_text_marks::draw_horizontal_text_marks(
                                bar_x,
                                bounds_y,
                                bar_width,
                                bounds_height,
                                &text_marks,
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

                let center_line = Primitive::Quad {
                    bounds: Rectangle {
                        x: (bounds_x + h_center - (center_line_width / 2.0))
                            .floor(),
                        y: bar_y,
                        width: center_line_width,
                        height: bar_height,
                    },
                    background: Background::Color(style.center_line_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                };

                if normal.value() < 0.499 || normal.value() > 0.501 {
                    let meter_span = bounds_width - twice_border_width;

                    let normal_offset =
                        ((normal.value() * meter_span) + border_width).floor();

                    let normal_tier = tier(normal, tier_positions);

                    match normal_tier {
                        PhaseTier::Bad => {
                            let poor_offset = (meter_span
                                * (tier_positions.poor.value() / 2.0)
                                + border_width)
                                .floor();

                            let bad_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bounds_x + normal_offset,
                                    y: bar_y,
                                    width: poor_offset - normal_offset,
                                    height: bar_height,
                                },
                                background: Background::Color(style.bad_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };
                            let poor_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bounds_x + poor_offset,
                                    y: bar_y,
                                    width: h_center - poor_offset,
                                    height: bar_height,
                                },
                                background: Background::Color(style.poor_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            (
                                Primitive::Group {
                                    primitives: vec![
                                        tick_marks,
                                        text_marks,
                                        back,
                                        bad_bar,
                                        poor_bar,
                                        center_line,
                                    ],
                                },
                                mouse::Interaction::default(),
                            )
                        }
                        PhaseTier::Poor => {
                            let poor_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bounds_x + normal_offset,
                                    y: bar_y,
                                    width: h_center - normal_offset,
                                    height: bar_height,
                                },
                                background: Background::Color(style.poor_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            (
                                Primitive::Group {
                                    primitives: vec![
                                        tick_marks,
                                        text_marks,
                                        back,
                                        poor_bar,
                                        center_line,
                                    ],
                                },
                                mouse::Interaction::default(),
                            )
                        }
                        PhaseTier::Okay => {
                            let okay_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bounds_x + h_center,
                                    y: bar_y,
                                    width: normal_offset - h_center,
                                    height: bar_height,
                                },
                                background: Background::Color(style.okay_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            (
                                Primitive::Group {
                                    primitives: vec![
                                        tick_marks,
                                        text_marks,
                                        back,
                                        okay_bar,
                                        center_line,
                                    ],
                                },
                                mouse::Interaction::default(),
                            )
                        }
                        PhaseTier::Good => {
                            let good_offset = (meter_span
                                * (0.5 + (tier_positions.good.value() / 2.0))
                                + border_width)
                                .floor();

                            let okay_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bounds_x + h_center,
                                    y: bar_y,
                                    width: good_offset - h_center,
                                    height: bar_height,
                                },
                                background: Background::Color(style.okay_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };
                            let good_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bounds_x + good_offset,
                                    y: bar_y,
                                    width: normal_offset - good_offset,
                                    height: bar_height,
                                },
                                background: Background::Color(style.good_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            (
                                Primitive::Group {
                                    primitives: vec![
                                        tick_marks,
                                        text_marks,
                                        back,
                                        okay_bar,
                                        good_bar,
                                        center_line,
                                    ],
                                },
                                mouse::Interaction::default(),
                            )
                        }
                    }
                } else {
                    (
                        Primitive::Group {
                            primitives: vec![
                                tick_marks,
                                text_marks,
                                back,
                                center_line,
                            ],
                        },
                        mouse::Interaction::default(),
                    )
                }
            }
            Orientation::Vertical => {
                let v_center = (bounds_height / 2.0).floor();

                let bar_x = bounds_x + border_width;
                let bar_width = bounds_width - twice_border_width;

                let bar_y = bounds_y + border_width;
                let bar_height = bounds_height - twice_border_width;

                let tick_marks: Primitive = {
                    if let Some(tick_marks) = tick_marks {
                        if let Some(style) = style_sheet.tick_mark_style() {
                            bar_tick_marks::draw_vertical_tick_marks(
                                bounds_x,
                                bar_y,
                                bounds_width,
                                bar_height,
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

                let text_marks: Primitive = {
                    if let Some(text_marks) = text_marks {
                        if let Some(style) = style_sheet.text_mark_style() {
                            bar_text_marks::draw_vertical_text_marks(
                                bounds_x,
                                bar_y,
                                bounds_width,
                                bar_height,
                                &text_marks,
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

                let center_line = Primitive::Quad {
                    bounds: Rectangle {
                        x: bar_x,
                        y: (bounds_y + v_center - (center_line_width / 2.0))
                            .floor(),
                        width: bar_width,
                        height: center_line_width,
                    },
                    background: Background::Color(style.center_line_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                };

                if normal.value() < 0.499 || normal.value() > 0.501 {
                    let meter_span = bounds_height - twice_border_width;

                    let normal_offset = (((1.0 - normal.value()) * meter_span)
                        + border_width)
                        .floor();

                    let normal_tier = tier(normal, tier_positions);

                    match normal_tier {
                        PhaseTier::Bad => {
                            let poor_offset = (meter_span
                                * (1.0 - (tier_positions.poor.value() / 2.0))
                                + border_width)
                                .floor();

                            let poor_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bar_x,
                                    y: bounds_y + v_center,
                                    width: bar_width,
                                    height: poor_offset - v_center,
                                },
                                background: Background::Color(style.poor_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };
                            let bad_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bar_x,
                                    y: bounds_y + poor_offset,
                                    width: bar_width,
                                    height: normal_offset - poor_offset,
                                },
                                background: Background::Color(style.bad_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            (
                                Primitive::Group {
                                    primitives: vec![
                                        tick_marks,
                                        text_marks,
                                        back,
                                        poor_bar,
                                        bad_bar,
                                        center_line,
                                    ],
                                },
                                mouse::Interaction::default(),
                            )
                        }
                        PhaseTier::Poor => {
                            let poor_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bar_x,
                                    y: bounds_y + v_center,
                                    width: bar_width,
                                    height: normal_offset - v_center,
                                },
                                background: Background::Color(style.poor_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            (
                                Primitive::Group {
                                    primitives: vec![
                                        tick_marks,
                                        text_marks,
                                        back,
                                        poor_bar,
                                        center_line,
                                    ],
                                },
                                mouse::Interaction::default(),
                            )
                        }
                        PhaseTier::Okay => {
                            let okay_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bar_x,
                                    y: bounds_y + normal_offset,
                                    width: bar_width,
                                    height: v_center - normal_offset,
                                },
                                background: Background::Color(style.okay_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            (
                                Primitive::Group {
                                    primitives: vec![
                                        tick_marks,
                                        text_marks,
                                        back,
                                        okay_bar,
                                        center_line,
                                    ],
                                },
                                mouse::Interaction::default(),
                            )
                        }
                        PhaseTier::Good => {
                            let good_offset = (meter_span
                                * (1.0
                                    - (0.5
                                        + (tier_positions.good.value()
                                            / 2.0)))
                                + border_width)
                                .floor();

                            let good_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bar_x,
                                    y: bounds_y + normal_offset,
                                    width: bar_width,
                                    height: good_offset - normal_offset,
                                },
                                background: Background::Color(style.good_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };
                            let okay_bar = Primitive::Quad {
                                bounds: Rectangle {
                                    x: bar_x,
                                    y: bounds_y + good_offset,
                                    width: bar_width,
                                    height: v_center - good_offset,
                                },
                                background: Background::Color(style.okay_color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            (
                                Primitive::Group {
                                    primitives: vec![
                                        tick_marks,
                                        text_marks,
                                        back,
                                        okay_bar,
                                        good_bar,
                                        center_line,
                                    ],
                                },
                                mouse::Interaction::default(),
                            )
                        }
                    }
                } else {
                    (
                        Primitive::Group {
                            primitives: vec![
                                tick_marks,
                                text_marks,
                                back,
                                center_line,
                            ],
                        },
                        mouse::Interaction::default(),
                    )
                }
            }
        }
    }
}
