//! Display an interactive vertical slider that controls a [`Param`]
//!
//! [`Param`]: ../core/param/trait.Param.html

use crate::core::{ModulationRange, Normal};
use crate::graphics::{text_marks, tick_marks};
use crate::native::v_slider;
use iced_graphics::Primitive;
use iced_native::{Background, Color, Point, Rectangle};

pub use crate::style::v_slider::{
    Appearance, ClassicHandle, ClassicRail, ClassicStyle, ModRangePlacement,
    ModRangeStyle, RectBipolarStyle, RectStyle, StyleSheet, TextMarksStyle,
    TextureStyle, TickMarksStyle,
};

struct ValueMarkers<'a> {
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
    tick_marks_style: Option<TickMarksStyle>,
    text_marks_style: Option<TextMarksStyle>,
    mod_range_style_1: Option<ModRangeStyle>,
    mod_range_style_2: Option<ModRangeStyle>,
}

/// A vertical slider GUI widget that controls a [`Param`]
///
/// a [`VSlider`] will try to fill the vertical space of its container.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`VSlider`]: struct.VSlider.html
pub type VSlider<'a, Message, Theme> =
    v_slider::VSlider<'a, Message, iced::Renderer<Theme>>;

impl v_slider::Renderer for iced::Renderer {
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
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

        let appearance = if is_dragging {
            style_sheet.dragging(style)
        } else if is_mouse_over {
            style_sheet.hovered(style)
        } else {
            style_sheet.active(style)
        };

        let bounds = Rectangle {
            x: bounds.x.round(),
            y: bounds.y.round(),
            width: bounds.width.round(),
            height: bounds.height.round(),
        };

        let value_markers = ValueMarkers {
            tick_marks,
            text_marks,
            mod_range_1,
            mod_range_2,
            tick_marks_style: style_sheet.tick_marks_style(style),
            text_marks_style: style_sheet.text_marks_style(style),
            mod_range_style_1: style_sheet.mod_range_style(style),
            mod_range_style_2: style_sheet.mod_range_style_2(style),
        };

        let primitives = match appearance {
            Appearance::Texture(style) => draw_texture_style(
                normal,
                &bounds,
                style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
            Appearance::Classic(style) => draw_classic_style(
                normal,
                &bounds,
                &style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
            Appearance::Rect(style) => draw_rect_style(
                normal,
                &bounds,
                &style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
            Appearance::RectBipolar(style) => draw_rect_bipolar_style(
                normal,
                &bounds,
                &style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
        };

        self.draw_primitive(primitives)
    }
}

fn draw_value_markers<'a>(
    mark_bounds: &Rectangle,
    mod_bounds: &Rectangle,
    value_markers: &ValueMarkers<'a>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> (Primitive, Primitive, Primitive, Primitive) {
    (
        draw_tick_marks(
            mark_bounds,
            value_markers.tick_marks,
            &value_markers.tick_marks_style,
            tick_marks_cache,
        ),
        draw_text_marks(
            mark_bounds,
            value_markers.text_marks,
            &value_markers.text_marks_style,
            text_marks_cache,
        ),
        draw_mod_range(
            mod_bounds,
            value_markers.mod_range_1,
            &value_markers.mod_range_style_1,
        ),
        draw_mod_range(
            mod_bounds,
            value_markers.mod_range_2,
            &value_markers.mod_range_style_2,
        ),
    )
}

fn draw_tick_marks(
    bounds: &Rectangle,
    tick_marks: Option<&tick_marks::Group>,
    tick_marks_style: &Option<TickMarksStyle>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
) -> Primitive {
    if let Some(tick_marks) = tick_marks {
        if let Some(style) = tick_marks_style {
            tick_marks::draw_vertical_tick_marks(
                bounds,
                tick_marks,
                &style.style,
                &style.placement,
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
    bounds: &Rectangle,
    text_marks: Option<&text_marks::Group>,
    text_marks_style: &Option<TextMarksStyle>,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> Primitive {
    if let Some(text_marks) = text_marks {
        if let Some(style) = text_marks_style {
            text_marks::draw_vertical_text_marks(
                bounds,
                text_marks,
                &style.style,
                &style.placement,
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

fn draw_mod_range(
    bounds: &Rectangle,
    mod_range: Option<&ModulationRange>,
    style: &Option<ModRangeStyle>,
) -> Primitive {
    if let Some(mod_range) = mod_range {
        if let Some(style) = style {
            let (x, width) = match style.placement {
                ModRangePlacement::Center { width, offset } => (
                    bounds.x
                        + f32::from(offset)
                        + ((bounds.width - f32::from(width)) / 2.0),
                    f32::from(width),
                ),
                ModRangePlacement::CenterFilled { edge_padding } => (
                    bounds.x + f32::from(edge_padding),
                    bounds.width - (f32::from(edge_padding) * 2.0),
                ),
                ModRangePlacement::Left { width, offset } => (
                    bounds.x + f32::from(offset) - f32::from(width),
                    f32::from(width),
                ),
                ModRangePlacement::Right { width, offset } => (
                    bounds.x + bounds.width + f32::from(offset),
                    f32::from(width),
                ),
            };

            let back: Primitive = if let Some(back_color) = style.back_color {
                Primitive::Quad {
                    bounds: Rectangle {
                        x,
                        y: bounds.y,
                        width,
                        height: bounds.height,
                    },
                    background: Background::Color(back_color),
                    border_radius: style.back_border_radius,
                    border_width: style.back_border_width,
                    border_color: style.back_border_color,
                }
            } else {
                Primitive::None
            };

            let filled: Primitive = {
                if mod_range.filled_visible
                    && (mod_range.start.as_f32() != mod_range.end.as_f32())
                {
                    let (start, end, color) =
                        if mod_range.start.as_f32() > mod_range.end.as_f32() {
                            (
                                mod_range.start.as_f32_inv(),
                                mod_range.end.as_f32_inv(),
                                style.filled_color,
                            )
                        } else {
                            (
                                mod_range.end.as_f32_inv(),
                                mod_range.start.as_f32_inv(),
                                style.filled_inverse_color,
                            )
                        };

                    let start_offset = bounds.height * start;
                    let filled_height = (bounds.height * end) - start_offset;

                    Primitive::Quad {
                        bounds: Rectangle {
                            x,
                            y: bounds.y + start_offset,
                            width,
                            height: filled_height,
                        },
                        background: Background::Color(color),
                        border_radius: style.back_border_radius,
                        border_width: style.back_border_width,
                        border_color: Color::TRANSPARENT,
                    }
                } else {
                    Primitive::None
                }
            };

            Primitive::Group {
                primitives: vec![back, filled],
            }
        } else {
            Primitive::None
        }
    } else {
        Primitive::None
    }
}

fn draw_texture_style<'a>(
    normal: Normal,
    bounds: &Rectangle,
    style: TextureStyle,
    value_markers: &ValueMarkers<'a>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> Primitive {
    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (f32::from(style.handle_height) / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - f32::from(style.handle_height),
    };

    let (tick_marks, text_marks, mod_range_1, mod_range_2) = draw_value_markers(
        &value_bounds,
        &value_bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    let (left_rail, right_rail) = draw_classic_rail(&bounds, &style.rail);

    let handle = Primitive::Image {
        handle: style.image_handle,
        bounds: Rectangle {
            x: (bounds.center_x() + style.image_bounds.x).round(),
            y: (value_bounds.y
                + style.image_bounds.y
                + normal.scale_inv(value_bounds.height))
            .round(),
            width: style.image_bounds.width,
            height: style.image_bounds.height,
        },
    };

    Primitive::Group {
        primitives: vec![
            tick_marks,
            text_marks,
            left_rail,
            right_rail,
            handle,
            mod_range_1,
            mod_range_2,
        ],
    }
}

fn draw_classic_style<'a>(
    normal: Normal,
    bounds: &Rectangle,
    style: &ClassicStyle,
    value_markers: &ValueMarkers<'a>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> Primitive {
    let handle_height = f32::from(style.handle.height);

    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (handle_height / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - handle_height,
    };

    let (tick_marks, text_marks, mod_range_1, mod_range_2) = draw_value_markers(
        &value_bounds,
        &value_bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    let (left_rail, right_rail) = draw_classic_rail(&bounds, &style.rail);

    let handle_border_radius = style.handle.border_radius;
    let handle_offset = normal.scale_inv(value_bounds.height).round();
    let notch_width = f32::from(style.handle.notch_width);

    let handle = Primitive::Quad {
        bounds: Rectangle {
            x: bounds.x,
            y: bounds.y + handle_offset,
            width: bounds.width,
            height: handle_height,
        },
        background: Background::Color(style.handle.color),
        border_radius: handle_border_radius,
        border_width: style.handle.border_width,
        border_color: style.handle.border_color,
    };

    let handle_notch: Primitive = if style.handle.notch_width != 0.0 {
        Primitive::Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: (bounds.y + handle_offset + (handle_height / 2.0)
                    - (notch_width / 2.0))
                    .round(),
                width: bounds.width,
                height: notch_width,
            },
            background: Background::Color(style.handle.notch_color),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    } else {
        Primitive::None
    };

    Primitive::Group {
        primitives: vec![
            tick_marks,
            text_marks,
            left_rail,
            right_rail,
            handle,
            handle_notch,
            mod_range_1,
            mod_range_2,
        ],
    }
}

fn draw_rect_style<'a>(
    normal: Normal,
    bounds: &Rectangle,
    style: &RectStyle,
    value_markers: &ValueMarkers<'a>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> Primitive {
    let handle_height = f32::from(style.handle_height);

    let border_width = f32::from(style.back_border_width);
    let twice_border_width = border_width * 2.0;

    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (handle_height / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - handle_height,
    };

    let (tick_marks, text_marks, mod_range_1, mod_range_2) = draw_value_markers(
        &value_bounds,
        &bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    let empty_rect = Primitive::Quad {
        bounds: Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: bounds.height,
        },
        background: Background::Color(style.back_color),
        border_radius: style.back_border_radius,
        border_width: style.back_border_width,
        border_color: style.back_border_color,
    };

    let handle_offset = normal
        .scale_inv(value_bounds.height - twice_border_width)
        .round();

    let filled_offset =
        handle_offset + handle_height + f32::from(style.handle_filled_gap);
    let filled_rect = Primitive::Quad {
        bounds: Rectangle {
            x: bounds.x,
            y: bounds.y + filled_offset,
            width: bounds.width,
            height: bounds.height - filled_offset,
        },
        background: Background::Color(style.filled_color),
        border_radius: style.back_border_radius,
        border_width: style.back_border_width,
        border_color: Color::TRANSPARENT,
    };

    let handle = Primitive::Quad {
        bounds: Rectangle {
            x: bounds.x,
            y: bounds.y + handle_offset,
            width: bounds.width,
            height: handle_height + twice_border_width,
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
            text_marks,
            filled_rect,
            handle,
            mod_range_1,
            mod_range_2,
        ],
    }
}

fn draw_rect_bipolar_style<'a>(
    normal: Normal,
    bounds: &Rectangle,
    style: &RectBipolarStyle,
    value_markers: &ValueMarkers<'a>,
    tick_marks_cache: &tick_marks::PrimitiveCache,
    text_marks_cache: &text_marks::PrimitiveCache,
) -> Primitive {
    let handle_height = f32::from(style.handle_height);

    let border_width = f32::from(style.back_border_width);
    let twice_border_width = border_width * 2.0;

    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (handle_height / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - handle_height,
    };

    let (tick_marks, text_marks, mod_range_1, mod_range_2) = draw_value_markers(
        &value_bounds,
        &bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    let empty_rect = Primitive::Quad {
        bounds: Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: bounds.height,
        },
        background: Background::Color(style.back_color),
        border_radius: style.back_border_radius,
        border_width: style.back_border_width,
        border_color: style.back_border_color,
    };

    let handle_offset = normal
        .scale_inv(value_bounds.height - twice_border_width)
        .round();

    let (handle_color, filled_rect) = if normal.as_f32() > 0.499
        && normal.as_f32() < 0.501
    {
        (style.handle_center_color, Primitive::None)
    } else if normal.as_f32() > 0.5 {
        let filled_rect_offset =
            handle_offset + handle_height + f32::from(style.handle_filled_gap);
        (
            style.handle_top_color,
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.y + filled_rect_offset,
                    width: bounds.width,
                    height: ((bounds.height / 2.0) - filled_rect_offset
                        + twice_border_width)
                        .round(),
                },
                background: Background::Color(style.top_filled_color),
                border_radius: style.back_border_radius,
                border_width: style.back_border_width,
                border_color: Color::TRANSPARENT,
            },
        )
    } else {
        let filled_rect_offset = (bounds.height / 2.0).round() - border_width;
        (
            style.handle_bottom_color,
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.y + filled_rect_offset,
                    width: bounds.width,
                    height: handle_offset - filled_rect_offset
                        + twice_border_width
                        - f32::from(style.handle_filled_gap),
                },
                background: Background::Color(style.bottom_filled_color),
                border_radius: style.back_border_radius,
                border_width: style.back_border_width,
                border_color: Color::TRANSPARENT,
            },
        )
    };

    let handle = Primitive::Quad {
        bounds: Rectangle {
            x: bounds.x,
            y: bounds.y + handle_offset,
            width: bounds.width,
            height: handle_height + twice_border_width,
        },
        background: Background::Color(handle_color),
        border_radius: style.back_border_radius,
        border_width: style.back_border_width,
        border_color: Color::TRANSPARENT,
    };

    Primitive::Group {
        primitives: vec![
            empty_rect,
            tick_marks,
            text_marks,
            filled_rect,
            handle,
            mod_range_1,
            mod_range_2,
        ],
    }
}

fn draw_classic_rail(
    bounds: &Rectangle,
    style: &ClassicRail,
) -> (Primitive, Primitive) {
    let (left_width, right_width) = style.rail_widths;
    let (left_color, right_color) = style.rail_colors;

    let left_width = f32::from(left_width);
    let right_width = f32::from(right_width);

    let full_width = left_width + right_width;

    let start_x = (bounds.x + ((bounds.width - full_width) / 2.0)).round();

    let y = bounds.y + f32::from(style.rail_padding);
    let height = bounds.height - (f32::from(style.rail_padding) * 2.0);

    (
        Primitive::Quad {
            bounds: Rectangle {
                x: start_x,
                y,
                width: left_width,
                height,
            },
            background: Background::Color(left_color),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        Primitive::Quad {
            bounds: Rectangle {
                x: start_x + left_width,
                y,
                width: right_width,
                height,
            },
            background: Background::Color(right_color),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    )
}
