use crate::{
    style::text_marks::{Align, Appearance, Placement},
    text_marks::Group,
};
use iced_core::{
    Pixels, Point, Rectangle, Size, Text,
    alignment::Vertical,
    widget::text::{Alignment, LineHeight, Shaping, Wrapping},
};

fn draw_aligned<R: iced_core::text::Renderer<Font = iced_core::Font>>(
    renderer: &mut R,
    bounds: &Rectangle,
    y: f32,
    text_marks: &Group,
    style: &Appearance,
    inverse: bool,
    align: Vertical,
) {
    let color = style.color;
    let font = style.font;
    let text_size = f32::from(style.text_size);
    let text_bounds_width = f32::from(style.bounds_width);
    let text_bounds_height = f32::from(style.bounds_height);

    if inverse {
        for text_mark in &text_marks.group {
            let x = (bounds.x + (text_mark.0.scale_inv(bounds.width))).round();

            renderer.fill_text(
                Text {
                    content: text_mark.1.clone(),
                    size: Pixels(text_size),
                    bounds: Size {
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    align_x: Alignment::Center,
                    align_y: align,
                    line_height: LineHeight::default(),
                    wrapping: Wrapping::default(),
                    shaping: Shaping::Basic,
                    font,
                },
                Point { x, y },
                color,
                Rectangle {
                    x,
                    y,
                    width: text_bounds_width,
                    height: text_bounds_height,
                },
            );
        }
    } else {
        for text_mark in &text_marks.group {
            let x = (bounds.x + (text_mark.0.scale(bounds.width))).round();
            renderer.fill_text(
                Text {
                    content: text_mark.1.clone(),
                    size: Pixels(text_size),
                    bounds: Size {
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    align_x: Alignment::Center,
                    align_y: align,
                    line_height: LineHeight::default(),
                    wrapping: Wrapping::default(),
                    shaping: Shaping::Basic,
                    font,
                },
                Point { x, y },
                color,
                // TODO: What is this?
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 1000.0,
                    height: 1000.0,
                },
            );
        }
    }
}

/// Draws text marks on a horizontal axis.
///
/// * `bounds` - The bounds of the widget to place the text marks in/outside of.
/// * `text_marks` - The group of text marks.
/// * `style` - The text marks style.
/// * `placement` - The placement of the text marks relative to the bounds.
/// * `inverse` - Whether to inverse the positions of the text marks (true) or
///   not (false).
pub fn draw_horizontal_text_marks<R: iced_core::text::Renderer<Font = iced_core::Font>>(
    renderer: &mut R,
    bounds: &Rectangle,
    text_marks: &Group,
    style: &Appearance,
    placement: &Placement,
    inverse: bool,
    //cache: &PrimitiveCache,
) {
    match placement {
        Placement::BothSides { inside, offset } => {
            let bounds = offset.offset_rect(bounds);

            if *inside {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.y,
                    text_marks,
                    style,
                    inverse,
                    Vertical::Top,
                );
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.y + bounds.height,
                    text_marks,
                    style,
                    inverse,
                    Vertical::Bottom,
                );
            } else {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.y,
                    text_marks,
                    style,
                    inverse,
                    Vertical::Bottom,
                );
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.y + bounds.height,
                    text_marks,
                    style,
                    inverse,
                    Vertical::Top,
                );
            }
        }
        Placement::LeftOrTop { inside, offset } => {
            let bounds = offset.offset_rect(bounds);

            if *inside {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.y,
                    text_marks,
                    style,
                    inverse,
                    Vertical::Top,
                );
            } else {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.y,
                    text_marks,
                    style,
                    inverse,
                    Vertical::Bottom,
                );
            }
        }
        Placement::RightOrBottom { inside, offset } => {
            let bounds = offset.offset_rect(bounds);

            if *inside {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.y + bounds.height,
                    text_marks,
                    style,
                    inverse,
                    Vertical::Bottom,
                );
            } else {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.y + bounds.height,
                    text_marks,
                    style,
                    inverse,
                    Vertical::Top,
                );
            }
        }
        Placement::Center { align, offset } => {
            let bounds = offset.offset_rect(bounds);

            match align {
                Align::Start => {
                    draw_aligned(
                        renderer,
                        &bounds,
                        bounds.center_y(),
                        text_marks,
                        style,
                        inverse,
                        Vertical::Top,
                    );
                }
                Align::End => {
                    draw_aligned(
                        renderer,
                        &bounds,
                        bounds.center_y(),
                        text_marks,
                        style,
                        inverse,
                        Vertical::Bottom,
                    );
                }
                Align::Center => {
                    draw_aligned(
                        renderer,
                        &bounds,
                        bounds.center_y(),
                        text_marks,
                        style,
                        inverse,
                        Vertical::Center,
                    );
                }
            }
        }
    };
}
