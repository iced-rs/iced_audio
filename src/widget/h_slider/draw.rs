use iced::{
    Border, Color, Rectangle, Renderer, Shadow,
    advanced::{Renderer as _, renderer::Quad},
    border::Radius,
};

use crate::{
    ModulationRange, Normal,
    core::{text_marks, tick_marks},
    style::h_slider::{
        ClassicAppearance, ClassicRail, ModRangeAppearance, ModRangePlacement, RectAppearance,
        RectBipolarAppearance, TextMarksAppearance, TickMarksAppearance,
    },
    widget::h_slider::ValueMarkers,
};

#[cfg(feature = "texture")]
use crate::style::h_slider::TextureAppearance;

fn markers(
    renderer: &mut Renderer,
    mark_bounds: &Rectangle,
    mod_bounds: &Rectangle,
    value_markers: &ValueMarkers<'_>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    tick_marks(
        renderer,
        mark_bounds,
        value_markers.tick_marks,
        &value_markers.tick_marks_style,
        //tick_marks_cache,
    );
    text_marks(
        renderer,
        mark_bounds,
        value_markers.text_marks,
        &value_markers.text_marks_style,
        //text_marks_cache,
    );

    modulation(
        renderer,
        mod_bounds,
        value_markers.mod_range_1,
        &value_markers.mod_range_style_1,
    );
    modulation(
        renderer,
        mod_bounds,
        value_markers.mod_range_2,
        &value_markers.mod_range_style_2,
    );
}

fn tick_marks(
    renderer: &mut Renderer,
    bounds: &Rectangle,
    tick_marks: Option<&tick_marks::Group>,
    tick_marks_style: &Option<TickMarksAppearance>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
) {
    if let Some(tick_marks) = tick_marks {
        if let Some(style) = tick_marks_style {
            tick_marks::draw_horizontal_tick_marks(
                renderer,
                bounds,
                tick_marks,
                &style.style,
                &style.placement,
                false,
                //tick_marks_cache,
            )
        }
    }
}

fn text_marks(
    renderer: &mut Renderer,
    bounds: &Rectangle,
    text_marks: Option<&text_marks::Group>,
    text_marks_style: &Option<TextMarksAppearance>,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    if let Some(text_marks) = text_marks {
        if let Some(style) = text_marks_style {
            text_marks::draw_horizontal_text_marks(
                renderer,
                bounds,
                text_marks,
                &style.style,
                &style.placement,
                false,
                //text_marks_cache,
            )
        }
    }
}

fn modulation(
    renderer: &mut Renderer,
    bounds: &Rectangle,
    mod_range: Option<&ModulationRange>,
    style: &Option<ModRangeAppearance>,
) {
    if let Some(mod_range) = mod_range {
        if let Some(style) = style {
            let (y, height) = match style.placement {
                ModRangePlacement::Center { height, offset } => {
                    (bounds.y + offset + ((bounds.height - height) / 2.0), height)
                }
                ModRangePlacement::CenterFilled { edge_padding } => (
                    bounds.y + edge_padding,
                    bounds.height - (edge_padding * 2.0),
                ),
                ModRangePlacement::Top { height, offset } => (bounds.y + offset - height, height),
                ModRangePlacement::Bottom { height, offset } => {
                    (bounds.y + bounds.height + offset, height)
                }
            };

            if let Some(back_color) = style.back_color {
                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: bounds.x,
                            y,
                            width: bounds.width,
                            height,
                        },
                        border: Border {
                            color: style.back_border_color,
                            width: style.back_border_width,
                            radius: Radius::new(style.back_border_radius),
                        },
                        shadow: Shadow::default(),
                        snap: false,
                    },
                    back_color,
                );
            };

            if mod_range.filled_visible && (mod_range.start.as_f32() != mod_range.end.as_f32()) {
                let (start, end, color) = if mod_range.start.as_f32() < mod_range.end.as_f32() {
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

                let start_offset = bounds.width * start;
                let filled_width = (bounds.width * end) - start_offset;

                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: bounds.x + start_offset,
                            y,
                            width: filled_width,
                            height,
                        },
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: style.back_border_width,
                            radius: Radius::new(style.back_border_radius),
                        },
                        shadow: Shadow::default(),
                        snap: false,
                    },
                    color,
                );
            }
        }
    }
}

#[cfg(feature = "texture")]
pub fn texture_style(
    renderer: &mut Renderer,
    normal: Normal,
    bounds: &Rectangle,
    style: TextureAppearance,
    value_markers: &ValueMarkers<'_>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    use iced::advanced::image::Renderer;

    let value_bounds = Rectangle {
        x: (bounds.x + (f32::from(style.handle_width) / 2.0)).round(),
        y: bounds.y,
        width: bounds.width - f32::from(style.handle_width),
        height: bounds.height,
    };

    markers(
        renderer,
        &value_bounds,
        &value_bounds,
        value_markers,
        //tick_marks_cache,
        //text_marks_cache,
    );

    classic_rail(renderer, bounds, &style.rail);

    let bounds = Rectangle {
        x: (value_bounds.x + style.image_bounds.x + normal.scale(value_bounds.width)).round(),
        y: (bounds.center_y() + style.image_bounds.y).round(),
        width: style.image_bounds.width,
        height: style.image_bounds.height,
    };

    renderer.draw_image(
        iced::advanced::image::Image::from(&style.image_handle),
        bounds,
        bounds,
    );
}

pub fn classic_style(
    renderer: &mut Renderer,
    normal: Normal,
    bounds: &Rectangle,
    style: &ClassicAppearance,
    value_markers: &ValueMarkers<'_>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    let handle_width = f32::from(style.handle.width);

    let value_bounds = Rectangle {
        x: (bounds.x + (handle_width / 2.0)).round(),
        y: bounds.y,
        width: bounds.width - handle_width,
        height: bounds.height,
    };

    markers(
        renderer,
        &value_bounds,
        &value_bounds,
        value_markers,
        //tick_marks_cache,
        //text_marks_cache,
    );

    classic_rail(renderer, bounds, &style.rail);

    let handle_offset = normal.scale(value_bounds.width).round();
    let notch_width = style.handle.notch_width;

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset,
                y: bounds.y,
                width: handle_width,
                height: bounds.height,
            },
            border: Border {
                color: style.handle.border_color,
                width: style.handle.border_width,
                radius: Radius::new(style.handle.border_radius),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        style.handle.color,
    );

    if style.handle.notch_width != 0.0 {
        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: (bounds.x + handle_offset + (handle_width / 2.0) - (notch_width / 2.0))
                        .round(),
                    y: bounds.y,
                    width: notch_width,
                    height: bounds.height,
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::new(0.0),
                },
                shadow: Shadow::default(),
                snap: false,
            },
            style.handle.notch_color,
        );
    }
}

pub fn rect_style(
    renderer: &mut Renderer,
    normal: Normal,
    bounds: &Rectangle,
    style: &RectAppearance,
    value_markers: &ValueMarkers<'_>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    let handle_width = f32::from(style.handle_width);
    let border_width = style.back_border_width;
    let twice_border_width = border_width * 2.0;

    let value_bounds = Rectangle {
        x: (bounds.x + (handle_width / 2.0)).round(),
        y: bounds.y,
        width: bounds.width - handle_width,
        height: bounds.height,
    };

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: bounds.width,
                height: bounds.height,
            },
            border: Border {
                color: style.back_border_color,
                width: style.back_border_width,
                radius: Radius::new(style.back_border_radius),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        style.back_color,
    );

    let handle_offset = normal
        .scale(value_bounds.width - twice_border_width)
        .round();

    let filled_offset = handle_offset + handle_width + style.handle_filled_gap;

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: filled_offset,
                height: bounds.height,
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: style.back_border_width,
                radius: Radius::new(style.back_border_radius),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        style.filled_color,
    );

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset,
                y: bounds.y,
                width: handle_width + twice_border_width,
                height: bounds.height,
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: style.back_border_width,
                radius: Radius::new(style.back_border_radius),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        style.handle_color,
    );

    markers(
        renderer,
        &value_bounds,
        bounds,
        value_markers,
        //tick_marks_cache,
        //text_marks_cache,
    );
}

pub fn rect_bipolar_style(
    renderer: &mut Renderer,
    normal: Normal,
    bounds: &Rectangle,
    style: &RectBipolarAppearance,
    value_markers: &ValueMarkers<'_>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    let handle_width = f32::from(style.handle_width);
    let border_width = style.back_border_width;
    let twice_border_width = border_width * 2.0;

    let value_bounds = Rectangle {
        x: (bounds.x + (handle_width / 2.0)).round(),
        y: bounds.y,
        width: bounds.width - handle_width,
        height: bounds.height,
    };

    markers(
        renderer,
        &value_bounds,
        bounds,
        value_markers,
        //tick_marks_cache,
        //text_marks_cache,
    );

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: bounds.width,
                height: bounds.height,
            },
            border: Border {
                color: style.back_border_color,
                width: style.back_border_width,
                radius: Radius::new(style.back_border_radius),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        style.back_color,
    );

    let handle_offset = normal
        .scale(value_bounds.width - twice_border_width)
        .round();

    if normal.as_f32() < 0.5 {
        let filled_rect_offset = handle_offset + handle_width + style.handle_filled_gap;
        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: bounds.x + filled_rect_offset,
                    y: bounds.y,
                    width: ((bounds.width / 2.0) - filled_rect_offset + twice_border_width).round(),
                    height: bounds.height,
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: style.back_border_width,
                    radius: Radius::new(style.back_border_radius),
                },
                shadow: Shadow::default(),
                snap: false,
            },
            style.left_filled_color,
        );
    } else {
        let filled_rect_offset = (bounds.width / 2.0).round() - border_width;
        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: bounds.x + filled_rect_offset,
                    y: bounds.y,
                    width: handle_offset - filled_rect_offset + twice_border_width
                        - style.handle_filled_gap,
                    height: bounds.height,
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: style.back_border_width,
                    radius: Radius::new(style.back_border_radius),
                },
                shadow: Shadow::default(),
                snap: false,
            },
            style.right_filled_color,
        );
    };

    let handle_color = if normal.as_f32() > 0.499 && normal.as_f32() < 0.501 {
        style.handle_center_color
    } else if normal.as_f32() < 0.5 {
        style.handle_left_color
    } else {
        style.handle_right_color
    };

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset,
                y: bounds.y,
                width: handle_width + twice_border_width,
                height: bounds.height,
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: style.back_border_width,
                radius: Radius::new(style.back_border_radius),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        handle_color,
    );
}

fn classic_rail(renderer: &mut Renderer, bounds: &Rectangle, style: &ClassicRail) {
    let (top_width, bottom_width) = style.rail_widths;
    let (top_color, bottom_color) = style.rail_colors;

    let full_width = top_width + bottom_width;

    let x = bounds.x + style.rail_padding;
    let width = bounds.width - (style.rail_padding * 2.0);

    let start_y = (bounds.y + ((bounds.height - full_width) / 2.0)).round();

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x,
                y: start_y,
                width,
                height: top_width,
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(0.0),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        top_color,
    );

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x,
                y: start_y + top_width,
                width,
                height: bottom_width,
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(0.0),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        bottom_color,
    );
}
