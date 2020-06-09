//! wgpu renderer for the [`DBMeter`] widget
//!
//! [`DBMeter`]: ../native/db_meter/struct.DBMeter.html

use crate::core::{Normal, TickMarkGroup, TickMarkTier};
use crate::native::db_meter;
use iced_native::{Background, Color, MouseCursor, Rectangle};
use iced_wgpu::{Primitive, Renderer};

pub use crate::native::db_meter::{
    BarState, Orientation, State, TierPositions,
};
pub use crate::style::db_meter::{
    Style, StyleSheet, TickMarkPlacement, TickMarkStyle,
};

/// This is an alias of a `crate::native` [`DBMeter`] with an
/// `iced_wgpu::Renderer`.
///
/// [`DBMeter`]: ../../native/db_meter/struct.DBMeter.html
pub type DBMeter<'a> = db_meter::DBMeter<'a, Renderer>;

#[derive(PartialEq)]
enum DBTier {
    Low,
    Med,
    High,
    Clipping,
}

fn tier(normal: Normal, tier_positions: db_meter::TierPositions) -> DBTier {
    let value = normal.value();

    if value >= tier_positions.clipping.value() {
        return DBTier::Clipping;
    }

    if let Some(high) = tier_positions.high {
        if value >= high.value() {
            return DBTier::High;
        }

        if let Some(med) = tier_positions.med {
            if value >= med.value() {
                return DBTier::Med;
            }
        }
    }

    DBTier::Low
}

fn v_meter(
    normal: Normal,
    peak_normal: Option<Normal>,
    tier_positions: TierPositions,
    style: &Style,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Primitive {
    let normal_tier = tier(normal, tier_positions);

    let mut color_all_clip_color =
        style.color_all_clip_color && normal_tier == DBTier::Clipping;

    let peak_line: Primitive = {
        if let Some(peak_normal) = peak_normal {
            if peak_normal.value() != 0.0 {
                let peak_tier = tier(peak_normal, tier_positions);
                color_all_clip_color =
                    style.color_all_clip_color && peak_tier == DBTier::Clipping;

                let peak_height = style.peak_line_width as f32;

                let peak_offset = ((height - peak_height)
                    * (1.0 - peak_normal.value()))
                .round();

                let peak_color = {
                    if peak_tier == DBTier::Clipping {
                        style.clip_color
                    } else if let Some(peak_color) = style.peak_line_color {
                        peak_color
                    } else {
                        match peak_tier {
                            DBTier::Low => style.low_color,
                            DBTier::Med => style.med_color,
                            DBTier::High => style.high_color,
                            DBTier::Clipping => style.clip_color,
                        }
                    }
                };

                Primitive::Quad {
                    bounds: Rectangle {
                        x,
                        y: y + peak_offset,
                        width,
                        height: peak_height,
                    },
                    background: Background::Color(peak_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        }
    };

    if normal.value() != 0.0 {
        let normal_offset = height * (1.0 - normal.value());

        let clip_offset = height * (1.0 - tier_positions.clipping.value());

        let high_offset = {
            if let Some(high) = tier_positions.high {
                height * (1.0 - high.value())
            } else {
                clip_offset
            }
        };

        let med_offset = {
            if let Some(med) = tier_positions.med {
                height * (1.0 - med.value())
            } else {
                high_offset
            }
        };

        let low_bar_y = match normal_tier {
            DBTier::Low => y + normal_offset,
            _ => y + med_offset,
        };

        let med_bar_y = match normal_tier {
            DBTier::Med => y + normal_offset,
            _ => y + high_offset,
        };

        let high_bar_y = match normal_tier {
            DBTier::High => y + normal_offset,
            _ => y + clip_offset,
        };

        let clip_bar_y = match normal_tier {
            DBTier::Clipping => y + normal_offset,
            _ => y,
        };

        let low_bar = Primitive::Quad {
            bounds: Rectangle {
                x,
                y: low_bar_y,
                width,
                height: y + height - low_bar_y,
            },
            background: Background::Color(if color_all_clip_color {
                style.clip_color
            } else {
                style.low_color
            }),
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        };

        let med_bar: Primitive = {
            if normal_tier != DBTier::Low && med_offset != high_offset {
                Primitive::Quad {
                    bounds: Rectangle {
                        x,
                        y: med_bar_y,
                        width,
                        height: low_bar_y - med_bar_y,
                    },
                    background: Background::Color(if color_all_clip_color {
                        style.clip_color
                    } else {
                        style.med_color
                    }),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            } else {
                Primitive::None
            }
        };

        let high_bar: Primitive = {
            if normal_tier != DBTier::Low
                && normal_tier != DBTier::Med
                && high_offset != clip_offset
            {
                Primitive::Quad {
                    bounds: Rectangle {
                        x,
                        y: high_bar_y,
                        width,
                        height: med_bar_y - high_bar_y,
                    },
                    background: Background::Color(if color_all_clip_color {
                        style.clip_color
                    } else {
                        style.high_color
                    }),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            } else {
                Primitive::None
            }
        };

        let clip_bar: Primitive = {
            if normal_tier == DBTier::Clipping {
                Primitive::Quad {
                    bounds: Rectangle {
                        x,
                        y: clip_bar_y,
                        width,
                        height: high_bar_y - clip_bar_y,
                    },
                    background: Background::Color(style.clip_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            } else {
                Primitive::None
            }
        };

        Primitive::Group {
            primitives: vec![low_bar, med_bar, high_bar, clip_bar, peak_line],
        }
    } else {
        peak_line
    }
}

fn h_meter(
    normal: Normal,
    peak_normal: Option<Normal>,
    tier_positions: TierPositions,
    style: &Style,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Primitive {
    let normal_tier = tier(normal, tier_positions);

    let mut color_all_clip_color =
        style.color_all_clip_color && normal_tier == DBTier::Clipping;

    let peak_line: Primitive = {
        if let Some(peak_normal) = peak_normal {
            if peak_normal.value() != 0.0 {
                let peak_tier = tier(peak_normal, tier_positions);
                color_all_clip_color =
                    style.color_all_clip_color && peak_tier == DBTier::Clipping;

                let peak_width = style.peak_line_width as f32;

                let peak_offset =
                    ((width - peak_width) * peak_normal.value()).round();

                let peak_color = {
                    if peak_tier == DBTier::Clipping {
                        style.clip_color
                    } else if let Some(peak_color) = style.peak_line_color {
                        peak_color
                    } else {
                        match peak_tier {
                            DBTier::Low => style.low_color,
                            DBTier::Med => style.med_color,
                            DBTier::High => style.high_color,
                            DBTier::Clipping => style.clip_color,
                        }
                    }
                };

                Primitive::Quad {
                    bounds: Rectangle {
                        x: x + peak_offset,
                        y,
                        width: peak_width,
                        height,
                    },
                    background: Background::Color(peak_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        }
    };

    if normal.value() != 0.0 {
        let normal_offset = width * normal.value();

        let clip_offset = width * tier_positions.clipping.value();

        let high_offset = {
            if let Some(high) = tier_positions.high {
                width * high.value()
            } else {
                clip_offset
            }
        };

        let med_offset = {
            if let Some(med) = tier_positions.med {
                width * med.value()
            } else {
                high_offset
            }
        };

        let low_bar_x = match normal_tier {
            DBTier::Low => x + normal_offset,
            _ => x + med_offset,
        };

        let med_bar_x = match normal_tier {
            DBTier::Med => x + normal_offset,
            _ => x + high_offset,
        };

        let high_bar_x = match normal_tier {
            DBTier::High => x + normal_offset,
            _ => x + clip_offset,
        };

        let clip_bar_x = match normal_tier {
            DBTier::Clipping => x + normal_offset,
            _ => x,
        };

        let low_bar = Primitive::Quad {
            bounds: Rectangle {
                x,
                y,
                width: low_bar_x - x,
                height,
            },
            background: Background::Color(if color_all_clip_color {
                style.clip_color
            } else {
                style.low_color
            }),
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        };

        let med_bar: Primitive = {
            if normal_tier != DBTier::Low && med_offset != high_offset {
                Primitive::Quad {
                    bounds: Rectangle {
                        x: low_bar_x,
                        y,
                        width: med_bar_x - low_bar_x,
                        height,
                    },
                    background: Background::Color(if color_all_clip_color {
                        style.clip_color
                    } else {
                        style.med_color
                    }),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            } else {
                Primitive::None
            }
        };

        let high_bar: Primitive = {
            if normal_tier != DBTier::Low
                && normal_tier != DBTier::Med
                && high_offset != clip_offset
            {
                Primitive::Quad {
                    bounds: Rectangle {
                        x: med_bar_x,
                        y,
                        width: high_bar_x - med_bar_x,
                        height,
                    },
                    background: Background::Color(if color_all_clip_color {
                        style.clip_color
                    } else {
                        style.high_color
                    }),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            } else {
                Primitive::None
            }
        };

        let clip_bar: Primitive = {
            if normal_tier == DBTier::Clipping {
                Primitive::Quad {
                    bounds: Rectangle {
                        x: high_bar_x,
                        y,
                        width: clip_bar_x - high_bar_x,
                        height,
                    },
                    background: Background::Color(style.clip_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            } else {
                Primitive::None
            }
        };

        Primitive::Group {
            primitives: vec![low_bar, med_bar, high_bar, clip_bar, peak_line],
        }
    } else {
        peak_line
    }
}

impl db_meter::Renderer for Renderer {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        left_bar: BarState,
        right_bar: Option<BarState>,
        tier_positions: TierPositions,
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
            border_radius: 0,
            border_width: style.back_border_width,
            border_color: style.back_border_color,
        };

        match orientation {
            Orientation::Vertical => {
                let bar_height = bounds_height - (border_width * 2.0);
                let bar_y = bounds_y + border_width;

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

                let clip_marker_height = style.clip_marker_width as f32;
                let half_clip_marker_height =
                    (clip_marker_height * 0.5).round();
                let clip_y = (bar_y
                    + (bar_height * (1.0 - tier_positions.clipping.value()))
                    - half_clip_marker_height)
                    .floor();

                let clip_marker = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + border_width,
                        y: clip_y,
                        width: bounds_width - (border_width * 2.0),
                        height: clip_marker_height,
                    },
                    background: Background::Color(style.clip_marker_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                };

                if let Some(right_bar) = right_bar {
                    let inner_gap = style.inner_gap as f32;

                    let bar_width =
                        ((bounds_width - (border_width * 2.0) - inner_gap)
                            * 0.5)
                            .floor();

                    let left_bar_x = bounds_x + border_width;
                    let right_bar_x =
                        bounds_x + bounds_width - border_width - bar_width;

                    let left_meter = v_meter(
                        left_bar.normal,
                        left_bar.peak_normal,
                        tier_positions,
                        &style,
                        left_bar_x,
                        bar_y,
                        bar_width,
                        bar_height,
                    );

                    let right_meter = v_meter(
                        right_bar.normal,
                        right_bar.peak_normal,
                        tier_positions,
                        &style,
                        right_bar_x,
                        bar_y,
                        bar_width,
                        bar_height,
                    );

                    let inner_gap = Primitive::Quad {
                        bounds: Rectangle {
                            x: left_bar_x + bar_width,
                            y: bounds_y,
                            width: right_bar_x - (left_bar_x + bar_width),
                            height: bounds_height,
                        },
                        background: Background::Color(style.inner_gap_color),
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    };

                    (
                        Primitive::Group {
                            primitives: vec![
                                tick_marks,
                                back,
                                clip_marker,
                                inner_gap,
                                left_meter,
                                right_meter,
                            ],
                        },
                        MouseCursor::default(),
                    )
                } else {
                    let bar_width = bounds_width - (border_width * 2.0);
                    let bar_height = bounds_height - (border_width * 2.0);
                    let bar_x = bounds_x + border_width;
                    let bar_y = bounds_y + border_width;

                    let meter = v_meter(
                        left_bar.normal,
                        left_bar.peak_normal,
                        tier_positions,
                        &style,
                        bar_x,
                        bar_y,
                        bar_width,
                        bar_height,
                    );

                    (
                        Primitive::Group {
                            primitives: vec![
                                tick_marks,
                                back,
                                clip_marker,
                                meter,
                            ],
                        },
                        MouseCursor::default(),
                    )
                }
            }
            Orientation::Horizontal => {
                let bar_width = bounds_width - (border_width * 2.0);
                let bar_x = bounds_x + border_width;

                let tick_marks: Primitive = {
                    if let Some(tick_marks) = tick_marks {
                        if let Some(style) = style_sheet.tick_mark_style() {
                            let notch_span =
                                bounds_width - (border_width * 2.0);

                            let mut primitives: Vec<Primitive> = Vec::new();

                            if style.placement != TickMarkPlacement::Right {
                                let notch_y = bounds_y - style.offset as f32;

                                for left_tick_mark in tick_marks.group.iter() {
                                    let x_offset = notch_span
                                        * left_tick_mark.position.value();

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
                                            x: (bar_x + x_offset - half_width)
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
                                        * right_tick_mark.position.value();

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
                                            x: (bar_x + x_offset - half_width)
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

                let clip_marker_width = style.clip_marker_width as f32;
                let half_clip_marker_width = clip_marker_width * 0.5;
                let clip_x = (bar_x
                    + (bar_width * tier_positions.clipping.value())
                    - half_clip_marker_width)
                    .floor();

                let clip_marker = Primitive::Quad {
                    bounds: Rectangle {
                        x: clip_x,
                        y: bounds_y + border_width,
                        width: clip_marker_width,
                        height: bounds_height - (border_width * 2.0),
                    },
                    background: Background::Color(style.clip_marker_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                };

                if let Some(right_bar) = right_bar {
                    let inner_gap = style.inner_gap as f32;

                    let bar_height =
                        ((bounds_height - (border_width * 2.0) - inner_gap)
                            * 0.5)
                            .floor();

                    let left_bar_y = bounds_y + border_width;
                    let right_bar_y =
                        bounds_y + bounds_height - border_width - bar_height;

                    let left_meter = h_meter(
                        left_bar.normal,
                        left_bar.peak_normal,
                        tier_positions,
                        &style,
                        bar_x,
                        left_bar_y,
                        bar_width,
                        bar_height,
                    );

                    let right_meter = h_meter(
                        right_bar.normal,
                        right_bar.peak_normal,
                        tier_positions,
                        &style,
                        bar_x,
                        right_bar_y,
                        bar_width,
                        bar_height,
                    );

                    let inner_gap = Primitive::Quad {
                        bounds: Rectangle {
                            x: bounds_x,
                            y: left_bar_y + bar_height,
                            width: bounds_width,
                            height: right_bar_y - (left_bar_y + bar_height),
                        },
                        background: Background::Color(style.inner_gap_color),
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    };

                    (
                        Primitive::Group {
                            primitives: vec![
                                tick_marks,
                                back,
                                clip_marker,
                                inner_gap,
                                left_meter,
                                right_meter,
                            ],
                        },
                        MouseCursor::default(),
                    )
                } else {
                    let bar_width = bounds_width - (border_width * 2.0);
                    let bar_height = bounds_height - (border_width * 2.0);
                    let bar_x = bounds_x + border_width;
                    let bar_y = bounds_y + border_width;

                    let meter = h_meter(
                        left_bar.normal,
                        left_bar.peak_normal,
                        tier_positions,
                        &style,
                        bar_x,
                        bar_y,
                        bar_width,
                        bar_height,
                    );

                    (
                        Primitive::Group {
                            primitives: vec![
                                tick_marks,
                                back,
                                clip_marker,
                                meter,
                            ],
                        },
                        MouseCursor::default(),
                    )
                }
            }
        }
    }
}
