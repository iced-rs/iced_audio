//! wgpu renderer for the [`DBMeter`] widget
//!
//! [`DBMeter`]: ../native/db_meter/struct.DBMeter.html

use crate::core::{Normal, TickMarkGroup};
use crate::native::db_meter;
use crate::wgpu::bar_tick_marks;
use iced_native::{Background, Color, MouseCursor, Rectangle};
use iced_wgpu::{Primitive, Renderer};

pub use crate::native::db_meter::{
    BarState, Orientation, State, TierPositions,
};
pub use crate::style::db_meter::{Style, StyleSheet};

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

        match orientation {
            Orientation::Vertical => {
                let bar_height = bounds_height - twice_border_width;
                let bar_y = bounds_y + border_width;

                let tick_marks: Primitive = {
                    if let Some(tick_marks) = tick_marks {
                        if let Some(style) = style_sheet.tick_mark_style() {
                            bar_tick_marks::draw_vertical_tick_marks(
                                bounds_x,
                                bar_y,
                                bounds_width,
                                bar_height,
                                &tick_marks,
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

                let clip_marker_height = style.clip_marker_width as f32;
                let half_clip_marker_height =
                    (clip_marker_height / 2.0).round();
                let clip_y = (bar_y
                    + (bar_height * (1.0 - tier_positions.clipping.value()))
                    - half_clip_marker_height)
                    .floor();

                let clip_marker = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds_x + border_width,
                        y: clip_y,
                        width: bounds_width - twice_border_width,
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
                        ((bounds_width - twice_border_width - inner_gap) / 2.0)
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
                    let bar_width = bounds_width - twice_border_width;
                    let bar_height = bounds_height - twice_border_width;
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
                let bar_width = bounds_width - twice_border_width;
                let bar_x = bounds_x + border_width;

                let tick_marks: Primitive = {
                    if let Some(tick_marks) = tick_marks {
                        if let Some(style) = style_sheet.tick_mark_style() {
                            bar_tick_marks::draw_horizontal_tick_marks(
                                bar_x,
                                bounds_y,
                                bar_width,
                                bounds_height,
                                &tick_marks,
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

                let clip_marker_width = style.clip_marker_width as f32;
                let half_clip_marker_width = clip_marker_width / 2.0;
                let clip_x = (bar_x
                    + (bar_width * tier_positions.clipping.value())
                    - half_clip_marker_width)
                    .floor();

                let clip_marker = Primitive::Quad {
                    bounds: Rectangle {
                        x: clip_x,
                        y: bounds_y + border_width,
                        width: clip_marker_width,
                        height: bounds_height - twice_border_width,
                    },
                    background: Background::Color(style.clip_marker_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                };

                if let Some(right_bar) = right_bar {
                    let inner_gap = style.inner_gap as f32;

                    let bar_height =
                        ((bounds_height - twice_border_width - inner_gap)
                            / 2.0)
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
                    let bar_width = bounds_width - twice_border_width;
                    let bar_height = bounds_height - twice_border_width;
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
