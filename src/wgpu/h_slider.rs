//! wgpu renderer for the [`HSlider`] widget
//!
//! [`HSlider`]: ../native/h_slider/struct.HSlider.html

use crate::core::{ModulationRange, Normal, TickMarkGroup};
use crate::native::h_slider;
use iced_native::{Background, Color, MouseCursor, Point, Rectangle};
use iced_wgpu::{Primitive, Renderer};

pub use crate::native::h_slider::State;
pub use crate::style::h_slider::{
    ClassicHandle, ClassicStyle, ModRangePlacement, ModRangeStyle,
    RectBipolarStyle, RectStyle, Style, StyleSheet, TextureStyle,
    TickMarkStyle,
};

/// This is an alias of a `crate::native` [`HSlider`] with an
/// `iced_wgpu::Renderer`.
///
/// [`HSlider`]: ../../native/h_slider/struct.HSlider.html
pub type HSlider<'a, Message, ID> =
    h_slider::HSlider<'a, Message, Renderer, ID>;

impl h_slider::Renderer for Renderer {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        mod_range: Option<ModulationRange>,
        tick_marks: Option<&TickMarkGroup>,
        style_sheet: &Self::Style,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_dragging {
            style_sheet.dragging()
        } else if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.active()
        };

        let tick_mark_style = style_sheet.tick_mark_style();

        let bounds_x = bounds.x.floor();
        let bounds_y = bounds.y.floor();

        let bounds_width = bounds.width.floor();
        let bounds_height = bounds.height.floor();

        let rail_y = (bounds_y + (bounds_height / 2.0)).floor();

        let mod_range_line = {
            if let Some(mod_range) = mod_range {
                if mod_range.visible {
                    if let Some(style) = style_sheet.mod_range_style() {
                        draw_mod_range(
                            bounds_x,
                            bounds_y,
                            bounds_width,
                            bounds_height,
                            mod_range,
                            &style,
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

        let primitives = match style {
            Style::Texture(style) => draw_texture_style(
                normal,
                rail_y,
                bounds_x,
                bounds_y,
                bounds_width,
                bounds_height,
                tick_marks,
                &tick_mark_style,
                style,
                mod_range_line,
            ),
            Style::Classic(style) => draw_classic_style(
                normal,
                rail_y,
                bounds_x,
                bounds_y,
                bounds_width,
                bounds_height,
                tick_marks,
                &tick_mark_style,
                &style,
                mod_range_line,
            ),
            Style::Rect(style) => draw_rect_style(
                normal,
                rail_y,
                bounds_x,
                bounds_y,
                bounds_width,
                bounds_height,
                tick_marks,
                &tick_mark_style,
                &style,
                mod_range_line,
            ),
            Style::RectBipolar(style) => draw_rect_bipolar_style(
                normal,
                rail_y,
                bounds_x,
                bounds_y,
                bounds_width,
                bounds_height,
                tick_marks,
                &tick_mark_style,
                &style,
                mod_range_line,
            ),
        };

        (primitives, MouseCursor::default())
    }
}

fn draw_mod_range(
    bounds_x: f32,
    bounds_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    mod_range: ModulationRange,
    style: &ModRangeStyle,
) -> Primitive {
    let offset = style.offset as f32;

    let (y, height) = match style.placement {
        ModRangePlacement::Center => {
            (bounds_y + offset, bounds_height - (offset * 2.0))
        }
        ModRangePlacement::Top => {
            (bounds_y - offset - style.width as f32, style.width as f32)
        }
        ModRangePlacement::Bottom => {
            (bounds_y + bounds_height + offset, style.width as f32)
        }
    };

    let back: Primitive = {
        if let Some(empty_color) = style.empty_color {
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds_x,
                    y,
                    width: bounds_width,
                    height,
                },
                background: Background::Color(empty_color),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            }
        } else {
            Primitive::None
        }
    };

    let filled: Primitive = {
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

            let start_offset = bounds_width * start;
            let filled_width = (bounds_width * end) - start_offset;

            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds_x + start_offset,
                    y,
                    width: filled_width,
                    height,
                },
                background: Background::Color(color),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            }
        } else {
            Primitive::None
        }
    };

    Primitive::Group {
        primitives: vec![back, filled],
    }
}

fn draw_texture_style(
    normal: Normal,
    rail_y: f32,
    bounds_x: f32,
    bounds_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    tick_marks: Option<&TickMarkGroup>,
    tick_mark_style: &Option<TickMarkStyle>,
    style: TextureStyle,
    mod_range_line: Primitive,
) -> Primitive {
    let handle_width = style.handle_width as f32;

    let tick_marks: Primitive = {
        if let Some(tick_marks) = tick_marks {
            if let Some(style) = tick_mark_style {
                draw_tick_marks(
                    rail_y,
                    (bounds_x + (handle_width / 2.0)).floor(),
                    bounds_width - handle_width,
                    bounds_height,
                    tick_marks,
                    &style,
                )
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        }
    };

    let (top_rail_width, bottom_rail_width) = style.rail_widths;
    let (top_rail_color, bottom_rail_color) = style.rail_colors;
    let (top_rail, bottom_rail) = draw_rails(
        rail_y,
        bounds_x,
        bounds_width,
        top_rail_width,
        bottom_rail_width,
        &top_rail_color,
        &bottom_rail_color,
    );

    let handle_offset = normal.scale(bounds_width - handle_width).floor();

    let handle = {
        if let Some(pad) = style.texture_padding {
            Primitive::Image {
                handle: style.texture,
                bounds: Rectangle {
                    x: bounds_x + handle_offset - pad.left as f32,
                    y: bounds_y - pad.top as f32,
                    width: handle_width + (pad.left + pad.right) as f32,
                    height: bounds_height + (pad.top + pad.bottom) as f32,
                },
            }
        } else {
            Primitive::Image {
                handle: style.texture,
                bounds: Rectangle {
                    x: bounds_x + handle_offset,
                    y: bounds_y,
                    width: handle_width,
                    height: bounds_height,
                },
            }
        }
    };

    Primitive::Group {
        primitives: vec![
            tick_marks,
            top_rail,
            bottom_rail,
            handle,
            mod_range_line,
        ],
    }
}

fn draw_classic_style(
    normal: Normal,
    rail_y: f32,
    bounds_x: f32,
    bounds_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    tick_marks: Option<&TickMarkGroup>,
    tick_mark_style: &Option<TickMarkStyle>,
    style: &ClassicStyle,
    mod_range_line: Primitive,
) -> Primitive {
    let handle_width = style.handle.width as f32;

    let tick_marks: Primitive = {
        if let Some(tick_marks) = tick_marks {
            if let Some(style) = tick_mark_style {
                draw_tick_marks(
                    rail_y,
                    (bounds_x + (handle_width / 2.0)).floor(),
                    bounds_width - handle_width,
                    bounds_height,
                    tick_marks,
                    &style,
                )
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        }
    };

    let (top_rail_width, bottom_rail_width) = style.rail_widths;
    let (top_rail_color, bottom_rail_color) = style.rail_colors;
    let (top_rail, bottom_rail) = draw_rails(
        rail_y,
        bounds_x,
        bounds_width,
        top_rail_width,
        bottom_rail_width,
        &top_rail_color,
        &bottom_rail_color,
    );

    let handle_border_radius = style.handle.border_radius;

    let handle_offset = normal.scale(bounds_width - handle_width).floor();

    let notch_width = style.handle.notch_width as f32;

    let handle = Primitive::Quad {
        bounds: Rectangle {
            x: bounds_x + handle_offset,
            y: bounds_y,
            width: handle_width,
            height: bounds_height,
        },
        background: Background::Color(style.handle.color),
        border_radius: handle_border_radius,
        border_width: style.handle.border_width,
        border_color: style.handle.border_color,
    };

    let handle_notch: Primitive = {
        if style.handle.notch_width != 0 {
            Primitive::Quad {
                bounds: Rectangle {
                    x: (bounds_x + handle_offset + (handle_width / 2.0)
                        - (notch_width / 2.0))
                        .floor(),
                    y: bounds_y,
                    width: notch_width,
                    height: bounds_height,
                },
                background: Background::Color(style.handle.notch_color),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            }
        } else {
            Primitive::None
        }
    };

    Primitive::Group {
        primitives: vec![
            tick_marks,
            top_rail,
            bottom_rail,
            handle,
            handle_notch,
            mod_range_line,
        ],
    }
}

fn draw_rect_style(
    normal: Normal,
    rail_y: f32,
    bounds_x: f32,
    bounds_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    tick_marks: Option<&TickMarkGroup>,
    tick_mark_style: &Option<TickMarkStyle>,
    style: &RectStyle,
    mod_range_line: Primitive,
) -> Primitive {
    let handle_width = style.handle_width as f32;

    let tick_marks: Primitive = {
        if let Some(tick_marks) = tick_marks {
            if let Some(style) = tick_mark_style {
                draw_tick_marks(
                    rail_y,
                    (bounds_x + (handle_width / 2.0)).floor(),
                    bounds_width - handle_width,
                    bounds_height,
                    tick_marks,
                    &style,
                )
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        }
    };

    let empty_rect = Primitive::Quad {
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

    let border_width = style.back_border_width as f32;
    let twice_border_width = border_width * 2.0;

    let handle_offset = normal
        .scale(bounds_width - twice_border_width - handle_width)
        .floor();

    let filled_rect = Primitive::Quad {
        bounds: Rectangle {
            x: bounds_x,
            y: bounds_y,
            width: handle_offset + twice_border_width
                - style.handle_filled_gap as f32,
            height: bounds_height,
        },
        background: Background::Color(style.filled_color),
        border_radius: style.back_border_radius,
        border_width: style.back_border_width,
        border_color: Color::TRANSPARENT,
    };

    let handle = Primitive::Quad {
        bounds: Rectangle {
            x: bounds_x + handle_offset,
            y: bounds_y,
            width: handle_width + twice_border_width,
            height: bounds_height,
        },
        background: Background::Color(style.handle_color),
        border_radius: style.back_border_radius,
        border_width: style.back_border_width,
        border_color: Color::TRANSPARENT,
    };

    Primitive::Group {
        primitives: vec![
            empty_rect,
            tick_marks,
            filled_rect,
            mod_range_line,
            handle,
        ],
    }
}

fn draw_rect_bipolar_style(
    normal: Normal,
    rail_y: f32,
    bounds_x: f32,
    bounds_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    tick_marks: Option<&TickMarkGroup>,
    tick_mark_style: &Option<TickMarkStyle>,
    style: &RectBipolarStyle,
    mod_range_line: Primitive,
) -> Primitive {
    let handle_width = style.handle_width as f32;

    let tick_marks: Primitive = {
        if let Some(tick_marks) = tick_marks {
            if let Some(style) = tick_mark_style {
                draw_tick_marks(
                    rail_y,
                    (bounds_x + (handle_width / 2.0)).floor(),
                    bounds_width - handle_width,
                    bounds_height,
                    tick_marks,
                    &style,
                )
            } else {
                Primitive::None
            }
        } else {
            Primitive::None
        }
    };

    let border_width = style.back_border_width as f32;
    let twice_border_width = border_width * 2.0;

    let empty_rect = Primitive::Quad {
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

    let half_bounds_width = (bounds_width / 2.0).floor();

    let handle_offset = normal
        .scale(bounds_width - twice_border_width - handle_width)
        .floor();

    if normal.value() > 0.499 && normal.value() < 0.501 {
        let handle = Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x + handle_offset,
                y: bounds_y,
                width: handle_width + twice_border_width,
                height: bounds_height,
            },
            background: Background::Color(style.handle_center_color),
            border_radius: style.back_border_radius,
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        };

        Primitive::Group {
            primitives: vec![empty_rect, tick_marks, mod_range_line, handle],
        }
    } else if normal.value() < 0.5 {
        let filled_rect_offset =
            handle_offset + handle_width + style.handle_filled_gap as f32;

        let filled_rect = Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x + filled_rect_offset,
                y: bounds_y,
                width: half_bounds_width - filled_rect_offset
                    + twice_border_width,
                height: bounds_height,
            },
            background: Background::Color(style.left_filled_color),
            border_radius: style.back_border_radius,
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        };

        let handle = Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x + handle_offset,
                y: bounds_y,
                width: handle_width + twice_border_width,
                height: bounds_height,
            },
            background: Background::Color(style.handle_left_color),
            border_radius: style.back_border_radius,
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        };

        Primitive::Group {
            primitives: vec![
                empty_rect,
                tick_marks,
                filled_rect,
                mod_range_line,
                handle,
            ],
        }
    } else {
        let filled_rect_offset = half_bounds_width;
        let filled_rect = Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x + filled_rect_offset,
                y: bounds_y,
                width: handle_offset - filled_rect_offset + twice_border_width
                    - style.handle_filled_gap as f32,
                height: bounds_height,
            },
            background: Background::Color(style.right_filled_color),
            border_radius: style.back_border_radius,
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        };

        let handle = Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x + handle_offset,
                y: bounds_y,
                width: handle_width + twice_border_width,
                height: bounds_height,
            },
            background: Background::Color(style.handle_right_color),
            border_radius: style.back_border_radius,
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        };

        Primitive::Group {
            primitives: vec![
                empty_rect,
                tick_marks,
                filled_rect,
                mod_range_line,
                handle,
            ],
        }
    }
}

fn draw_rails(
    rail_y: f32,
    bounds_x: f32,
    bounds_width: f32,
    top_rail_width: u16,
    bottom_rail_width: u16,
    top_rail_color: &Color,
    bottom_rail_color: &Color,
) -> (Primitive, Primitive) {
    let top_rail_width = top_rail_width as f32;
    let bottom_rail_width = bottom_rail_width as f32;
    let full_rail_width = top_rail_width + bottom_rail_width;
    let half_full_rail_width = (full_rail_width / 2.0).floor();

    (
        Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x,
                y: rail_y - half_full_rail_width,
                width: bounds_width,
                height: top_rail_width,
            },
            background: Background::Color(*top_rail_color),
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        },
        Primitive::Quad {
            bounds: Rectangle {
                x: bounds_x,
                y: rail_y - half_full_rail_width + top_rail_width,
                width: bounds_width,
                height: bottom_rail_width,
            },
            background: Background::Color(*bottom_rail_color),
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        },
    )
}

fn draw_tick_mark_tier_merged(
    primitives: &mut Vec<Primitive>,
    tick_mark_positions: &Vec<Normal>,
    width: f32,
    length_scale: f32,
    color: &Color,
    bounds_x: f32,
    rail_y: f32,
    bounds_width: f32,
    bounds_height: f32,
) {
    let length = (length_scale * bounds_height).floor();
    let color = Background::Color(*color);
    let start_x = bounds_x - (width / 2.0);
    let y = (rail_y - (length / 2.0)).floor();

    for position in tick_mark_positions.iter() {
        let x = (start_x + position.scale(bounds_width)).floor();

        primitives.push(Primitive::Quad {
            bounds: Rectangle {
                x,
                y,
                width,
                height: length,
            },
            background: color,
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        });
    }
}

fn draw_tick_mark_tier(
    primitives: &mut Vec<Primitive>,
    tick_mark_positions: &Vec<Normal>,
    width: f32,
    length_scale: f32,
    color: &Color,
    bounds_x: f32,
    rail_y: f32,
    bounds_width: f32,
    bounds_height: f32,
    center_offset: f32,
) {
    let length = (length_scale * bounds_height).floor();
    let half_length = (length / 2.0).floor();
    let color = Background::Color(*color);
    let start_x = bounds_x - (width / 2.0);

    let top_y = rail_y - center_offset - half_length;
    let bottom_y = rail_y + center_offset;

    for position in tick_mark_positions.iter() {
        let x = (start_x + position.scale(bounds_width)).floor();

        primitives.push(Primitive::Quad {
            bounds: Rectangle {
                x,
                y: top_y,
                width: width,
                height: half_length,
            },
            background: color,
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        });

        primitives.push(Primitive::Quad {
            bounds: Rectangle {
                x,
                y: bottom_y,
                width: width,
                height: half_length,
            },
            background: color,
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        });
    }
}

fn draw_tick_marks(
    rail_y: f32,
    bounds_x: f32,
    bounds_width: f32,
    bounds_height: f32,
    tick_marks: &TickMarkGroup,
    style: &TickMarkStyle,
) -> Primitive {
    let mut primitives: Vec<Primitive> = Vec::new();

    if style.center_offset == 0 {
        primitives.reserve_exact(tick_marks.len());

        if tick_marks.has_tier_1() {
            draw_tick_mark_tier_merged(
                &mut primitives,
                &tick_marks.tier_1_positions(),
                style.width_tier_1 as f32,
                style.length_scale_tier_1,
                &style.color_tier_1,
                bounds_x,
                rail_y,
                bounds_width,
                bounds_height,
            );
        }
        if tick_marks.has_tier_2() {
            draw_tick_mark_tier_merged(
                &mut primitives,
                &tick_marks.tier_2_positions(),
                style.width_tier_2 as f32,
                style.length_scale_tier_2,
                &style.color_tier_2,
                bounds_x,
                rail_y,
                bounds_width,
                bounds_height,
            );
        }
        if tick_marks.has_tier_3() {
            draw_tick_mark_tier_merged(
                &mut primitives,
                &tick_marks.tier_3_positions(),
                style.width_tier_3 as f32,
                style.length_scale_tier_3,
                &style.color_tier_3,
                bounds_x,
                rail_y,
                bounds_width,
                bounds_height,
            );
        }
    } else {
        primitives.reserve_exact(tick_marks.len() * 2);

        let center_offset = style.center_offset as f32;

        if tick_marks.has_tier_1() {
            draw_tick_mark_tier(
                &mut primitives,
                &tick_marks.tier_1_positions(),
                style.width_tier_1 as f32,
                style.length_scale_tier_1,
                &style.color_tier_1,
                bounds_x,
                rail_y,
                bounds_width,
                bounds_height,
                center_offset,
            );
        }
        if tick_marks.has_tier_2() {
            draw_tick_mark_tier(
                &mut primitives,
                &tick_marks.tier_2_positions(),
                style.width_tier_2 as f32,
                style.length_scale_tier_2,
                &style.color_tier_2,
                bounds_x,
                rail_y,
                bounds_width,
                bounds_height,
                center_offset,
            );
        }
        if tick_marks.has_tier_3() {
            draw_tick_mark_tier(
                &mut primitives,
                &tick_marks.tier_3_positions(),
                style.width_tier_3 as f32,
                style.length_scale_tier_3,
                &style.color_tier_3,
                bounds_x,
                rail_y,
                bounds_width,
                bounds_height,
                center_offset,
            );
        }
    }

    Primitive::Group { primitives }
}
