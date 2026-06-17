use crate::{
    style::text_marks::{Align, Appearance, Placement},
    text_marks::Group,
};
use iced::{
    Pixels, Point, Rectangle, Renderer, Size,
    advanced::{Text, text::Renderer as _},
    alignment::Vertical,
    widget::text::{Alignment, LineHeight, Shaping, Wrapping},
};

fn draw_aligned(
    renderer: &mut Renderer,
    bounds: &Rectangle,
    x: f32,
    text_marks: &Group,
    style: &Appearance,
    inverse: bool,
    align: Alignment,
) {
    let color = style.color;
    let font = style.font;
    let text_size = f32::from(style.text_size);
    let text_bounds_width = f32::from(style.bounds_width);
    let text_bounds_height = f32::from(style.bounds_height);

    if inverse {
        for text_mark in &text_marks.group {
            let y = (bounds.y + (text_mark.0.scale(bounds.height))).round();

            renderer.fill_text(
                Text {
                    content: text_mark.1.clone(),
                    size: Pixels(text_size),
                    bounds: Size {
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    align_x: align,
                    align_y: Vertical::Center,
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
    } else {
        for text_mark in &text_marks.group {
            let y = (bounds.y + (text_mark.0.scale_inv(bounds.height))).round();

            renderer.fill_text(
                Text {
                    content: text_mark.1.clone(),
                    size: Pixels(text_size),
                    bounds: Size {
                        width: text_bounds_width,
                        height: text_bounds_height,
                    },
                    align_x: align,
                    align_y: Vertical::Center,
                    line_height: LineHeight::default(),
                    wrapping: Wrapping::default(),
                    shaping: Shaping::Basic,
                    font,
                },
                Point { x, y },
                color,
                // TODO: What is this?
                Rectangle {
                    x,
                    y,
                    width: text_bounds_width,
                    height: text_bounds_height,
                },
            );
        }
    }
}

/// Draws text marks on a vertical axis.
///
/// * bounds - The bounds of the widget to place the text marks in/outside of.
/// * text_marks - The group of text marks.
/// * style - The text marks style.
/// * placement - The placement of the text marks relative to the bounds.
/// * inverse - Whether to inverse the positions of the text marks (true) or
///   not (false).
pub fn draw_vertical_text_marks(
    renderer: &mut Renderer,
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
                    bounds.x,
                    text_marks,
                    style,
                    inverse,
                    Alignment::Left,
                );
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.x + bounds.width,
                    text_marks,
                    style,
                    inverse,
                    Alignment::Right,
                );
            } else {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.x,
                    text_marks,
                    style,
                    inverse,
                    Alignment::Right,
                );
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.x + bounds.width,
                    text_marks,
                    style,
                    inverse,
                    Alignment::Left,
                );
            }
        }
        Placement::LeftOrTop { inside, offset } => {
            let bounds = offset.offset_rect(bounds);

            if *inside {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.x,
                    text_marks,
                    style,
                    inverse,
                    Alignment::Left,
                );
            } else {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.x,
                    text_marks,
                    style,
                    inverse,
                    Alignment::Right,
                );
            }
        }
        Placement::RightOrBottom { inside, offset } => {
            let bounds = offset.offset_rect(bounds);

            if *inside {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.x + bounds.width,
                    text_marks,
                    style,
                    inverse,
                    Alignment::Right,
                );
            } else {
                draw_aligned(
                    renderer,
                    &bounds,
                    bounds.x + bounds.width,
                    text_marks,
                    style,
                    inverse,
                    Alignment::Left,
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
                        bounds.center_x(),
                        text_marks,
                        style,
                        inverse,
                        Alignment::Left,
                    );
                }
                Align::End => {
                    draw_aligned(
                        renderer,
                        &bounds,
                        bounds.center_x(),
                        text_marks,
                        style,
                        inverse,
                        Alignment::Right,
                    );
                }
                Align::Center => {
                    draw_aligned(
                        renderer,
                        &bounds,
                        bounds.center_x(),
                        text_marks,
                        style,
                        inverse,
                        Alignment::Center,
                    );
                }
            }
        }
    };
}
