use crate::{
    ModulationRange, Normal,
    core::{text_marks, tick_marks},
    style::v_slider::{
        ClassicAppearance, ClassicRail, ModRangeAppearance, ModRangePlacement, RectAppearance,
        RectBipolarAppearance, TextMarksAppearance, TextureAppearance, TickMarksAppearance,
    },
    widget::v_slider::ValueMarkers,
};
use iced_core::{Border, Color, Rectangle, Shadow, border::Radius, renderer::Quad};

fn markers<R: iced_core::Renderer + iced_core::text::Renderer<Font = iced_core::Font>>(
    renderer: &mut R,
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

fn tick_marks<R: iced_core::Renderer>(
    renderer: &mut R,
    bounds: &Rectangle,
    tick_marks: Option<&tick_marks::Group>,
    tick_marks_style: &Option<TickMarksAppearance>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
) {
    if let Some(tick_marks) = tick_marks
        && let Some(style) = tick_marks_style
    {
        tick_marks::draw_vertical_tick_marks(
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

fn text_marks<R: iced_core::Renderer + iced_core::text::Renderer<Font = iced_core::Font>>(
    renderer: &mut R,
    bounds: &Rectangle,
    text_marks: Option<&text_marks::Group>,
    text_marks_style: &Option<TextMarksAppearance>,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    if let Some(text_marks) = text_marks
        && let Some(style) = text_marks_style
    {
        text_marks::draw_vertical_text_marks(
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

fn modulation<R: iced_core::Renderer>(
    renderer: &mut R,
    bounds: &Rectangle,
    mod_range: Option<&ModulationRange>,
    style: &Option<ModRangeAppearance>,
) {
    if let Some(mod_range) = mod_range
        && let Some(style) = style
    {
        let (x, width) = match style.placement {
            ModRangePlacement::Center { width, offset } => {
                (bounds.x + offset + ((bounds.width - width) / 2.0), width)
            }
            ModRangePlacement::CenterFilled { edge_padding } => {
                (bounds.x + edge_padding, bounds.width - (edge_padding * 2.0))
            }
            ModRangePlacement::Left { width, offset } => (bounds.x + offset - width, width),
            ModRangePlacement::Right { width, offset } => (bounds.x + bounds.width + offset, width),
        };

        if let Some(back_color) = style.back_color {
            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x,
                        y: bounds.y,
                        width,
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
                back_color,
            );
        }

        if mod_range.filled_visible && (mod_range.start.as_f32() != mod_range.end.as_f32()) {
            let (start, end, color) = if mod_range.start.as_f32() > mod_range.end.as_f32() {
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

            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x,
                        y: bounds.y + start_offset,
                        width,
                        height: filled_height,
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

pub fn texture_style<
    R: iced_core::Renderer
        + iced_core::image::Renderer<Handle = iced_core::image::Handle>
        + iced_core::text::Renderer<Font = iced_core::Font>,
>(
    renderer: &mut R,
    normal: Normal,
    bounds: &Rectangle,
    style: TextureAppearance,
    value_markers: &ValueMarkers<'_>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (f32::from(style.handle_height) / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - f32::from(style.handle_height),
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
        x: (bounds.center_x() + style.image_bounds.x).round(),
        y: (value_bounds.y + style.image_bounds.y + normal.scale_inv(value_bounds.height)).round(),
        width: style.image_bounds.width,
        height: style.image_bounds.height,
    };

    renderer.draw_image(iced_core::Image::from(&style.image_handle), bounds, bounds)
}

pub fn classic_style<R: iced_core::Renderer + iced_core::text::Renderer<Font = iced_core::Font>>(
    renderer: &mut R,
    normal: Normal,
    bounds: &Rectangle,
    style: &ClassicAppearance,
    value_markers: &ValueMarkers<'_>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    let handle_height = f32::from(style.handle.height);

    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (handle_height / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - handle_height,
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

    let handle_offset = normal.scale_inv(value_bounds.height).round();
    let notch_width = style.handle.notch_width;

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y + handle_offset,
                width: bounds.width,
                height: handle_height,
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
                    x: bounds.x,
                    y: (bounds.y + handle_offset + (handle_height / 2.0) - (notch_width / 2.0))
                        .round(),
                    width: bounds.width,
                    height: notch_width,
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

pub fn rect_style<R: iced_core::Renderer + iced_core::text::Renderer<Font = iced_core::Font>>(
    renderer: &mut R,
    normal: Normal,
    bounds: &Rectangle,
    style: &RectAppearance,
    value_markers: &ValueMarkers<'_>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    let handle_height = f32::from(style.handle_height);
    let border_width = style.back_border_width;
    let twice_border_width = border_width * 2.0;

    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (handle_height / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - handle_height,
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
        .scale_inv(value_bounds.height - twice_border_width)
        .round();

    let filled_offset = handle_offset + handle_height + style.handle_filled_gap;

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y + filled_offset,
                width: bounds.width,
                height: bounds.height - filled_offset,
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
                x: bounds.x,
                y: bounds.y + handle_offset,
                width: bounds.width,
                height: handle_height + twice_border_width,
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

pub fn rect_bipolar_style<
    R: iced_core::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
>(
    renderer: &mut R,
    normal: Normal,
    bounds: &Rectangle,
    style: &RectBipolarAppearance,
    value_markers: &ValueMarkers<'_>,
    //tick_marks_cache: &tick_marks::PrimitiveCache,
    //text_marks_cache: &text_marks::PrimitiveCache,
) {
    let handle_height = f32::from(style.handle_height);
    let border_width = style.back_border_width;
    let twice_border_width = border_width * 2.0;

    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (handle_height / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - handle_height,
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
        .scale_inv(value_bounds.height - twice_border_width)
        .round();

    if normal.as_f32() > 0.5 {
        let filled_rect_offset = handle_offset + handle_height + style.handle_filled_gap;

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.y + filled_rect_offset,
                    width: bounds.width,
                    height: ((bounds.height / 2.0) - filled_rect_offset + twice_border_width)
                        .round(),
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: style.back_border_width,
                    radius: Radius::new(style.back_border_radius),
                },
                shadow: Shadow::default(),
                snap: false,
            },
            style.top_filled_color,
        );
    } else {
        let filled_rect_offset = (bounds.height / 2.0).round() - border_width;
        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.y + filled_rect_offset,
                    width: bounds.width,
                    height: handle_offset - filled_rect_offset + twice_border_width
                        - style.handle_filled_gap,
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: style.back_border_width,
                    radius: Radius::new(style.back_border_radius),
                },
                shadow: Shadow::default(),
                snap: false,
            },
            style.bottom_filled_color,
        );
    };

    let handle_color = if normal.as_f32() > 0.499 && normal.as_f32() < 0.501 {
        style.handle_center_color
    } else if normal.as_f32() > 0.5 {
        style.handle_top_color
    } else {
        style.handle_bottom_color
    };

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y + handle_offset,
                width: bounds.width,
                height: handle_height + twice_border_width,
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

fn classic_rail<R: iced_core::Renderer>(renderer: &mut R, bounds: &Rectangle, style: &ClassicRail) {
    let (left_width, right_width) = style.rail_widths;
    let (left_color, right_color) = style.rail_colors;

    let full_width = left_width + right_width;

    let start_x = (bounds.x + ((bounds.width - full_width) / 2.0)).round();

    let y = bounds.y + style.rail_padding;
    let height = bounds.height - (style.rail_padding * 2.0);

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: start_x,
                y,
                width: left_width,
                height,
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(0.0),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        left_color,
    );

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: start_x + left_width,
                y,
                width: right_width,
                height,
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(0.0),
            },
            shadow: Shadow::default(),
            snap: false,
        },
        right_color,
    );
}
